#pragma once
#include <cstdint>

#include "rust/cxx.h"

namespace digidoc_rs::exception {

auto throw_digidoc_exception(rust::Slice<const std::uint8_t> message) -> void;
auto throw_std_exception(rust::Slice<const std::uint8_t> message) -> void;
auto throw_non_std_exception() -> void;
auto throw_digidoc_exception_with_causes() -> void;
auto throw_digidoc_exception_with_invalid_code() -> void;

}  // namespace digidoc_rs::exception
