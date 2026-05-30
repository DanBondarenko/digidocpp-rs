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

use libdigidocpp::{app_info, init_conf, initialize_with_user_agent, terminate, user_agent};
use std::error::Error;
use tempfile::TempDir;

// MUST BE IN SEPARATE FILE: `terminate` breaks `libdigidocpp` for process duration
#[test]
fn terminate_clears_library_info() -> Result<(), Box<dyn Error>> {
    let directory = TempDir::new()?;
    common::copy_fixtures(directory.path(), &["TSL.xml"])?;
    let conf = common::offline_conf(directory.path(), "TSL.xml")?;
    init_conf(conf)?;
    initialize_with_user_agent("expected app info", "expected user agent")?;
    assert_eq!(app_info()?, "expected app info");
    assert_eq!(user_agent()?, "expected user agent");

    terminate()?;

    assert!(app_info()?.is_empty());
    assert!(user_agent()?.is_empty());
    Ok(())
}
