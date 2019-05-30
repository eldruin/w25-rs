# Rust Winbond's W25 Serial Flash Memory Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/w25.svg)](https://crates.io/crates/w25)
[![Docs](https://docs.rs/w25/badge.svg)](https://docs.rs/w25)
-->
[![Build Status](https://travis-ci.org/eldruin/w25-rs.svg?branch=master)](https://travis-ci.org/eldruin/w25-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/w25-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/w25-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the W25 serial flash memory devices from Winbond using the [`embedded-hal`] traits.

## The devices

Winbond's W25X and W25Q SpiFlash® Multi-I/O Memories feature the popular Serial Peripheral Interface (SPI), densities from 512K-bit to 512M-bit, small erasable sectors and the industry's highest performance.

Datasheets:
- [W25Q64FW](https://www.winbond.com/resource-files/w25q64fw%20revn%2005182017%20sfdp.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

Please find additional examples using hardware in this repository: [driver-examples].

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
```

## Support

For questions, issues, feature requests like compatibility with similar devices
and other changes, please file an
[issue in the github project](https://github.com/eldruin/w25-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
