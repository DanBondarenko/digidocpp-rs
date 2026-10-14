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
use digidocpp::{Conf, app_info, init_conf, initialize_with_user_agent, terminate, user_agent};
use rstest::rstest;
use serial_test::serial;
use std::error::Error;
use tempfile::TempDir;

// MUST BE IN SEPARATE FILE: `terminate` breaks `libdigidocpp` for process duration
#[rstest]
#[serial]
fn terminate_clears_library_info(
    #[from(dir_with_fixtures)]
    #[allow(unused_variables)]
    #[with(&["TSL.xml"])]
    dir: TempDir,
    #[from(conf_for_testing)]
    #[with(dir.path(), "TSL.xml")]
    conf: Conf,
) -> Result<(), Box<dyn Error>> {
    init_conf(conf)?;
    initialize_with_user_agent("expected app info", "expected user agent")?;
    assert_eq!(app_info()?, "expected app info");
    assert_eq!(user_agent()?, "expected user agent");

    // SAFETY: only this test thread uses library, no library object is alive
    unsafe { terminate() }?;

    assert!(app_info()?.is_empty());
    assert!(user_agent()?.is_empty());
    Ok(())
}
