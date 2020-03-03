# Armistice

There is no convenient way to run security sensitive parts of software isolated from general purpose software. In response, we built **Armistice**.

**Armistice** is a programmable platform for running security sensitive applications and provides access to the cryptographic keys stored on the device. It is implemented as a "bare metal" (i.e. `no_std`) Rust application, providing functionality similar to a Hardware Security Module (HSM) or Trusted Platform Module (TPM). Armistice aims to provide superior developer ergonomics, making it particularly appealing for implementing advanced cryptography and novel application specific authorization logic.

Armistice is designed to fill a gap in the tools available to developers for running sensitive software applications securely. Currently, the targets for secret management hardware are devices with difficult C based API's like *trousers* with limited ability to isolate the secret handling software from general purpose computation.

We've seen the limitations of current options while developing the architecture of Iqlusion's various validator offerings. The USB Armory MkII is an excellent hardware platform. Armistice adds to the USB Armory platform an extremely limited attack surface area made possible by safe Rust to existing options like full blown Linux and Tamago.

Armistice is intended to enable the development of secure application that involve *online key storage*. It provides a permanently-online device capable of unattended boot with a fully verified secure boot process (i.e. High Assurance Boot). This is particularly appealing when signing logic is highly automated and an airgap is impractical.

## Use Cases

Armistice is designed to be a flexible platform for securing online
cryptographic keys with the goal of supporting arbitrary
user-defined authorization programs eventually.

- **Application-specific key storage**: support for storing and managing keys for arbitrary applications, including support for user-provided logic to authorize key usage (i.e. "HSM" use cases).

- **Infrastructure security**: traditional use cases like X.509 and SSH
  certificate authorities, package signing keys (e.g. RPM/DEB package),
  host identity and attestation (i.e. "TPM" use cases).
  
  The device will be able to enforce policies like requiring multi-party or multi-factor authorization before a signature is generated for a sensitive namespace or data is decrypted.
  
- **Endpoint key storage**: user's cryptographic keys which
  are kept offline until ready to be used, e.g. code signing keys, cryptocurrency "wallet" keys, zero trust remote access credentials.

## Organization

The Armistice source code repository is organized into the following
submodules:

- `core`: Armistice application code, implemented as a "heapless" `no_std`
  Rust application. Includes all cryptographic access control functionality.
- `client`: host-side client for communicating with Armistice core, including
  terminating an authenticated end-to-end encrypted channel for all
  communication. Contains pluggable support for accessing it in a variety of
  different ways, including a direct USB connection, via a network proxy to a
  USB device, or directly embedding Armistice Core into a host-side Rust
  application (useful for prototyping/CI)
- `schema`: message schema client/core use to communicate with each other
- `usbarmory`: Armistice Core embedded in a bare metal Rust application
  (developed using Realtime for the Masses - framework for realtime embedded
  Rust applications)
  
## Future directions
  
We are committed to keeping the core of Armistice open source and open hardware. We are hoping to build a robust developer community around the platform. We are exploring options for productizing this design to fill an open niche in the security tool ecosystem.
