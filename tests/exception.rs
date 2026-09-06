#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
        clippy::indexing_slicing,
        clippy::used_underscore_binding,
    )
)]
mod common;

use crate::common::{assert_general_digidoc_err, init, init_with_valid_tsl, open_container};
use libdigidocpp::{Container, DigiDocError, ExceptionCode};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use tempfile::TempDir;

#[track_caller]
fn assert_error(error: &DigiDocError, message: &str, code: ExceptionCode) {
    assert_eq!(error.message(), message);
    assert_eq!(error.code(), Some(code));
    assert!(!error.cpp_file().is_empty());
    assert!(error.cpp_line() > 0);
}

#[rstest]
#[case::dot_name("dot.asice", "Document file '.' contains invalid path.")]
#[case::relative_path(
    "asice-relative.asice",
    "Document file 'folder/../test1.txt' contains invalid path."
)]
#[case::empty_path("pt-empty.asice", "Manifest file entry full-path is empty.")]
#[serial]
fn errors_have_cause(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[case] name: &str,
    #[case] cause_message: &str,
) {
    let result = Container::open(common::fixture_path(name));
    let error = assert_general_digidoc_err!(result);

    assert_error(&error, "Failed to parse manifest", ExceptionCode::General);

    let [cause] = error.causes() else {
        panic!("expected one digidoc cause");
    };

    assert_error(cause, cause_message, ExceptionCode::General);

    let source = error
        .source()
        .expect("Expected source")
        .downcast_ref::<DigiDocError>()
        .expect("Expected digidoc error");

    assert_eq!(source.message(), cause.message());
    assert!(source.source().is_none());
}

#[rstest]
#[serial]
fn validation_errors_have_cause_tree(
    #[from(init)]
    #[with("EE_T-CA-non-qa.xml")]
    _temp_dir: TempDir,
    #[from(open_container)]
    #[with("tsl.asice")]
    container: Container,
) -> Result<(), Box<dyn StdError>> {
    let result = container.signatures()?[0].validate();

    let error = assert_general_digidoc_err!(result);

    assert_error(&error, "Signature validation", ExceptionCode::General);

    let [
        unverified_signing_certificate,
        second_issuer_missing,
        unknown_certificate_status,
    ] = error.causes()
    else {
        panic!("expected three causes: {error:?}");
    };
    assert_error(
        unverified_signing_certificate,
        "Unable to verify signing certificate",
        ExceptionCode::General,
    );
    assert_error(
        second_issuer_missing,
        "unable to get local issuer certificate",
        ExceptionCode::CertificateIssuerMissing,
    );
    assert_error(
        unknown_certificate_status,
        "Certificate status: unknown",
        ExceptionCode::CertificateUnknown,
    );

    let [first_issuer_missing] = unverified_signing_certificate.causes() else {
        panic!("expected one cause: {unverified_signing_certificate:?}");
    };
    assert_error(
        first_issuer_missing,
        "unable to get local issuer certificate",
        ExceptionCode::CertificateIssuerMissing,
    );

    Ok(())
}
