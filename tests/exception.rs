#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::redundant_pub_crate
    )
)]
mod common;

use libdigidocpp::{Container, DigiDocError, Error, LibraryError};
use serial_test::serial;
use std::error::Error as StdError;

#[test]
#[serial]
fn open_errors_carry_the_reason_as_a_cause() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;

    for (name, reason) in [
        ("dot.asice", "invalid path"),
        ("asice-relative.asice", "invalid path"),
        ("pt-empty.asice", "full-path is empty"),
    ] {
        let result = Container::open(&common::fixture_path(name));
        let Err(Error::Library(LibraryError::DigiDoc(error))) = result else {
            panic!("Expected DigiDoc error");
        };

        let expected_start = "digidoc error: General: Failed to parse manifest";
        let error_message = error.to_string();
        assert!(
            error_message.starts_with(expected_start),
            "{name}: Expected error message to start with \"{expected_start}\", but instead was \"{error_message}\""
        );
        let [cause] = error.causes() else {
            panic!("{name}: expected one DigiDoc cause");
        };
        assert!(cause.to_string().contains(reason), "{name}: {cause}");
        assert!(!cause.cpp_file().is_empty());
        assert!(cause.cpp_line() > 0);
    }
    Ok(())
}

#[test]
#[serial]
fn error_and_source_report_details_separately() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;

    let result = Container::open(&common::fixture_path("dot.asice"));
    let Err(Error::Library(LibraryError::DigiDoc(digidoc_error))) = result else {
        panic!("Expected DigiDoc error");
    };

    assert!(
        digidoc_error
            .to_string()
            .contains("Failed to parse manifest")
    );
    assert!(!digidoc_error.to_string().contains("invalid path"));
    assert!(!digidoc_error.cpp_file().is_empty());
    assert!(digidoc_error.cpp_line() > 0);

    let source = digidoc_error
        .source()
        .expect("Expected error to have source")
        .downcast_ref::<DigiDocError>()
        .expect("Expected digidoc error as source");
    assert!(source.to_string().contains("invalid path"));
    assert!(source.source().is_none());
    assert!(!source.cpp_file().is_empty());
    assert!(source.cpp_line() > 0);
    assert_ne!(source.message(), digidoc_error.message());

    Ok(())
}
