#include "libdigidocpp-sys/include/conf_ffi.h"

#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "digidocpp/Conf.h"
#include "digidocpp/XmlConf.h"
#include "digidocpp/crypto/X509Cert.h"
#include "libdigidocpp-sys/include/ffi_util.h"
#include "libdigidocpp-sys/src/conf.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::conf {
namespace {
auto to_cert(const rust::Slice<const uint8_t> der) -> digidoc::X509Cert {
  return digidoc::X509Cert(der.data(), der.size(), digidoc::X509Cert::Der);
}

auto add_cert(std::optional<std::vector<digidoc::X509Cert>>& certs,
              const rust::Slice<const uint8_t> der) -> void {
  if (der.empty()) {
    throw std::invalid_argument("certificate is empty");
  }
  digidoc::X509Cert cert = to_cert(der);
  if (!certs) {
    certs.emplace();
  }
  certs->push_back(std::move(cert));
}

auto to_der_list(const std::vector<digidoc::X509Cert>& certs)
    -> rust::Vec<CertificateDer> {
  return util::to_rust_vec<CertificateDer>(
      certs, [](const digidoc::X509Cert& cert) -> CertificateDer {
        return CertificateDer{util::to_rust_der(cert)};
      });
}
}  // namespace

SettableConf::SettableConf(const std::string& path, const std::string& schema)
    : Base(path, schema) {}

auto SettableConf::create(const rust::Str path, const rust::Str schema)
    -> std::unique_ptr<SettableConf> {
  return std::make_unique<SettableConf>(std::string(path), std::string(schema));
}

auto SettableConf::logLevel() const -> int {
  return log_level_.value_or(Base::logLevel());
}

auto SettableConf::logFile() const -> std::string {
  return log_file_.value_or(Base::logFile());
}

auto SettableConf::OCSPTMProfiles() const -> std::set<std::string> {
  return ocsp_tm_profiles_.value_or(Base::OCSPTMProfiles());
}

auto SettableConf::TSLCache() const -> std::string {
  return tsl_cache_.value_or(Base::TSLCache());
}

auto SettableConf::TSLCerts() const -> std::vector<digidoc::X509Cert> {
  return tsl_certs_.value_or(Base::TSLCerts());
}

auto SettableConf::TSLUrl() const -> std::string {
  return tsl_url_.value_or(Base::TSLUrl());
}

auto SettableConf::xsdPath() const -> std::string {
  return xsd_path_.value_or(Base::xsdPath());
}

auto SettableConf::TSUrl() const -> std::string {
  return ts_url_.value_or(Base::TSUrl());
}

auto SettableConf::TSLAllowExpired() const -> bool {
  return tsl_allow_expired_.value_or(Base::TSLAllowExpired());
}

auto SettableConf::TSLAutoUpdate() const -> bool {
  return tsl_auto_update_.value_or(Base::TSLAutoUpdate());
}

auto SettableConf::TSLOnlineDigest() const -> bool {
  return tsl_online_digest_.value_or(Base::TSLOnlineDigest());
}

auto SettableConf::TSLTimeOut() const -> int {
  return tsl_timeout_.value_or(Base::TSLTimeOut());
}

auto SettableConf::TSCerts() const -> std::vector<digidoc::X509Cert> {
  return ts_certs_.value_or(Base::TSCerts());
}

auto SettableConf::ocsp(const std::string& issuer) const -> std::string {
  if (!ocsp_urls_) {
    return Base::ocsp(issuer);
  }
  const auto url = ocsp_urls_->find(issuer);
  return url == ocsp_urls_->cend() ? std::string() : url->second;
}

auto SettableConf::proxyHost() const -> std::string {
  return proxy_host_.value_or(Base::proxyHost());
}

auto SettableConf::proxyPort() const -> std::string {
  return proxy_port_.value_or(Base::proxyPort());
}

auto SettableConf::proxyUser() const -> std::string {
  return proxy_user_.value_or(Base::proxyUser());
}

auto SettableConf::proxyPass() const -> std::string {
  return proxy_pass_.value_or(Base::proxyPass());
}

auto SettableConf::proxyTunnelSSL() const -> bool {
  return proxy_tunnel_ssl_.value_or(Base::proxyTunnelSSL());
}

auto SettableConf::verifyServiceCert() const -> digidoc::X509Cert {
  if (!verify_service_certs_) {
    return Base::verifyServiceCert();
  }
  return verify_service_certs_->empty() ? digidoc::X509Cert()
                                        : verify_service_certs_->front();
}

auto SettableConf::verifyServiceCerts() const
    -> std::vector<digidoc::X509Cert> {
  return verify_service_certs_.value_or(Base::verifyServiceCerts());
}

auto SettableConf::verifyServiceUri() const -> std::string {
  return verify_service_uri_.value_or(Base::verifyServiceUri());
}

auto SettableConf::log_level() const -> int { return logLevel(); }

auto SettableConf::log_file() const -> rust::String { return logFile(); }

auto SettableConf::ocsp_tm_profiles() const -> rust::Vec<rust::String> {
  return util::to_rust_vec<rust::String>(OCSPTMProfiles());
}

auto SettableConf::ocsp_url(const rust::Str issuer) const -> rust::String {
  return ocsp(std::string(issuer));
}

auto SettableConf::tsl_cache() const -> rust::String { return TSLCache(); }

auto SettableConf::tsl_certs() const -> rust::Vec<CertificateDer> {
  return to_der_list(TSLCerts());
}

auto SettableConf::tsl_url() const -> rust::String { return TSLUrl(); }

auto SettableConf::xsd_path() const -> rust::String { return xsdPath(); }

auto SettableConf::ts_url() const -> rust::String { return TSUrl(); }

auto SettableConf::ts_certs() const -> rust::Vec<CertificateDer> {
  return to_der_list(TSCerts());
}

auto SettableConf::tsl_allow_expired() const -> bool {
  return TSLAllowExpired();
}

auto SettableConf::tsl_auto_update() const -> bool { return TSLAutoUpdate(); }

auto SettableConf::tsl_online_digest() const -> bool {
  return TSLOnlineDigest();
}

auto SettableConf::tsl_timeout() const -> int { return TSLTimeOut(); }

auto SettableConf::proxy_host() const -> rust::String { return proxyHost(); }

auto SettableConf::proxy_port() const -> rust::String { return proxyPort(); }

auto SettableConf::proxy_user() const -> rust::String { return proxyUser(); }

auto SettableConf::proxy_pass() const -> rust::String { return proxyPass(); }

auto SettableConf::proxy_force_ssl() const -> bool { return proxyForceSSL(); }

auto SettableConf::proxy_tunnel_ssl() const -> bool { return proxyTunnelSSL(); }

auto SettableConf::verify_service_certs() const -> rust::Vec<CertificateDer> {
  return to_der_list(verifyServiceCerts());
}

auto SettableConf::verify_service_uri() const -> rust::String {
  return verifyServiceUri();
}

auto SettableConf::pkcs11_driver() const -> rust::String {
  return PKCS11Driver();
}

auto SettableConf::digest_uri() const -> rust::String { return digestUri(); }

auto SettableConf::signature_digest_uri() const -> rust::String {
  return signatureDigestUri();
}

// Setters used by Rust

auto SettableConf::set_log_level(const int level) noexcept -> void {
  log_level_ = level;
}

auto SettableConf::set_log_file(const rust::Str path) -> void {
  log_file_ = std::string(path);
}

auto SettableConf::set_tsl_url(const rust::Str url) -> void {
  tsl_url_ = std::string(url);
}

auto SettableConf::clear_tsl_certs() noexcept -> void { tsl_certs_.emplace(); }

auto SettableConf::add_tsl_cert(const rust::Slice<const uint8_t> der) -> void {
  add_cert(tsl_certs_, der);
}

auto SettableConf::set_ocsp_tm_profiles(const rust::Vec<rust::String>& profiles)
    -> void {
  ocsp_tm_profiles_.emplace(profiles.begin(), profiles.end());
}

auto SettableConf::set_ocsp_urls(const rust::Vec<OcspUrl>& urls) -> void {
  ocsp_urls_.emplace();
  for (const OcspUrl& entry : urls) {
    ocsp_urls_->insert_or_assign(std::string(entry.issuer),
                                 std::string(entry.url));
  }
}

auto SettableConf::set_ts_url(const rust::Str url) -> void {
  ts_url_ = std::string(url);
}

auto SettableConf::clear_ts_certs() noexcept -> void { ts_certs_.emplace(); }

auto SettableConf::add_ts_cert(const rust::Slice<const uint8_t> der) -> void {
  add_cert(ts_certs_, der);
}

auto SettableConf::set_tsl_cache(const rust::Str path) -> void {
  tsl_cache_ = std::string(path);
}

auto SettableConf::set_xsd_path(const rust::Str path) -> void {
  xsd_path_ = std::string(path);
}

auto SettableConf::set_tsl_allow_expired(const bool enable) noexcept -> void {
  tsl_allow_expired_ = enable;
}

auto SettableConf::set_tsl_auto_update(const bool enable) noexcept -> void {
  tsl_auto_update_ = enable;
}

auto SettableConf::set_tsl_online_digest(const bool enable) noexcept -> void {
  tsl_online_digest_ = enable;
}

auto SettableConf::set_tsl_timeout(const int seconds) noexcept -> void {
  tsl_timeout_ = seconds;
}

auto SettableConf::set_proxy_host(const rust::Str host) -> void {
  proxy_host_ = std::string(host);
}

auto SettableConf::set_proxy_port(const rust::Str port) -> void {
  proxy_port_ = std::string(port);
}

auto SettableConf::set_proxy_user(const rust::Str user) -> void {
  proxy_user_ = std::string(user);
}

auto SettableConf::set_proxy_pass(const rust::Str pass) -> void {
  proxy_pass_ = std::string(pass);
}

auto SettableConf::set_proxy_tunnel_ssl(const bool enable) noexcept -> void {
  proxy_tunnel_ssl_ = enable;
}

auto SettableConf::clear_verify_service_certs() noexcept -> void {
  verify_service_certs_.emplace();
}

auto SettableConf::add_verify_service_cert(const rust::Slice<const uint8_t> der)
    -> void {
  add_cert(verify_service_certs_, der);
}

auto SettableConf::set_verify_service_uri(const rust::Str url) -> void {
  verify_service_uri_ = std::string(url);
}

auto init_conf(std::unique_ptr<SettableConf> conf) -> void {
  digidoc::Conf::init(conf.release());
}

}  // namespace digidoc_rs::conf
