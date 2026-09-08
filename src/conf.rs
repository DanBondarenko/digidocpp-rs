use crate::util::path_to_str;
use crate::{DerCert, Error};
use cxx::UniquePtr;
use libdigidocpp_sys::conf::ffi;
pub use secrecy::{ExposeSecret, SecretString};
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};

pub struct Conf(UniquePtr<ffi::SettableConf>);

impl Conf {
    pub fn new() -> Result<Self, Error> {
        Self::create("", "")
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::create(path_to_str(path.as_ref())?, "")
    }

    pub fn from_file_with_schema(
        path: impl AsRef<Path>,
        schema: impl AsRef<Path>,
    ) -> Result<Self, Error> {
        Self::create(path_to_str(path.as_ref())?, path_to_str(schema.as_ref())?)
    }

    pub fn with_schema(schema: impl AsRef<Path>) -> Result<Self, Error> {
        Self::create("", path_to_str(schema.as_ref())?)
    }

    fn create(path: &str, schema: &str) -> Result<Self, Error> {
        Ok(Self(ffi::SettableConf::create(path, schema)?))
    }
}

impl Conf {
    pub fn set_log_level(&mut self, level: i32) {
        self.0.pin_mut().set_log_level(level);
    }

    pub fn set_log_file(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_log_file(path_to_str(path.as_ref())?)?)
    }

    pub fn set_tsl_url(&mut self, url: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_tsl_url(url.as_ref())?)
    }

    pub fn clear_tsl_certs(&mut self) {
        self.0.pin_mut().clear_tsl_certs();
    }

    pub fn add_tsl_cert(&mut self, cert: impl AsRef<[u8]>) -> Result<(), Error> {
        Ok(self.0.pin_mut().add_tsl_cert(cert.as_ref())?)
    }

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

    pub fn set_ts_url(&mut self, url: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_ts_url(url.as_ref())?)
    }

    pub fn clear_ts_certs(&mut self) {
        self.0.pin_mut().clear_ts_certs();
    }

    pub fn add_ts_cert(&mut self, cert: impl AsRef<[u8]>) -> Result<(), Error> {
        Ok(self.0.pin_mut().add_ts_cert(cert.as_ref())?)
    }

    pub fn set_tsl_cache(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(self
            .0
            .pin_mut()
            .set_tsl_cache(path_to_str(path.as_ref())?)?)
    }

    pub fn set_xsd_path(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_xsd_path(path_to_str(path.as_ref())?)?)
    }

    pub fn set_tsl_allow_expired(&mut self, enable: bool) {
        self.0.pin_mut().set_tsl_allow_expired(enable);
    }

    pub fn set_tsl_auto_update(&mut self, enable: bool) {
        self.0.pin_mut().set_tsl_auto_update(enable);
    }

    pub fn set_tsl_online_digest(&mut self, enable: bool) {
        self.0.pin_mut().set_tsl_online_digest(enable);
    }

    pub fn set_tsl_timeout(&mut self, seconds: i32) {
        self.0.pin_mut().set_tsl_timeout(seconds);
    }

    pub fn set_proxy_host(&mut self, host: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_proxy_host(host.as_ref())?)
    }

    pub fn set_proxy_port(&mut self, port: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_proxy_port(port.as_ref())?)
    }

    pub fn set_proxy_user(&mut self, user: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_proxy_user(user.as_ref())?)
    }

    pub fn set_proxy_pass(&mut self, pass: impl Into<SecretString>) -> Result<(), Error> {
        let pass = pass.into();

        Ok(self.0.pin_mut().set_proxy_pass(pass.expose_secret())?)
    }

    pub fn set_proxy_tunnel_ssl(&mut self, enable: bool) {
        self.0.pin_mut().set_proxy_tunnel_ssl(enable);
    }

    pub fn clear_verify_service_certs(&mut self) {
        self.0.pin_mut().clear_verify_service_certs();
    }

    pub fn add_verify_service_cert(&mut self, cert: impl AsRef<[u8]>) -> Result<(), Error> {
        Ok(self.0.pin_mut().add_verify_service_cert(cert.as_ref())?)
    }

    pub fn set_verify_service_uri(&mut self, url: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.0.pin_mut().set_verify_service_uri(url.as_ref())?)
    }
}

impl Conf {
    pub fn log_level(&self) -> Result<i32, Error> {
        Ok(self.0.log_level()?)
    }

    pub fn log_file(&self) -> Result<PathBuf, Error> {
        Ok(PathBuf::from(self.0.log_file()?))
    }

    pub fn ocsp_tm_profiles(&self) -> Result<Vec<String>, Error> {
        Ok(self.0.ocsp_tm_profiles()?)
    }

    pub fn ocsp_url(&self, issuer: &str) -> Result<String, Error> {
        Ok(self.0.ocsp_url(issuer)?)
    }

    pub fn tsl_cache(&self) -> Result<PathBuf, Error> {
        Ok(PathBuf::from(self.0.tsl_cache()?))
    }

    pub fn tsl_certs(&self) -> Result<Vec<DerCert>, Error> {
        Ok(build_der_vec(self.0.tsl_certs()?))
    }

    pub fn tsl_url(&self) -> Result<String, Error> {
        Ok(self.0.tsl_url()?)
    }

    pub fn xsd_path(&self) -> Result<PathBuf, Error> {
        Ok(PathBuf::from(self.0.xsd_path()?))
    }

    pub fn ts_url(&self) -> Result<String, Error> {
        Ok(self.0.ts_url()?)
    }

    pub fn ts_certs(&self) -> Result<Vec<DerCert>, Error> {
        Ok(build_der_vec(self.0.ts_certs()?))
    }

    pub fn tsl_allow_expired(&self) -> Result<bool, Error> {
        Ok(self.0.tsl_allow_expired()?)
    }

    pub fn tsl_auto_update(&self) -> Result<bool, Error> {
        Ok(self.0.tsl_auto_update()?)
    }

    pub fn tsl_online_digest(&self) -> Result<bool, Error> {
        Ok(self.0.tsl_online_digest()?)
    }

    pub fn tsl_timeout(&self) -> Result<i32, Error> {
        Ok(self.0.tsl_timeout()?)
    }

    pub fn proxy_host(&self) -> Result<String, Error> {
        Ok(self.0.proxy_host()?)
    }

    pub fn proxy_port(&self) -> Result<String, Error> {
        Ok(self.0.proxy_port()?)
    }

    pub fn proxy_user(&self) -> Result<String, Error> {
        Ok(self.0.proxy_user()?)
    }

    pub fn proxy_pass(&self) -> Result<SecretString, Error> {
        Ok(SecretString::from(self.0.proxy_pass()?))
    }

    pub fn proxy_force_ssl(&self) -> Result<bool, Error> {
        Ok(self.0.proxy_force_ssl()?)
    }

    pub fn proxy_tunnel_ssl(&self) -> Result<bool, Error> {
        Ok(self.0.proxy_tunnel_ssl()?)
    }

    pub fn verify_service_certs(&self) -> Result<Vec<DerCert>, Error> {
        Ok(build_der_vec(self.0.verify_service_certs()?))
    }

    pub fn verify_service_uri(&self) -> Result<String, Error> {
        Ok(self.0.verify_service_uri()?)
    }

    pub fn pkcs11_driver(&self) -> Result<String, Error> {
        Ok(self.0.pkcs11_driver()?)
    }

    pub fn digest_uri(&self) -> Result<String, Error> {
        Ok(self.0.digest_uri()?)
    }

    pub fn signature_digest_uri(&self) -> Result<String, Error> {
        Ok(self.0.signature_digest_uri()?)
    }
}

/// Must be called before [`initialize`](crate::initialize)
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
