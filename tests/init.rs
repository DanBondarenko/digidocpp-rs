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

use libdigidocpp::{
    app_info, initialize_with_app_info, initialize_with_user_agent, user_agent, version,
};
use serial_test::serial;
use std::error::Error;

#[test]
#[serial]
fn gets_library_info() -> Result<(), Box<dyn Error>> {
    libdigidocpp::initialize()?;

    assert!(!version()?.is_empty());
    assert!(!app_info()?.is_empty());
    assert!(!user_agent()?.is_empty());

    Ok(())
}

#[test]
#[serial]
fn sets_and_gets_library_info() -> Result<(), Box<dyn Error>> {
    let expected_app_info = "expected app info";
    let expected_user_agent = "expected user agent";

    initialize_with_user_agent(expected_app_info, expected_user_agent)?;

    assert_eq!(app_info()?, expected_app_info);
    assert_eq!(user_agent()?, expected_user_agent);

    Ok(())
}

#[test]
#[serial]
fn initialize_again_applies_new_settings() -> Result<(), Box<dyn Error>> {
    initialize_with_app_info("first app")?;
    assert_eq!(app_info()?, "first app");

    initialize_with_app_info("second app")?;
    assert_eq!(app_info()?, "second app");

    Ok(())
}
