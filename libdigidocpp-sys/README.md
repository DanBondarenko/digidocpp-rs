# libdigidocpp-sys

Low-level Rust bindings to [libdigidocpp](https://github.com/open-eid/libdigidocpp), the C++ engine
behind [DigiDoc4](https://github.com/open-eid/DigiDoc4-Client), the Estonian state's e-signature
client. Built with [cxx](https://cxx.rs).

Most users want the safe [`digidocpp`](https://crates.io/crates/digidocpp) crate instead.

Covers the main libdigidocpp workflows:
- Open, create, and save containers.
- Create and validate signatures.
- Add, extract and stream data files.

Unofficial. Not affiliated with the Estonian Information System Authority (RIA).

## Install

```sh
cargo add libdigidocpp-sys
```

Requires:

- libdigidocpp 4.5.0 or newer, with headers.
- A C++20 compiler.
- Rust 1.88 or newer.


libdigidocpp is linked dynamically. It is not bundled, and there is no `vendored` or `static`
option. See [Installation](https://docs.rs/digidocpp/latest/digidocpp/#installation) docs of
`digidocpp` for how the build finds the library and how a program finds it at run time. The same
rules apply to this crate.

## Usage

```rust,no_run
use libdigidocpp_sys::{container, init, signature};

fn main() -> Result<(), cxx::Exception> {
    init::ffi::initialize_default()?;

    let container = container::ffi::open("path/to/document.asice")?;
    let first_signature = container::ffi::signature_at(&container, 0)?;
    let is_valid = signature::ffi::validate(first_signature).is_ok();

    println!("{is_valid}");
    Ok(())
}
```

## Documentation

See [docs.rs](https://docs.rs/libdigidocpp-sys) for the layout of the bindings and the usage
rules.

Source and issue tracker: [digidocpp-rs](https://github.com/DanBondarenko/digidocpp-rs).

## License

Licensed under either of
[Apache License, Version 2.0](https://github.com/DanBondarenko/digidocpp-rs/blob/main/LICENSE-APACHE)
or [MIT license](https://github.com/DanBondarenko/digidocpp-rs/blob/main/LICENSE-MIT) at your
option.

[libdigidocpp](https://github.com/open-eid/libdigidocpp) itself is licensed under
[LGPL 2.1](https://github.com/open-eid/libdigidocpp/blob/master/LICENSE.LGPL).
