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

use crate::common::{conf_for_testing, dir_with_fixtures};
use digidocpp::library::{Access, AccessError, acquire, try_acquire};
use digidocpp::{Library, Uninitialized};
use rstest::rstest;
use std::error::Error;
use tempfile::TempDir;

#[rstest]
#[case::acquire(acquire)]
#[case::try_acquire(try_acquire)]
fn access_is_refused_after_terminate(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    dir: TempDir,
    #[case] acquire_again: fn() -> Result<Access, AccessError>,
) -> Result<(), Box<dyn Error>> {
    let library = Library::<Uninitialized>::acquire()?;
    let conf = conf_for_testing(&library, dir.path(), "TSL.xml");
    common::initialize_with(library, conf).terminate()?;

    let result = acquire_again();
    assert_eq!(result.unwrap_err(), AccessError::Terminated);
    Ok(())
}
