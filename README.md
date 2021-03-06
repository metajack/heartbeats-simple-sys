# Simple Heartbeats Rust Bindings

The `heartbeats-simple-sys` crate provides declarations and linkage for the
`heartbeats-simple` C libraries.
Following the *-sys package conventions, the `heartbeats-simple-sys` crate
does not define higher-level abstractions over the native `heartbeats-simple`
library functions.

This project provides separate crates with bindings for the four
`heartbeats-simple` libraries:

* hbs-sys
* hbs-acc-sys
* hbs-pow-sys
* hbs-acc-pow-sys

The top-level `heartbeats-simple-sys` crate re-exports bindings for all four.

The latest `heartbeats-simple` C libraries can be found at
[https://github.com/libheartbeats/heartbeats-simple](https://github.com/libheartbeats/heartbeats-simple).

## Dependencies

In order to use the `heartbeats-simple-sys` crate, you should have the
`heartbeats-simple` libraries installed to the system where they can be found
by `pkg-config`.

If the libraries are not found, the build process will try to compile them.

## Usage
Add `heartbeats-simple-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
heartbeats-simple-sys = "0.2"
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
