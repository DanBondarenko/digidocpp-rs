#pragma once
#include <exception>
#include <string>
#include <string_view>

#include "digidocpp/Exception.h"
#include "rust/cxx.h"

namespace digidoc {
using ExceptionCode = Exception::ExceptionCode;
}

namespace digidoc_rs::exception {
auto serialize(const digidoc::Exception& exception) -> std::string;
inline constexpr std::string_view kMagic = "DDE1";
auto magic() -> rust::String;
}  // namespace digidoc_rs::exception

namespace rust::behavior {

template <typename Try, typename Fail>
static auto trycatch(Try&& func, Fail&& fail) noexcept -> void try {
  std::forward<Try>(func)();
} catch (const digidoc::Exception& exception) {
  std::forward<Fail>(fail)(digidoc_rs::exception::serialize(exception).c_str());
} catch (const std::exception& exception) {
  std::forward<Fail>(fail)(exception.what());
}

}  // namespace rust::behavior