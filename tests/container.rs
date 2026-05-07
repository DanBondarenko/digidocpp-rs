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

use crate::common::{ContainerExpectation, assert_general_digidoc_err};
use libdigidocpp::{Container, Error};
use serial_test::serial;
use std::error::Error as StdError;

#[test]
#[serial]
fn opens_asice_container_with_expired_certificates() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICE)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("test.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn opens_asics_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICS)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("test.asics"));
    container_expectation.assert(&container?)
}

#[test]
#[serial]
fn opens_asics_container_with_metadata() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICS)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("test-meta.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn opens_asics_container_with_invalid_timestamp_token() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICS)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("test-invalidts.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn opens_asics_container_with_zero_size_contents() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICS)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("test-tera-empty.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn opens_asics_container_with_large_contents() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICS)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("test-tera.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn opens_asice_container_with_subfolder_contents() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICE)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("asice-path.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn opens_asice_container_with_forged_signature() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICE)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("forged_lt.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn opens_asice_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICE)
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("tsl.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn fails_opening_invalid_containers() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;

    for (name, defect) in [
        ("test-invalid.asics", "invalid mimetype"),
        ("asics-subfolder.asics", "subfolder"),
        ("asice-relative.asice", "relative path in contents"),
        ("dot.asice", "content file with a dot name"),
        ("pt-empty.asice", "content file with an empty name"),
    ] {
        let result = Container::open(common::fixture_path(name));
        assert_general_digidoc_err!(result, "{name} ({defect})");
    }
    Ok(())
}

#[test]
#[serial]
fn creates_empty_asice_container() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;
    let container = Container::create("new.asice")?;

    ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICE)
        .build()
        .assert(&container)
}

#[test]
#[serial]
fn open_rejects_non_utf8_path() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;

    let result = Container::open(common::non_utf8_path());
    let Err(Error::Internal(message)) = result else {
        panic!("Expected internal error, but was {result:?}")
    };
    assert!(message.contains("UTF-8"));

    Ok(())
}

#[test]
#[serial]
fn create_rejects_non_utf8_path() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;

    let result = Container::create(common::non_utf8_path());
    let Err(Error::Internal(message)) = result else {
        panic!("Expected internal error, but was {result:?}")
    };
    assert!(message.contains("UTF-8"));

    Ok(())
}
