# mem-barrier

[![Crates.io](https://img.shields.io/crates/v/mem-barrier)](https://crates.io/crates/mem-barrier)
[![docs.rs](https://img.shields.io/docsrs/mem-barrier)](https://docs.rs/mem-barrier)
[![CI](https://github.com/rust-osdev/mem-barrier/actions/workflows/ci.yml/badge.svg)](https://github.com/rust-osdev/mem-barrier/actions/workflows/ci.yml)

This crate provides cross-architecture, no-std memory barriers.

For API documentation, see the [docs].

[docs]: https://docs.rs/mem-barrier

## Examples

```rust
use mem_barrier::{mem_barrier, BarrierKind, BarrierType};

mem_barrier(BarrierKind::Mmio, BarrierType::General);
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
