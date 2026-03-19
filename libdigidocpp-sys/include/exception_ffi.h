#pragma once
#include <exception>
#include <string>

#include "digidocpp/Exception.h"

namespace digidoc {
using ExceptionCode = Exception::ExceptionCode;
}

namespace digidoc_rs::exception {
auto serialize_exception(const digidoc::Exception& exception) noexcept
    -> std::string;
auto serialize_exception(const std::exception& exception) noexcept
    -> std::string;
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
}

}  // namespace rust::behavior