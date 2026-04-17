#include "libdigidocpp-sys/include/test_util.h"

#include <cstdint>
#include <stdexcept>
#include <string>

#include "digidocpp/Exception.h"
#include "libdigidocpp-sys/src/exception.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::exception {
namespace {
struct UnrelatedException {};

auto to_std_string(const rust::Slice<const std::uint8_t> bytes) -> std::string {
  return {bytes.begin(), bytes.end()};
}

auto make_digidoc_exception_with_causes() -> digidoc::Exception {
  digidoc::Exception root("root.cpp", 1, "root");
  root.addCause(digidoc::Exception("a.cpp", 2, "cause a",
                                   digidoc::Exception("c.cpp", 3, "cause c")));
  root.addCause(digidoc::Exception("b.cpp", 4, "cause b"));
  return root;
}

auto make_digidoc_exception_with_invalid_code() -> digidoc::Exception {
  digidoc::Exception exception("root.cpp", 1, "root");
  constexpr int kUnknownExceptionCode = 999;
  // NOLINTNEXTLINE(clang-analyzer-optin.core.EnumCastOutOfRange)
  exception.setCode(static_cast<digidoc::ExceptionCode>(kUnknownExceptionCode));
  return exception;
}
}  // namespace

auto throw_digidoc_exception(const rust::Slice<const std::uint8_t> message)
    -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw digidoc::Exception("test.cpp", 1, to_std_string(message));
}

auto throw_std_exception(const rust::Slice<const std::uint8_t> message)
    -> void {
  throw std::runtime_error(to_std_string(message));
}

auto throw_non_std_exception() -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw UnrelatedException();
}

auto throw_digidoc_exception_with_causes() -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw make_digidoc_exception_with_causes();
}

auto throw_digidoc_exception_with_invalid_code() -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw make_digidoc_exception_with_invalid_code();
}

}  // namespace digidoc_rs::exception
