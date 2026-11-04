#![allow(missing_docs)]
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
    TestEnv, assert_general_digidoc_err, assert_internal_err, init_with_valid_tsl, pkcs12_signer,
};
use digidocpp::algorithm::{Algorithm, AlgorithmSetting, DigestAlgorithm, SigningProfile};
use digidocpp::signer::{Pkcs11Signer, Pkcs12Signer};
use digidocpp::{Error, Initialized, Library, PartialSigner, Signer};
use rstest::rstest;
use std::error::Error as StdError;
use std::path::Path;

#[rstest]
#[case::wrong_pass(
    common::fixture_path("signer1.p12"),
    "signer0",
    "Failed to parse PKCS12 certificate."
)]
#[case::missing(
    String::from("missing.p12"),
    "signer1",
    "Failed to open PKCS12 certificate: missing.p12."
)]
#[case::not_pkcs12(
    common::fixture_path("empty.conf"),
    "signer1",
    "Failed to read PKCS12 certificate:"
)]
fn rejects_invalid_pkcs12_signers(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
    #[case] path: String,
    #[case] password: &str,
    #[case] message_prefix: &str,
) {
    let result = Pkcs12Signer::new(&library, path, password);

    let error = assert_general_digidoc_err!(result);
    assert!(
        error.message().starts_with(message_prefix),
        "error message: {}",
        error.message()
    );
}

#[rstest]
fn gets_signer_cert_common_name(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let signer = pkcs12_signer(&library, "signer1.p12");
    let common_name = common::parse_common_name(&signer.cert()?.unwrap())?;

    assert_eq!(common_name, "signer1");
    Ok(())
}

#[rstest]
fn signs_digest(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    const SHA256_OF_SIGNATURE_AND_NUL: &str =
        "8c83544270b525024a3c933aa9965594ee19361796fb85771362b84429df1069";
    const EXPECTED_SIGNATURE: &str = "5f3ee2c043933007969d7e1ecfcf2da32fde3a048b33d3202fea94668799b35777eced4860e334ded77c3bcb3819bbe8b8d40b25d78454675fd8cd050b921cf2fbee362d88f0ce760c7677f02db0b26400ef05a6da2c7c64be6b874e15a66d8bb282aa7cb1727e1135ee1805ceef13e2f6f3ccd18d38d533fb955978ccbd937e65bdbf71176b6075f8d00d2bfaf4acb6b9f8aef10732d76d77f2421110612e5c4f3d3adde70a25b13122c4929dfa47f6e815edf8ec14d4bcb226b895af39f561d45e27ec878d867bb8b309f7c0c931bfa12224ad08c9cbf1595f832026eb81e2f1f935d8e2a7bf10e532db7ffe1423e1389c2f729c5644f285a0f43d12614d00";

    let signer = pkcs12_signer(&library, "signer1.p12");
    let signature = signer.sign(
        Algorithm::from(DigestAlgorithm::Sha256),
        &hex::decode(SHA256_OF_SIGNATURE_AND_NUL)?,
    )?;

    assert_eq!(hex::encode(signature), EXPECTED_SIGNATURE);
    Ok(())
}

#[rstest]
fn sets_signer_params(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = pkcs12_signer(&library, "signer1.p12");
    let roles = [String::from("Role1")];
    assert!(signer.signer_roles()?.is_empty());
    assert_eq!(signer.city()?, "");
    assert_eq!(signer.state_or_province()?, "");
    assert_eq!(signer.postal_code()?, "");
    assert_eq!(signer.country_name()?, "");
    assert_eq!(signer.street_address()?, "");
    assert!(!signer.using_en_profile()?);

    signer.set_signature_production_place("Tartu", "Tartumaa", "12345", "Estonia")?;
    signer.set_signer_roles(&roles)?;

    assert_eq!(signer.signer_roles()?, roles);
    assert_eq!(signer.city()?, "Tartu");
    assert_eq!(signer.state_or_province()?, "Tartumaa");
    assert_eq!(signer.postal_code()?, "12345");
    assert_eq!(signer.country_name()?, "Estonia");
    assert_eq!(signer.street_address()?, "");
    assert!(!signer.using_en_profile()?);
    Ok(())
}

#[rstest]
fn street_address_activates_en_profile(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = pkcs12_signer(&library, "signer1.p12");
    signer.set_signature_production_place_v2("Tartu", "", "Tartumaa", "12345", "Estonia")?;
    assert!(!signer.using_en_profile()?);

    signer
        .set_signature_production_place_v2("Tartu", "Street 1", "Tartumaa", "12345", "Estonia")?;
    assert_eq!(signer.street_address()?, "Street 1");
    assert!(signer.using_en_profile()?);

    signer.set_en_profile(false)?;
    assert!(!signer.using_en_profile()?);
    Ok(())
}

#[rstest]
#[case::time_stamp_archive("TSA", SigningProfile::TimeStampArchive)]
#[case::bes("BES", SigningProfile::Bes)]
#[case::time_stamp_token("time-stamp-token", SigningProfile::TimeStampToken)]
fn sets_profile(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
    #[case] alias: &str,
    #[case] expected: SigningProfile,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = pkcs12_signer(&library, "signer1.p12");
    assert_eq!(signer.profile()?, SigningProfile::TimeStamp);

    signer.set_profile(alias.parse()?)?;

    assert_eq!(signer.profile()?, expected);
    Ok(())
}

#[rstest]
fn sets_user_agent(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = pkcs12_signer(&library, "signer1.p12");
    assert_eq!(signer.user_agent()?, "");

    signer.set_user_agent("test-agent")?;

    assert_eq!(signer.user_agent()?, "test-agent");
    Ok(())
}

#[rstest]
fn defaults_to_sha384_for_ec_key(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = pkcs12_signer(&library, "signerEC.p12");
    assert_eq!(signer.method()?, Algorithm::from(DigestAlgorithm::Sha384));

    signer.set_method(DigestAlgorithm::Sha256.into())?;
    assert_eq!(signer.method()?, Algorithm::from(DigestAlgorithm::Sha256));

    signer.set_method(AlgorithmSetting::Default)?;
    assert_eq!(signer.method()?, Algorithm::from(DigestAlgorithm::Sha384));
    Ok(())
}

#[rstest]
fn rejects_missing_pkcs11_driver(#[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv) {
    let result = Pkcs11Signer::with_driver(&library, "/missing/driver.so");

    let error = assert_general_digidoc_err!(result);
    assert_eq!(
        error.message(),
        "Failed to load driver for PKCS #11 engine: /missing/driver.so."
    );
}

#[rstest]
fn rejects_empty_pkcs11_driver(#[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv) {
    let result = Pkcs11Signer::with_driver(&library, "");

    assert_eq!(assert_internal_err!(result), "driver path is empty");
}

#[rstest]
#[case::pkcs12(
    |library: &Library<Initialized>, path: &Path| {
        Pkcs12Signer::new(library, path, "signer1").map(|_| ())
    }
)]
#[case::pkcs11(
    |library: &Library<Initialized>, path: &Path| {
        Pkcs11Signer::with_driver(library, path).map(|_| ())
    }
)]
fn rejects_non_utf8_path(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
    #[case] constructor: fn(&Library<Initialized>, &Path) -> Result<(), Error>,
) {
    let result = constructor(&library, &common::non_utf8_path());

    assert!(assert_internal_err!(result).contains("UTF-8"));
}
