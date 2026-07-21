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
use rand_pcg::Pcg64Mcg;
use rand_pcg::rand_core::{Rng, SeedableRng};
use serial_test::serial;
use std::cell::Cell;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fs;
use std::io::{self, Cursor, ErrorKind, Read, Seek, SeekFrom};
use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;
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
fn rejects_reader_data_file_names_with_a_directory() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let mut container = Container::create("new.asice")?;

    let result = container.add_data_file_from_reader(
        "folder/test.txt",
        DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        Cursor::new(b"x"),
    );

    assert_general_digidoc_err!(result);
    ContainerExpectation::builder()
        .data_files([])
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

#[test]
#[serial]
fn calculates_the_digests_of_a_data_file_from_a_reader() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let mut container = Container::create("new.asice")?;
    container.add_data_file_from_reader(
        "test1.txt",
        DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        Cursor::new(b"1234\n".to_vec()),
    )?;

    ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("test1.txt")
            .file_name("test1.txt")
            .file_size(5u64)
            .media_type(DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN)
            .build()])
        .build()
        .assert(&container)?;

    assert_digests(&container.data_files()?[0])
}

fn container_with_reader(
    reader: impl Read + Seek + 'static,
) -> Result<Container, Box<dyn StdError>> {
    let mut container = Container::create("new.asice")?;
    container.add_data_file_from_reader(
        "data.bin",
        DataFileExpectation::MEDIA_TYPE_OCTET_STREAM,
        reader,
    )?;
    ContainerExpectation::builder()
        .data_files([DataFileExpectation::builder()
            .id("data.bin")
            .file_name("data.bin")
            .media_type(DataFileExpectation::MEDIA_TYPE_OCTET_STREAM)
            .build()])
        .build()
        .assert(&container)?;

    Ok(container)
}

#[track_caller]
fn assert_io_error<T: Debug>(
    result: &Result<T, Error>,
    expected_kind: ErrorKind,
    expected_message: &str,
) {
    let Err(Error::Io(io_error)) = &result else {
        panic!("expected io error, got {result:?}");
    };
    assert_eq!(io_error.kind(), expected_kind);
    assert_eq!(io_error.to_string(), expected_message);
}

#[track_caller]
fn assert_scripted_panic<T: Debug>(outcome: std::thread::Result<T>) {
    let payload = outcome.expect_err("expected panic");
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"scripted panic"));
}

struct FailingReader {
    bytes: Cursor<Vec<u8>>,
    calls: Rc<Cell<usize>>,
    fails_seeks: bool,
}

impl FailingReader {
    fn failing_reads() -> Self {
        Self {
            bytes: Cursor::new(vec![0; 100]),
            calls: Rc::default(),
            fails_seeks: false,
        }
    }

    fn failing_seeks() -> Self {
        Self {
            bytes: Cursor::new(vec![0; 100]),
            calls: Rc::default(),
            fails_seeks: true,
        }
    }
}

impl Read for FailingReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.calls.set(self.calls.get() + 1);
        if self.bytes.position() > 0 {
            return Err(io::Error::new(
                ErrorKind::BrokenPipe,
                "scripted read failure",
            ));
        }
        self.bytes.read(buffer)
    }
}

impl Seek for FailingReader {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        self.calls.set(self.calls.get() + 1);
        if self.fails_seeks {
            return Err(io::Error::new(
                ErrorKind::Unsupported,
                "scripted seek failure",
            ));
        }
        self.bytes.seek(position)
    }
}

#[derive(Default)]
struct PanickingReader {
    calls: Rc<Cell<usize>>,
}

impl Read for PanickingReader {
    fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
        self.calls.set(self.calls.get() + 1);
        panic!("scripted panic");
    }
}

impl Seek for PanickingReader {
    fn seek(&mut self, _position: SeekFrom) -> io::Result<u64> {
        self.calls.set(self.calls.get() + 1);
        panic!("scripted panic");
    }
}

#[test]
#[serial]
fn reports_reader_failures_as_io_errors() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;
    let saved_path = data_directory.path().join("saved.bin");

    let container = container_with_reader(FailingReader::failing_reads())?;
    let result = container.data_files()?[0].calc_digest(common::URI_SHA256);
    assert_io_error(&result, ErrorKind::BrokenPipe, "scripted read failure");

    let container = container_with_reader(FailingReader::failing_reads())?;
    let result = container.data_files()?[0].write_to(Vec::new());
    assert_io_error(&result, ErrorKind::BrokenPipe, "scripted read failure");

    let container = container_with_reader(FailingReader::failing_reads())?;
    let result = container.data_files()?[0].save_as(&saved_path);
    assert_io_error(&result, ErrorKind::BrokenPipe, "scripted read failure");

    let container = container_with_reader(FailingReader::failing_seeks())?;
    let result = container.data_files()?[0].file_size();
    assert_io_error(&result, ErrorKind::Unsupported, "scripted seek failure");

    Ok(())
}

#[test]
#[serial]
fn never_uses_a_failed_reader_again() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let reader = FailingReader::failing_reads();
    let calls = Rc::clone(&reader.calls);
    let container = container_with_reader(reader)?;
    let data_file = &container.data_files()?[0];

    data_file
        .calc_digest(common::URI_SHA256)
        .expect_err("the first call should fail");
    let calls_after_failure = calls.get();
    let result = data_file.calc_digest(common::URI_SHA256);

    assert!(
        matches!(result, Err(Error::Io(_))),
        "expected io error, got {result:?}"
    );
    assert_eq!(
        calls.get(),
        calls_after_failure,
        "reader used after failure"
    );

    Ok(())
}

#[test]
#[serial]
fn reader_panic_is_transferred_across_ffi() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let data_directory = TempDir::new()?;
    let saved_path = data_directory.path().join("saved.bin");

    let container = container_with_reader(PanickingReader::default())?;
    let data_file = &container.data_files()?[0];
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        data_file.calc_digest(common::URI_SHA256)
    }));
    assert_scripted_panic(outcome);

    let container = container_with_reader(PanickingReader::default())?;
    let data_file = &container.data_files()?[0];
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| data_file.file_size()));
    assert_scripted_panic(outcome);

    let container = container_with_reader(PanickingReader::default())?;
    let data_file = &container.data_files()?[0];
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| data_file.write_to(Vec::new())));
    assert_scripted_panic(outcome);

    let container = container_with_reader(PanickingReader::default())?;
    let data_file = &container.data_files()?[0];
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| data_file.save_as(&saved_path)));
    assert_scripted_panic(outcome);

    Ok(())
}

#[test]
#[serial]
fn never_uses_a_panicked_reader_again() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let reader = PanickingReader::default();
    let calls = Rc::clone(&reader.calls);
    let container = container_with_reader(reader)?;
    let data_file = &container.data_files()?[0];

    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        data_file.calc_digest(common::URI_SHA256)
    }));
    assert_scripted_panic(outcome);
    let calls_after_panic = calls.get();
    let result = data_file.calc_digest(common::URI_SHA256);

    assert!(
        matches!(result, Err(Error::Io(_))),
        "expected io error, got {result:?}"
    );
    assert_eq!(calls.get(), calls_after_panic, "reader used after panic");

    Ok(())
}

#[test]
#[serial]
fn reader_ownership_is_transferred_to_cpp() -> Result<(), Box<dyn StdError>> {
    struct TrackedBytes {
        _reference_counter: Rc<()>,
    }

    impl AsRef<[u8]> for TrackedBytes {
        fn as_ref(&self) -> &[u8] {
            b"x"
        }
    }

    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let reference_counter = Rc::new(());
    let tracked_reader = || {
        Cursor::new(TrackedBytes {
            _reference_counter: Rc::clone(&reference_counter),
        })
    };

    let mut container = Container::create("new.asice")?;
    container.add_data_file_from_reader(
        "removed.bin",
        DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        tracked_reader(),
    )?;
    assert_eq!(Rc::strong_count(&reference_counter), 2);
    container.remove_data_file(0)?;
    assert_eq!(Rc::strong_count(&reference_counter), 1);

    let result = container.add_data_file_from_reader(
        "folder/rejected.bin",
        DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        tracked_reader(),
    );
    assert_general_digidoc_err!(result);
    assert_eq!(Rc::strong_count(&reference_counter), 1);

    container.add_data_file_from_reader(
        "kept.bin",
        DataFileExpectation::MEDIA_TYPE_TEXT_PLAIN,
        tracked_reader(),
    )?;
    drop(container);
    assert_eq!(Rc::strong_count(&reference_counter), 1);

    Ok(())
}

#[test]
#[serial]
fn writes_out_the_exact_contents_of_data_files() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &["EE_T.xml"])?;
    let output_directory = TempDir::new()?;

    for (fixture, expected) in [
        ("test.asice", b"1234\n".as_slice()),
        ("test-tera-empty.asics", b"".as_slice()),
    ] {
        let container = Container::open(common::fixture_path(fixture))?;
        let data_file = &container.data_files()?[0];

        let mut written = Vec::new();
        data_file.write_to(&mut written)?;
        assert_eq!(written, expected, "{fixture}");

        let saved_path = output_directory.path().join("saved.bin");
        data_file.save_as(&saved_path)?;
        assert_eq!(fs::read(&saved_path)?, expected, "{fixture}");
    }

    Ok(())
}

#[test]
#[serial]
fn reports_writer_failures_as_io_errors() -> Result<(), Box<dyn StdError>> {
    struct FailingOnSecondWriteWriter {
        calls: usize,
    }

    impl io::Write for FailingOnSecondWriteWriter {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            if self.calls == 2 {
                return Err(io::Error::new(ErrorKind::BrokenPipe, "scripted failure"));
            }
            Ok(data.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let container = container_with_reader(Cursor::new(vec![0; 200_000]))?;
    let data_file = &container.data_files()?[0];
    let mut writer = FailingOnSecondWriteWriter { calls: 0 };

    let result = data_file.write_to(&mut writer);

    let error = result.expect_err("write should fail");
    assert_eq!(error.to_string(), "I/O error: scripted failure");
    let Error::Io(inner) = &error else {
        panic!("expected io error, got {error:?}");
    };
    assert_eq!(inner.kind(), ErrorKind::BrokenPipe);
    assert_eq!(writer.calls, 2, "writer called after failure");

    Ok(())
}

#[test]
#[serial]
#[expect(clippy::manual_assert_eq, clippy::manual_assert)]
fn writer_panic_is_transmitted_across_ffi() -> Result<(), Box<dyn StdError>> {
    struct PanickingWriter {
        calls: usize,
    }

    impl io::Write for PanickingWriter {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            if self.calls == 2 {
                panic!("scripted panic");
            }
            Ok(data.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let mut expected = vec![0; 200_000];
    Pcg64Mcg::seed_from_u64(0).fill_bytes(&mut expected);
    let container = container_with_reader(Cursor::new(expected.clone()))?;
    let data_file = &container.data_files()?[0];
    let mut panicking_writer = PanickingWriter { calls: 0 };

    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        data_file.write_to(&mut panicking_writer)
    }));

    let payload = outcome.err().unwrap();
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"scripted panic"));
    assert_eq!(panicking_writer.calls, 2, "writer used after panic");

    let mut written = Vec::new();
    data_file.write_to(&mut written)?;
    assert!(written == expected);

    Ok(())
}
