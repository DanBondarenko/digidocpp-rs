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

use crate::common::assert_general_digidoc_err;
use serial_test::serial;
use std::error::Error as StdError;

const REJECTING_TSLS: [&str; 18] = [
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
    "EE_T-TSA-noqscd-granted-later.xml",
];

const ACCEPTING_TSLS: [&str; 6] = [
    "EE_T-CA-withdrawn-granted-before.xml",
    "EE_T-CA-noqscd-granted-before.xml",
    "EE_T-OCSP-withdrawn-granted-before.xml",
    "EE_T-OCSP-noqscd-granted-before.xml",
    "EE_T-TSA-withdrawn-granted-before.xml",
    "EE_T-TSA-noqscd-granted-before.xml",
];

#[test]
#[serial]
fn rejects_signature_when_tsl_does_not_grant_trust() -> Result<(), Box<dyn StdError>> {
    for tsl_name in REJECTING_TSLS {
        let result = common::validate_tsl_asice_with(tsl_name)?;
        assert_general_digidoc_err!(result, "{tsl_name}");
    }

    Ok(())
}

#[test]
#[serial]
fn accepts_signature_when_tsl_grants_trust() -> Result<(), Box<dyn StdError>> {
    for tsl_name in ACCEPTING_TSLS {
        let validation = common::validate_tsl_asice_with(tsl_name)?;

        assert!(
            validation.is_ok(),
            "signature should be valid with {tsl_name}: {validation:?}"
        );
    }

    Ok(())
}
