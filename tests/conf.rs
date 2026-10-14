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
use libdigidocpp::{init_conf, initialize, Conf, Container, LogLevel};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use std::fs;
use tempfile::TempDir;

#[rstest]
#[serial]
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

    let mut conf = Conf::from_file(config_file)?;
    conf.set_tsl_url(common::path_to_string(directory.path().join("EE_T.xml")))?;
    conf.add_tsl_cert(common::der_cert("TSL.der"))?;
    init_conf(conf)?;
    initialize()?;

    let container = Container::open(common::fixture_path("tsl.asice"))?;
    let signature = &container.signatures()?[0];

    // signature only valid if using `EE_T.xml`
    signature.validate()?;
    Ok(())
}

#[rstest]
#[serial]
fn writes_log_file(
    #[allow(unused_variables)]
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    cache_directory: TempDir,
    #[from(conf_for_testing)]
    #[with(cache_directory.path(), "TSL.xml")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    let log_directory = TempDir::new()?;
    let log_path = log_directory.path().join("libdigidocpp.log");
    conf.set_log_file(&log_path)?;
    init_conf(conf)?;
    initialize()?;

    let log = fs::read_to_string(&log_path)?;
    assert!(log.contains("digidocpp version"));
    Ok(())
}

#[rstest]
#[serial]
fn log_level_filters_log_entries(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    directory: TempDir,
    #[from(conf_for_testing)]
    #[with(directory.path(), "TSL.xml")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    conf.set_log_level(LogLevel::Error);
    init_conf(conf)?;
    initialize()?;

    let log = fs::read_to_string(directory.path().join("libdigidocpp.log")).unwrap_or_default();

    assert!(!log.contains("digidocpp version"));
    Ok(())
}

#[test]
#[serial]
fn writes_log_in_non_ascii_directory() -> Result<(), Box<dyn StdError>> {
    let directory = TempDir::new()?;
    let unicode_directory = directory.path().join("libdigidocpp-õ-ä-ö-ü");
    fs::create_dir(&unicode_directory)?;
    common::copy_fixtures(&unicode_directory, &["TSL.xml", "EE_T.xml"])?;
    let conf = common::conf_for_testing(&unicode_directory, "TSL.xml");
    init_conf(conf)?;
    initialize()?;

    let log = fs::read_to_string(unicode_directory.join("libdigidocpp.log"))?;
    let container = Container::open(common::fixture_path("tsl.asice"))?;
    let signature = &container.signatures()?[0];

    assert!(log.contains("certificates into TSL certificate store"));
    signature.validate()?;
    Ok(())
}

#[rstest]
#[serial]
fn initialize_fails_without_trusted_tsl_certificate(
    #[allow(unused_variables)]
    #[from(dir_with_fixtures)]
    #[with(&[])]
    directory: TempDir,
    #[from(conf_for_testing)]
    #[with(directory.path(), "TSL.xml")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    conf.clear_tsl_certs();
    init_conf(conf)?;

    let result = initialize();
    let error = assert_general_digidoc_err!(result);
    assert!(
        error
            .message()
            .contains("trusted signing certificates empty")
    );

    Ok(())
}
