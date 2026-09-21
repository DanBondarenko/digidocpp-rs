#include "libdigidocpp-sys/include/signature_ffi.h"

#include <cstdint>
#include <string>
#include <vector>

#include "digidocpp/Signature.h"
#include "digidocpp/crypto/Signer.h"
#include "libdigidocpp-sys/include/ffi_util.h"
#include "libdigidocpp-sys/src/signature.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::signature {

auto id(const digidoc::Signature& signature) -> rust::String {
  return signature.id();
}

auto claimed_signing_time(const digidoc::Signature& signature) -> rust::String {
  return signature.claimedSigningTime();
}

auto trusted_signing_time(const digidoc::Signature& signature) -> rust::String {
  return signature.trustedSigningTime();
}

auto signing_certificate(const digidoc::Signature& signature)
    -> rust::Vec<uint8_t> {
  return util::to_rust_der(signature.signingCertificate());
}

auto signature_method(const digidoc::Signature& signature) -> rust::String {
  return signature.signatureMethod();
}

auto validate(const digidoc::Signature& signature) -> void {
  signature.validate();
}

auto validate_with_policy(const digidoc::Signature& signature,
                          const rust::Str policy) -> void {
  signature.validate(std::string(policy));
}
auto data_to_sign(const digidoc::Signature& signature) -> rust::Vec<uint8_t> {
  return util::to_rust_vec(signature.dataToSign());
}

auto set_signature_value(digidoc::Signature& signature,
                         const rust::Slice<const uint8_t> value) -> void {
  signature.setSignatureValue(
      std::vector<unsigned char>(value.begin(), value.end()));
}

auto extend_signature_profile(digidoc::Signature& signature,
                              digidoc::Signer& signer) -> void {
  signature.extendSignatureProfile(&signer);
}

auto policy(const digidoc::Signature& signature) -> rust::String {
  return signature.policy();
}

auto sp_uri(const digidoc::Signature& signature) -> rust::String {
  return signature.SPUri();
}

auto profile(const digidoc::Signature& signature) -> rust::String {
  return signature.profile();
}

auto city(const digidoc::Signature& signature) -> rust::String {
  return signature.city();
}

auto state_or_province(const digidoc::Signature& signature) -> rust::String {
  return signature.stateOrProvince();
}

auto postal_code(const digidoc::Signature& signature) -> rust::String {
  return signature.postalCode();
}

auto country_name(const digidoc::Signature& signature) -> rust::String {
  return signature.countryName();
}

auto street_address(const digidoc::Signature& signature) -> rust::String {
  return signature.streetAddress();
}

auto signer_roles(const digidoc::Signature& signature)
    -> rust::Vec<rust::String> {
  return util::to_rust_vec<rust::String>(signature.signerRoles());
}

auto ocsp_produced_at(const digidoc::Signature& signature) -> rust::String {
  return signature.OCSPProducedAt();
}

auto ocsp_certificate(const digidoc::Signature& signature)
    -> rust::Vec<uint8_t> {
  return util::to_rust_der(signature.OCSPCertificate());
}

auto time_stamp_certificate(const digidoc::Signature& signature)
    -> rust::Vec<uint8_t> {
  return util::to_rust_der(signature.TimeStampCertificate());
}

auto time_stamp_time(const digidoc::Signature& signature) -> rust::String {
  return signature.TimeStampTime();
}

auto archive_time_stamps(const digidoc::Signature& signature)
    -> rust::Vec<TSAInfo> {
  const auto convert = [](const digidoc::TSAInfo& tsa_info) -> TSAInfo {
    return TSAInfo{
        .cert = util::to_rust_der(tsa_info.cert),
        .time = tsa_info.time,
    };
  };
  return util::to_rust_vec<TSAInfo>(signature.ArchiveTimeStamps(), convert);
}

auto signed_by(const digidoc::Signature& signature) -> rust::String {
  return signature.signedBy();
}

auto message_imprint(const digidoc::Signature& signature)
    -> rust::Vec<uint8_t> {
  return util::to_rust_vec(signature.messageImprint());
}

auto validator(const digidoc::Signature& signature) -> ValidatorResult {
  digidoc::Signature::Validator const validator(&signature);
  return ValidatorResult{
      .status = validator.status(),
      .diagnostics = validator.diagnostics(),
      .warnings = util::to_rust_vec(validator.warnings()),
  };
}

}  // namespace digidoc_rs::signature
