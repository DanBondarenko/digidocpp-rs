#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::redundant_pub_crate
    )
)]
mod common;

use crate::common::assert_general_digidoc_err;
use libdigidocpp::{init_conf, initialize};
use std::error::Error as StdError;
use std::fs;
use tempfile::TempDir;

// MUST BE IN SEPARATE FILE: `set_xsd_path` can only be set once per process
#[test]
fn initialize_fails_when_the_xsd_path_has_no_schemas() -> Result<(), Box<dyn StdError>> {
    let directory = TempDir::new()?;
    common::copy_fixtures(directory.path(), &["TSL.xml"])?;
    let mut conf = common::offline_conf(directory.path(), "TSL.xml")?;
    let empty_schema_directory = TempDir::new()?;
    conf.set_xsd_path(empty_schema_directory.path())?;
    init_conf(conf)?;

    let result = initialize();

    assert_general_digidoc_err!(result);

    let log = fs::read_to_string(directory.path().join("libdigidocpp.log"))?;
    assert!(log.contains("Failed to validate TSL schema"));
    Ok(())
}
