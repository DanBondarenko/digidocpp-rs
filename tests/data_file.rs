#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::redundant_pub_crate,
        clippy::indexing_slicing,
    )
)]
mod common;

use crate::common::{ContainerExpectation, DataFileExpectation, assert_general_digidoc_err};
use libdigidocpp::{Container, DataFile, Error};
use serial_test::serial;
use std::error::Error as StdError;
use tempfile::TempDir;

#[test]
#[serial]
fn reads_data_file_from_asice_container_with_expired_certificates() -> Result<(), Box<dyn StdError>>
{
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type("file")
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_data_file_from_asics_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test.asics"));
    container_expectation.assert(&container?)
}

#[test]
#[serial]
fn reads_data_file_from_asics_container_with_metadata() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-meta.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_data_file_from_asics_container_with_invalid_timestamp_token()
-> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test2.txt")
            .file_name("test2.txt")
            .file_size(10u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-invalidts.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_data_file_with_zero_size_contents_from_asics_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("emptyFile.ddoc")
            .file_name("emptyFile.ddoc")
            .file_size(0u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-tera-empty.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_data_file_with_large_contents_from_asics_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("ddoc_for_testing.ddoc")
            .file_name("ddoc_for_testing.ddoc")
            .file_size(8736u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("test-tera.asics"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_data_file_from_asice_container_with_subfolder_contents() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("folder/test1.txt")
            .file_name("folder/test1.txt")
            .file_size(4u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let container = Container::open(common::fixture_path("asice-path.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_data_file_from_asice_container_with_forged_signature() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("example-for-signing.txt")
            .file_name("example-for-signing.txt")
            .file_size(57u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let container = Container::open(common::fixture_path("forged_lt.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn reads_data_file_from_asice_container() -> Result<(), Box<dyn StdError>> {
    let container_expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test.txt")
            .file_name("test.txt")
            .file_size(4u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build();

    let _temp_dir = common::initialize_offline("EE_T.xml", &[])?;
    let container = Container::open(common::fixture_path("tsl.asice"))?;
    container_expectation.assert(&container)
}

#[test]
#[serial]
fn newly_crated_asice_container_has_no_data_files() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let container = Container::create("new.asice")?;

    ContainerExpectation::builder()
        .data_files([])
        .build()
        .assert(&container)
}

fn assert_digests(data_file: &DataFile) -> Result<(), Box<dyn StdError>> {
    const TEST1_DIGESTS: [(&str, &str); 9] = [
        (
            "http://www.w3.org/2000/09/xmldsig#sha1",
            "1be168ff837f043bde17c0314341c84271047b31",
        ),
        (
            "http://www.w3.org/2001/04/xmldsig-more#sha224",
            "d76041112e343b2bdc14d43934e5aed7abf90192c6543bdf2ae4f81b",
        ),
        (
            common::URI_SHA256,
            "a883dafc480d466ee04e0d6da986bd78eb1fdd2178d04693723da3a8f95d42f4",
        ),
        (
            "http://www.w3.org/2001/04/xmldsig-more#sha384",
            "637e2edd555570eda966d99d4e77d9fbb3aab84a498ff55ac21b963c1e05c2addfb5c15cd2071e7edd47359d787941d3",
        ),
        (
            "http://www.w3.org/2001/04/xmlenc#sha512",
            "7985558370f0de86a864e0050afdf45d7029b8798bcd72cddbf781329f99380e3f3b1afdca6765d89fc388b213df8f6a193cfc56d4ff2ef6e0a99bd883a6d98c",
        ),
        (
            "http://www.w3.org/2007/05/xmldsig-more#sha3-224",
            "8a020a4c681207362dea91d59f660a47a183e1e3d488328ddd64a7e4",
        ),
        (
            "http://www.w3.org/2007/05/xmldsig-more#sha3-256",
            "859a7a7603028deeb3b66234cffa5191466d1a0538e449a19812273b0d98dc1c",
        ),
        (
            "http://www.w3.org/2007/05/xmldsig-more#sha3-384",
            "81ec6a885d9f59160b931855f3422e37072cda8059dae063eb64716fe4c4e9c0e701e3e6471315ba446bad4031811e5d",
        ),
        (
            "http://www.w3.org/2007/05/xmldsig-more#sha3-512",
            "0505da4c583d0bfa95692bf9845abf2a40f32e34caef5acc57bd3015aefa54777847cda68d017acfed634f07957a5ce601b23b1cc040a9dec5443a6c9f56c948",
        ),
    ];

    for (method, expected) in TEST1_DIGESTS {
        let digest = hex::encode(data_file.calc_digest(method)?);
        assert_eq!(digest, expected, "digest for {method}");
    }

    Ok(())
}

#[test]
#[serial]
fn calculates_data_file_digests() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let container = Container::open(common::fixture_path("test.asice"))?;
    let data_files = container.data_files()?;
    let data_file = data_files.first().expect("test.asice should hold a file");

    assert_digests(data_file)
}

#[test]
#[serial]
fn adds_data_files() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;
    let first = common::write_file(data_directory.path(), "first.txt", b"12345")?;
    let second = common::write_file(data_directory.path(), "second.bin", b"abc")?;
    let mut container = Container::create("new.asice")?;

    container.add_data_file(&first, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)?;
    container.add_data_file(&second, DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)?;

    ContainerExpectation::builder()
        .data_files([
            DataFileExpectation::builder()
                .id("first.txt")
                .file_name("first.txt")
                .file_size(5u64)
                .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
                .build(),
            DataFileExpectation::builder()
                .id("second.bin")
                .file_name("second.bin")
                .file_size(3u64)
                .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
                .build(),
        ])
        .build()
        .assert(&container)
}

#[test]
#[serial]
fn adds_data_files_with_special_characters() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;

    for name in ["0123456789~#%&()=`@{[]}'.txt", "öäüõ.txt"] {
        let path = common::write_file(data_directory.path(), name, b"x")?;
        let mut container = Container::create("new.asice")?;
        container.add_data_file(&path, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)?;

        ContainerExpectation::builder()
            .data_files([DataFileExpectation::builder()
                .id(name)
                .file_name(name)
                .file_size(1u64)
                .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
                .build()])
            .build()
            .assert(&container)?;
    }

    Ok(())
}

#[test]
#[serial]
fn rejects_data_files_with_problems() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;
    let accepted = common::write_file(data_directory.path(), "accepted.txt", b"x")?;
    let reserved = common::write_file(data_directory.path(), "mimetype", b"x")?;
    let other = common::write_file(data_directory.path(), "other.txt", b"x")?;
    let missing = data_directory.path().join("missing.txt");
    let mut container = Container::create("new.asice")?;
    container.add_data_file(&accepted, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)?;

    let cases = [
        (
            "reserved file name",
            &reserved,
            DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        ),
        ("media type without a slash", &other, "textplain"),
        (
            "missing file",
            &missing,
            DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        ),
        (
            "duplicate file name",
            &accepted,
            DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        ),
    ];
    for (defect, path, media_type) in cases {
        let result = container.add_data_file(path, media_type);

        assert_general_digidoc_err!(result, "{defect}");

        ContainerExpectation::builder()
            .data_files([DataFileExpectation::builder()
                .id("accepted.txt")
                .file_name("accepted.txt")
                .file_size(1u64)
                .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
                .build()])
            .build()
            .assert(&container)?;
    }

    Ok(())
}

#[test]
#[serial]
fn rejects_adding_a_data_file_to_a_signed_container() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let data_directory = TempDir::new()?;
    let path = common::write_file(data_directory.path(), "extra.txt", b"x")?;
    let mut container = Container::open(common::fixture_path("test.asice"))?;

    let result = container.add_data_file(&path, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN);

    assert_general_digidoc_err!(result);
    ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type("file")
            .build()])
        .build()
        .assert(&container)
}

#[test]
#[serial]
fn removes_data_files_one_by_one() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;
    let mut container = Container::create("new.asice")?;

    let first = common::write_file(data_directory.path(), "first.txt", b"x")?;
    container.add_data_file(&first, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)?;
    let first_file_expectation = DataFileExpectation::builder()
        .id("first.txt")
        .file_name("first.txt")
        .file_size(1u64)
        .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
        .build();

    let second = common::write_file(data_directory.path(), "second.bin", b"x")?;
    container.add_data_file(&second, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)?;
    let second_file_expectation = DataFileExpectation::builder()
        .id("second.bin")
        .file_name("second.bin")
        .file_size(1u64)
        .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
        .build();

    ContainerExpectation::builder()
        .data_files([first_file_expectation, second_file_expectation.clone()])
        .build()
        .assert(&container)?;

    container.remove_data_file(0)?;
    ContainerExpectation::builder()
        .data_files([second_file_expectation])
        .build()
        .assert(&container)?;

    container.remove_data_file(0)?;
    ContainerExpectation::builder()
        .data_files([])
        .build()
        .assert(&container)
}

#[test]
#[serial]
fn rejects_removing_a_data_file_at_an_invalid_index() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;
    let path = common::write_file(data_directory.path(), "only.txt", b"x")?;
    let mut container = Container::create("new.asice")?;
    ContainerExpectation::builder()
        .data_files([])
        .build()
        .assert(&container)?;

    let result = container.remove_data_file(0);
    assert_general_digidoc_err!(result);
    ContainerExpectation::builder()
        .data_files([])
        .build()
        .assert(&container)?;

    container.add_data_file(&path, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)?;
    let result = container.remove_data_file(1);
    assert_general_digidoc_err!(result);
    ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("only.txt")
            .file_name("only.txt")
            .file_size(1u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
            .build()])
        .build()
        .assert(&container)
}

#[test]
#[serial]
fn rejects_removing_a_data_file_from_a_signed_container() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let mut container = Container::open(common::fixture_path("test.asice"))?;

    let result = container.remove_data_file(0);
    assert_general_digidoc_err!(result);
    ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type("file")
            .build()])
        .build()
        .assert(&container)
}

#[test]
#[serial]
fn rejects_an_index_that_does_not_fit_in_32_bits() -> Result<(), Box<dyn StdError>> {
    let Ok(too_large) = usize::try_from(u64::from(u32::MAX) + 1) else {
        return Ok(());
    };
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;
    let path = common::write_file(data_directory.path(), "only.txt", b"x")?;
    let mut container = Container::create("new.asice")?;
    container.add_data_file(&path, DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)?;
    let expectation = ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("only.txt")
            .file_name("only.txt")
            .file_size(1u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
            .build()])
        .build();

    expectation.assert(&container)?;
    let result = container.remove_data_file(too_large);

    assert!(
        matches!(result, Err(Error::Internal(_))),
        "expected internal err, got {result:?}"
    );

    expectation.assert(&container)
}
