use anyhow::{Context, Result};
use predicate::str;
use predicates::prelude::*;
use rstest::rstest;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

#[derive(Clone, Copy)]
enum TslSource {
    Fixture(&'static str),
    #[cfg(feature = "_online_tests")]
    Download,
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn cli(tsl_source: TslSource, temp_dir: &Path) -> Result<Command> {
    let tsl_fixture = match tsl_source {
        TslSource::Fixture(name) => Some(name),
        #[cfg(feature = "_online_tests")]
        TslSource::Download => None,
    };
    let is_online = tsl_fixture.is_none();
    let cache_path = temp_dir.to_str().context("invalid path")?;
    let conf_path = temp_dir.join("test.conf");
    let conf_content = format!(
        r#"<configuration>
            <param name="tsl.autoupdate">{is_online}</param>
            <param name="tsl.onlineDigest">{is_online}</param>
            <param name="tsl.cache">{cache_path}</param>
        </configuration>"#,
    );

    fs::write(&conf_path, conf_content)?;
    if let Some(name) = tsl_fixture {
        fs::copy(fixture_path(name), temp_dir.join("EE_T.xml"))?;
    }

    let mut command = Command::new(env!("CARGO_BIN_EXE_digidocpp-example-cli"));
    command.arg("--conf").arg(conf_path);
    Ok(command)
}

#[rstest]
#[case::signed_by_person(
    TslSource::Fixture("EE_T.xml"),
    "tsl.asice",
    0,
    "All signatures are valid.",
    "signature is valid",
    "MÄNNIK,MARI-LIIS,47101010033",
    "2016-11-28T13:46:43Z"
)]
#[case::signed_by_person_without_qualification(
    TslSource::Fixture("EE_T-no_QCSD.xml"),
    "tsl.asice",
    0,
    "All signatures are valid, but some have warnings or are not qualified.",
    "signature is valid, but not qualified",
    "MÄNNIK,MARI-LIIS,47101010033",
    "2016-11-28T13:46:43Z"
)]
#[case::time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test.asics",
    0,
    "All signatures are valid.",
    "signature is valid",
    "DEMO of SK TSA 2014",
    "2016-11-02T11:07:45Z"
)]
#[case::invalid_time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test-invalidts.asics",
    1,
    "Invalid signatures detected.",
    "signature is not valid",
    "DEMO of SK TSA 2014",
    "2016-11-02T11:07:45Z"
)]
#[cfg_attr(
    feature = "_online_tests",
    case::signed_by_person_online(
        TslSource::Download,
        "tsl.asice",
        0,
        "All signatures are valid.",
        "signature is valid",
        "MÄNNIK,MARI-LIIS,47101010033",
        "2016-11-28T13:46:43Z"
    )
)]
#[serial]
fn validates(
    #[case] tsl_source: TslSource,
    #[case] fixture_file: &str,
    #[case] exit_code: i32,
    #[case] summary: &str,
    #[case] validity: &str,
    #[case] signed_by: &str,
    #[case] signed_at: &str,
) -> Result<()> {
    let directory = TempDir::new()?;

    let output = cli(tsl_source, directory.path())?
        .arg("validate")
        .arg(fixture_path(fixture_file))
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(output.status.code(), Some(exit_code));

    let contains_field = |label, value| {
        str::is_match(format!(
            r"{}\s+{}",
            regex::escape(label),
            regex::escape(value)
        ))
    };
    let expected = str::contains(summary)
        .and(contains_field("validity:", validity)?)
        .and(contains_field("signed by:", signed_by)?)
        .and(contains_field("signed at:", signed_at)?);
    assert!(expected.eval(&stdout), "{stdout}");

    Ok(())
}

#[rstest]
#[case::signed_by_person(
    TslSource::Fixture("EE_T.xml"),
    "tsl.asice",
    "type: AsicE",
    "file: test.txt (4B)",
    "signature: MÄNNIK,MARI-LIIS,47101010033 at 2016-11-28T13:46:43Z (valid)"
)]
#[case::signed_by_person_without_qualification(
    TslSource::Fixture("EE_T-no_QCSD.xml"),
    "tsl.asice",
    "type: AsicE",
    "file: test.txt (4B)",
    "signature: MÄNNIK,MARI-LIIS,47101010033 at 2016-11-28T13:46:43Z (valid, but unqualified)"
)]
#[case::time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test.asics",
    "type: AsicS",
    "file: test1.txt (5B)",
    "signature: DEMO of SK TSA 2014 at 2016-11-02T11:07:45Z (valid)"
)]
#[case::invalid_time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test-invalidts.asics",
    "type: AsicS",
    "file: test2.txt (10B)",
    "signature: DEMO of SK TSA 2014 at 2016-11-02T11:07:45Z (invalid)"
)]
#[cfg_attr(
    feature = "_online_tests",
    case::signed_by_person_online(
        TslSource::Download,
        "tsl.asice",
        "type: AsicE",
        "file: test.txt (4B)",
        "signature: MÄNNIK,MARI-LIIS,47101010033 at 2016-11-28T13:46:43Z (valid)"
    )
)]
#[serial]
fn shows_info(
    #[case] tsl_source: TslSource,
    #[case] fixture_file: &str,
    #[case] container_type: &str,
    #[case] data_file: &str,
    #[case] signature: &str,
) -> Result<()> {
    let directory = TempDir::new()?;

    let output = cli(tsl_source, directory.path())?
        .arg("info")
        .arg(fixture_path(fixture_file))
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;

    let expected = str::contains(container_type)
        .and(str::contains(data_file))
        .and(str::contains(signature));
    assert!(expected.eval(&stdout), "{stdout}");

    Ok(())
}

#[rstest]
#[case::signed_by_person(TslSource::Fixture("EE_T.xml"), "tsl.asice", "test.txt", "123\n")]
#[case::time_stamp(TslSource::Fixture("EE_T.xml"), "test.asics", "test1.txt", "1234\n")]
#[case::invalid_time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test-invalidts.asics",
    "test2.txt",
    "1234\n1234\n"
)]
#[cfg_attr(
    feature = "_online_tests",
    case::signed_by_person_online(TslSource::Download, "tsl.asice", "test.txt", "123\n")
)]
#[serial]
fn extracts_data_files(
    #[case] tsl_source: TslSource,
    #[case] fixture_file: &str,
    #[case] file_name: &str,
    #[case] contents: &str,
) -> Result<()> {
    let directory = TempDir::new()?;
    let extracted = directory.path().join("extracted");

    let output = cli(tsl_source, directory.path())?
        .arg("extract")
        .arg(fixture_path(fixture_file))
        .arg("--output")
        .arg(&extracted)
        .output()?;

    assert!(output.status.success());
    assert_eq!(fs::read_to_string(extracted.join(file_name))?, contents);

    Ok(())
}

#[cfg(feature = "_online_tests")]
#[test]
#[serial]
fn wraps_container_with_expired_signature() -> Result<()> {
    let directory = TempDir::new()?;
    let container = directory.path().join("test.asice");
    fs::copy(fixture_path("test.asice"), &container)?;

    let new = directory.path().join("test.asics");
    let output = cli(TslSource::Download, directory.path())?
        .arg("extend")
        .arg(&container)
        .arg("--output")
        .arg(&new)
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Wrapped"), "{stdout}");
    assert!(new.exists());

    Ok(())
}
