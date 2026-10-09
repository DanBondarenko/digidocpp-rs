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

use crate::common::{assert_general_digidoc_err, init, open_container};
use libdigidocpp::{Container, ExceptionCode, ValidatorStatus};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use tempfile::TempDir;

#[rstest]
#[case::ca_non_qa("EE_T-CA-non-qa.xml", ExceptionCode::CertificateIssuerMissing)]
#[case::no_qc_statement("EE_T-no_QCStatement.xml", ExceptionCode::CertificateIssuerMissing)]
#[case::no_qcsd("EE_T-no_QCSD.xml", ExceptionCode::CertificateIssuerMissing)]
#[case::ca_invalid_type("EE_T-CA-invalid-type.xml", ExceptionCode::CertificateIssuerMissing)]
#[case::ca_withdrawn("EE_T-CA-withdrawn.xml", ExceptionCode::CertificateIssuerMissing)]
#[case::ca_granted_later("EE_T-CA-granted-later.xml", ExceptionCode::CertificateIssuerMissing)]
#[case::ca_withdrawn_granted_later(
    "EE_T-CA-withdrawn-granted-later.xml",
    ExceptionCode::CertificateIssuerMissing
)]
#[case::ca_noqscd_granted_later(
    "EE_T-CA-noqscd-granted-later.xml",
    ExceptionCode::CertificateIssuerMissing
)]
#[case::ocsp_invalid_type("EE_T-OCSP-invalid-type.xml", ExceptionCode::CertificateUnknown)]
#[case::ocsp_withdrawn("EE_T-OCSP-withdrawn.xml", ExceptionCode::CertificateUnknown)]
#[case::ocsp_granted_later("EE_T-OCSP-granted-later.xml", ExceptionCode::CertificateUnknown)]
#[case::ocsp_withdrawn_granted_later(
    "EE_T-OCSP-withdrawn-granted-later.xml",
    ExceptionCode::CertificateUnknown
)]
#[case::ocsp_noqscd_granted_later(
    "EE_T-OCSP-noqscd-granted-later.xml",
    ExceptionCode::CertificateUnknown
)]
#[case::tsa_invalid_type("EE_T-TSA-invalid-type.xml", ExceptionCode::CertificateUnknown)]
#[case::tsa_withdrawn("EE_T-TSA-withdrawn.xml", ExceptionCode::CertificateUnknown)]
#[case::tsa_granted_later("EE_T-TSA-granted-later.xml", ExceptionCode::CertificateUnknown)]
#[case::tsa_withdrawn_granted_later(
    "EE_T-TSA-withdrawn-granted-later.xml",
    ExceptionCode::CertificateUnknown
)]
#[case::tsa_noqscd_granted_later(
    "EE_T-TSA-noqscd-granted-later.xml",
    ExceptionCode::CertificateUnknown
)]
#[serial]
fn rejects_signature_unless_tsl_grants_trust(
    #[allow(unused_variables)]
    #[case]
    tsl_name: &str,
    #[case] expected_code: ExceptionCode,
    #[from(init)]
    #[with(tsl_name)]
    _temp_dir: TempDir,
    #[from(open_container)]
    #[with("tsl.asice")]
    container: Container,
) -> Result<(), Box<dyn StdError>> {
    let result = container.signatures()?[0].validate();

    let error = assert_general_digidoc_err!(result);
    assert!(error.has_code(expected_code), "{error:?}");
    Ok(())
}

#[rstest]
#[case::no_qc_statement("EE_T-no_QCStatement.xml")]
#[case::no_qcsd("EE_T-no_QCSD.xml")]
#[serial]
fn reports_unqualified_signature(
    #[allow(unused_variables)]
    #[case]
    tsl_name: &str,
    #[from(init)]
    #[with(tsl_name)]
    _temp_dir: TempDir,
    #[from(open_container)]
    #[with("tsl.asice")]
    container: Container,
) -> Result<(), Box<dyn StdError>> {
    let signatures = container.signatures()?;

    let result = signatures[0].validator_result()?;
    let validation = signatures[0].validate();

    assert_eq!(result.status(), Some(ValidatorStatus::NonQSCD));
    assert!(result.warnings().is_empty(), "{result:?}");
    assert!(validation.is_err(), "{validation:?}");
    Ok(())
}

#[rstest]
#[serial]
fn accepts_signature_when_tsl_grants_trust(
    #[allow(unused_variables)]
    #[values(
        "EE_T-CA-withdrawn-granted-before.xml",
        "EE_T-CA-noqscd-granted-before.xml",
        "EE_T-OCSP-withdrawn-granted-before.xml",
        "EE_T-OCSP-noqscd-granted-before.xml",
        "EE_T-TSA-withdrawn-granted-before.xml",
        "EE_T-TSA-noqscd-granted-before.xml"
    )]
    tsl_name: &str,
    #[from(init)]
    #[with(tsl_name)]
    _temp_dir: TempDir,
    #[from(open_container)]
    #[with("tsl.asice")]
    container: Container,
) -> Result<(), Box<dyn StdError>> {
    let validation = container.signatures()?[0].validate();

    assert!(validation.is_ok(), "{validation:?}");
    Ok(())
}
