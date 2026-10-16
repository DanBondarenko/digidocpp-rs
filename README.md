# digidocpp

Create and validate eIDAS digital signatures in ASiC-E/XAdES containers.

This crate provides safe Rust bindings for
[libdigidocpp](https://www.id.ee/en/article/digidoc-libraries-c-library-libdigidocpp/), the C++
library for creating and validating digital signatures maintained by the Estonian Information System
Authority.

Main workflows:
- Open, create, and save containers.
- Create and validate digital signatures.
- Extend the validity of signed containers.
- Add, extract and stream data files.

Low-level bindings to libdigidocpp are provided in the
[libdigidocpp-sys](https://crates.io/crates/libdigidocpp-sys) crate.

Unofficial. Not affiliated with the Estonian Information System Authority (RIA).

## Code examples

### Reading a container

Inspect the signatures and files of a container. Check the validity of signatures. Write contained
files to a target of your choosing.

```rust,no_run
use digidocpp::Container;

fn main() -> Result<(), digidocpp::Error> {
    digidocpp::initialize()?;
    let container = Container::open("path/to/document.asice")?;

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

### Creating and signing a container

Digitally sign a file with an ID card through PKCS #11. With the default profile, signing contacts
time-stamp and OCSP services.

```rust,no_run
use digidocpp::Container;
use digidocpp::signer::Pkcs11Signer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    digidocpp::initialize()?;

    let mut container = Container::create("contract.asice")?;
    container.add_data_file("path/to/contract.pdf", "application/pdf")?;
    let mut signer = Pkcs11Signer::new()?;

    signer.set_pin(std::env::var("PIN2")?)?;
    container.sign(&mut signer)?;
    container.save()?;

    Ok(())
}
```

## Installation

```sh
cargo add digidocpp
```

### Requires:

- libdigidocpp 4.5.0 or newer, with headers. It is linked dynamically, not bundled.
- A C++20 compiler.
- Rust 1.88 or newer.

### Running your program

The loader must find libdigidocpp at start-up. If it cannot:

- Linux and macOS: emit a rpath from your `build.rs`. You may use a helper
  (`cargo add --build libdigidocpp-build`).

  ```rust,ignore
  // build.rs
  fn main() -> Result<(), Box<dyn std::error::Error>> {
      libdigidocpp_build::Install::discover()?.emit_rpath()?;
      Ok(())
  }
  ```

- Windows: add the DLL directory to `PATH`, or place the DLL next to the executable.

## Documentation

See [docs.rs](https://docs.rs/digidocpp) for documentation. The repository also contains functional
[examples](https://github.com/DanBondarenko/digidocpp-rs/tree/main/examples) demonstrating common
use cases with test data. The upstream library and API are documented in the
[libdigidocpp manual](https://open-eid.github.io/libdigidocpp/).

Source and issue tracker: [digidocpp-rs](https://github.com/DanBondarenko/digidocpp-rs).

## License

Licensed under either of
[Apache License, Version 2.0](https://github.com/DanBondarenko/digidocpp-rs/blob/main/LICENSE-APACHE)
or [MIT license](https://github.com/DanBondarenko/digidocpp-rs/blob/main/LICENSE-MIT) at your
option.

[libdigidocpp](https://github.com/open-eid/libdigidocpp) itself is licensed under
[LGPL 2.1](https://github.com/open-eid/libdigidocpp/blob/master/LICENSE.LGPL).
