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
    MEDIA_TYPE_TEXT_PLAIN, TestEnv, assert_general_digidoc_err, assert_is_recent,
    create_unsigned_asice_with_data_file, init_with_valid_tsl,
};
use digidocpp::algorithm::{
    Algorithm, AlgorithmSetting, ProfileSetting, SignatureAlgorithm, SignatureProfile,
};
use digidocpp::container::{ContainerType, ValidityExtension};
use digidocpp::error::ExceptionCode;
use digidocpp::{Container, Initialized, Library, PartialSigner, Signer};
use rstest::rstest;
use std::error::Error as StdError;
use std::path::Path;

#[rstest]
fn signs_with_two_signers_and_validates_both(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut container =
        create_unsigned_asice_with_data_file(&library, directory.path(), "default.asice");
    let mut first_signer = common::pkcs12_signer(&library, "signer1.p12");
    let mut second_signer = common::pkcs12_signer(&library, "signer2.p12");
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
fn adds_signature_and_validates_reopened_container(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut container =
        create_unsigned_asice_with_data_file(&library, directory.path(), "signed.asice");
    let mut first_signer = common::pkcs12_signer(&library, "signer1.p12");
    let mut second_signer = common::pkcs12_signer(&library, "signer2.p12");
    container.sign(&mut first_signer)?;
    container.save()?;

    let mut reopened = Container::open(&library, directory.path().join("signed.asice"))?;
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
fn signs_and_validates(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
    #[case] signer_file: &str,
    #[case] method_to_set: Option<AlgorithmSetting>,
    #[case] expected_method: SignatureAlgorithm,
) -> Result<(), Box<dyn StdError>> {
    let mut container =
        create_unsigned_asice_with_data_file(&library, directory.path(), "default.asice");
    let mut signer = common::pkcs12_signer(&library, signer_file);
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
fn extending_archive_signature_adds_time_stamp(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut container =
        create_unsigned_asice_with_data_file(&library, directory.path(), "default.asice");
    let mut signer = common::pkcs12_signer(&library, "signer2.p12");
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
fn signs_prepared_container_after_reopening(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut container =
        create_unsigned_asice_with_data_file(&library, directory.path(), "signed.asice");
    let signed_path = directory.path().join("signed.asice");
    let mut signer = common::pkcs12_signer(&library, "signer2.p12");

    let signature = container.prepare_signature(&mut signer)?;
    let signature = signature.as_signature();
    let signature_value = signer.sign(signature.signature_method()?, &signature.data_to_sign()?)?;

    container.save()?;

    let mut reloaded = Container::open(&library, &signed_path)?;

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
fn validates_signature_after_reopening(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
    #[case] name: &str,
    #[values("signer1.p12", "signerEC.p12")] signer_file: &str,
) -> Result<(), Box<dyn StdError>> {
    let data_file = common::write_file(directory.path(), &format!("{name}.txt"), b"x")?;
    let container_path = directory.path().join(format!("{name}.asice"));
    let mut container = Container::create(&library, &container_path)?;
    container.add_data_file(data_file, MEDIA_TYPE_TEXT_PLAIN)?;
    container.sign(&mut common::pkcs12_signer(&library, signer_file))?;
    container.save()?;

    let saved = Container::open(&library, &container_path)?;
    let signatures = saved.signatures()?;
    assert_eq!(signatures.len(), 1);
    signatures[0].validate()?;
    assert_eq!(saved.data_files()?[0].file_name()?, format!("{name}.txt"));
    Ok(())
}

#[rstest]
fn rejects_unknown_signer(#[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv) {
    let mut container =
        create_unsigned_asice_with_data_file(&library, directory.path(), "default.asice");
    let mut signer = common::pkcs12_signer(&library, "signer3.p12");

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

fn open_signed_asice<'lib>(
    library: &'lib Library<Initialized>,
    directory: &Path,
) -> Container<'lib> {
    let mut container = create_unsigned_asice_with_data_file(library, directory, "signed.asice");
    container
        .sign(&mut common::pkcs12_signer(library, "signer2.p12"))
        .unwrap();
    container.save().unwrap();
    Container::open(library, directory.join("signed.asice")).unwrap()
}

fn open_asics<'lib>(library: &'lib Library<Initialized>, _: &Path) -> Container<'lib> {
    Container::open(library, common::fixture_path("test.asics")).unwrap()
}

#[rstest]
#[case::signed_asice(open_signed_asice)]
#[case::asics(open_asics)]
fn extends_signatures_in_place(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
    #[case] open_signed_container: for<'lib> fn(
        &'lib Library<Initialized>,
        &Path,
    ) -> Container<'lib>,
) -> Result<(), Box<dyn StdError>> {
    let mut container = open_signed_container(&library, directory.path());
    let mut signer = common::pkcs12_signer(&library, "signer2.p12");
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
fn wraps_container_with_expired_signature(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut container = Container::open(&library, common::fixture_path("test.asice"))?;
    let mut signer = common::pkcs12_signer(&library, "signer1.p12");

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
