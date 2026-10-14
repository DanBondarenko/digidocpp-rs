//! Configuration: upstream [`digidoc::Conf`] and [`digidoc::XmlConf`], exposed as one settable
//! object, [`SettableConf`](ffi::SettableConf).
//!
//! [`digidoc::Conf`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1Conf.html
//! [`digidoc::XmlConf`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1XmlConf.html

use std::fmt::{self, Debug, Formatter};

/// Bridge for the settable configuration object.
#[cxx::bridge]
#[allow(clippy::exhaustive_structs, reason = "cxx")]
#[allow(clippy::missing_errors_doc, reason = "cxx")]
pub mod ffi {
    /// A DER-encoded certificate.
    #[namespace = "digidoc_rs::conf"]
    #[derive(Debug, Clone)]
    struct DerCert {
        /// The DER bytes.
        der: Vec<u8>,
    }

    /// An OCSP responder URL for one issuer.
    #[namespace = "digidoc_rs::conf"]
    #[derive(Debug, Clone)]
    struct OcspUrl {
        /// Common name of the issuer.
        issuer: String,
        /// Responder URL.
        url: String,
    }

    #[namespace = "digidoc_rs::conf"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("libdigidocpp-sys/include/conf_ffi.h");

        /// A subclass of upstream [`XmlConf`] with in-memory overrides.
        ///
        /// Each getter returns the override if set, else the config file value, else the
        /// library default. Setters never write a file.
        ///
        /// [`XmlConf`]: https://open-eid.github.io/libdigidocpp/classdigidoc_1_1XmlConf.html
        type SettableConf;

        /// Creates the configuration.
        ///
        /// An empty `path` defaults to system and user config files. An empty `schema` defaults to
        /// `conf.xsd` in the XSD directory.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        #[Self = "SettableConf"]
        fn create(path: &str, schema: &str) -> Result<UniquePtr<SettableConf>>;

        /// Calls upstream `Conf::init`.
        ///
        /// Deletes the previous configuration.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn init_conf(conf: UniquePtr<SettableConf>) -> Result<()>;

        /// Returns the effective value of upstream `Conf::logLevel`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn log_level(self: &SettableConf) -> Result<i32>;

        /// Returns the effective value of upstream `Conf::logFile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn log_file(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `ConfV3::OCSPTMProfiles`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn ocsp_tm_profiles(self: &SettableConf) -> Result<Vec<String>>;

        /// Returns the effective value of upstream `Conf::ocsp`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn ocsp_url(self: &SettableConf, issuer: &str) -> Result<String>;

        /// Returns the effective value of upstream `Conf::TSLCache`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn tsl_cache(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::TSLCerts`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn tsl_certs(self: &SettableConf) -> Result<Vec<DerCert>>;

        /// Returns the effective value of upstream `Conf::TSLUrl`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn tsl_url(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::xsdPath`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn xsd_path(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::TSUrl`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn ts_url(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `ConfV5::TSCerts`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn ts_certs(self: &SettableConf) -> Result<Vec<DerCert>>;

        /// Returns the effective value of upstream `Conf::TSLAllowExpired`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn tsl_allow_expired(self: &SettableConf) -> Result<bool>;

        /// Returns the effective value of upstream `Conf::TSLAutoUpdate`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn tsl_auto_update(self: &SettableConf) -> Result<bool>;

        /// Returns the effective value of upstream `Conf::TSLOnlineDigest`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn tsl_online_digest(self: &SettableConf) -> Result<bool>;

        /// Returns the effective value of upstream `Conf::TSLTimeOut`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn tsl_timeout(self: &SettableConf) -> Result<i32>;

        /// Returns the effective value of upstream `Conf::proxyHost`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn proxy_host(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::proxyPort`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn proxy_port(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::proxyUser`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn proxy_user(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::proxyPass`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn proxy_pass(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::proxyForceSSL`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn proxy_force_ssl(self: &SettableConf) -> Result<bool>;

        /// Returns the effective value of upstream `Conf::proxyTunnelSSL`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn proxy_tunnel_ssl(self: &SettableConf) -> Result<bool>;

        /// Returns the effective value of upstream `ConfV4::verifyServiceCerts`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn verify_service_certs(self: &SettableConf) -> Result<Vec<DerCert>>;

        /// Returns the effective value of upstream `Conf::verifyServiceUri`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn verify_service_uri(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::PKCS11Driver`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn pkcs11_driver(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::digestUri`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn digest_uri(self: &SettableConf) -> Result<String>;

        /// Returns the effective value of upstream `Conf::signatureDigestUri`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn signature_digest_uri(self: &SettableConf) -> Result<String>;

        /// Overrides upstream `Conf::logLevel`.
        fn set_log_level(self: Pin<&mut SettableConf>, level: i32);

        /// Overrides upstream `Conf::logFile`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_log_file(self: Pin<&mut SettableConf>, path: &str) -> Result<()>;

        /// Overrides upstream `Conf::TSLUrl`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_tsl_url(self: Pin<&mut SettableConf>, url: &str) -> Result<()>;

        /// Overrides upstream `Conf::TSLCerts` with an empty list.
        fn clear_tsl_certs(self: Pin<&mut SettableConf>);

        /// Appends to the upstream `Conf::TSLCerts` override.
        ///
        /// The first call replaces inherited certificates.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `der` is empty or
        /// not a valid certificate.
        fn add_tsl_cert(self: Pin<&mut SettableConf>, der: &[u8]) -> Result<()>;

        /// Overrides upstream `ConfV3::OCSPTMProfiles`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_ocsp_tm_profiles(self: Pin<&mut SettableConf>, profiles: &Vec<String>)
        -> Result<()>;

        /// Overrides upstream `Conf::ocsp` with a table.
        ///
        /// Issuers not in `urls` get an empty URL.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_ocsp_urls(self: Pin<&mut SettableConf>, urls: &Vec<OcspUrl>) -> Result<()>;

        /// Overrides upstream `Conf::TSUrl`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_ts_url(self: Pin<&mut SettableConf>, url: &str) -> Result<()>;

        /// Overrides upstream `ConfV5::TSCerts` with an empty list.
        fn clear_ts_certs(self: Pin<&mut SettableConf>);

        /// Appends to the upstream `ConfV5::TSCerts` override.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `der` is empty or
        /// not a valid certificate.
        fn add_ts_cert(self: Pin<&mut SettableConf>, der: &[u8]) -> Result<()>;

        /// Overrides upstream `Conf::TSLCache`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_tsl_cache(self: Pin<&mut SettableConf>, path: &str) -> Result<()>;

        /// Overrides upstream `Conf::xsdPath`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_xsd_path(self: Pin<&mut SettableConf>, path: &str) -> Result<()>;

        /// Overrides upstream `Conf::TSLAllowExpired`.
        fn set_tsl_allow_expired(self: Pin<&mut SettableConf>, enable: bool);

        /// Overrides upstream `Conf::TSLAutoUpdate`.
        fn set_tsl_auto_update(self: Pin<&mut SettableConf>, enable: bool);

        /// Overrides upstream `Conf::TSLOnlineDigest`.
        fn set_tsl_online_digest(self: Pin<&mut SettableConf>, enable: bool);

        /// Overrides upstream `Conf::TSLTimeOut`.
        fn set_tsl_timeout(self: Pin<&mut SettableConf>, seconds: i32);

        /// Overrides upstream `Conf::proxyHost`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_proxy_host(self: Pin<&mut SettableConf>, host: &str) -> Result<()>;

        /// Overrides upstream `Conf::proxyPort`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_proxy_port(self: Pin<&mut SettableConf>, port: &str) -> Result<()>;

        /// Overrides upstream `Conf::proxyUser`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_proxy_user(self: Pin<&mut SettableConf>, user: &str) -> Result<()>;

        /// Overrides upstream `Conf::proxyPass`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_proxy_pass(self: Pin<&mut SettableConf>, pass: &str) -> Result<()>;

        /// Overrides upstream `Conf::proxyTunnelSSL`.
        fn set_proxy_tunnel_ssl(self: Pin<&mut SettableConf>, enable: bool);

        /// Overrides upstream `ConfV4::verifyServiceCerts` with an empty list.
        fn clear_verify_service_certs(self: Pin<&mut SettableConf>);

        /// Appends to the upstream `ConfV4::verifyServiceCerts` override.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when `der` is empty or
        /// not a valid certificate.
        fn add_verify_service_cert(self: Pin<&mut SettableConf>, der: &[u8]) -> Result<()>;

        /// Overrides upstream `Conf::verifyServiceUri`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn set_verify_service_uri(self: Pin<&mut SettableConf>, url: &str) -> Result<()>;
    }
}

impl Debug for ffi::SettableConf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SettableConf").finish_non_exhaustive()
    }
}
