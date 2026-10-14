#![allow(missing_docs)]
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

use crate::common::{
    MEDIA_TYPE_TEXT_PLAIN, assert_general_digidoc_err, assert_is_recent,
    create_empty_asice_container, create_unsigned_asice_with_data_file, init_with_valid_tsl,
};
use digidocpp::{
    Algorithm, AlgorithmSetting, Container, Error, PartialSigner, Pkcs12Signer, ProfileSetting,
    SignatureAlgorithm, SignatureProfile, Signer,
};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use tempfile::TempDir;

fn bes_signer(file_name: &str) -> Pkcs12Signer {
    let mut signer = common::pkcs12_signer(file_name);
    signer.set_profile(ProfileSetting::Bes).unwrap();
    signer
}

#[rstest]
#[serial]
fn rejects_signing_container_without_data_files(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
) {
    let mut signer = bes_signer("signer1.p12");

    let result = container.sign(&mut signer);

    let error = assert_general_digidoc_err!(result);
    assert_eq!(
        error.message(),
        "No documents in container, can not sign container."
    );
}

#[rstest]
#[case::extending_validity(
    |container: &mut Container, signer: &mut Pkcs12Signer| {
        container.extend_container_validity(signer).map(|_| ())
    },
    "Failed to extend signature"
)]
#[case::removing_a_signature(
    |container: &mut Container, _: &mut Pkcs12Signer| container.remove_signature(0),
    "Incorrect signature id 0, there are only 0 signatures in container."
)]
#[serial]
fn rejects_signature_ops_on_unsigned_container(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
    #[case] operation: fn(&mut Container, &mut Pkcs12Signer) -> Result<(), Error>,
    #[case] expected_message: &str,
) {
    let mut signer = bes_signer("signer1.p12");

    let result = operation(&mut container, &mut signer);

    let error = assert_general_digidoc_err!(result);
    assert_eq!(error.message(), expected_message);
}

#[rstest]
#[case::rsa("signer1.p12", None, SignatureAlgorithm::RsaSha256)]
#[case::ec("signerEC.p12", None, SignatureAlgorithm::EcdsaSha384)]
#[case::rsa_pss(
    "signer1.p12",
    Some(SignatureAlgorithm::RsaPssSha256.into()),
    SignatureAlgorithm::RsaPssSha256
)]
#[serial]
fn signs_container(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
    #[case] signer_file: &str,
    #[case] method: Option<AlgorithmSetting>,
    #[case] expected_method: SignatureAlgorithm,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = bes_signer(signer_file);
    if let Some(method) = method {
        signer.set_method(method)?;
    }

    let signature = container.sign(&mut signer)?;

    let signature = signature.as_signature();
    assert_eq!(signature.id()?, "S0");
    assert_eq!(signature.profile()?, SignatureProfile::Bes);
    assert_eq!(
        signature.signature_method()?,
        Algorithm::Signature(expected_method)
    );
    assert_eq!(signature.signing_certificate()?, signer.cert()?);
    assert_is_recent(&signature.claimed_signing_time()?);
    assert_eq!(container.signatures()?.len(), 1);
    Ok(())
}

#[rstest]
#[serial]
fn adds_signatures(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let mut first_signer = bes_signer("signer1.p12");
    let mut second_signer = bes_signer("signer2.p12");

    container.sign(&mut first_signer)?;
    container.sign(&mut second_signer)?;

    let signatures = container.signatures()?;
    assert_eq!(signatures.len(), 2);
    assert_eq!(signatures[0].id()?, "S0");
    assert_eq!(signatures[0].signing_certificate()?, first_signer.cert()?);
    assert_eq!(signatures[1].id()?, "S1");
    assert_eq!(signatures[1].signing_certificate()?, second_signer.cert()?);
    Ok(())
}

#[rstest]
#[serial]
fn adds_signature_to_reopened_container(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path(), "signed.asice")]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let mut first_signer = bes_signer("signer1.p12");
    let mut second_signer = bes_signer("signer2.p12");
    container.sign(&mut first_signer)?;
    container.save()?;
    let mut reopened = Container::open(temp_dir.path().join("signed.asice"))?;

    reopened.sign(&mut second_signer)?;

    let signatures = reopened.signatures()?;
    assert_eq!(signatures.len(), 2);
    assert_eq!(signatures[0].id()?, "S0");
    assert_eq!(signatures[0].signing_certificate()?, first_signer.cert()?);
    assert_eq!(signatures[1].id()?, "S1");
    assert_eq!(signatures[1].signing_certificate()?, second_signer.cert()?);
    Ok(())
}

#[rstest]
#[serial]
fn removes_signature(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let mut second_signer = bes_signer("signer2.p12");
    container.sign(&mut bes_signer("signer1.p12"))?;
    container.sign(&mut second_signer)?;

    let result = container.remove_signature(2);
    let error = assert_general_digidoc_err!(result);
    assert_eq!(
        error.message(),
        "Incorrect signature id 2, there are only 2 signatures in container."
    );
    assert_eq!(container.signatures()?.len(), 2);

    container.remove_signature(0)?;

    let signatures = container.signatures()?;
    assert_eq!(signatures.len(), 1);
    assert_eq!(signatures[0].signing_certificate()?, second_signer.cert()?);
    Ok(())
}

#[rstest]
#[serial]
fn keeps_signer_params_after_reopening(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path(), "signed.asice")]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = bes_signer("signer1.p12");
    let roles = [String::from("Role1")];
    signer.set_signature_production_place("Tartu", "Tartumaa", "12345", "Estonia")?;
    signer.set_signer_roles(&roles)?;

    container.sign(&mut signer)?;
    container.save()?;
    let signed = Container::open(temp_dir.path().join("signed.asice"))?;

    let signatures = signed.signatures()?;
    assert_eq!(signatures.len(), 1);

    let signature = signatures[0];
    assert_eq!(signature.signer_roles()?, roles);
    assert_eq!(signature.city()?, "Tartu");
    assert_eq!(signature.state_or_province()?, "Tartumaa");
    assert_eq!(signature.postal_code()?, "12345");
    assert_eq!(signature.country_name()?, "Estonia");
    Ok(())
}

#[rstest]
#[case::plain("plain")]
#[case::punctuation("0123456789~#%&()=`@{[]}'")]
#[case::non_ascii("öäüõ")]
#[serial]
fn saves_signed_container_with_special_chars(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[case] name: &str,
) -> Result<(), Box<dyn StdError>> {
    let data_file = common::write_file(temp_dir.path(), &format!("{name}.txt"), b"x")?;
    let container_path = temp_dir.path().join(format!("{name}.asice"));
    let mut container = Container::create(&container_path)?;
    container.add_data_file(data_file, MEDIA_TYPE_TEXT_PLAIN)?;
    container.sign(&mut bes_signer("signer1.p12"))?;

    container.save()?;

    let saved = Container::open(&container_path)?;
    assert_eq!(saved.signatures()?.len(), 1);
    assert_eq!(saved.data_files()?[0].file_name()?, format!("{name}.txt"));
    Ok(())
}

#[rstest]
#[serial]
fn adds_signature_value_to_prepared_signature(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path(), "signed.asice")]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let signed_path = temp_dir.path().join("signed.asice");
    let mut signer = bes_signer("signer2.p12");

    let signature = container.prepare_signature(&mut signer)?;
    let signature = signature.as_signature();
    let signature_value = signer.sign(signature.signature_method()?, &signature.data_to_sign()?)?;
    container.save()?;

    let mut reloaded = Container::open(&signed_path)?;
    let mut signature = reloaded.signature_mut(0)?;
    signature.set_signature_value(&signature_value)?;
    signature.extend_signature_profile(&mut signer)?;
    reloaded.save()?;

    let signed = Container::open(&signed_path)?;
    let signatures = signed.signatures()?;
    assert_eq!(signatures.len(), 1);
    assert_eq!(signatures[0].signing_certificate()?, signer.cert()?);
    Ok(())
}
