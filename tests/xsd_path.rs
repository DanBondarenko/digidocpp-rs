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
use digidocpp::{Library, Uninitialized};
use rstest::rstest;
use std::error::Error as StdError;
use std::fs;
use tempfile::TempDir;

#[rstest]
fn initialize_fails_when_the_xsd_path_has_no_schemas(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    directory: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let empty_schema_directory = TempDir::new()?;
    let mut library = Library::<Uninitialized>::acquire()?;
    let mut conf = conf_for_testing(&library, directory.path(), "TSL.xml");
    conf.set_xsd_path(empty_schema_directory.path())?;
    library.configure(conf)?;

    let result = library.initialize();

    assert_general_digidoc_err!(result);

    let log = fs::read_to_string(directory.path().join("libdigidocpp.log"))?;
    assert!(log.contains("Failed to validate TSL schema"));
    Ok(())
}
