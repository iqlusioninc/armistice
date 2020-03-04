//! Sample USB client
//!
//! `usbc some message` will send the command line arguments to the device using
//! bulk transfers. The program will then block wait for response bulk
//! transfers (one for each transfer sent) and then prints their contents to the
//! console, one line per response transfer.

use std::{str, time::Duration};

use anyhow::bail;
use rusb::{DeviceHandle, Direction, GlobalContext, TransferType};

/// Initialize USB, send a bulk message, and receive a response
pub fn run(args: &[&str]) -> Result<Vec<String>, anyhow::Error> {
    if args.is_empty() {
        bail!("expected at least one argument")
    }

    let mut bulk = BulkPair::open(consts::VID, consts::PID)?;
    let mut results = vec![];

    for msg in args {
        bulk.write(msg.as_bytes())?;
        let mut buf = vec![0; bulk.in_max_packet_size().into()];
        let resp = bulk.read(&mut buf)?;

        if let Ok(s) = str::from_utf8(resp) {
            results.push(s.to_owned());
        } else {
            eprintln!("{:?}", resp);
        }
    }

    Ok(results)
}

fn default_timeout() -> Duration {
    2 * consts::frame()
}

/// IN/OUT bulk endpoint pair
struct BulkPair {
    in_ep_addr: u8,
    in_max_packet_size: u16,

    out_ep_addr: u8,
    out_max_packet_size: u16,

    handle: DeviceHandle<GlobalContext>,
    timeout: Duration,
}

impl BulkPair {
    /// Opens the USB device identified with `vid` and `pid` and claims the
    /// interface that has exactly one IN/OUT bulk endpoint pair
    pub fn open(vid: u16, pid: u16) -> Result<Self, anyhow::Error> {
        for dev in rusb::devices()?.iter() {
            let desc = dev.device_descriptor()?;

            if desc.vendor_id() == vid && desc.product_id() == pid {
                for i in 0..desc.num_configurations() {
                    let config_desc = dev.config_descriptor(i)?;

                    for iface in config_desc.interfaces() {
                        'iface: for iface_desc in iface.descriptors() {
                            let mut in_ep = None;
                            let mut out_ep = None;
                            let mut seen = 0;

                            for ep_desc in iface_desc.endpoint_descriptors() {
                                if ep_desc.transfer_type() == TransferType::Bulk {
                                    seen += 1;
                                    let addr = ep_desc.address();
                                    let max_packet_size = ep_desc.max_packet_size();
                                    if ep_desc.direction() == Direction::In {
                                        in_ep = Some((addr, max_packet_size));
                                    } else {
                                        out_ep = Some((addr, max_packet_size));
                                    };
                                }
                            }

                            if let (
                                Some((in_ep_addr, in_max_packet_size)),
                                Some((out_ep_addr, out_max_packet_size)),
                            ) = (in_ep, out_ep)
                            {
                                if seen != 2 {
                                    // more than one IN/OUT endpoint pair; try the next interface
                                    continue 'iface;
                                }

                                let mut handle = dev.open()?;
                                handle.set_auto_detach_kernel_driver(true).or_else(|err| {
                                    if err == rusb::Error::NotSupported {
                                        Ok(())
                                    } else {
                                        Err(err)
                                    }
                                })?;

                                let iface_nr = iface_desc.interface_number();
                                let active_config = handle.active_configuration()?;
                                let config_nr = config_desc.number();
                                if config_nr != active_config {
                                    handle.set_active_configuration(config_nr)?;
                                }

                                handle.claim_interface(iface_nr)?;

                                handle
                                    .set_alternate_setting(iface_nr, iface_desc.setting_number())?;

                                return Ok(BulkPair {
                                    handle,
                                    in_ep_addr,
                                    in_max_packet_size,
                                    out_ep_addr,
                                    out_max_packet_size,
                                    timeout: default_timeout(),
                                });
                            }
                        }
                    }
                }

                bail!("found matching device but not a matching interface");
            }
        }

        bail!("USB device {:04x}:{:04x} not found", vid, pid);
    }

    /// Reads data from the IN endpoint
    pub fn read<'b>(&mut self, buf: &'b mut [u8]) -> Result<&'b [u8], anyhow::Error> {
        let n = self.handle.read_bulk(self.in_ep_addr, buf, self.timeout)?;
        Ok(&buf[..n])
    }

    /// Returns the max packet size of the IN endpoint
    #[allow(dead_code)]
    pub fn in_max_packet_size(&self) -> u16 {
        self.in_max_packet_size
    }

    /// Writes data into the OUT endpoint
    ///
    /// *NOTE* The length of the `bytes` argument cannot exceed the endpoint
    /// maximum packet size
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), anyhow::Error> {
        // XXX alternatively we could split `bytes` in `out_max_packet_size`
        // chunks
        assert!(
            bytes.len() < self.out_max_packet_size.into(),
            "the length of `write` argument cannot exceed the max_packet_size ({} bytes) ",
            self.out_max_packet_size
        );

        self.handle
            .write_bulk(self.out_ep_addr, bytes, self.timeout)?;
        Ok(())
    }

    /// Sets a timeout for bulk transfer
    ///
    /// The default is twice the interval between USB frames, so 2 milliseconds
    #[allow(dead_code)]
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}
