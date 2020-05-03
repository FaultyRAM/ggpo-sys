# ggpo-sys

[![Crates.io](https://img.shields.io/crates/v/ggpo-sys.svg)](https://crates.io/crates/ggpo-sys)
[![Docs.rs](https://docs.rs/ggpo-sys/badge.svg)](https://docs.rs/ggpo-sys)

ggpo-sys is a Rust library that provides raw, unsafe bindings to [GGPO](https://ggpo.net), a library
for rollback networking.

## Requirements

ggpo-sys depends on the following tools at build time:

* A recent version of stable Rust 🦀
* A C++ toolchain, in order to build the GGPO sources
    * ggpo-sys uses [cc](https://github.com/alexcrichton/cc-rs) to build GGPO. CMake is *not*
      required.
    * 32-bit targets are untested and unsupported. **Use at your own risk.**
    * GGPO only builds under Visual C++ and only for Windows targets. Support for other toolchains
      is therefore blocked pending fixes to upstream.
* libclang
    * Used for generating bindings to ggponet.h.

## Usage

To use ggpo-sys, simply add it to your `Cargo.toml`:

```toml
[dependencies]
ggpo-sys = "^0.1.0"
```

For more details, refer to the source comments in [ggponet.h](libggpo/src/include/ggponet.h).

## License

ggpo-sys is licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Additionally, ggpo-sys bundles a copy of the GGPO source code, which is licensed separately under
the MIT license ([libggpo/LICENSE](libggpo/LICENSE) or http://opensource.org/licenses/MIT).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
