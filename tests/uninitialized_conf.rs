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

use crate::common::{assert_internal_err, conf_from_fixture, der_cert, dir_with_fixtures};
use libdigidocpp::{Conf, Error, ExposeSecret};
use rstest::rstest;
use serial_test::serial;
use std::error::Error as StdError;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// MUST BE IN SEPARATE FILE: no test here may initialize the library
const SHA512_URI: &str = "http://www.w3.org/2001/04/xmlenc#sha512";
const DEFAULT_OCSP_TM_PROFILE: &str = "1.3.6.1.4.1.10015.4.1.2";

#[rstest]
#[serial]
fn gets_values_from_config_file(
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    conf: Conf,
) -> Result<(), Box<dyn StdError>> {
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

#[rstest]
#[serial]
fn verify_service_uri_defaults_to_https(
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    let url = conf.verify_service_uri()?;

    assert!(url.starts_with("https://"), "{url}");
    Ok(())
}

#[rstest]
#[serial]
fn gets_unset_values_from_config_file(
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    conf.set_proxy_host("other-host")?;

    assert_eq!(conf.proxy_port()?, "port");
    assert_eq!(conf.proxy_user()?, "user");
    assert_eq!(conf.log_level()?, 2);
    assert_eq!(conf.digest_uri()?, SHA512_URI);
    assert_eq!(conf.ocsp_url("ISSUER NAME")?, "http://ocsp.issuer.com");
    Ok(())
}

#[rstest]
#[serial]
fn each_setter_overrides_matching_getter(
    #[from(dir_with_fixtures)]
    #[with(&[])]
    directory: TempDir,
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    let tsl_allow_expired = conf.tsl_allow_expired()?;
    let tsl_auto_update = conf.tsl_auto_update()?;
    let tsl_online_digest = conf.tsl_online_digest()?;
    let proxy_tunnel_ssl = conf.proxy_tunnel_ssl()?;

    conf.set_log_level(1);
    conf.set_log_file(directory.path().join("set.log"))?;
    conf.set_tsl_url("https://tsl.example/lotl.xml")?;
    conf.set_ts_url("https://ts.example/tsa")?;
    conf.set_tsl_cache(directory.path().join("tsl-cache"))?;
    conf.set_xsd_path(directory.path().join("schemas"))?;
    conf.set_tsl_timeout(17);
    conf.set_proxy_host("set-host")?;
    conf.set_proxy_port("8080")?;
    conf.set_proxy_user("set-user")?;
    conf.set_proxy_pass("set-pass")?;
    conf.set_verify_service_uri("https://verify.example/validate")?;
    conf.set_tsl_allow_expired(!tsl_allow_expired);
    conf.set_tsl_auto_update(!tsl_auto_update);
    conf.set_tsl_online_digest(!tsl_online_digest);
    conf.set_proxy_tunnel_ssl(!proxy_tunnel_ssl);

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

#[rstest]
#[serial]
fn clearing_and_adding_certificates_sets_each_certificate_list(
    #[from(der_cert)]
    #[with("TSL.der")]
    cert: Vec<u8>,
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    assert!(!conf.tsl_certs()?.is_empty());
    conf.clear_tsl_certs();
    assert!(conf.tsl_certs()?.is_empty());

    conf.add_tsl_cert(&cert)?;
    conf.add_ts_cert(&cert)?;
    conf.add_verify_service_cert(&cert)?;

    assert_eq!(conf.tsl_certs()?, vec![cert.clone()]);
    assert_eq!(conf.ts_certs()?, vec![cert.clone()]);
    assert_eq!(conf.verify_service_certs()?, vec![cert]);
    Ok(())
}

#[rstest]
#[serial]
fn first_certificate_replaces_defaults(
    #[from(der_cert)]
    #[with("TSL.der")]
    cert: Vec<u8>,
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    assert!(!conf.tsl_certs()?.is_empty());

    conf.add_tsl_cert(&cert)?;

    assert_eq!(conf.tsl_certs()?, vec![cert]);
    Ok(())
}

#[rstest]
#[serial]
fn appends_certificates(
    #[from(der_cert)]
    #[with("TSL.der")]
    cert: Vec<u8>,
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    conf.clear_tsl_certs();
    conf.add_tsl_cert(&cert)?;
    conf.add_tsl_cert(&cert)?;

    assert_eq!(conf.tsl_certs()?, vec![cert.clone(), cert]);
    Ok(())
}

#[rstest]
#[case::empty(b"")]
#[case::not_certificate(b"not certificate")]
#[serial]
fn rejects_adding_bad_certificate(
    #[from(der_cert)]
    #[with("TSL.der")]
    cert: Vec<u8>,
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
    #[case] bad_certificate: &[u8],
) -> Result<(), Box<dyn StdError>> {
    let defaults = conf.tsl_certs()?;

    assert!(conf.add_tsl_cert(bad_certificate).is_err());
    assert_eq!(conf.tsl_certs()?, defaults);

    conf.add_tsl_cert(&cert)?;

    assert!(conf.add_tsl_cert(bad_certificate).is_err());
    assert_eq!(conf.tsl_certs()?, vec![cert]);
    Ok(())
}

#[rstest]
#[serial]
fn ocsp_urls_replace_config_file_urls_and_keep_last(
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    conf.set_ocsp_urls([
        ("issuer", "http://first.example"),
        ("other issuer", "http://other.example"),
        ("issuer", "http://last.example"),
    ])?;

    assert_eq!(conf.ocsp_url("issuer")?, "http://last.example");
    assert_eq!(conf.ocsp_url("other issuer")?, "http://other.example");
    assert!(conf.ocsp_url("ISSUER NAME")?.is_empty());
    Ok(())
}

#[rstest]
#[serial]
fn ocsp_tm_profiles_replace_defaults(
    #[from(conf_from_fixture)]
    #[with("digidocpp.conf")]
    mut conf: Conf,
) -> Result<(), Box<dyn StdError>> {
    let custom_profile = "1.3.6.1.4.1.10015.3.1.1";
    let profiles = [custom_profile, "1.3.6.1.4.1.10015.4.1.2"];

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

fn assert_no_settings_loaded(conf: &Conf) -> Result<(), Box<dyn StdError>> {
    assert_eq!(conf.proxy_host()?, "");
    assert_eq!(conf.log_file()?, Path::new(""));
    assert_ne!(conf.digest_uri()?, SHA512_URI);
    Ok(())
}

#[rstest]
#[serial]
fn ignores_missing_config_file(
    #[from(dir_with_fixtures)]
    #[with(&[])]
    directory: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let conf = Conf::from_file(directory.path().join("missing.conf"))?;

    assert_no_settings_loaded(&conf)
}

#[rstest]
#[case::empty("")]
#[case::invalid("this is not xml")]
#[serial]
fn ignores_unusable_config_files(
    #[from(dir_with_fixtures)]
    #[with(&[])]
    directory: TempDir,
    #[case] contents: &str,
) -> Result<(), Box<dyn StdError>> {
    let config_file = directory.path().join("unusable.conf");
    fs::write(&config_file, contents)?;

    let conf = Conf::from_file(&config_file)?;

    assert_no_settings_loaded(&conf)
}

#[rstest]
#[serial]
fn reads_config_file_in_non_ascii_dir(
    #[from(dir_with_fixtures)]
    #[with(&[])]
    directory: TempDir,
) -> Result<(), Box<dyn StdError>> {
    let unicode_directory = directory.path().join("libdigidocpp-õ-ä-ö-ü");
    fs::create_dir(&unicode_directory)?;
    let config_file = unicode_directory.join("digidocpp.conf");
    fs::copy(common::fixture_path("digidocpp.conf"), &config_file)?;

    let conf = Conf::from_file(config_file)?;

    assert_eq!(conf.log_level()?, 2);
    assert_eq!(conf.proxy_host()?, "host");
    Ok(())
}

#[rstest]
#[case::from_file(|path: &Path| Conf::from_file(path).map(drop))]
#[case::set_log_file(|path: &Path| conf_from_fixture("empty.conf").set_log_file(path))]
#[case::from_file_with_bad_config(|path: &Path| {
    Conf::from_file_with_schema(path, "schema.xsd").map(drop)
})]
#[case::from_file_with_bad_schema(|path: &Path| {
    Conf::from_file_with_schema("digidocpp.conf", path).map(drop)
})]
#[case::with_schema(|path: &Path| Conf::with_schema(path).map(drop))]
#[case::set_tsl_cache(|path: &Path| conf_from_fixture("empty.conf").set_tsl_cache(path))]
#[case::set_xsd_path(|path: &Path| conf_from_fixture("empty.conf").set_xsd_path(path))]
#[serial]
fn rejects_non_utf8_path(#[case] operation: fn(&Path) -> Result<(), Error>) {
    let result = operation(&common::non_utf8_path());

    assert!(assert_internal_err!(result).contains("UTF-8"));
}
