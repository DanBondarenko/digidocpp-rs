#include "libdigidocpp-sys/include/signer_ffi.h"

#include <cstdint>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "digidocpp/crypto/PKCS11Signer.h"
#include "digidocpp/crypto/PKCS12Signer.h"
#include "digidocpp/crypto/Signer.h"
#include "libdigidocpp-sys/include/ffi_util.h"
#include "rust/cxx.h"

namespace digidoc_rs::signer {
ExternalPartialSigner::ExternalPartialSigner(digidoc::X509Cert cert)
    : cert_(std::move(cert)) {}

auto ExternalPartialSigner::cert() const -> digidoc::X509Cert { return cert_; }

auto ExternalPartialSigner::sign(const std::string& /*method*/,
                                 const std::vector<unsigned char>& /*digest*/)
    const -> std::vector<unsigned char> {
  throw std::runtime_error("external partial signer cannot sign");
}

#ifdef _WIN32
auto make_win_signer() -> std::unique_ptr<digidoc::WinSigner> {
  return std::make_unique<digidoc::WinSigner>();
}

auto win_set_pin(digidoc::WinSigner& signer, const rust::Str pin) -> void {
  signer.setPin(std::string(pin));
}

auto win_set_select_first(digidoc::WinSigner& signer, const bool select_first)
    -> void {
  signer.setSelectFirst(select_first);
}

auto win_set_thumbprint(digidoc::WinSigner& signer,
                        const rust::Slice<const uint8_t> thumbprint) -> void {
  signer.setThumbprint(
      std::vector<unsigned char>(thumbprint.begin(), thumbprint.end()));
}

auto win_as_signer(const digidoc::WinSigner& signer) noexcept
    -> const digidoc::Signer& {
  return signer;
}

auto win_as_signer_mut(digidoc::WinSigner& signer) noexcept
    -> digidoc::Signer& {
  return signer;
}
#endif

auto make_pkcs11_signer(const rust::Str driver)
    -> std::unique_ptr<digidoc::PKCS11Signer> {
  return std::make_unique<digidoc::PKCS11Signer>(std::string(driver));
}

auto pkcs11_set_pin(digidoc::PKCS11Signer& signer, const rust::Str pin)
    -> void {
  signer.setPin(std::string(pin));
}

auto pkcs11_as_signer(const digidoc::PKCS11Signer& signer) noexcept
    -> const digidoc::Signer& {
  return signer;
}

auto pkcs11_as_signer_mut(digidoc::PKCS11Signer& signer) noexcept
    -> digidoc::Signer& {
  return signer;
}

auto make_pkcs12_signer(const rust::Str path, const rust::Str pass)
    -> std::unique_ptr<digidoc::PKCS12Signer> {
  return std::make_unique<digidoc::PKCS12Signer>(std::string(path),
                                                 std::string(pass));
}

auto pkcs12_as_signer(const digidoc::PKCS12Signer& signer) noexcept
    -> const digidoc::Signer& {
  return signer;
}

auto pkcs12_as_signer_mut(digidoc::PKCS12Signer& signer) noexcept
    -> digidoc::Signer& {
  return signer;
}

auto make_external_partial_signer(const rust::Slice<const uint8_t> der)
    -> std::unique_ptr<ExternalPartialSigner> {
  return std::make_unique<ExternalPartialSigner>(
      digidoc::X509Cert(der.data(), der.size(), digidoc::X509Cert::Der));
}

auto external_as_signer(const ExternalPartialSigner& signer) noexcept
    -> const digidoc::Signer& {
  return signer;
}

auto external_as_signer_mut(ExternalPartialSigner& signer) noexcept
    -> digidoc::Signer& {
  return signer;
}

auto cert(const digidoc::Signer& signer) -> rust::Vec<uint8_t> {
  return util::to_rust_der(signer.cert());
}

auto sign(const digidoc::Signer& signer, const rust::Str method,
          const rust::Slice<const uint8_t> digest) -> rust::Vec<uint8_t> {
  return util::to_rust_vec(
      signer.sign(std::string(method),
                  std::vector<unsigned char>(digest.begin(), digest.end())));
}

auto method(const digidoc::Signer& signer) -> rust::String {
  return signer.method();
}

auto profile(const digidoc::Signer& signer) -> rust::String {
  return signer.profile();
}

auto user_agent(const digidoc::Signer& signer) -> rust::String {
  return signer.userAgent();
}

auto using_en_profile(const digidoc::Signer& signer) -> bool {
  return signer.usingENProfile();
}

auto city(const digidoc::Signer& signer) -> rust::String {
  return signer.city();
}

auto street_address(const digidoc::Signer& signer) -> rust::String {
  return signer.streetAddress();
}

auto state_or_province(const digidoc::Signer& signer) -> rust::String {
  return signer.stateOrProvince();
}

auto postal_code(const digidoc::Signer& signer) -> rust::String {
  return signer.postalCode();
}

auto country_name(const digidoc::Signer& signer) -> rust::String {
  return signer.countryName();
}

auto signer_roles(const digidoc::Signer& signer) -> rust::Vec<rust::String> {
  return util::to_rust_vec<rust::String>(signer.signerRoles());
}

auto set_method(digidoc::Signer& signer, const rust::Str method) -> void {
  signer.setMethod(std::string(method));
}

auto set_profile(digidoc::Signer& signer, const rust::Str profile) -> void {
  signer.setProfile(std::string(profile));
}

auto set_user_agent(digidoc::Signer& signer, const rust::Str user_agent)
    -> void {
  signer.setUserAgent(std::string(user_agent));
}

auto set_en_profile(digidoc::Signer& signer, const bool enable) -> void {
  signer.setENProfile(enable);
}

auto set_signature_production_place(digidoc::Signer& signer,
                                    const rust::Str city,
                                    const rust::Str state_or_province,
                                    const rust::Str postal_code,
                                    const rust::Str country_name) -> void {
  signer.setSignatureProductionPlace(
      std::string(city), std::string(state_or_province),
      std::string(postal_code), std::string(country_name));
}

auto set_signature_production_place_v2(digidoc::Signer& signer,
                                       const rust::Str city,
                                       const rust::Str street_address,
                                       const rust::Str state_or_province,
                                       const rust::Str postal_code,
                                       const rust::Str country_name) -> void {
  signer.setSignatureProductionPlaceV2(
      std::string(city), std::string(street_address),
      std::string(state_or_province), std::string(postal_code),
      std::string(country_name));
}

auto set_signer_roles(digidoc::Signer& signer,
                      const rust::Vec<rust::String>& roles) -> void {
  signer.setSignerRoles(std::vector<std::string>(roles.begin(), roles.end()));
}
}  // namespace digidoc_rs::signer
