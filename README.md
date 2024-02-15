# `namigator-rs`

Rust bindings for the [`namigator` pathfinding library for WoW 1.12 through 3.3.5](https://github.com/namreeb/namigator).

Requires a C++ compiler to build. Make sure to check out the `namigator-sys/vendor` submodule with `git submodule update --init --recursive` when building from source.

# Usage

Add to your project with
```bash
cargo add namigator
```

And then [read the docs](https://docs.rs/namigator/latest/namigator).

# Workspace

* `namigator-sys` contains direct bindings to the C++ library.
* `namigator` contains uses the `-sys` crate to create better Rust bindings.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
