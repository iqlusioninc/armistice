# Armistice for USB armory MkII <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][msrv-image]
[![Gitter Chat][gitter-image]][gitter-link]

Support package for running the [Armistice Core] cryptographic key storage
application on [USB armory Mk II devices] from [F-Secure].

<img src="https://storage.googleapis.com/iqlusion-production-web/github/usbarmory/usbarmory-mkII.png" alt="USB armory mkII" width="375" height="175">

## Minimum Supported Rust Version

- Rust **1.42**

## Security Warning

No security audits of this crate have ever been performed. Presently it is in
an experimental stage and may still contain high-severity issues.

USE AT YOUR OWN RISK!

## Status

This project is an incomplete work-in-progress in an early developmental
stage and will not be ready to use for some time.

## Contributing

If you are interested in contributing to this repository, please make sure to
read the [CONTRIBUTING.md] and [CODE_OF_CONDUCT.md] files first.

## License

Copyright © 2020 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[build-image]: https://github.com/iqlusioninc/armistice/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/iqlusioninc/armistice/actions
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/armistice/blob/develop/LICENSE
[msrv-image]: https://img.shields.io/badge/rustc-1.42+-red.svg
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[Armistice Core]: https://github.com/iqlusioninc/armistice/tree/develop/core
[USB armory Mk II devices]: https://github.com/f-secure-foundry/usbarmory/wiki
[F-Secure]: https://foundry.f-secure.com/
[CONTRIBUTING.md]: https://github.com/iqlusioninc/armistice/blob/develop/CONTRIBUTING.md
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/armistice/blob/develop/CODE_OF_CONDUCT.md
