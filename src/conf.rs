//! Config and log level types

use crate::algorithm::Algorithm;
use crate::util::{
    ffi_variant_doc, library_and_ffi_variants_doc, library_variant_doc, path_to_str,
};
use crate::{DerCert, Error};
use cxx::UniquePtr;
use libdigidocpp_sys::conf::ffi;
pub use secrecy::{ExposeSecret, SecretString};
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};
use std::time::Duration;
use strum::FromRepr;

/// libdigidocpp configuration.
///
/// Acts as an overridable wrapper around upstream's [`digidoc::XmlConf`]. Configuration is
/// read from a XML config file, but setters allow values to be overridden. If a value has
/// been set with a setter, then it takes precedence over the configuration file. A value resolves
/// from, in order: a setter on this object, the config file, the library default.
///
/// Install it with [`init_conf`] before initializing the library
/// (e.g. [`initialize`](crate::initialize)). Read more about libdigidocpp's XML-based config,
/// including default parameters for most settings, from [their documentation].
///
/// # Examples
///
/// ```no_run
/// use digidocpp::conf::Conf;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut conf = Conf::new()?;
/// conf.set_tsl_url("https://open-eid.github.io/test-TL/EE_T.xml")?;
/// conf.clear_tsl_certs();
/// conf.add_tsl_cert(std::fs::read("test-tsl.der")?)?;
///
/// digidocpp::init_conf(conf)?;
/// digidocpp::initialize()?;
/// # Ok(())
/// # }
/// ```
/// [`digidoc::XmlConf`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1XmlConfV5.html
/// [their documentation]: https://open-eid.github.io/libdigidocpp/manual.html#conf
pub struct Conf(UniquePtr<ffi::SettableConf>);

impl Conf {
    /// Loads the configuration from the default libdigidocpp locations.
    ///
    /// Upstream libdigidocpp searches for two config files: the system-level file (e.g.
    /// `/etc/digidocpp/digidocpp.conf`) and the user-level file (e.g.
    /// `~/.digidocpp/digidocpp.conf`). Read more about how libdigidocpp handles configuration
    /// files from [their documentation].
    ///
    /// A missing file is skipped, in which case library default configuration is used. A file that
    /// does not parse or does not match the schema is skipped with a logged warning.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    ///
    /// [their documentation]: https://open-eid.github.io/libdigidocpp/manual.html#conf
    pub fn new() -> Result<Self, Error> {
        Self::create("", "")
    }

    /// Loads the configuration from a single XML config file.
    ///
    /// A missing file is skipped, in which case library default configuration is used. A file that
    /// does not parse or does not match the schema is skipped with a logged warning.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::create(path_to_str(path.as_ref())?, "")
    }

    /// Loads the configuration from a single XML config and validates the XML against `schema`.
    ///
    /// A missing file is skipped, in which case library default configuration is used. A file that
    /// does not parse or does not match the schema is skipped with a logged warning.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `path` or `schema` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn from_file_with_schema(
        path: impl AsRef<Path>,
        schema: impl AsRef<Path>,
    ) -> Result<Self, Error> {
        Self::create(path_to_str(path.as_ref())?, path_to_str(schema.as_ref())?)
    }

    /// Loads the configuration from the default libdigidocpp locations, validating XML against
    /// `schema`.
    ///
    /// Upstream libdigidocpp searches for two config files: the system-level file (e.g.
    /// `/etc/digidocpp/digidocpp.conf`) and the user-level file (e.g.
    /// `~/.digidocpp/digidocpp.conf`). Read more about how libdigidocpp handles configuration
    /// files from [their documentation].
    ///
    /// A missing file is skipped, in which case library default configuration is used. A file that
    /// does not parse or does not match the schema is skipped with a logged warning.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `schema` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn with_schema(schema: impl AsRef<Path>) -> Result<Self, Error> {
        Self::create("", path_to_str(schema.as_ref())?)
    }

    fn create(path: &str, schema: &str) -> Result<Self, Error> {
        Ok(Self(ffi::SettableConf::create(path, schema)?))
    }

    /// Sets the log level.
    pub fn set_log_level(&mut self, level: LogLevel) {
        #[expect(clippy::as_conversions)]
        self.0.pin_mut().set_log_level(level as i32);
    }

    /// Sets the log file.
    ///
    /// By default libdigidocpp logs to standard output.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn set_log_file(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_log_file(path_to_str(path.as_ref())?)?)
    }

    /// Sets the URL or file path of the trust list (TSL).
    ///
    /// [`Conf::add_tsl_cert`] sets the matching certificates.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_tsl_url(&mut self, url: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_tsl_url(url.as_ref())?)
    }

    /// Removes all certificates that may sign the TSL, including the built-in ones.
    pub fn clear_tsl_certs(&mut self) {
        self.0.pin_mut().clear_tsl_certs();
    }

    /// Adds a DER certificate that may sign the TSL.
    ///
    /// The first call replaces the built-in certificates instead of extending them.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when `cert` is empty or not a valid certificate.
    #[doc = ffi_variant_doc!()]
    pub fn add_tsl_cert(&mut self, cert: impl AsRef<[u8]>) -> Result<(), Error> {
        Ok(self.0.pin_mut().add_tsl_cert(cert.as_ref())?)
    }

    /// Sets the certificate policy OIDs that identify an OCSP responder for `time-mark` signatures.
    ///
    /// Such a signature has no time-stamp. The OCSP response proves the time.
    ///
    /// Replaces the whole set.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_ocsp_tm_profiles(
        &mut self,
        profiles: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(), Error> {
        let profiles: Vec<String> = profiles
            .into_iter()
            .map(|profile| profile.as_ref().to_owned())
            .collect();

        Ok(self.0.pin_mut().set_ocsp_tm_profiles(&profiles)?)
    }

    /// Sets OCSP responder URLs.
    ///
    /// Each entry is an `(issuer, url)` pair. `issuer` is the common name of the signing
    /// certificate's issuer.
    ///
    /// Replaces the whole set. For an issuer without an entry, libdigidocpp falls back to the OCSP
    /// URL inside the certificate.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_ocsp_urls(
        &mut self,
        urls: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Result<(), Error> {
        let urls: Vec<ffi::OcspUrl> = urls
            .into_iter()
            .map(|(issuer, url)| ffi::OcspUrl {
                issuer: issuer.as_ref().to_owned(),
                url: url.as_ref().to_owned(),
            })
            .collect();

        Ok(self.0.pin_mut().set_ocsp_urls(&urls)?)
    }

    /// Sets the URL of the time-stamp authority (TSA) used when signing.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_ts_url(&mut self, url: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_ts_url(url.as_ref())?)
    }

    /// Removes all extra TLS certificates for the time-stamp service.
    pub fn clear_ts_certs(&mut self) {
        self.0.pin_mut().clear_ts_certs();
    }

    /// Adds a DER certificate to the TLS trust store used to connect to the time-stamp service.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when `cert` is empty or not a valid certificate.
    #[doc = ffi_variant_doc!()]
    pub fn add_ts_cert(&mut self, cert: impl AsRef<[u8]>) -> Result<(), Error> {
        Ok(self.0.pin_mut().add_ts_cert(cert.as_ref())?)
    }

    /// Sets the directory where downloaded TSLs are cached.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn set_tsl_cache(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(self
            .0
            .pin_mut()
            .set_tsl_cache(path_to_str(path.as_ref())?)?)
    }

    /// Sets the directory that holds the XSD schemas libdigidocpp validates against.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// Returns [`Error::Internal`] when `path` is not valid UTF-8.
    #[doc = ffi_variant_doc!()]
    pub fn set_xsd_path(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_xsd_path(path_to_str(path.as_ref())?)?)
    }

    /// Sets whether an expired TSL is accepted.
    pub fn set_tsl_allow_expired(&mut self, enable: bool) {
        self.0.pin_mut().set_tsl_allow_expired(enable);
    }

    /// Sets whether a new TSL is downloaded when the cached one is expired or invalid.
    pub fn set_tsl_auto_update(&mut self, enable: bool) {
        self.0.pin_mut().set_tsl_auto_update(enable);
    }

    /// Sets whether the cached TSL is compared to its online digest.
    ///
    /// The comparison uses the digest published online, to detect a newer version.
    pub fn set_tsl_online_digest(&mut self, enable: bool) {
        self.0.pin_mut().set_tsl_online_digest(enable);
    }

    /// Sets the connection timeout for TSL downloads.
    ///
    /// # Errors
    /// Returns [`Error::Internal`] if `duration` as seconds doesn't fit into i32.
    pub fn set_tsl_timeout(&mut self, duration: impl Into<Duration>) -> Result<(), Error> {
        let seconds = duration
            .into()
            .as_secs()
            .try_into()
            .map_err(|err| Error::Internal(format!("{err}")))?;
        self.0.pin_mut().set_tsl_timeout(seconds);
        Ok(())
    }

    /// Sets the proxy host.
    ///
    /// libdigidocpp uses the proxy only when host and port are both set.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_proxy_host(&mut self, host: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_proxy_host(host.as_ref())?)
    }

    /// Sets the proxy port.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_proxy_port(&mut self, port: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_proxy_port(port.as_ref())?)
    }

    /// Sets the proxy user.
    ///
    /// Proxy authentication needs both user and password.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_proxy_user(&mut self, user: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_proxy_user(user.as_ref())?)
    }

    /// Sets the proxy password.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_proxy_pass(&mut self, pass: impl Into<SecretString>) -> Result<(), Error> {
        let pass = pass.into();

        Ok(self.0.pin_mut().set_proxy_pass(pass.expose_secret())?)
    }

    /// Sets whether SSL traffic is tunnelled through the proxy.
    pub fn set_proxy_tunnel_ssl(&mut self, enable: bool) {
        self.0.pin_mut().set_proxy_tunnel_ssl(enable);
    }

    /// Removes all extra TLS certificates for the verify service.
    pub fn clear_verify_service_certs(&mut self) {
        self.0.pin_mut().clear_verify_service_certs();
    }

    /// Adds a DER certificate to the TLS trust store used to connect to the verify service.
    ///
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when `cert` is empty or not a valid certificate.
    #[doc = ffi_variant_doc!()]
    pub fn add_verify_service_cert(&mut self, cert: impl AsRef<[u8]>) -> Result<(), Error> {
        Ok(self.0.pin_mut().add_verify_service_cert(cert.as_ref())?)
    }

    /// Sets the URL of the SiVa service.
    ///
    /// SiVa validates formats that libdigidocpp cannot validate itself (DDOC, PDF, and ASiC-E or
    /// ASiC-S with CAdES).
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn set_verify_service_uri(&mut self, url: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_verify_service_uri(url.as_ref())?)
    }

    /// Returns the log level.
    ///
    /// See [`Conf::set_log_level`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn log_level(&self) -> Result<ReportedLogLevel, Error> {
        let raw = self.0.log_level()?;
        let level =
            LogLevel::from_repr(raw).map_or(ReportedLogLevel::Other(raw), ReportedLogLevel::Known);
        Ok(level)
    }

    /// Returns the log file.
    ///
    /// Empty means standard output.
    ///
    /// See [`Conf::set_log_file`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn log_file(&self) -> Result<PathBuf, Error> {
        Ok(PathBuf::from(self.0.log_file()?))
    }

    /// Returns the certificate policy OIDs that identify an OCSP responder for `time-mark`
    /// signatures.
    ///
    /// See [`Conf::set_ocsp_tm_profiles`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn ocsp_tm_profiles(&self) -> Result<Vec<String>, Error> {
        Ok(self.0.ocsp_tm_profiles()?)
    }

    /// Returns the OCSP responder URL for the issuer common name `issuer`.
    ///
    /// Empty when none is configured.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn ocsp_url(&self, issuer: &str) -> Result<String, Error> {
        Ok(self.0.ocsp_url(issuer)?)
    }

    /// Returns the directory where downloaded TSLs are cached.
    ///
    /// See [`Conf::set_tsl_cache`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn tsl_cache(&self) -> Result<PathBuf, Error> {
        Ok(PathBuf::from(self.0.tsl_cache()?))
    }

    /// Returns the certificates that may sign the TSL.
    ///
    /// See [`Conf::add_tsl_cert`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn tsl_certs(&self) -> Result<Vec<DerCert>, Error> {
        Ok(build_der_vec(self.0.tsl_certs()?))
    }

    /// Returns the URL or file path of the trust list (TSL).
    ///
    /// See [`Conf::set_tsl_url`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn tsl_url(&self) -> Result<String, Error> {
        Ok(self.0.tsl_url()?)
    }

    /// Returns the directory that holds the XSD schemas libdigidocpp validates against.
    ///
    /// See [`Conf::set_xsd_path`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn xsd_path(&self) -> Result<PathBuf, Error> {
        Ok(PathBuf::from(self.0.xsd_path()?))
    }

    /// Returns the URL of the time-stamp authority (TSA) used when signing.
    ///
    /// See [`Conf::set_ts_url`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn ts_url(&self) -> Result<String, Error> {
        Ok(self.0.ts_url()?)
    }

    /// Returns the extra TLS certificates for the time-stamp service.
    ///
    /// See [`Conf::add_ts_cert`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn ts_certs(&self) -> Result<Vec<DerCert>, Error> {
        Ok(build_der_vec(self.0.ts_certs()?))
    }

    /// Returns whether an expired TSL is accepted.
    ///
    /// See [`Conf::set_tsl_allow_expired`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn tsl_allow_expired(&self) -> Result<bool, Error> {
        Ok(self.0.tsl_allow_expired()?)
    }

    /// Returns whether a new TSL is downloaded when the cached one is expired or invalid.
    ///
    /// See [`Conf::set_tsl_auto_update`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn tsl_auto_update(&self) -> Result<bool, Error> {
        Ok(self.0.tsl_auto_update()?)
    }

    /// Returns whether the cached TSL is compared to its online digest.
    ///
    /// See [`Conf::set_tsl_online_digest`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn tsl_online_digest(&self) -> Result<bool, Error> {
        Ok(self.0.tsl_online_digest()?)
    }

    /// Returns the connection timeout for TSL downloads, in seconds.
    ///
    /// See [`Conf::set_tsl_timeout`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn tsl_timeout(&self) -> Result<i32, Error> {
        Ok(self.0.tsl_timeout()?)
    }

    /// Returns the proxy host.
    ///
    /// See [`Conf::set_proxy_host`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn proxy_host(&self) -> Result<String, Error> {
        Ok(self.0.proxy_host()?)
    }

    /// Returns the proxy port.
    ///
    /// See [`Conf::set_proxy_port`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn proxy_port(&self) -> Result<String, Error> {
        Ok(self.0.proxy_port()?)
    }

    /// Returns the proxy user.
    ///
    /// See [`Conf::set_proxy_user`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn proxy_user(&self) -> Result<String, Error> {
        Ok(self.0.proxy_user()?)
    }

    /// Returns the proxy password.
    ///
    /// See [`Conf::set_proxy_pass`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn proxy_pass(&self) -> Result<SecretString, Error> {
        Ok(SecretString::from(self.0.proxy_pass()?))
    }

    /// Returns whether SSL traffic is redirected over the proxy.
    ///
    /// The default is `false`.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn proxy_force_ssl(&self) -> Result<bool, Error> {
        Ok(self.0.proxy_force_ssl()?)
    }

    /// Returns whether SSL traffic is tunnelled through the proxy.
    ///
    /// See [`Conf::set_proxy_tunnel_ssl`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn proxy_tunnel_ssl(&self) -> Result<bool, Error> {
        Ok(self.0.proxy_tunnel_ssl()?)
    }

    /// Returns the extra TLS certificates for the verify service.
    ///
    /// See [`Conf::add_verify_service_cert`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn verify_service_certs(&self) -> Result<Vec<DerCert>, Error> {
        Ok(build_der_vec(self.0.verify_service_certs()?))
    }

    /// Returns the URL of the SiVa service.
    ///
    /// See [`Conf::set_verify_service_uri`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn verify_service_uri(&self) -> Result<String, Error> {
        Ok(self.0.verify_service_uri()?)
    }

    /// Returns the path of the PKCS #11 driver.
    ///
    /// [`Pkcs11Signer::new`](crate::signer::Pkcs11Signer::new) loads this driver. The default is chosen
    /// when libdigidocpp is built.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn pkcs11_driver(&self) -> Result<String, Error> {
        Ok(self.0.pkcs11_driver()?)
    }

    /// Returns the hash function for [`DigestSetting::Default`](crate::algorithm::DigestSetting::Default).
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn digest_uri(&self) -> Result<Algorithm, Error> {
        Ok(Algorithm::from(self.0.digest_uri()?.as_str()))
    }

    /// Returns the hash function a signer uses unless one is set on it.
    ///
    /// For an EC certificate libdigidocpp currently derives the digest from the key size instead.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn signature_digest_uri(&self) -> Result<Algorithm, Error> {
        Ok(Algorithm::from(self.0.signature_digest_uri()?.as_str()))
    }
}

/// Installs `conf` as the global configuration and drops the previous one.
///
/// Call it before initializing the library (e.g. [`initialize`](crate::initialize)). Otherwise,
/// upstream uses the default, which is the same as [`Conf::new`].
///
/// # Errors
#[doc = library_and_ffi_variants_doc!()]
pub fn init_conf(conf: Conf) -> Result<(), Error> {
    Ok(ffi::init_conf(conf.0)?)
}

fn build_der_vec(certificates: Vec<ffi::DerCert>) -> Vec<DerCert> {
    certificates
        .into_iter()
        .map(|certificate| DerCert(certificate.der))
        .collect()
}

impl Debug for Conf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conf")
            .field("log_level()", &self.log_level())
            .field("log_file()", &self.log_file())
            .field("ocsp_tm_profiles()", &self.ocsp_tm_profiles())
            .field("tsl_cache()", &self.tsl_cache())
            .field(
                "tsl_certs().len()",
                &self.tsl_certs().map(|certificates| certificates.len()),
            )
            .field("tsl_url()", &self.tsl_url())
            .field("xsd_path()", &self.xsd_path())
            .field("ts_url()", &self.ts_url())
            .field(
                "ts_certs().len()",
                &self.ts_certs().map(|certificates| certificates.len()),
            )
            .field("tsl_allow_expired()", &self.tsl_allow_expired())
            .field("tsl_auto_update()", &self.tsl_auto_update())
            .field("tsl_online_digest()", &self.tsl_online_digest())
            .field("tsl_timeout()", &self.tsl_timeout())
            .field("proxy_host()", &self.proxy_host())
            .field("proxy_port()", &self.proxy_port())
            .field("proxy_user()", &self.proxy_user())
            .field("proxy_pass()", &self.proxy_pass())
            .field("proxy_force_ssl()", &self.proxy_force_ssl())
            .field("proxy_tunnel_ssl()", &self.proxy_tunnel_ssl())
            .field(
                "verify_service_certs().len()",
                &self
                    .verify_service_certs()
                    .map(|certificates| certificates.len()),
            )
            .field("verify_service_uri()", &self.verify_service_uri())
            .field("pkcs11_driver()", &self.pkcs11_driver())
            .field("digest_uri()", &self.digest_uri())
            .field("signature_digest_uri()", &self.signature_digest_uri())
            .finish_non_exhaustive()
    }
}

/// Log verbosity, see [`Conf::set_log_level`].
///
/// Each level also enables all levels above it. Upstream has no level that disables logging
#[derive(Clone, Debug, Copy, FromRepr, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum LogLevel {
    /// Errors only.
    Error = 0,
    /// Errors and warnings.
    Warn = 1,
    /// Errors, warnings and informational messages.
    Info = 2,
    /// Everything, including debug output.
    Debug = 3,
}

/// Log level as libdigidocpp reports it, see [`Conf::log_level`].
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReportedLogLevel {
    /// Value that this crate knows.
    Known(LogLevel),
    /// Value outside [`LogLevel`], might originate from XML conf.
    Other(i32),
}
