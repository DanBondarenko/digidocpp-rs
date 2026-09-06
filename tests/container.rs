#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
        clippy::used_underscore_binding,
    )
)]
mod common;

use crate::common::{
    MEDIA_TYPE_ASICE, MEDIA_TYPE_ASICS, MEDIA_TYPE_TEXT_PLAIN, assert_general_digidoc_err,
    assert_internal_err, create_empty_asice_container, init_with_valid_tsl,
};
use libdigidocpp::{Container, Error};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use std::path::Path;
use tempfile::TempDir;

#[rstest]
#[case::asice_with_expired_certificates("test.asice", MEDIA_TYPE_ASICE)]
#[case::asics("test.asics", MEDIA_TYPE_ASICS)]
#[case::asics_with_metadata("test-meta.asics", MEDIA_TYPE_ASICS)]
#[case::asics_with_invalid_timestamp_token("test-invalidts.asics", MEDIA_TYPE_ASICS)]
#[case::asics_with_zero_size_contents("test-tera-empty.asics", MEDIA_TYPE_ASICS)]
#[case::asics_with_large_contents("test-tera.asics", MEDIA_TYPE_ASICS)]
#[case::asice_with_subfolder_contents("asice-path.asice", MEDIA_TYPE_ASICE)]
#[case::asice_with_forged_signature("forged_lt.asice", MEDIA_TYPE_ASICE)]
#[case::asice_for_tsl_validation("tsl.asice", MEDIA_TYPE_ASICE)]
#[serial]
fn opens_containers(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[case] fixture: &str,
    #[case] media_type: &str,
) -> Result<(), Box<dyn StdError>> {
    let container = Container::open(common::fixture_path(fixture))?;

    assert_eq!(container.media_type()?, media_type);
    Ok(())
}

#[rstest]
#[case::invalid_mimetype("test-invalid.asics")]
#[case::subfolder("asics-subfolder.asics")]
#[serial]
fn fails_opening_invalid_containers(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[case] name: &str,
) {
    let result = Container::open(common::fixture_path(name));

    assert_general_digidoc_err!(result);
}

#[rstest]
#[serial]
fn creates_empty_asice_container(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] container: Container,
) -> Result<(), Box<dyn StdError>> {
    assert_eq!(container.media_type()?, MEDIA_TYPE_ASICE);
    Ok(())
}

#[rstest]
#[case::open(|path: &Path| Container::open(path).map(drop))]
#[case::create(|path: &Path| Container::create(path).map(drop))]
#[case::add_data_file(|path: &Path| {
    create_empty_asice_container().add_data_file(path, MEDIA_TYPE_TEXT_PLAIN)
})]
#[case::save_as(|path: &Path| create_empty_asice_container().save_as(path))]
#[serial]
fn rejects_non_utf8_path(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[case] operation: fn(&Path) -> Result<(), Error>,
) {
    let result = operation(&common::non_utf8_path());

    assert!(assert_internal_err!(result).contains("UTF-8"));
}
