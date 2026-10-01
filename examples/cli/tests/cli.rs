// these tests use real APIs over network
use anyhow::Result;
use predicate::str;
use predicates::prelude::*;
use rstest::rstest;
use serial_test::serial;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_digidocpp-example-cli"))
}

#[rstest]
#[case::signed_by_person(
    "tsl.asice",
    0,
    "All signatures are valid.",
    "signature is valid",
    "MÄNNIK,MARI-LIIS,47101010033",
    "2016-11-28T13:46:43Z"
)]
#[case::time_stamp(
    "test.asics",
    0,
    "All signatures are valid.",
    "signature is valid",
    "DEMO of SK TSA 2014",
    "2016-11-02T11:07:45Z"
)]
#[case::invalid_time_stamp(
    "test-invalidts.asics",
    1,
    "Invalid signatures detected.",
    "signature is not valid",
    "DEMO of SK TSA 2014",
    "2016-11-02T11:07:45Z"
)]
#[serial]
fn validates(
    #[case] fixture: &str,
    #[case] exit_code: i32,
    #[case] summary: &str,
    #[case] validity: &str,
    #[case] signed_by: &str,
    #[case] signed_at: &str,
) -> Result<()> {
    let container = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    let output = cli().arg("validate").arg(container).output()?;
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
    "tsl.asice",
    "type: AsicE",
    "file: test.txt (4B)",
    "signature: MÄNNIK,MARI-LIIS,47101010033 at 2016-11-28T13:46:43Z (valid)"
)]
#[case::time_stamp(
    "test.asics",
    "type: AsicS",
    "file: test1.txt (5B)",
    "signature: DEMO of SK TSA 2014 at 2016-11-02T11:07:45Z (valid)"
)]
#[case::invalid_time_stamp(
    "test-invalidts.asics",
    "type: AsicS",
    "file: test2.txt (10B)",
    "signature: DEMO of SK TSA 2014 at 2016-11-02T11:07:45Z (invalid)"
)]
#[serial]
fn shows_info(
    #[case] fixture: &str,
    #[case] container_type: &str,
    #[case] data_file: &str,
    #[case] signature: &str,
) -> Result<()> {
    let container = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    let output = cli().arg("info").arg(container).output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;

    let expected = str::contains(container_type)
        .and(str::contains(data_file))
        .and(str::contains(signature));
    assert!(expected.eval(&stdout), "{stdout}");

    Ok(())
}

#[rstest]
#[case::signed_by_person("tsl.asice", "test.txt", "123\n")]
#[case::time_stamp("test.asics", "test1.txt", "1234\n")]
#[case::invalid_time_stamp("test-invalidts.asics", "test2.txt", "1234\n1234\n")]
#[serial]
fn extracts_data_files(
    #[case] fixture: &str,
    #[case] file_name: &str,
    #[case] contents: &str,
) -> Result<()> {
    let container = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let directory = TempDir::new()?;
    let extracted = directory.path().join("extracted");

    let output = cli()
        .arg("extract")
        .arg(container)
        .arg("--output")
        .arg(&extracted)
        .output()?;

    assert!(output.status.success());
    assert_eq!(fs::read_to_string(extracted.join(file_name))?, contents);

    Ok(())
}

#[test]
#[serial]
fn wraps_container_with_expired_signature() -> Result<()> {
    let directory = TempDir::new()?;
    let container = directory.path().join("test.asice");
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/test.asice"),
        &container,
    )?;

    let new = directory.path().join("test.asics");
    let output = cli()
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
