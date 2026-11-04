#![allow(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
        clippy::used_underscore_binding,
    )
)]
mod common;

use crate::common::{TestEnv, conf_for_testing, dir_with_fixtures, init_with_valid_tsl};
use digidocpp::{Library, Uninitialized};
use rstest::rstest;
use std::error::Error;
use tempfile::TempDir;

#[rstest]
fn gets_library_info(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn Error>> {
    assert!(!library.version()?.is_empty());
    assert!(!library.app_info()?.is_empty());
    assert!(!library.user_agent()?.is_empty());

    Ok(())
}

#[rstest]
fn sets_and_gets_library_info(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    dir: TempDir,
) -> Result<(), Box<dyn Error>> {
    let expected_app_info = "expected app info";
    let expected_user_agent = "expected user agent";

    let mut library = Library::<Uninitialized>::acquire()?;
    let conf = conf_for_testing(&library, dir.path(), "TSL.xml");
    library.configure(conf)?;
    let library = library.initialize_with_user_agent(expected_app_info, expected_user_agent)?;

    assert_eq!(library.app_info()?, expected_app_info);
    assert_eq!(library.user_agent()?, expected_user_agent);

    Ok(())
}
