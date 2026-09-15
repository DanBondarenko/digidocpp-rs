#pragma once
#include <cstdint>
#include <memory>

#include "digidocpp/crypto/PKCS11Signer.h"
#include "digidocpp/crypto/PKCS12Signer.h"
#include "digidocpp/crypto/Signer.h"
#include "rust/cxx.h"

#ifdef _WIN32
#include "digidocpp/crypto/WinSigner.h"
#endif

namespace digidoc_rs::signer {
#ifdef _WIN32
auto make_win_signer() -> std::unique_ptr<digidoc::WinSigner>;
auto win_set_pin(digidoc::WinSigner& signer, rust::Str pin) -> void;
auto win_set_select_first(digidoc::WinSigner& signer, bool select_first)
    -> void;
auto win_set_thumbprint(digidoc::WinSigner& signer,
                        rust::Slice<const uint8_t> thumbprint) -> void;
auto win_as_signer(const digidoc::WinSigner& signer) noexcept
    -> const digidoc::Signer&;
auto win_as_signer_mut(digidoc::WinSigner& signer) noexcept -> digidoc::Signer&;
#endif

auto make_pkcs11_signer(rust::Str driver)
    -> std::unique_ptr<digidoc::PKCS11Signer>;
auto pkcs11_set_pin(digidoc::PKCS11Signer& signer, rust::Str pin) -> void;
auto pkcs11_as_signer(const digidoc::PKCS11Signer& signer) noexcept
    -> const digidoc::Signer&;
auto pkcs11_as_signer_mut(digidoc::PKCS11Signer& signer) noexcept
    -> digidoc::Signer&;

auto make_pkcs12_signer(rust::Str path, rust::Str pass)
    -> std::unique_ptr<digidoc::PKCS12Signer>;
auto pkcs12_as_signer(const digidoc::PKCS12Signer& signer) noexcept
    -> const digidoc::Signer&;
auto pkcs12_as_signer_mut(digidoc::PKCS12Signer& signer) noexcept
    -> digidoc::Signer&;

auto cert(const digidoc::Signer& signer) -> rust::Vec<uint8_t>;
auto sign(const digidoc::Signer& signer, rust::Str method,
          rust::Slice<const uint8_t> digest) -> rust::Vec<uint8_t>;
auto method(const digidoc::Signer& signer) -> rust::String;
auto profile(const digidoc::Signer& signer) -> rust::String;
auto user_agent(const digidoc::Signer& signer) -> rust::String;
auto using_en_profile(const digidoc::Signer& signer) -> bool;
auto city(const digidoc::Signer& signer) -> rust::String;
auto street_address(const digidoc::Signer& signer) -> rust::String;
auto state_or_province(const digidoc::Signer& signer) -> rust::String;
auto postal_code(const digidoc::Signer& signer) -> rust::String;
auto country_name(const digidoc::Signer& signer) -> rust::String;
auto signer_roles(const digidoc::Signer& signer) -> rust::Vec<rust::String>;

auto set_method(digidoc::Signer& signer, rust::Str method) -> void;
auto set_profile(digidoc::Signer& signer, rust::Str profile) -> void;
auto set_user_agent(digidoc::Signer& signer, rust::Str user_agent) -> void;
auto set_en_profile(digidoc::Signer& signer, bool enable) -> void;
auto set_signature_production_place(digidoc::Signer& signer, rust::Str city,
                                    rust::Str state_or_province,
                                    rust::Str postal_code,
                                    rust::Str country_name) -> void;
auto set_signature_production_place_v2(digidoc::Signer& signer, rust::Str city,
                                       rust::Str street_address,
                                       rust::Str state_or_province,
                                       rust::Str postal_code,
                                       rust::Str country_name) -> void;
auto set_signer_roles(digidoc::Signer& signer,
                      const rust::Vec<rust::String>& roles) -> void;
}  // namespace digidoc_rs::signer
