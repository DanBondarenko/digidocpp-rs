#![allow(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
        clippy::used_underscore_binding,
        clippy::indexing_slicing
    )
)]
mod common;

use crate::common::{assert_general_digidoc_err, conf_for_testing, dir_with_fixtures};
use digidocpp::conf::{Conf, LogLevel};
use digidocpp::{Container, Library, Uninitialized};
use rstest::rstest;
use std::error::Error as StdError;
use std::fs;
use tempfile::TempDir;

#[rstest]
fn config_file_tsl_settings_take_effect(
    #[from(dir_with_fixtures)]
    #[with(&["EE_T.xml"])]
    directory: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let config_file = directory.path().join("digidocpp.conf");
    let path = common::path_to_string(directory.path());
    fs::write(
        &config_file,
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <configuration xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="schema/conf.xsd">
                <param name="tsl.autoupdate" lock="false">false</param>
                <param name="tsl.onlineDigest" lock="false">false</param>
                <param name="tsl.cache" lock="false">{path}</param>
            </configuration>
            "#
        ),
    )?;

    let mut library = Library::<Uninitialized>::acquire()?;
    let mut conf = Conf::from_file(&library, config_file)?;
    conf.set_tsl_url(common::path_to_string(directory.path().join("EE_T.xml")))?;
    conf.add_tsl_cert(common::der_cert("TSL.der"))?;
    library.configure(conf)?;
    let library = library.initialize()?;

    let container = Container::open(&library, common::fixture_path("tsl.asice"))?;
    let signature = &container.signatures()?[0];

    // signature only valid if using `EE_T.xml`
    signature.validate()?;
    Ok(())
}

#[rstest]
fn writes_log_file(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    cache_directory: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let library = Library::<Uninitialized>::acquire()?;
    let mut conf = conf_for_testing(&library, cache_directory.path(), "TSL.xml");
    let log_directory = TempDir::new()?;
    let log_path = log_directory.path().join("libdigidocpp.log");
    conf.set_log_file(&log_path)?;
    common::initialize_with(library, conf);

    let log = fs::read_to_string(&log_path)?;
    assert!(log.contains("digidocpp version"));
    Ok(())
}

#[rstest]
fn log_level_filters_log_entries(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    directory: TempDir,
) {
    let library = Library::<Uninitialized>::acquire().unwrap();
    let mut conf = conf_for_testing(&library, directory.path(), "TSL.xml");
    conf.set_log_level(LogLevel::Error);
    common::initialize_with(library, conf);

    let log = fs::read_to_string(directory.path().join("libdigidocpp.log")).unwrap_or_default();

    assert!(!log.contains("digidocpp version"));
}

#[test]
fn writes_log_in_non_ascii_directory() -> Result<(), Box<dyn StdError>> {
    let directory = TempDir::new()?;
    let unicode_directory = directory.path().join("libdigidocpp-õ-ä-ö-ü");
    fs::create_dir(&unicode_directory)?;
    common::copy_fixtures(&unicode_directory, &["TSL.xml", "EE_T.xml"])?;
    let library = Library::<Uninitialized>::acquire()?;
    let conf = conf_for_testing(&library, &unicode_directory, "TSL.xml");
    let library = common::initialize_with(library, conf);

    let log = fs::read_to_string(unicode_directory.join("libdigidocpp.log"))?;
    let container = Container::open(&library, common::fixture_path("tsl.asice"))?;
    let signature = &container.signatures()?[0];

    assert!(log.contains("certificates into TSL certificate store"));
    signature.validate()?;
    Ok(())
}

#[rstest]
fn failed_initialize_leaves_library_uninitialized_and_retryable(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    directory: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let mut library = Library::<Uninitialized>::acquire()?;
    let mut conf_without_tsl_certs = conf_for_testing(&library, directory.path(), "TSL.xml");
    conf_without_tsl_certs.clear_tsl_certs();
    library.configure(conf_without_tsl_certs)?;

    let result = library.initialize();
    let error = assert_general_digidoc_err!(result);
    assert!(
        error
            .message()
            .contains("trusted signing certificates empty")
    );

    let mut library = Library::<Uninitialized>::acquire()?;
    let fixed_conf = conf_for_testing(&library, directory.path(), "TSL.xml");
    library.configure(fixed_conf)?;
    let library = library.initialize_with_app_info("retry app")?;
    assert_eq!(library.app_info()?, "retry app");
    Ok(())
}
