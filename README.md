# ggpo-sys

[![Crates.io](https://img.shields.io/crates/v/ggpo-sys.svg)](https://crates.io/crates/ggpo-sys)
[![Docs.rs](https://docs.rs/ggpo-sys/badge.svg)](https://docs.rs/ggpo-sys)

ggpo-sys provides raw, unsafe bindings to [GGPO](https://ggpo.net), a library for rollback
networking.

## Usage

To use ggpo-sys, simply add it to your `Cargo.toml`:

```toml
[dependencies]
ggpo-sys = "^0.1.0"
```

For more details, refer to the source comments in
[ggponet.h](https://github.com/pond3r/ggpo/blob/master/src/include/ggponet.h).

## License

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
