//! Armistice for USB armory MkII
//!
//! Implemented as a `cortex-a-rtfm` application

#![no_main]
#![no_std]
#![deny(warnings, rust_2018_idioms, unused_qualifications)]
#![forbid(unsafe_code)]

use exception_reset as _; // default exception handler
use heapless::{
    pool,
    pool::singleton::{Box, Pool},
};
use panic_serial as _; // panic handler
use usb_device::{
    bus::{InterfaceNumber, UsbBus, UsbBusAllocator},
    class::UsbClass,
    descriptor::DescriptorWriter,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
    endpoint::{EndpointAddress, EndpointIn, EndpointOut},
};
use usbarmory::{memlog, serial::Serial, usbd::Usbd};

/// Max packet size for bulk transfers to/from High-Speed USB devices
const MAX_PACKET_SIZE: u16 = 512;

// Memory pool used for bulk packets
pool!(P: [u8; MAX_PACKET_SIZE as usize]);

#[rtfm::app()]
const APP: () = {
    struct Resources {
        bulk_class: BulkClass<'static, Usbd>,
        dev: UsbDevice<'static, Usbd>,
        serial: Serial,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        // enough memory for 2 bulk packets
        static mut MEMORY: [u8; (2 * MAX_PACKET_SIZE + 64) as usize] =
            [0; (2 * MAX_PACKET_SIZE + 64) as usize];
        static mut ALLOCATOR: Option<UsbBusAllocator<Usbd>> = None;

        // the pool will manage this memory
        P::grow(MEMORY);

        let serial = Serial::take().expect("Serial");
        let usbd = Usbd::take().expect("Usbd");

        let allocator = ALLOCATOR.get_or_insert(UsbBusAllocator::new(usbd));
        let mut bulk_class = BulkClass::new(allocator);
        let mut dev = UsbDeviceBuilder::new(allocator, UsbVidPid(consts::VID, consts::PID))
            .self_powered(true)
            .max_packet_size_0(64)
            .product("Ferris")
            .manufacturer("Rustaceans Inc")
            .build();

        // this first call to `poll` triggers an attach event
        dev.poll(&mut [&mut bulk_class]);

        init::LateResources {
            serial,
            dev,
            bulk_class,
        }
    }

    // background task that logs data to the serial port
    #[idle(resources = [serial])]
    fn idle(cx: idle::Context) -> ! {
        let serial = cx.resources.serial;

        loop {
            usbarmory::memlog_try_flush();

            if serial.try_read().is_some() {
                usbarmory::memlog_flush_and_reset!();
            }
        }
    }

    // main USB logic: handles enumeration, the control endpoint 0, etc
    #[task(
    binds = USB_OTG1,
    priority = 2,
    resources = [dev, bulk_class],
    spawn = [process_packet],
    )]
    fn usb(cx: usb::Context) {
        let dev = cx.resources.dev;
        let bulk_class = cx.resources.bulk_class;

        if dev.poll(&mut [bulk_class]) {
            // new `bulk_class` event
            // owned pointer equivalent to `alloc::boxed::Box<[u8; N]>`
            let mut buf: Box<P> = P::alloc().expect("OOM").freeze();

            if let Ok(n) = bulk_class.ep_bulk_out.read(&mut *buf) {
                cx.spawn.process_packet(buf, n).ok().expect("OOM");
            } else {
                // WouldBlock; try again later
                // `drop` returns memory to the pool
                // this will happen automatically but we are explicit for the
                // sake of the example
                drop(buf);
            }
        }
    }

    // transmit bulk packets
    #[task(priority = 2, resources = [bulk_class])]
    fn usb_tx(cx: usb_tx::Context, packet: Box<P>, len: usize) {
        let bulk_class = cx.resources.bulk_class;

        bulk_class
            .ep_bulk_in
            .write(&packet[..len])
            .expect("I/O error?");
    }

    // lower priority task that performs work based on the content of the packet
    #[task(priority = 1, spawn =[usb_tx])]
    fn process_packet(cx: process_packet::Context, mut packet: Box<P>, len: usize) {
        packet[..len].reverse();

        cx.spawn.usb_tx(packet, len).ok().expect("OOM");
    }
};

pub struct BulkClass<'a, B>
where
    B: UsbBus,
{
    iface: InterfaceNumber,
    ep_bulk_in: EndpointIn<'a, B>,
    ep_bulk_out: EndpointOut<'a, B>,
}

impl<'b, B> BulkClass<'b, B>
where
    B: UsbBus,
{
    fn new(alloc: &'b UsbBusAllocator<B>) -> Self {
        Self {
            iface: alloc.interface(),
            ep_bulk_in: alloc.bulk(MAX_PACKET_SIZE),
            ep_bulk_out: alloc.bulk(MAX_PACKET_SIZE),
        }
    }
}

impl<B> UsbClass<B> for BulkClass<'_, B>
where
    B: UsbBus,
{
    fn get_configuration_descriptors(
        &self,
        writer: &mut DescriptorWriter<'_>,
    ) -> usb_device::Result<()> {
        writer.interface(self.iface, 0xff, 0x00, 0x00)?;
        writer.endpoint(&self.ep_bulk_in)?;
        writer.endpoint(&self.ep_bulk_out)?;

        Ok(())
    }

    fn endpoint_out(&mut self, addr: EndpointAddress) {
        memlog!("endpoint_out(addr={:?})", addr);
    }
}
