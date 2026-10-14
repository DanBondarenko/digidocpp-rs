#![allow(missing_docs)]
// TODO: find a way to deduplicate (vs signing.rs) while keeping conditional compilation
#![cfg(feature = "_online_tests")]
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
    create_unsigned_asice_with_data_file, init_with_valid_tsl,
};
use libdigidocpp::{
    Algorithm, AlgorithmSetting, Container, ContainerType, ExceptionCode, PartialSigner,
    ProfileSetting, SignatureAlgorithm, SignatureProfile, Signer, ValidityExtension,
};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use std::path::Path;
use tempfile::TempDir;

#[rstest]
#[serial]
fn signs_with_two_signers_and_validates_both(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let mut first_signer = common::pkcs12_signer("signer1.p12");
    let mut second_signer = common::pkcs12_signer("signer2.p12");
    container.sign(&mut first_signer)?;
    container.sign(&mut second_signer)?;

    let signatures = container.signatures()?;
    assert_eq!(signatures.len(), 2);

    assert_eq!(signatures[0].id()?, "S0");
    assert_eq!(signatures[0].signing_certificate()?, first_signer.cert()?);
    signatures[0].validate()?;

    assert_eq!(signatures[1].id()?, "S1");
    assert_eq!(signatures[1].signing_certificate()?, second_signer.cert()?);
    signatures[1].validate()?;

    Ok(())
}

#[rstest]
#[serial]
fn adds_signature_and_validates_reopened_container(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path(), "signed.asice")]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let mut first_signer = common::pkcs12_signer("signer1.p12");
    let mut second_signer = common::pkcs12_signer("signer2.p12");
    container.sign(&mut first_signer)?;
    container.save()?;

    let mut reopened = Container::open(temp_dir.path().join("signed.asice"))?;
    reopened.sign(&mut second_signer)?;

    let signatures = reopened.signatures()?;
    assert_eq!(signatures.len(), 2);

    assert_eq!(signatures[0].id()?, "S0");
    assert_eq!(signatures[0].signing_certificate()?, first_signer.cert()?);
    signatures[0].validate()?;

    assert_eq!(signatures[1].id()?, "S1");
    assert_eq!(signatures[1].signing_certificate()?, second_signer.cert()?);
    signatures[1].validate()?;

    Ok(())
}

#[rstest]
#[case::rsa("signer1.p12", None, SignatureAlgorithm::RsaSha256)]
#[case::ec("signerEC.p12", None, SignatureAlgorithm::EcdsaSha384)]
// needs xmlsec1 >= 1.3
#[cfg_attr(
    feature = "_rsa_pss_validation_tests",
    case::rsa_pss(
        "signer1.p12",
        Some(SignatureAlgorithm::RsaPssSha256.into()),
        SignatureAlgorithm::RsaPssSha256
    )
)]
#[serial]
fn signs_and_validates(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
    #[case] signer_file: &str,
    #[case] method_to_set: Option<AlgorithmSetting>,
    #[case] expected_method: SignatureAlgorithm,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = common::pkcs12_signer(signer_file);
    if let Some(method) = method_to_set {
        signer.set_method(method)?;
    }

    let signature = container.sign(&mut signer)?;
    let signature = signature.as_signature();

    assert_eq!(signature.id()?, "S0");
    assert_eq!(signature.profile()?, SignatureProfile::BesTimeStamp);
    assert_eq!(
        signature.signature_method()?,
        Algorithm::Signature(expected_method)
    );
    assert_eq!(signature.signing_certificate()?, signer.cert()?);
    let ocsp_common_name = common::parse_common_name(&signature.ocsp_certificate()?.unwrap())?;
    assert!(ocsp_common_name.contains("OCSP RESPONDER"));
    assert_is_recent(&signature.claimed_signing_time()?);
    assert_is_recent(&signature.ocsp_produced_at()?);
    signature.validate()?;

    assert_eq!(container.signatures()?.len(), 1);
    Ok(())
}

#[rstest]
#[serial]
fn extending_archive_signature_adds_time_stamp(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let mut signer = common::pkcs12_signer("signer2.p12");
    signer.set_profile(ProfileSetting::TimeStampArchive)?;

    let mut signature = container.sign(&mut signer)?;
    signature.as_signature().validate()?;
    assert_eq!(signature.as_signature().archive_time_stamps()?.len(), 1);
    signature.extend_signature_profile(&mut signer)?;

    signature.as_signature().validate()?;
    assert_eq!(signature.as_signature().archive_time_stamps()?.len(), 2);
    Ok(())
}

#[rstest]
#[serial]
fn signs_prepared_container_after_reopening(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path(), "signed.asice")]
    mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let signed_path = temp_dir.path().join("signed.asice");
    let mut signer = common::pkcs12_signer("signer2.p12");

    let signature = container.prepare_signature(&mut signer)?;
    let signature = signature.as_signature();
    let signature_value = signer.sign(signature.signature_method()?, &signature.data_to_sign()?)?;

    container.save()?;

    let mut reloaded = Container::open(&signed_path)?;

    let mut signature = reloaded.signature_mut(0)?;
    let error = assert_general_digidoc_err!(signature.as_signature().validate());
    assert_eq!(error.message(), "Signature validation");

    signature.set_signature_value(&signature_value)?;
    signature.extend_signature_profile(&mut signer)?;
    signature.as_signature().validate()?;
    assert_eq!(
        signature.as_signature().signing_certificate()?,
        signer.cert()?
    );

    Ok(())
}

#[rstest]
#[case::plain("plain")]
#[case::symbols("0123456789~#%&()=`@{[]}'")]
#[case::non_ascii("öäüõ")]
#[serial]
fn validates_signature_after_reopening(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[case] name: &str,
    #[values("signer1.p12", "signerEC.p12")] signer_file: &str,
) -> Result<(), Box<dyn StdError>> {
    let data_file = common::write_file(temp_dir.path(), &format!("{name}.txt"), b"x")?;
    let container_path = temp_dir.path().join(format!("{name}.asice"));
    let mut container = Container::create(&container_path)?;
    container.add_data_file(data_file, MEDIA_TYPE_TEXT_PLAIN)?;
    container.sign(&mut common::pkcs12_signer(signer_file))?;
    container.save()?;

    let saved = Container::open(&container_path)?;
    let signatures = saved.signatures()?;
    assert_eq!(signatures.len(), 1);
    signatures[0].validate()?;
    assert_eq!(saved.data_files()?[0].file_name()?, format!("{name}.txt"));
    Ok(())
}

#[rstest]
#[serial]
fn rejects_unknown_signer(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[from(create_unsigned_asice_with_data_file)]
    #[with(temp_dir.path())]
    mut container: Container,
) {
    let mut signer = common::pkcs12_signer("signer3.p12");

    let result = container.sign(&mut signer);

    let error = assert_general_digidoc_err!(result);
    assert_eq!(error.message(), "Failed to sign container.");
    let [cause] = error.causes() else {
        panic!("expected one cause: {error:?}");
    };
    assert_eq!(cause.message(), "Certificate status: unknown");
    assert!(
        error.has_code(ExceptionCode::CertificateUnknown),
        "{error:?}"
    );
}

#[rstest]
#[case::signed_asice(|directory: &Path| {
    let mut container = create_unsigned_asice_with_data_file(directory, "signed.asice");
    container.sign(&mut common::pkcs12_signer("signer2.p12")).unwrap();
    container.save().unwrap();
    Container::open(directory.join("signed.asice")).unwrap()
})]
#[case::asics(|_: &Path| Container::open(common::fixture_path("test.asics")).unwrap())]
#[serial]
fn extends_signatures_in_place(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[case] open_signed_container: fn(&Path) -> Container,
) -> Result<(), Box<dyn StdError>> {
    let mut container = open_signed_container(temp_dir.path());
    let mut signer = common::pkcs12_signer("signer2.p12");
    assert_eq!(container.signatures()?[0].archive_time_stamps()?.len(), 0);

    let outcome = container.extend_container_validity(&mut signer)?;

    let ValidityExtension::InPlace { extended_count } = outcome else {
        panic!("expected inplace, got {outcome:?}");
    };
    assert_eq!(extended_count, 1);
    let signatures = container.signatures()?;
    assert_eq!(signatures.len(), 1);
    assert_eq!(signatures[0].archive_time_stamps()?.len(), 1);
    signatures[0].validate()?;
    Ok(())
}

#[rstest]
#[serial]
fn wraps_container_with_expired_signature(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let mut container = Container::open(common::fixture_path("test.asice"))?;
    let mut signer = common::pkcs12_signer("signer1.p12");

    let outcome = container.extend_container_validity(&mut signer)?;

    let ValidityExtension::Wrapped(wrapping_container) = &outcome else {
        panic!("expected wrapped, got {outcome:?}");
    };
    assert_eq!(wrapping_container.media_type()?, ContainerType::AsicS);
    let data_files = wrapping_container.data_files()?;
    assert_eq!(data_files.len(), 1);
    assert_eq!(data_files[0].file_name()?, "test.asice");
    let signatures = wrapping_container.signatures()?;
    assert_eq!(signatures.len(), 1);
    signatures[0].validate()?;
    Ok(())
}
