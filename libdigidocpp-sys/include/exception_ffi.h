#pragma once
#include <exception>
#include <string>

#include "digidocpp/Exception.h"

namespace digidoc {
using ExceptionCode = Exception::ExceptionCode;
}

namespace digidoc_rs::exception {
auto serialize_exception(const digidoc::Exception& exception) -> std::string;
auto serialize_exception(const std::exception& exception) -> std::string;
auto serialize_unknown_exception() -> std::string;
auto serialize_ffi_exception(const char* message) -> std::string;

}  // namespace digidoc_rs::exception

namespace rust::behavior {

template <typename Try, typename Fail>
static auto trycatch(Try&& func, Fail&& fail) noexcept -> void try {
  std::forward<Try>(func)();
} catch (const digidoc::Exception& exception) {
  std::forward<Fail>(fail)(
      std::string(digidoc_rs::exception::serialize_exception(exception)));
} catch (const std::exception& exception) {
  std::forward<Fail>(fail)(
      std::string(digidoc_rs::exception::serialize_exception(exception)));
} catch (...) {
  std::forward<Fail>(fail)(
      std::string(digidoc_rs::exception::serialize_unknown_exception()));
}

}  // namespace rust::behavior
