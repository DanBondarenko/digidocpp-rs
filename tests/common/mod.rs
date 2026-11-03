#![allow(dead_code, reason = "integration test helpers")]

use digidocpp::conf::Conf;
use digidocpp::signer::Pkcs12Signer;
use digidocpp::{Container, DerCert, Initialized, Library, Uninitialized};
use rstest::fixture;
use std::error::Error as StdError;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

#[allow(unused_macros)]
macro_rules! assert_general_digidoc_err {
    ($val:expr $(,)?) => {
        match $val {
            result => {
                let ::core::result::Result::Err(::digidocpp::Error::Library(::digidocpp::error::LibraryError::DigiDoc(error))) = result else {
                    ::core::panic!("Expected digidoc error, instead got {result:?}");
                };
                ::core::assert_eq!(error.code(), ::core::option::Option::Some(::digidocpp::error::ExceptionCode::General));
                error
            }
        }
    };
    ($val:expr, $($arg:tt)+) => {
        match $val {
            result => {
                let ::core::result::Result::Err(::digidocpp::Error::Library(::digidocpp::error::LibraryError::DigiDoc(error))) = result else {
                    ::core::panic!(
                        "{}: Expected digidoc error, instead got {result:?}",
                        ::core::format_args!($($arg)+)
                    );
                };
                ::core::assert_eq!(error.code(), ::core::option::Option::Some(::digidocpp::error::ExceptionCode::General));
                error
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use assert_general_digidoc_err;

#[allow(unused_macros)]
macro_rules! assert_internal_err {
    ($val:expr $(,)?) => {
        match $val {
            result => {
                let ::core::result::Result::Err(::digidocpp::Error::Internal(message)) = result else {
                    ::core::panic!("Expected internal error, instead got {result:?}");
                };
                message
            }
        }
    };
    ($val:expr, $($arg:tt)+) => {
        match $val {
            result => {
                let ::core::result::Result::Err(::digidocpp::Error::Internal(message)) = result else {
                    ::core::panic!(
                        "{}: Expected internal error, instead got {result:?}",
                        ::core::format_args!($($arg)+)
                    );
                };
                message
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use assert_internal_err;

#[expect(clippy::unnecessary_debug_formatting)]
pub fn fixture_path(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);

    assert!(path.exists(), "Path {path:?} doesn't exist");

    path.into_os_string().into_string().unwrap()
}

pub fn path_to_string(path: impl AsRef<Path>) -> String {
    path.as_ref().to_str().unwrap().to_owned()
}

pub struct TestEnv {
    pub directory: TempDir,
    pub library: Library<Initialized>,
}

#[track_caller]
pub fn initialize_with(mut library: Library<Uninitialized>, conf: Conf) -> Library<Initialized> {
    library.configure(conf).unwrap();
    library.initialize().unwrap()
}

pub fn conf_from_fixture(library: &Library<Uninitialized>, file_name: &str) -> Conf {
    Conf::from_file(library, fixture_path(file_name)).unwrap()
}

pub fn copy_fixtures(directory: &Path, names: &[&str]) -> Result<(), Box<dyn StdError>> {
    for name in names {
        fs::copy(fixture_path(name), directory.join(name))?;
    }

    Ok(())
}

pub fn write_file(directory: &Path, name: &str, contents: &[u8]) -> io::Result<PathBuf> {
    let path = directory.join(name);
    fs::write(&path, contents)?;

    Ok(path)
}

pub fn pkcs12_signer<'lib>(
    library: &'lib Library<Initialized>,
    file_name: &str,
) -> Pkcs12Signer<'lib> {
    // pass of signer files is file name w/o extension
    let password = file_name.strip_suffix(".p12").unwrap();
    Pkcs12Signer::new(library, fixture_path(file_name), password).unwrap()
}

#[track_caller]
pub fn required<T>() -> T {
    panic!()
}

#[fixture]
pub fn der_cert(#[default(required())] file_name: &str) -> DerCert {
    fs::read(fixture_path(file_name)).unwrap().into()
}

pub fn conf_for_testing(
    library: &Library<Uninitialized>,
    directory: &Path,
    tsl_name: &str,
) -> Conf {
    // if we dont load conf from a specific file, `libdigidocpp` searches system dirs
    let mut conf = conf_from_fixture(library, "empty.conf");

    conf.set_log_file(directory.join("libdigidocpp.log"))
        .unwrap();
    conf.set_tsl_url(path_to_string(directory.join(tsl_name)))
        .unwrap();

    // `libdigidocpp` writes here
    conf.set_tsl_cache(directory).unwrap();

    // fixture TSLs accepted only if auto-update off and allow_expired on
    conf.set_tsl_auto_update(false);
    conf.set_tsl_allow_expired(true);

    conf.set_tsl_online_digest(false);
    conf.add_tsl_cert(der_cert("TSL.der")).unwrap();
    conf.set_ts_url("http://demo.sk.ee/tsa/").unwrap();

    conf
}

#[fixture]
pub fn init(
    #[default(required())] tsl_name: &str,
    #[default(&[])] extra_files: &[&str],
) -> TestEnv {
    let directory = dir_with_fixtures(&[&[tsl_name], extra_files].concat());
    let library = Library::<Uninitialized>::acquire().unwrap();
    let conf = conf_for_testing(&library, directory.path(), tsl_name);
    let library = initialize_with(library, conf);

    TestEnv { directory, library }
}

#[fixture]
pub fn init_with_valid_tsl() -> TestEnv {
    init("TSL.xml", &["EE_T.xml"])
}

#[fixture]
pub fn dir_with_fixtures(#[default(required())] names: &[&str]) -> TempDir {
    let directory = TempDir::new().unwrap();
    copy_fixtures(directory.path(), names).unwrap();

    directory
}

pub fn create_unsigned_asice_with_data_file<'lib>(
    library: &'lib Library<Initialized>,
    directory: &Path,
    container_name: &str,
) -> Container<'lib> {
    let data_file = write_file(directory, "test1.txt", b"1234\n").unwrap();
    let mut container = Container::create(library, directory.join(container_name)).unwrap();
    container
        .add_data_file(data_file, MEDIA_TYPE_TEXT_PLAIN)
        .unwrap();

    container
}

pub fn open_container<'lib>(
    library: &'lib Library<Initialized>,
    file_name: &str,
) -> Container<'lib> {
    Container::open(library, fixture_path(file_name)).unwrap()
}

pub fn parse_common_name(der: &DerCert) -> Result<String, Box<dyn StdError>> {
    let (_, certificate) = x509_parser::parse_x509_certificate(der.as_ref())?;
    let common_name = certificate
        .subject()
        .iter_common_name()
        .next()
        .ok_or("no common name")?
        .as_str()?;

    Ok(common_name.to_owned())
}

#[track_caller]
pub fn assert_is_recent(timestamp: &str) {
    const MAX_CLOCK_DIFFERENCE: Duration = Duration::minutes(5);

    let parsed = OffsetDateTime::parse(timestamp, &Rfc3339).unwrap();
    let difference = (OffsetDateTime::now_utc() - parsed).abs();

    assert!(difference <= MAX_CLOCK_DIFFERENCE);
}

pub const MEDIA_TYPE_OCTET_STREAM: &str = "application/octet-stream";
pub const MEDIA_TYPE_TEXT_PLAIN: &str = "text/plain";

#[cfg(unix)]
pub fn non_utf8_path() -> PathBuf {
    use std::os::unix::ffi::OsStringExt;

    PathBuf::from(OsString::from_vec(b"invalid-\xff.asice".to_vec()))
}

#[cfg(windows)]
pub fn non_utf8_path() -> PathBuf {
    use std::os::windows::ffi::OsStringExt;

    // 0xD800 is a lone surrogate, so it is not valid UTF-16.
    PathBuf::from(OsString::from_wide(&[0x0069, 0xD800, 0x002E, 0x0061]))
}
