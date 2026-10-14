# digidocpp

Create, sign, validate and extend ASiC-E containers with XAdES signatures.

Rust bindings for [libdigidocpp](https://github.com/open-eid/libdigidocpp), the C++ engine behind
[DigiDoc4](https://github.com/open-eid/DigiDoc4-Client), the Estonian state's e-signature client.
Main workflows:
- Open, create, and save containers.
- Create and validate signatures.
- Add, extract and stream data files.

Unofficial. Not affiliated with the Estonian Information System Authority (RIA).

## Install

```sh
cargo add digidocpp
```

Requires:

- libdigidocpp 4.5.0 or newer, with headers. It is linked dynamically, not bundled.
- A C++20 compiler.
- Rust 1.88 or newer.

See [Building](https://docs.rs/digidocpp/latest/digidocpp/#building) for how the build finds
the library and how a program finds it at run time.

## Quick start

```rust,no_run
use digidocpp::Container;

fn main() -> Result<(), digidocpp::Error> {
    digidocpp::initialize()?;
    let container = Container::open("document.asice")?;

    for signature in container.signatures()? {
        println!("Signed by: {}", signature.signed_by()?);
        println!("Signed at: {}", signature.trusted_signing_time()?);
        println!("Validity: {:?}", signature.validator_result()?.status());
    }

    for file in container.data_files()? {
        println!("File name: {}", file.file_name()?);
        println!("File size: {}", file.file_size()?);
        let mut contents = Vec::new();
        file.write_to(&mut contents)?;
    }

    Ok(())
}
```

See [docs.rs](https://docs.rs/digidocpp) and
[examples](https://github.com/DanBondarenko/digidocpp-rs/tree/main/examples) for more information.

## Documentation

See [docs.rs](https://docs.rs/digidocpp) for the API. The C++ API is documented in the
[upstream reference](https://open-eid.github.io/libdigidocpp/).

Source and issue tracker: [digidocpp-rs](https://github.com/DanBondarenko/digidocpp-rs).

## License

Licensed under either of
[Apache License, Version 2.0](https://github.com/DanBondarenko/digidocpp-rs/blob/main/LICENSE-APACHE)
or [MIT license](https://github.com/DanBondarenko/digidocpp-rs/blob/main/LICENSE-MIT) at your
option.

[libdigidocpp](https://github.com/open-eid/libdigidocpp) itself is licensed under
[LGPL 2.1](https://github.com/open-eid/libdigidocpp/blob/master/LICENSE.LGPL).
