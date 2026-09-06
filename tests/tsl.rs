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
use libdigidocpp::Container;
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use tempfile::TempDir;

#[rstest]
#[serial]
fn rejects_signature_unless_tsl_grants_trust(
    #[allow(unused_variables)]
    #[values(
        "EE_T-CA-non-qa.xml",
        "EE_T-no_QCStatement.xml",
        "EE_T-no_QCSD.xml",
        "EE_T-CA-invalid-type.xml",
        "EE_T-CA-withdrawn.xml",
        "EE_T-CA-granted-later.xml",
        "EE_T-CA-withdrawn-granted-later.xml",
        "EE_T-CA-noqscd-granted-later.xml",
        "EE_T-OCSP-invalid-type.xml",
        "EE_T-OCSP-withdrawn.xml",
        "EE_T-OCSP-granted-later.xml",
        "EE_T-OCSP-withdrawn-granted-later.xml",
        "EE_T-OCSP-noqscd-granted-later.xml",
        "EE_T-TSA-invalid-type.xml",
        "EE_T-TSA-withdrawn.xml",
        "EE_T-TSA-granted-later.xml",
        "EE_T-TSA-withdrawn-granted-later.xml",
        "EE_T-TSA-noqscd-granted-later.xml"
    )]
    tsl_name: &str,
    #[from(init)]
    #[with(tsl_name)]
    _temp_dir: TempDir,
    #[from(open_container)]
    #[with("tsl.asice")]
    container: Container,
) -> Result<(), Box<dyn StdError>> {
    let result = container.signatures()?[0].validate();

    assert_general_digidoc_err!(result);
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
