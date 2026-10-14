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
    MEDIA_TYPE_OCTET_STREAM, MEDIA_TYPE_TEXT_PLAIN, assert_general_digidoc_err,
    assert_internal_err, create_empty_asice_container, init_with_valid_tsl, open_container,
    required,
};
use digidocpp::{Container, DataFile, DigestAlgorithm, Error};
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rstest::{fixture, rstest};
use serial_test::serial;
use std::cell::{Cell, RefCell};
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fs;
use std::io::{self, Cursor, ErrorKind, Read, Seek, SeekFrom};
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::rc::{Rc, Weak};
use tempfile::TempDir;

#[fixture]
fn create_container_with_accepted_txt(
    #[default(required())] directory: &Path,
    #[from(create_empty_asice_container)] mut container: Container,
) -> Container {
    let accepted = common::write_file(directory, "accepted.txt", b"x").unwrap();
    container
        .add_data_file(&accepted, MEDIA_TYPE_TEXT_PLAIN)
        .unwrap();

    container
}

fn assert_data_files(
    container: &Container,
    expected: &[(&str, u64, &str)],
) -> Result<(), Box<dyn StdError>> {
    let data_files = container.data_files()?;
    assert_eq!(data_files.len(), expected.len());
    for (data_file, &(name, size, media_type)) in data_files.iter().zip_eq(expected) {
        assert_eq!(data_file.id()?, name);
        assert_eq!(data_file.file_name()?, name);
        assert_eq!(data_file.file_size()?, size);
        assert_eq!(data_file.media_type()?, media_type);
    }
    Ok(())
}

fn assert_holds_only_the_accepted_file(container: &Container) -> Result<(), Box<dyn StdError>> {
    assert_data_files(container, &[("accepted.txt", 1, MEDIA_TYPE_TEXT_PLAIN)])
}

#[rstest]
#[case::asice_with_expired_certificates("test.asice", "test1.txt", 5, "file")]
#[case::asics("test.asics", "test1.txt", 5, MEDIA_TYPE_OCTET_STREAM)]
#[case::asics_with_metadata("test-meta.asics", "test1.txt", 5, MEDIA_TYPE_OCTET_STREAM)]
#[case::asics_with_invalid_timestamp_token(
    "test-invalidts.asics",
    "test2.txt",
    10,
    MEDIA_TYPE_OCTET_STREAM
)]
#[case::asics_with_zero_size_contents(
    "test-tera-empty.asics",
    "emptyFile.ddoc",
    0,
    MEDIA_TYPE_OCTET_STREAM
)]
#[case::asics_with_large_contents(
    "test-tera.asics",
    "ddoc_for_testing.ddoc",
    8736,
    MEDIA_TYPE_OCTET_STREAM
)]
#[case::asice_with_subfolder_contents(
    "asice-path.asice",
    "folder/test1.txt",
    4,
    MEDIA_TYPE_TEXT_PLAIN
)]
#[case::asice_with_forged_signature(
    "forged_lt.asice",
    "example-for-signing.txt",
    57,
    MEDIA_TYPE_TEXT_PLAIN
)]
#[case::asice_for_tsl_validation("tsl.asice", "test.txt", 4, MEDIA_TYPE_OCTET_STREAM)]
#[serial]
fn reads_data_files_from_containers(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[case] fixture: &str,
    #[case] file_name_and_id: &str,
    #[case] file_size: u64,
    #[case] media_type: &str,
) -> Result<(), Box<dyn StdError>> {
    let container = Container::open(common::fixture_path(fixture))?;

    assert_data_files(&container, &[(file_name_and_id, file_size, media_type)])
}

#[rstest]
#[serial]
fn newly_crated_asice_container_has_no_data_files(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] container: Container,
) -> Result<(), Box<dyn StdError>> {
    assert_data_files(&container, &[])
}

#[rstest]
#[case::sha1(DigestAlgorithm::Sha1, "1be168ff837f043bde17c0314341c84271047b31")]
#[case::sha224(
    DigestAlgorithm::Sha224,
    "d76041112e343b2bdc14d43934e5aed7abf90192c6543bdf2ae4f81b"
)]
#[case::sha256(
    DigestAlgorithm::Sha256,
    "a883dafc480d466ee04e0d6da986bd78eb1fdd2178d04693723da3a8f95d42f4"
)]
#[case::sha384(
    DigestAlgorithm::Sha384,
    "637e2edd555570eda966d99d4e77d9fbb3aab84a498ff55ac21b963c1e05c2addfb5c15cd2071e7edd47359d787941d3"
)]
#[case::sha512(
    DigestAlgorithm::Sha512,
    "7985558370f0de86a864e0050afdf45d7029b8798bcd72cddbf781329f99380e3f3b1afdca6765d89fc388b213df8f6a193cfc56d4ff2ef6e0a99bd883a6d98c"
)]
#[case::sha3_224(
    DigestAlgorithm::Sha3_224,
    "8a020a4c681207362dea91d59f660a47a183e1e3d488328ddd64a7e4"
)]
#[case::sha3_256(
    DigestAlgorithm::Sha3_256,
    "859a7a7603028deeb3b66234cffa5191466d1a0538e449a19812273b0d98dc1c"
)]
#[case::sha3_384(
    DigestAlgorithm::Sha3_384,
    "81ec6a885d9f59160b931855f3422e37072cda8059dae063eb64716fe4c4e9c0e701e3e6471315ba446bad4031811e5d"
)]
#[case::sha3_512(
    DigestAlgorithm::Sha3_512,
    "0505da4c583d0bfa95692bf9845abf2a40f32e34caef5acc57bd3015aefa54777847cda68d017acfed634f07957a5ce601b23b1cc040a9dec5443a6c9f56c948"
)]
#[serial]
fn calculates_data_file_digests(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[values(
        open_container("test.asice"),
        container_with_file_from_reader(
            "test1.txt",
            MEDIA_TYPE_TEXT_PLAIN,
            Cursor::new(b"1234\n".to_vec())
        )
    )]
    container: Container,
    #[case] method: DigestAlgorithm,
    #[case] expected: &str,
) -> Result<(), Box<dyn StdError>> {
    let digest = hex::encode(container.data_files()?[0].calc_digest(method)?);

    assert_eq!(digest, expected);
    Ok(())
}

#[rstest]
#[case::text_file("test1.txt", b"1234\n")]
#[case::symbols_name("0123456789~#%&()=`@{[]}'.txt", b"x")]
#[case::non_ascii_name("öäüõ.txt", b"x")]
#[serial]
fn adds_data_file_from_reader(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[case] file_name: &str,
    #[case] contents: &[u8],
) -> Result<(), Box<dyn StdError>> {
    let container = container_with_file_from_reader(
        file_name,
        MEDIA_TYPE_TEXT_PLAIN,
        Cursor::new(contents.to_vec()),
    );

    assert_data_files(
        &container,
        &[(
            file_name,
            u64::try_from(contents.len())?,
            MEDIA_TYPE_TEXT_PLAIN,
        )],
    )
}

#[rstest]
#[serial]
fn adds_multiple_data_files(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let first = common::write_file(temp_dir.path(), "first.txt", b"12345")?;
    let second = common::write_file(temp_dir.path(), "second.bin", b"abc")?;

    container.add_data_file(&first, MEDIA_TYPE_TEXT_PLAIN)?;
    container.add_data_file(&second, MEDIA_TYPE_OCTET_STREAM)?;

    assert_data_files(
        &container,
        &[
            ("first.txt", 5, MEDIA_TYPE_TEXT_PLAIN),
            ("second.bin", 3, MEDIA_TYPE_OCTET_STREAM),
        ],
    )
}

#[rstest]
#[case::text_file("first.txt", b"12345", MEDIA_TYPE_TEXT_PLAIN)]
#[case::binary_file("second.bin", b"abc", MEDIA_TYPE_OCTET_STREAM)]
#[case::symbols_name("0123456789~#%&()=`@{[]}'.txt", b"x", MEDIA_TYPE_TEXT_PLAIN)]
#[case::non_ascii_name("öäüõ.txt", b"x", MEDIA_TYPE_TEXT_PLAIN)]
#[serial]
fn adds_data_file(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
    #[case] file_name: &str,
    #[case] contents: &[u8],
    #[case] media_type: &str,
) -> Result<(), Box<dyn StdError>> {
    let path = common::write_file(temp_dir.path(), file_name, contents)?;

    container.add_data_file(&path, media_type)?;

    assert_data_files(
        &container,
        &[(file_name, u64::try_from(contents.len())?, media_type)],
    )
}

#[rstest]
#[case::reserved_name("mimetype", Some("x"), MEDIA_TYPE_TEXT_PLAIN)]
#[case::invalid_media_type("other.txt", Some("x"), "textplain")]
#[case::duplicate_name("accepted.txt", Some("x"), MEDIA_TYPE_TEXT_PLAIN)]
#[case::zero_bytes("empty.txt", Some(""), MEDIA_TYPE_TEXT_PLAIN)]
#[case::missing_file("missing.txt", None, MEDIA_TYPE_TEXT_PLAIN)]
#[serial]
fn rejects_adding_an_invalid_data_file(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[with(temp_dir.path())] mut create_container_with_accepted_txt: Container,
    #[case] file_name: &str,
    #[case] contents: Option<&str>,
    #[case] media_type: &str,
) -> Result<(), Box<dyn StdError>> {
    let path = temp_dir.path().join(file_name);
    if let Some(contents) = contents {
        fs::write(&path, contents)?;
    }

    let result = create_container_with_accepted_txt.add_data_file(&path, media_type);

    assert_general_digidoc_err!(result);
    assert_holds_only_the_accepted_file(&create_container_with_accepted_txt)
}

#[rstest]
#[case::add_data_file(|container: &mut Container, extra_file: &Path| {
    container.add_data_file(extra_file, MEDIA_TYPE_TEXT_PLAIN)
})]
#[case::add_data_file_from_reader(|container: &mut Container, _: &Path| {
    container.add_data_file_from_reader(
        "extra.txt",
        MEDIA_TYPE_TEXT_PLAIN,
        Cursor::new(b"x"),
    )
})]
#[case::remove_data_file(|container: &mut Container, _: &Path| container.remove_data_file(0))]
#[serial]
fn rejects_data_file_modification_when_signed(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(open_container)]
    #[with("test.asice")]
    mut container: Container,
    #[case] operation: fn(&mut Container, &Path) -> Result<(), Error>,
) -> Result<(), Box<dyn StdError>> {
    let extra_file = common::write_file(temp_dir.path(), "extra.txt", b"x")?;

    let result = operation(&mut container, &extra_file);

    assert_general_digidoc_err!(result);
    assert_data_files(&container, &[("test1.txt", 5, "file")])
}

#[rstest]
#[case::directory_in_name("folder/test.txt")]
#[case::parent_directory("../test.txt")]
#[case::backslash("a\\b.txt")]
#[case::dot(".")]
#[case::empty_name("")]
#[case::reserved_name("mimetype")]
#[serial]
fn rejects_invalid_data_file_from_reader(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
    #[case] file_name: &str,
) -> Result<(), Box<dyn StdError>> {
    let result =
        container.add_data_file_from_reader(file_name, MEDIA_TYPE_TEXT_PLAIN, Cursor::new(b"x"));

    assert_general_digidoc_err!(result);
    assert_data_files(&container, &[])
}

#[rstest]
#[serial]
fn removes_data_files_one_by_one(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    let first = common::write_file(temp_dir.path(), "first.txt", b"x")?;
    container.add_data_file(&first, MEDIA_TYPE_TEXT_PLAIN)?;
    let first_file_expectation = ("first.txt", 1, MEDIA_TYPE_TEXT_PLAIN);

    let second = common::write_file(temp_dir.path(), "second.bin", b"x")?;
    container.add_data_file(&second, MEDIA_TYPE_TEXT_PLAIN)?;
    let second_file_expectation = ("second.bin", 1, MEDIA_TYPE_TEXT_PLAIN);

    assert_data_files(
        &container,
        &[first_file_expectation, second_file_expectation],
    )?;

    container.remove_data_file(0)?;
    assert_data_files(&container, &[second_file_expectation])?;

    container.remove_data_file(0)?;
    assert_data_files(&container, &[])
}

#[rstest]
#[case::empty_container(&[], 0)]
#[case::too_high(&["accepted.txt"], 1)]
#[serial]
fn rejects_removing_data_file_if_index_invalid(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
    #[case] existing_files: &[&str],
    #[case] index: usize,
) -> Result<(), Box<dyn StdError>> {
    for file_name in existing_files {
        let path = common::write_file(temp_dir.path(), file_name, b"x")?;
        container.add_data_file(&path, MEDIA_TYPE_TEXT_PLAIN)?;
    }

    let result = container.remove_data_file(index);

    assert_general_digidoc_err!(result);
    let expected: Vec<_> = existing_files
        .iter()
        .map(|file_name| (*file_name, 1, MEDIA_TYPE_TEXT_PLAIN))
        .collect();
    assert_data_files(&container, &expected)
}

#[cfg(target_pointer_width = "64")]
#[rstest]
#[serial]
fn rejects_index_unless_fits_32_bits(
    #[allow(unused_variables)]
    #[from(init_with_valid_tsl)]
    temp_dir: TempDir,
    #[with(temp_dir.path())] mut create_container_with_accepted_txt: Container,
) -> Result<(), Box<dyn StdError>> {
    let too_large = usize::try_from(u64::from(u32::MAX) + 1)?;

    let result = create_container_with_accepted_txt.remove_data_file(too_large);

    assert_internal_err!(result);
    assert_holds_only_the_accepted_file(&create_container_with_accepted_txt)
}

#[rstest]
#[case::save_as(|data_file: &DataFile, path: &Path| data_file.save_as(path))]
#[serial]
fn rejects_non_utf8_path(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(open_container)]
    #[with("test.asice")]
    container: Container,
    #[case] operation: fn(&DataFile, &Path) -> Result<(), Error>,
) -> Result<(), Box<dyn StdError>> {
    let result = operation(&container.data_files()?[0], &common::non_utf8_path());

    assert!(assert_internal_err!(result).contains("UTF-8"));
    Ok(())
}

fn container_with_file_from_reader(
    file_name: &str,
    media_type: &str,
    reader: impl Read + Seek + 'static,
) -> Container {
    let mut container = create_empty_asice_container();
    container
        .add_data_file_from_reader(file_name, media_type, reader)
        .unwrap();

    container
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

#[rstest]
#[case::calc_digest(
    |data_file: &DataFile, _: &Path| data_file.calc_digest(DigestAlgorithm::Sha256).map(|_| ()),
    FailingReader::failing_reads(),
    ErrorKind::BrokenPipe,
    "scripted read failure"
)]
#[case::write_to(
    |data_file: &DataFile, _: &Path| data_file.write_to(Vec::new()),
    FailingReader::failing_reads(),
    ErrorKind::BrokenPipe,
    "scripted read failure"
)]
#[case::save_as(
    |data_file: &DataFile, directory: &Path| data_file.save_as(directory.join("saved.bin")),
    FailingReader::failing_reads(),
    ErrorKind::BrokenPipe,
    "scripted read failure"
)]
#[case::file_size(
    |data_file: &DataFile, _: &Path| data_file.file_size().map(|_| ()),
    FailingReader::failing_seeks(),
    ErrorKind::Unsupported,
    "scripted seek failure"
)]
#[serial]
fn reports_failing_reader(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[case] operation: fn(&DataFile, &Path) -> Result<(), Error>,
    #[case] reader: FailingReader,
    #[case] expected_kind: ErrorKind,
    #[case] expected_message: &str,
) -> Result<(), Box<dyn StdError>> {
    let container = container_with_file_from_reader("data.bin", MEDIA_TYPE_OCTET_STREAM, reader);

    let result = operation(&container.data_files()?[0], temp_dir.path());

    assert_io_error(&result, expected_kind, expected_message);
    Ok(())
}

#[rstest]
#[serial]
fn never_uses_failed_reader_twice(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let reader = FailingReader::failing_reads();
    let calls = Rc::clone(&reader.calls);
    let container = container_with_file_from_reader("data.bin", MEDIA_TYPE_OCTET_STREAM, reader);
    let data_file = &container.data_files()?[0];

    data_file
        .calc_digest(DigestAlgorithm::Sha256)
        .expect_err("the first call should fail");
    let calls_after_failure = calls.get();
    let result = data_file.calc_digest(DigestAlgorithm::Sha256);

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

#[rstest]
#[case::calc_digest(|data_file: &DataFile, _: &Path| data_file.calc_digest(DigestAlgorithm::Sha256).map(|_| ()))]
#[case::file_size(|data_file: &DataFile, _: &Path| data_file.file_size().map(|_| ()))]
#[case::write_to(|data_file: &DataFile, _: &Path| data_file.write_to(Vec::new()))]
#[case::save_as(|data_file: &DataFile, directory: &Path| data_file.save_as(directory.join("saved.bin")))]
#[serial]
fn transfers_reader_panic(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[case] operation: fn(&DataFile, &Path) -> Result<(), Error>,
) -> Result<(), Box<dyn StdError>> {
    let container = container_with_file_from_reader(
        "data.bin",
        MEDIA_TYPE_OCTET_STREAM,
        PanickingReader::default(),
    );
    let data_file = &container.data_files()?[0];

    let outcome = panic::catch_unwind(AssertUnwindSafe(|| operation(data_file, temp_dir.path())));

    let payload = outcome.err().unwrap();
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"scripted panic"));
    Ok(())
}

#[rstest]
#[serial]
fn never_uses_panicked_reader_twice(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let reader = PanickingReader::default();
    let calls = Rc::clone(&reader.calls);
    let container = container_with_file_from_reader("data.bin", MEDIA_TYPE_OCTET_STREAM, reader);
    let data_file = &container.data_files()?[0];

    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        data_file.calc_digest(DigestAlgorithm::Sha256)
    }));
    let payload = outcome.err().unwrap();
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"scripted panic"));
    let calls_after_panic = calls.get();
    let result = data_file.calc_digest(DigestAlgorithm::Sha256);

    assert!(
        matches!(result, Err(Error::Io(_))),
        "expected io error, got {result:?}"
    );
    assert_eq!(calls.get(), calls_after_panic, "reader used after panic");

    Ok(())
}

#[rstest]
#[serial]
fn rejects_reentrant_reader(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] container: Container,
) -> Result<(), Box<dyn StdError>> {
    type NestedResult = Result<Vec<u8>, Error>;

    struct ReenteringReader {
        bytes: Cursor<Vec<u8>>,
        container: Weak<RefCell<Container>>,
        nested_result: Rc<RefCell<Option<NestedResult>>>,
    }

    impl Read for ReenteringReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.nested_result.borrow().is_none() {
                let container = self
                    .container
                    .upgrade()
                    .ok_or_else(|| io::Error::other("container dropped"))?;
                let result = container.borrow().data_files().map_err(io::Error::other)?[0]
                    .calc_digest(DigestAlgorithm::Sha256);
                *self.nested_result.borrow_mut() = Some(result);
            }
            self.bytes.read(buffer)
        }
    }

    impl Seek for ReenteringReader {
        fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
            self.bytes.seek(position)
        }
    }

    let container = Rc::new(RefCell::new(container));
    let nested_result = Rc::new(RefCell::new(None));
    let reader = ReenteringReader {
        bytes: Cursor::new(b"1234\n".to_vec()),
        container: Rc::downgrade(&container),
        nested_result: Rc::clone(&nested_result),
    };
    container
        .borrow_mut()
        .add_data_file_from_reader("test1.txt", MEDIA_TYPE_TEXT_PLAIN, reader)?;

    let outer_result = container.borrow().data_files()?[0].calc_digest(DigestAlgorithm::Sha256);

    let nested_result = nested_result.take().expect("reader didnt make nested call");
    assert_io_error(
        &nested_result,
        ErrorKind::Other,
        "the reader was used again while it was already in use",
    );
    assert!(
        outer_result.is_err(),
        "outer read should fail, got {outer_result:?}"
    );

    Ok(())
}

#[rstest]
#[serial]
fn reader_ownership_is_transferred_to_cpp(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    struct TrackedBytes {
        _reference_counter: Rc<()>,
    }

    impl AsRef<[u8]> for TrackedBytes {
        fn as_ref(&self) -> &[u8] {
            b"x"
        }
    }

    let reference_counter = Rc::new(());
    let tracked_reader = || {
        Cursor::new(TrackedBytes {
            _reference_counter: Rc::clone(&reference_counter),
        })
    };

    container.add_data_file_from_reader("removed.bin", MEDIA_TYPE_TEXT_PLAIN, tracked_reader())?;
    assert_eq!(Rc::strong_count(&reference_counter), 2);
    container.remove_data_file(0)?;
    assert_eq!(Rc::strong_count(&reference_counter), 1);

    let result = container.add_data_file_from_reader(
        "folder/rejected.bin",
        MEDIA_TYPE_TEXT_PLAIN,
        tracked_reader(),
    );
    assert_general_digidoc_err!(result);
    assert_eq!(Rc::strong_count(&reference_counter), 1);

    container.add_data_file_from_reader("kept.bin", MEDIA_TYPE_TEXT_PLAIN, tracked_reader())?;
    drop(container);
    assert_eq!(Rc::strong_count(&reference_counter), 1);

    Ok(())
}

#[rstest]
#[case::asice("test.asice", b"1234\n")]
#[case::asics_with_zero_size_contents("test-tera-empty.asics", b"")]
#[serial]
fn writes_exact_content_of_data_file(
    #[from(init_with_valid_tsl)] temp_dir: TempDir,
    #[case] fixture: &str,
    #[case] expected: &[u8],
) -> Result<(), Box<dyn StdError>> {
    let container = Container::open(common::fixture_path(fixture))?;
    let data_file = &container.data_files()?[0];

    let mut written = Vec::new();
    data_file.write_to(&mut written)?;
    assert_eq!(written, expected);

    let saved_path = temp_dir.path().join("saved.bin");
    data_file.save_as(&saved_path)?;
    assert_eq!(fs::read(&saved_path)?, expected);

    Ok(())
}

#[rstest]
#[serial]
fn reports_writer_failures(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
) -> Result<(), Box<dyn StdError>> {
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

    let container = container_with_file_from_reader(
        "data.bin",
        MEDIA_TYPE_OCTET_STREAM,
        Cursor::new(vec![0; 200_000]),
    );
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

#[rstest]
#[serial]
#[expect(clippy::manual_assert_eq, clippy::manual_assert)]
fn writer_panic_is_transmitted_across_ffi(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
) -> Result<(), Box<dyn StdError>> {
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

    let mut expected = vec![0; 200_000];
    StdRng::seed_from_u64(0).fill_bytes(&mut expected);
    let container = container_with_file_from_reader(
        "data.bin",
        MEDIA_TYPE_OCTET_STREAM,
        Cursor::new(expected.clone()),
    );
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
