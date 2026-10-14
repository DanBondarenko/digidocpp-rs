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

use crate::common::{conf_for_testing, dir_with_fixtures, init_with_valid_tsl};
use digidocpp::{
    Conf, app_info, init_conf, initialize_with_app_info, initialize_with_user_agent, user_agent,
    version,
};
use rstest::rstest;
use serial_test::serial;
use std::error::Error;
use tempfile::TempDir;

#[rstest]
#[serial]
fn gets_library_info(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
) -> Result<(), Box<dyn Error>> {
    assert!(!version()?.is_empty());
    assert!(!app_info()?.is_empty());
    assert!(!user_agent()?.is_empty());

    Ok(())
}

#[rstest]
#[serial]
fn sets_and_gets_library_info(
    #[from(dir_with_fixtures)]
    #[allow(unused_variables)]
    #[with(&["TSL.xml"])]
    dir: TempDir,
    #[from(conf_for_testing)]
    #[with(dir.path(), "TSL.xml")]
    conf: Conf,
) -> Result<(), Box<dyn Error>> {
    let expected_app_info = "expected app info";
    let expected_user_agent = "expected user agent";

    init_conf(conf)?;
    initialize_with_user_agent(expected_app_info, expected_user_agent)?;

    assert_eq!(app_info()?, expected_app_info);
    assert_eq!(user_agent()?, expected_user_agent);

    Ok(())
}

#[test]
#[serial]
fn initialize_again_applies_new_settings() -> Result<(), Box<dyn Error>> {
    let directory = TempDir::new()?;
    common::copy_fixtures(directory.path(), &["TSL.xml"])?;
    let conf = common::conf_for_testing(directory.path(), "TSL.xml");
    init_conf(conf)?;
    initialize_with_app_info("first app")?;
    assert_eq!(app_info()?, "first app");

    let directory = TempDir::new()?;
    common::copy_fixtures(directory.path(), &["TSL.xml"])?;
    let conf = common::conf_for_testing(directory.path(), "TSL.xml");
    init_conf(conf)?;
    initialize_with_app_info("second app")?;
    assert_eq!(app_info()?, "second app");

    Ok(())
}
