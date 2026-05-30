#[cxx::bridge]
#[allow(clippy::exhaustive_structs)]
pub mod ffi {
    #[namespace = "digidoc_rs::conf"]
    #[derive(Debug, Clone)]
    struct CertificateDer {
        der: Vec<u8>,
    }

    #[namespace = "digidoc_rs::conf"]
    #[derive(Debug, Clone)]
    struct OcspUrl {
        issuer: String,
        url: String,
    }

    #[namespace = "digidoc_rs::conf"]
    #[allow(missing_debug_implementations)]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("libdigidocpp-sys/include/conf_ffi.h");

        type SettableConf;

        #[Self = "SettableConf"]
        fn create(path: &str, schema: &str) -> Result<UniquePtr<SettableConf>>;
        fn init_conf(conf: UniquePtr<SettableConf>) -> Result<()>;

        fn log_level(self: &SettableConf) -> Result<i32>;
        fn log_file(self: &SettableConf) -> Result<String>;
        fn ocsp_tm_profiles(self: &SettableConf) -> Result<Vec<String>>;
        fn ocsp_url(self: &SettableConf, issuer: &str) -> Result<String>;
        fn tsl_cache(self: &SettableConf) -> Result<String>;
        fn tsl_certs(self: &SettableConf) -> Result<Vec<CertificateDer>>;
        fn tsl_url(self: &SettableConf) -> Result<String>;
        fn xsd_path(self: &SettableConf) -> Result<String>;
        fn ts_url(self: &SettableConf) -> Result<String>;
        fn ts_certs(self: &SettableConf) -> Result<Vec<CertificateDer>>;
        fn tsl_allow_expired(self: &SettableConf) -> Result<bool>;
        fn tsl_auto_update(self: &SettableConf) -> Result<bool>;
        fn tsl_online_digest(self: &SettableConf) -> Result<bool>;
        fn tsl_timeout(self: &SettableConf) -> Result<i32>;
        fn proxy_host(self: &SettableConf) -> Result<String>;
        fn proxy_port(self: &SettableConf) -> Result<String>;
        fn proxy_user(self: &SettableConf) -> Result<String>;
        fn proxy_pass(self: &SettableConf) -> Result<String>;
        fn proxy_force_ssl(self: &SettableConf) -> Result<bool>;
        fn proxy_tunnel_ssl(self: &SettableConf) -> Result<bool>;
        fn verify_service_certs(self: &SettableConf) -> Result<Vec<CertificateDer>>;
        fn verify_service_uri(self: &SettableConf) -> Result<String>;
        fn pkcs11_driver(self: &SettableConf) -> Result<String>;
        fn digest_uri(self: &SettableConf) -> Result<String>;
        fn signature_digest_uri(self: &SettableConf) -> Result<String>;

        fn set_log_level(self: Pin<&mut SettableConf>, level: i32) -> Result<()>;
        fn set_log_file(self: Pin<&mut SettableConf>, path: &str) -> Result<()>;
        fn set_tsl_url(self: Pin<&mut SettableConf>, url: &str) -> Result<()>;
        fn clear_tsl_certs(self: Pin<&mut SettableConf>) -> Result<()>;
        fn add_tsl_cert(self: Pin<&mut SettableConf>, der: &[u8]) -> Result<()>;
        fn set_ocsp_tm_profiles(self: Pin<&mut SettableConf>, profiles: &Vec<String>)
        -> Result<()>;
        fn set_ocsp_urls(self: Pin<&mut SettableConf>, urls: &Vec<OcspUrl>) -> Result<()>;
        fn set_ts_url(self: Pin<&mut SettableConf>, url: &str) -> Result<()>;
        fn clear_ts_certs(self: Pin<&mut SettableConf>) -> Result<()>;
        fn add_ts_cert(self: Pin<&mut SettableConf>, der: &[u8]) -> Result<()>;
        fn set_tsl_cache(self: Pin<&mut SettableConf>, path: &str) -> Result<()>;
        fn set_xsd_path(self: Pin<&mut SettableConf>, path: &str) -> Result<()>;
        fn set_tsl_allow_expired(self: Pin<&mut SettableConf>, enable: bool) -> Result<()>;
        fn set_tsl_auto_update(self: Pin<&mut SettableConf>, enable: bool) -> Result<()>;
        fn set_tsl_online_digest(self: Pin<&mut SettableConf>, enable: bool) -> Result<()>;
        fn set_tsl_timeout(self: Pin<&mut SettableConf>, seconds: i32) -> Result<()>;
        fn set_proxy_host(self: Pin<&mut SettableConf>, host: &str) -> Result<()>;
        fn set_proxy_port(self: Pin<&mut SettableConf>, port: &str) -> Result<()>;
        fn set_proxy_user(self: Pin<&mut SettableConf>, user: &str) -> Result<()>;
        fn set_proxy_pass(self: Pin<&mut SettableConf>, pass: &str) -> Result<()>;
        fn set_proxy_tunnel_ssl(self: Pin<&mut SettableConf>, enable: bool) -> Result<()>;
        fn clear_verify_service_certs(self: Pin<&mut SettableConf>) -> Result<()>;
        fn add_verify_service_cert(self: Pin<&mut SettableConf>, der: &[u8]) -> Result<()>;
        fn set_verify_service_uri(self: Pin<&mut SettableConf>, url: &str) -> Result<()>;
    }
}
