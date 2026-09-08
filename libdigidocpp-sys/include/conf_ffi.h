#pragma once
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <vector>

#include "digidocpp/XmlConf.h"
#include "digidocpp/crypto/X509Cert.h"
#include "rust/cxx.h"

namespace digidoc_rs::conf {
struct DerCert;
struct OcspUrl;

// setter-based mecchanism for config
// `libdigidocpp` normally relies on subclassing the conf class
// since it's not possible in rust, we use in-memory setters
class SettableConf final : public digidoc::XmlConfCurrent {
  using Base = digidoc::XmlConfCurrent;

 public:
  SettableConf(const std::string& path, const std::string& schema);

  static auto create(rust::Str path, rust::Str schema)
      -> std::unique_ptr<SettableConf>;

  // setters used by Rust
  auto set_log_level(int level) noexcept -> void;
  auto set_log_file(rust::Str path) -> void;
  auto set_tsl_url(rust::Str url) -> void;
  auto clear_tsl_certs() noexcept -> void;
  auto add_tsl_cert(rust::Slice<const uint8_t> der) -> void;
  auto set_ocsp_tm_profiles(const rust::Vec<rust::String>& profiles) -> void;
  auto set_ocsp_urls(const rust::Vec<OcspUrl>& urls) -> void;
  auto set_ts_url(rust::Str url) -> void;
  auto clear_ts_certs() noexcept -> void;
  auto add_ts_cert(rust::Slice<const uint8_t> der) -> void;
  auto set_tsl_cache(rust::Str path) -> void;
  auto set_xsd_path(rust::Str path) -> void;
  auto set_tsl_allow_expired(bool enable) noexcept -> void;
  auto set_tsl_auto_update(bool enable) noexcept -> void;
  auto set_tsl_online_digest(bool enable) noexcept -> void;
  auto set_tsl_timeout(int seconds) noexcept -> void;
  auto set_proxy_host(rust::Str host) -> void;
  auto set_proxy_port(rust::Str port) -> void;
  auto set_proxy_user(rust::Str user) -> void;
  auto set_proxy_pass(rust::Str pass) -> void;
  auto set_proxy_tunnel_ssl(bool enable) noexcept -> void;
  auto clear_verify_service_certs() noexcept -> void;
  auto add_verify_service_cert(rust::Slice<const uint8_t> der) -> void;
  auto set_verify_service_uri(rust::Str url) -> void;

  // getters used by C++
  [[nodiscard]] auto logLevel() const -> int override;
  [[nodiscard]] auto logFile() const -> std::string override;
  [[nodiscard]] auto OCSPTMProfiles() const -> std::set<std::string> override;
  [[nodiscard]] auto TSLCache() const -> std::string override;
  [[nodiscard]] auto TSLCerts() const
      -> std::vector<digidoc::X509Cert> override;
  [[nodiscard]] auto TSLUrl() const -> std::string override;
  [[nodiscard]] auto xsdPath() const -> std::string override;
  [[nodiscard]] auto TSUrl() const -> std::string override;
  [[nodiscard]] auto TSLAllowExpired() const -> bool override;
  [[nodiscard]] auto TSLAutoUpdate() const -> bool override;
  [[nodiscard]] auto TSLOnlineDigest() const -> bool override;
  [[nodiscard]] auto TSLTimeOut() const -> int override;
  [[nodiscard]] auto TSCerts() const -> std::vector<digidoc::X509Cert> override;
  [[nodiscard]] auto ocsp(const std::string& issuer) const
      -> std::string override;
  [[nodiscard]] auto proxyHost() const -> std::string override;
  [[nodiscard]] auto proxyPort() const -> std::string override;
  [[nodiscard]] auto proxyUser() const -> std::string override;
  [[nodiscard]] auto proxyPass() const -> std::string override;
  [[nodiscard]] auto proxyTunnelSSL() const -> bool override;
  [[nodiscard]] auto verifyServiceCert() const -> digidoc::X509Cert override;
  [[nodiscard]] auto verifyServiceCerts() const
      -> std::vector<digidoc::X509Cert> override;
  [[nodiscard]] auto verifyServiceUri() const -> std::string override;

  // Getters used by Rust
  [[nodiscard]] auto log_level() const -> int;
  [[nodiscard]] auto log_file() const -> rust::String;
  [[nodiscard]] auto ocsp_tm_profiles() const -> rust::Vec<rust::String>;
  [[nodiscard]] auto ocsp_url(rust::Str issuer) const -> rust::String;
  [[nodiscard]] auto tsl_cache() const -> rust::String;
  [[nodiscard]] auto tsl_certs() const -> rust::Vec<DerCert>;
  [[nodiscard]] auto tsl_url() const -> rust::String;
  [[nodiscard]] auto xsd_path() const -> rust::String;
  [[nodiscard]] auto ts_url() const -> rust::String;
  [[nodiscard]] auto ts_certs() const -> rust::Vec<DerCert>;
  [[nodiscard]] auto tsl_allow_expired() const -> bool;
  [[nodiscard]] auto tsl_auto_update() const -> bool;
  [[nodiscard]] auto tsl_online_digest() const -> bool;
  [[nodiscard]] auto tsl_timeout() const -> int;
  [[nodiscard]] auto proxy_host() const -> rust::String;
  [[nodiscard]] auto proxy_port() const -> rust::String;
  [[nodiscard]] auto proxy_user() const -> rust::String;
  [[nodiscard]] auto proxy_pass() const -> rust::String;
  [[nodiscard]] auto proxy_force_ssl() const -> bool;
  [[nodiscard]] auto proxy_tunnel_ssl() const -> bool;
  [[nodiscard]] auto verify_service_certs() const -> rust::Vec<DerCert>;
  [[nodiscard]] auto verify_service_uri() const -> rust::String;
  [[nodiscard]] auto pkcs11_driver() const -> rust::String;
  [[nodiscard]] auto digest_uri() const -> rust::String;
  [[nodiscard]] auto signature_digest_uri() const -> rust::String;

 private:
  std::optional<int> log_level_;
  std::optional<std::string> log_file_;
  std::optional<std::string> tsl_url_;
  std::optional<std::string> ts_url_;
  std::optional<std::string> tsl_cache_;
  std::optional<std::string> xsd_path_;
  std::optional<bool> tsl_allow_expired_;
  std::optional<bool> tsl_auto_update_;
  std::optional<bool> tsl_online_digest_;
  std::optional<int> tsl_timeout_;
  std::optional<std::string> proxy_host_;
  std::optional<std::string> proxy_port_;
  std::optional<std::string> proxy_user_;
  std::optional<std::string> proxy_pass_;
  std::optional<bool> proxy_tunnel_ssl_;
  std::optional<std::string> verify_service_uri_;
  std::optional<std::vector<digidoc::X509Cert>> tsl_certs_;
  std::optional<std::vector<digidoc::X509Cert>> ts_certs_;
  std::optional<std::vector<digidoc::X509Cert>> verify_service_certs_;
  std::optional<std::set<std::string>> ocsp_tm_profiles_;
  std::optional<std::map<std::string, std::string>> ocsp_urls_;
};

auto init_conf(std::unique_ptr<SettableConf> conf) -> void;

}  // namespace digidoc_rs::conf
