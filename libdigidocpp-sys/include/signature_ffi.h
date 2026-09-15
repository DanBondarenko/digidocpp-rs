#pragma once
#include <cstdint>

#include "digidocpp/Signature.h"
#include "digidocpp/crypto/Signer.h"
#include "rust/cxx.h"

namespace digidoc {
using Status = Signature::Validator::Status;
}

namespace digidoc_rs::signature {
struct TSAInfo;
struct ValidatorResult;
}  // namespace digidoc_rs::signature

namespace digidoc_rs::signature {
auto pol_v1() -> rust::String;
auto pol_v2() -> rust::String;

auto id(const digidoc::Signature& signature) -> rust::String;
auto claimed_signing_time(const digidoc::Signature& signature) -> rust::String;
auto trusted_signing_time(const digidoc::Signature& signature) -> rust::String;
auto signing_certificate(const digidoc::Signature& signature)
    -> rust::Vec<uint8_t>;
auto signature_method(const digidoc::Signature& signature) -> rust::String;

void validate(const digidoc::Signature& signature);
void validate_with_policy(const digidoc::Signature& signature,
                          rust::Str policy);
auto data_to_sign(const digidoc::Signature& signature) -> rust::Vec<uint8_t>;
auto set_signature_value(digidoc::Signature& signature,
                         rust::Slice<const uint8_t> value) -> void;
auto extend_signature_profile(digidoc::Signature& signature,
                              digidoc::Signer& signer) -> void;

auto policy(const digidoc::Signature& signature) -> rust::String;
auto sp_uri(const digidoc::Signature& signature) -> rust::String;
auto profile(const digidoc::Signature& signature) -> rust::String;
auto city(const digidoc::Signature& signature) -> rust::String;
auto state_or_province(const digidoc::Signature& signature) -> rust::String;
auto postal_code(const digidoc::Signature& signature) -> rust::String;
auto country_name(const digidoc::Signature& signature) -> rust::String;
auto street_address(const digidoc::Signature& signature) -> rust::String;

auto signer_roles(const digidoc::Signature& signature)
    -> rust::Vec<rust::String>;
auto ocsp_produced_at(const digidoc::Signature& signature) -> rust::String;
auto ocsp_certificate(const digidoc::Signature& signature)
    -> rust::Vec<uint8_t>;
auto time_stamp_certificate(const digidoc::Signature& signature)
    -> rust::Vec<uint8_t>;
auto time_stamp_time(const digidoc::Signature& signature) -> rust::String;
auto archive_time_stamps(const digidoc::Signature& signature)
    -> rust::Vec<TSAInfo>;

auto signed_by(const digidoc::Signature& signature) -> rust::String;
auto message_imprint(const digidoc::Signature& signature) -> rust::Vec<uint8_t>;

auto validator(const digidoc::Signature& signature) -> ValidatorResult;

}  // namespace digidoc_rs::signature