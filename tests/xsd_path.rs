#![allow(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
    )
)]
mod common;

use crate::common::{assert_general_digidoc_err, conf_for_testing, dir_with_fixtures};
use libdigidocpp::{Conf, init_conf, initialize};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use std::fs;
use tempfile::TempDir;

// MUST BE IN SEPARATE FILE: `set_xsd_path` can only be set once per process
#[rstest]
#[serial]
fn initialize_fails_when_the_xsd_path_has_no_schemas(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    directory: TempDir,
    #[from(conf_for_testing)]
    #[with(directory.path(), "TSL.xml")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    let empty_schema_directory = TempDir::new()?;
    conf.set_xsd_path(empty_schema_directory.path())?;
    init_conf(conf)?;

    let result = initialize();

    assert_general_digidoc_err!(result);

    let log = fs::read_to_string(directory.path().join("libdigidocpp.log"))?;
    assert!(log.contains("Failed to validate TSL schema"));
    Ok(())
}
