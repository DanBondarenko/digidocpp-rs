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

use crate::common::{ContainerExpectation, DataFileExpectation};
use libdigidocpp::{Container, Error, ExceptionCode, LibraryError};
use serial_test::serial;
use std::error::Error as StdError;

fn assert_opening_container_fails(path: impl AsRef<str>) {
    let container = Container::open(path.as_ref());

    let Err(Error::Library(LibraryError::DigiDoc(error))) = container else {
        panic!("opening invalid container should fail");
    };

    assert_eq!(error.code(), Some(ExceptionCode::General));
}

#[test]
#[serial]
fn opens_asice_container_with_expired_certificates() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .media_type(ContainerExpectation::MEDIA_TYPE_ASICE)
        .data_files([DataFileExpectation::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type("file")
            .build()])
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
        .data_files([DataFileExpectation::test1_expectation()])
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
        .data_files([DataFileExpectation::test1_expectation()])
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
        .data_files([DataFileExpectation::builder()
            .id("test2.txt")
            .file_name("test2.txt")
            .file_size(10u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
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
        .data_files([DataFileExpectation::builder()
            .id("emptyFile.ddoc")
            .file_name("emptyFile.ddoc")
            .file_size(0u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
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
        .data_files([DataFileExpectation::builder()
            .id("ddoc_for_testing.ddoc")
            .file_name("ddoc_for_testing.ddoc")
            .file_size(8736u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
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
        .data_files([DataFileExpectation::builder()
            .id("folder/test1.txt")
            .file_name("folder/test1.txt")
            .file_size(4u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
            .build()])
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
        .data_files([DataFileExpectation::builder()
            .id("example-for-signing.txt")
            .file_name("example-for-signing.txt")
            .file_size(57u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
            .build()])
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
        .data_files([DataFileExpectation::builder()
            .id("test.txt")
            .file_name("test.txt")
            .file_size(4u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build();

    libdigidocpp::initialize()?;
    let container = Container::open(common::fixture_path("tsl.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn fails_opening_asics_container_with_invalid_mimetype() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;
    assert_opening_container_fails(common::fixture_path("test-invalid.asics"));
    Ok(())
}

#[test]
#[serial]
fn fails_opening_asics_container_with_subfolder() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;
    assert_opening_container_fails(common::fixture_path("asics-subfolder.asics"));
    Ok(())
}

#[test]
#[serial]
fn fails_opening_asice_container_with_relative_path_in_contents() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;
    assert_opening_container_fails(common::fixture_path("asice-relative.asice"));
    Ok(())
}

#[test]
#[serial]
fn fails_opening_asice_with_content_file_with_dot_name() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;
    assert_opening_container_fails(common::fixture_path("dot.asice"));
    Ok(())
}

#[test]
#[serial]
fn fails_opening_asice_with_content_file_with_empty_name() -> Result<(), Box<dyn StdError>> {
    libdigidocpp::initialize()?;
    assert_opening_container_fails(common::fixture_path("pt-empty.asice"));
    Ok(())
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
