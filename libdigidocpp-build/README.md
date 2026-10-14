# libdigidocpp-build

Build-script helpers that locate a [libdigidocpp](https://github.com/open-eid/libdigidocpp)
installation. They report the header, library and framework directories. They do not compile or
link anything.

Part of [digidocpp-rs](https://github.com/DanBondarenko/digidocpp-rs). Looking for the
bindings? See [`digidocpp`](https://crates.io/crates/digidocpp).

## Install

```sh
cargo add --build libdigidocpp-build
```

Requires:

- Rust 1.88 or newer.

## Usage

```rust,no_run
// build.rs
use libdigidocpp_build::{Install, Kind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = Install::discover()?;

    for dir in install.lib_dirs() {
        let dir = dir.to_str().ok_or("invalid UTF-8")?;
        println!("cargo:rustc-link-search=native={dir}");
    }
    for dir in install.framework_dirs() {
        let dir = dir.to_str().ok_or("invalid UTF-8")?;
        println!("cargo:rustc-link-search=framework={dir}");
    }
    match install.kind() {
        Kind::Framework => println!("cargo:rustc-link-lib=framework=digidocpp"),
        _ => println!("cargo:rustc-link-lib=dylib=digidocpp"),
    }
    install.emit_rpath()?;

    Ok(())
}
```

## Lookup

`Install::discover` searches common install locations by default. The environment variables that
override the search and the platform defaults are in
[Building](https://docs.rs/digidocpp/latest/digidocpp/#building) in the `digidocpp` docs.

## Documentation

See [docs.rs](https://docs.rs/libdigidocpp-build) for the API.

Source and issue tracker: [digidocpp-rs](https://github.com/DanBondarenko/digidocpp-rs).

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
