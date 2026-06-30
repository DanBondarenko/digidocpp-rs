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
use libdigidocpp::{Conf, Container, Error, ExposeSecret, init_conf, initialize};
use serial_test::serial;
use std::error::Error as StdError;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

const SHA512_URI: &str = "http://www.w3.org/2001/04/xmlenc#sha512";
const DEFAULT_OCSP_TM_PROFILE: &str = "1.3.6.1.4.1.10015.4.1.2";

#[test]
#[serial]
fn config_file_tsl_settings_take_effect() -> Result<(), Box<dyn StdError>> {
    let directory = TempDir::new()?;
    common::copy_fixtures(directory.path(), &["EE_T.xml"])?;
    let config_file = directory.path().join("digidocpp.conf");
    fs::write(
        &config_file,
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<configuration xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="schema/conf.xsd">
    <param name="tsl.autoupdate" lock="false">false</param>
    <param name="tsl.onlineDigest" lock="false">false</param>
    <param name="tsl.cache" lock="false">{}</param>
</configuration>
"#,
            common::path_to_string(directory.path())
        ),
    )?;

    let mut conf = Conf::from_file(config_file)?;
    conf.set_tsl_url(common::path_to_string(directory.path().join("EE_T.xml")))?;
    conf.add_tsl_cert(fs::read(common::fixture_path("TSL.der"))?)?;
    init_conf(conf)?;
    initialize()?;

    let container = Container::open(common::fixture_path("tsl.asice"))?;
    let signatures = container.signatures()?;
    let signature = signatures
        .first()
        .expect("tsl.asice should contain a signature");

    // signature only valid if using `EE_T.xml`
    signature.validate()?;
    Ok(())
}

#[test]
#[serial]
fn writes_log_file() -> Result<(), Box<dyn StdError>> {
    let cache_directory = TempDir::new()?;
    common::copy_fixtures(cache_directory.path(), &["TSL.xml"])?;
    let log_directory = TempDir::new()?;
    let mut conf = common::offline_conf(cache_directory.path(), "TSL.xml")?;
    conf.set_log_file(log_directory.path().join("libdigidocpp.log"))?;
    init_conf(conf)?;
    initialize()?;

    let log = fs::read_to_string(log_directory.path().join("libdigidocpp.log"))?;

    assert!(log.contains("digidocpp version"));
    Ok(())
}

#[test]
#[serial]
fn log_level_filters_log_entries() -> Result<(), Box<dyn StdError>> {
    let directory = TempDir::new()?;
    common::copy_fixtures(directory.path(), &["TSL.xml"])?;
    let mut conf = common::offline_conf(directory.path(), "TSL.xml")?;
    conf.set_log_level(0)?;
    init_conf(conf)?;
    initialize()?;

    let log = fs::read_to_string(directory.path().join("libdigidocpp.log")).unwrap_or_default();

    assert!(!log.contains("digidocpp version"));
    Ok(())
}

#[test]
#[serial]
fn loads_tsl_and_writes_log_in_a_non_ascii_directory() -> Result<(), Box<dyn StdError>> {
    let directory = TempDir::new()?;
    let unicode_directory = directory.path().join("libdigidocpp-õ-ä-ö-ü");
    fs::create_dir(&unicode_directory)?;
    common::copy_fixtures(&unicode_directory, &["TSL.xml", "EE_T.xml"])?;
    let conf = common::offline_conf(&unicode_directory, "TSL.xml")?;
    init_conf(conf)?;
    initialize()?;

    let log = fs::read_to_string(unicode_directory.join("libdigidocpp.log"))?;
    let container = Container::open(common::fixture_path("tsl.asice"))?;
    let signatures = container.signatures()?;
    let signature = signatures
        .first()
        .expect("tsl.asice should contain a signature");

    assert!(log.contains("certificates into TSL certificate store"));
    signature.validate()?;
    Ok(())
}

#[test]
#[serial]
fn initialize_fails_without_trusted_tsl_certificates() -> Result<(), Box<dyn StdError>> {
    let directory = TempDir::new()?;
    let mut conf = common::offline_conf(directory.path(), "TSL.xml")?;
    conf.clear_tsl_certs()?;
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

#[test]
#[serial]
fn returns_the_values_from_the_config_file() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let conf = common::conf_from_fixture("digidocpp.conf")?;
    let proxy_pass = conf.proxy_pass()?;

    assert_eq!(conf.log_level()?, 2);
    assert_eq!(conf.log_file()?, Path::new("digidocpp-test.log"));
    assert_eq!(conf.digest_uri()?, SHA512_URI);
    assert_eq!(conf.pkcs11_driver()?, "opensc-pkcs11");
    assert_eq!(conf.proxy_host()?, "host");
    assert_eq!(conf.proxy_port()?, "port");
    assert_eq!(conf.proxy_user()?, "user");
    assert_eq!(proxy_pass.expose_secret(), "pass");
    assert_eq!(conf.ocsp_url("ISSUER NAME")?, "http://ocsp.issuer.com");
    Ok(())
}

#[test]
#[serial]
fn verify_service_uri_defaults_to_an_https_url() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let url = common::conf_from_fixture("digidocpp.conf")?.verify_service_uri()?;

    assert!(url.starts_with("https://"), "{url}");
    Ok(())
}

#[test]
#[serial]
fn unset_values_still_come_from_config_file() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let mut conf = common::conf_from_fixture("digidocpp.conf")?;

    conf.set_proxy_host("other-host")?;

    assert_eq!(conf.proxy_port()?, "port");
    assert_eq!(conf.proxy_user()?, "user");
    assert_eq!(conf.log_level()?, 2);
    assert_eq!(conf.digest_uri()?, SHA512_URI);
    assert_eq!(conf.ocsp_url("ISSUER NAME")?, "http://ocsp.issuer.com");
    Ok(())
}

#[test]
#[serial]
fn each_setter_overrides_the_matching_getter() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let directory = TempDir::new()?;
    let mut conf = common::conf_from_fixture("digidocpp.conf")?;
    let tsl_allow_expired = conf.tsl_allow_expired()?;
    let tsl_auto_update = conf.tsl_auto_update()?;
    let tsl_online_digest = conf.tsl_online_digest()?;
    let proxy_tunnel_ssl = conf.proxy_tunnel_ssl()?;

    conf.set_log_level(1)?;
    conf.set_log_file(directory.path().join("set.log"))?;
    conf.set_tsl_url("https://tsl.example/lotl.xml")?;
    conf.set_ts_url("https://ts.example/tsa")?;
    conf.set_tsl_cache(directory.path().join("tsl-cache"))?;
    conf.set_xsd_path(directory.path().join("schemas"))?;
    conf.set_tsl_timeout(17)?;
    conf.set_proxy_host("set-host")?;
    conf.set_proxy_port("8080")?;
    conf.set_proxy_user("set-user")?;
    conf.set_proxy_pass("set-pass")?;
    conf.set_verify_service_uri("https://verify.example/validate")?;
    conf.set_tsl_allow_expired(!tsl_allow_expired)?;
    conf.set_tsl_auto_update(!tsl_auto_update)?;
    conf.set_tsl_online_digest(!tsl_online_digest)?;
    conf.set_proxy_tunnel_ssl(!proxy_tunnel_ssl)?;

    let proxy_pass = conf.proxy_pass()?;
    assert_eq!(conf.log_level()?, 1);
    assert_eq!(conf.log_file()?, directory.path().join("set.log"));
    assert_eq!(conf.tsl_url()?, "https://tsl.example/lotl.xml");
    assert_eq!(conf.ts_url()?, "https://ts.example/tsa");
    assert_eq!(conf.tsl_cache()?, directory.path().join("tsl-cache"));
    assert_eq!(conf.xsd_path()?, directory.path().join("schemas"));
    assert_eq!(conf.tsl_timeout()?, 17);
    assert_eq!(conf.proxy_host()?, "set-host");
    assert_eq!(conf.proxy_port()?, "8080");
    assert_eq!(conf.proxy_user()?, "set-user");
    assert_eq!(proxy_pass.expose_secret(), "set-pass");
    assert_eq!(
        conf.verify_service_uri()?,
        "https://verify.example/validate"
    );
    assert_eq!(conf.tsl_allow_expired()?, !tsl_allow_expired);
    assert_eq!(conf.tsl_auto_update()?, !tsl_auto_update);
    assert_eq!(conf.tsl_online_digest()?, !tsl_online_digest);
    assert_eq!(conf.proxy_tunnel_ssl()?, !proxy_tunnel_ssl);
    Ok(())
}

#[test]
#[serial]
fn clearing_and_adding_certificates_sets_each_certificate_list() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let certificate = fs::read(common::fixture_path("TSL.der"))?;
    let mut conf = common::conf_from_fixture("digidocpp.conf")?;

    assert!(!conf.tsl_certs()?.is_empty());
    conf.clear_tsl_certs()?;
    assert!(conf.tsl_certs()?.is_empty());

    conf.add_tsl_cert(&certificate)?;
    conf.add_ts_cert(&certificate)?;
    conf.add_verify_service_cert(&certificate)?;

    assert_eq!(conf.tsl_certs()?, vec![certificate.clone()]);
    assert_eq!(conf.ts_certs()?, vec![certificate.clone()]);
    assert_eq!(conf.verify_service_certs()?, vec![certificate]);
    Ok(())
}

#[test]
#[serial]
fn first_added_certificate_replaces_the_defaults() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let certificate = fs::read(common::fixture_path("TSL.der"))?;
    let mut conf = common::conf_from_fixture("digidocpp.conf")?;
    assert!(!conf.tsl_certs()?.is_empty());

    conf.add_tsl_cert(&certificate)?;

    assert_eq!(conf.tsl_certs()?, vec![certificate]);
    Ok(())
}

#[test]
#[serial]
fn later_added_certificates_are_appended() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let certificate = fs::read(common::fixture_path("TSL.der"))?;
    let mut conf = common::conf_from_fixture("digidocpp.conf")?;

    conf.clear_tsl_certs()?;
    conf.add_tsl_cert(&certificate)?;
    conf.add_tsl_cert(&certificate)?;

    assert_eq!(conf.tsl_certs()?, vec![certificate.clone(), certificate]);
    Ok(())
}

#[test]
#[serial]
fn adding_a_bad_certificate_is_an_error_and_changes_nothing() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let certificate = fs::read(common::fixture_path("TSL.der"))?;

    for (label, bad_certificate) in [
        ("empty", &b""[..]),
        ("not a certificate", b"not a certificate"),
    ] {
        let mut conf = common::conf_from_fixture("digidocpp.conf")?;
        let defaults = conf.tsl_certs()?;

        assert!(
            conf.add_tsl_cert(bad_certificate).is_err(),
            "{label}: adding to the default list should fail"
        );
        assert_eq!(
            conf.tsl_certs()?,
            defaults,
            "{label}: the default list should stay unchanged"
        );

        conf.add_tsl_cert(&certificate)?;

        assert!(
            conf.add_tsl_cert(bad_certificate).is_err(),
            "{label}: adding after a valid certificate should fail"
        );
        assert_eq!(
            conf.tsl_certs()?,
            vec![certificate.clone()],
            "{label}: the list should keep only the valid certificate"
        );
    }
    Ok(())
}

#[test]
#[serial]
fn ocsp_urls_replace_the_config_file_urls_and_keep_the_last_duplicate()
-> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let mut conf = common::conf_from_fixture("digidocpp.conf")?;

    conf.set_ocsp_urls([
        ("issuer", "http://first.example"),
        ("other issuer", "http://other.example"),
        ("issuer", "http://last.example"),
    ])?;

    assert_eq!(
        conf.ocsp_url("issuer")?,
        "http://last.example",
        "the last URL for a duplicate issuer should win"
    );
    assert_eq!(
        conf.ocsp_url("other issuer")?,
        "http://other.example",
        "a URL for another issuer should stay"
    );
    assert_eq!(
        conf.ocsp_url("ISSUER NAME")?,
        "",
        "the URL from the config file should be gone"
    );
    Ok(())
}

#[test]
#[serial]
fn ocsp_tm_profiles_replace_the_defaults() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let custom_profile = "1.3.6.1.4.1.10015.3.1.1";
    let profiles = [custom_profile, "1.3.6.1.4.1.10015.4.1.2"];
    let mut conf = common::conf_from_fixture("digidocpp.conf")?;

    assert!(
        conf.ocsp_tm_profiles()?
            .contains(&DEFAULT_OCSP_TM_PROFILE.to_owned())
    );
    conf.set_ocsp_tm_profiles([custom_profile])?;

    assert_eq!(conf.ocsp_tm_profiles()?, [custom_profile]);
    conf.set_ocsp_tm_profiles(profiles)?;
    assert_eq!(conf.ocsp_tm_profiles()?, profiles);
    Ok(())
}

#[test]
#[serial]
fn ignores_missing_empty_and_invalid_config_files() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let directory = TempDir::new()?;
    let empty_file = directory.path().join("empty.conf");
    fs::write(&empty_file, "")?;
    let invalid_file = directory.path().join("invalid.conf");
    fs::write(&invalid_file, "this is not xml")?;

    for path in [
        directory.path().join("missing.conf"),
        empty_file,
        invalid_file,
    ] {
        let conf = Conf::from_file(&path)?;

        assert_eq!(conf.proxy_host()?, "", "{}", path.display());
        assert_eq!(conf.log_file()?, Path::new(""), "{}", path.display());
        assert_ne!(conf.digest_uri()?, SHA512_URI, "{}", path.display());
    }
    Ok(())
}

#[test]
#[serial]
fn reads_config_file_in_a_non_ascii_directory() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let directory = TempDir::new()?;
    let unicode_directory = directory.path().join("libdigidocpp-õ-ä-ö-ü");
    fs::create_dir(&unicode_directory)?;
    let config_file = unicode_directory.join("digidocpp.conf");
    fs::copy(common::fixture_path("digidocpp.conf"), &config_file)?;

    let conf = Conf::from_file(config_file)?;

    assert_eq!(conf.log_level()?, 2);
    assert_eq!(conf.proxy_host()?, "host");
    Ok(())
}

#[test]
#[serial]
fn conf_from_file_rejects_non_utf8_path() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;

    let result = Conf::from_file(common::non_utf8_path());
    let Err(Error::Internal(message)) = result else {
        panic!("Expected internal error, but was {result:?}")
    };
    assert!(message.contains("UTF-8"));

    Ok(())
}

#[test]
#[serial]
fn conf_set_log_file_rejects_non_utf8_path() -> Result<(), Box<dyn StdError>> {
    let _temp_dir = common::initialize_offline("TSL.xml", &[])?;
    let mut conf = common::conf_from_fixture("empty.conf")?;

    let result = conf.set_log_file(common::non_utf8_path());
    let Err(Error::Internal(message)) = result else {
        panic!("Expected internal error, but was {result:?}")
    };
    assert!(message.contains("UTF-8"));

    Ok(())
}
