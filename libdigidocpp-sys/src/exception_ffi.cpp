#include "libdigidocpp-sys/include/exception_ffi.h"

#include <cstdint>
#include <exception>
#include <string>

#include "digidocpp/Exception.h"
#include "libdigidocpp-sys/src/exception.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::exception {
namespace {

// Depth of recursion equals length of causes in libdigidocpp.
// NOLINTNEXTLINE(misc-no-recursion)
auto append_causes(const digidoc::Exception& parent, const std::uint32_t depth,
                   rust::Vec<ExceptionCause>& causes) -> void {
  for (const digidoc::Exception& cause : parent.causes()) {
    causes.push_back(ExceptionCause{
        .code = cause.code(),
        .message = rust::String::lossy(cause.msg()),
        .file = rust::String::lossy(cause.file()),
        .line = cause.line(),
        .depth = depth,
    });
    append_causes(cause, depth + 1, causes);
  }
}

auto collect_causes(const digidoc::Exception& exception)
    -> rust::Vec<ExceptionCause> {
  rust::Vec<ExceptionCause> causes;
  append_causes(exception, 0, causes);
  return causes;
}

}  // namespace

auto serialize_exception(const digidoc::Exception& exception) -> std::string {
  try {
    return std::string(ExceptionData{
        .is_ffi = false,
        .is_digidoc = true,
        .is_unknown = false,
        .message = rust::String::lossy(exception.msg()),
        .code = exception.code(),
        .file = rust::String::lossy(exception.file()),
        .line = exception.line(),
        .causes = collect_causes(exception),
    }
                           .to_json());
  } catch (const std::exception& serialization_exception) {
    return serialize_ffi_exception(serialization_exception.what());
  } catch (...) {
    return serialize_ffi_exception("Failed to serialize C++ exception");
  }
}

auto serialize_exception(const std::exception& exception) -> std::string {
  return std::string(ExceptionData{
      .is_ffi = false,
      .is_digidoc = false,
      .is_unknown = true,
      .message = rust::String::lossy(exception.what()),

      // Rest of the field values are stubs and meaningless
      .code = digidoc::ExceptionCode::General,
      .file = rust::String{},
      .line = 0,
      .causes = {},
  }
                         .to_json());
}

auto serialize_unknown_exception() -> std::string {
  return std::string(ExceptionData{
      .is_ffi = false,
      .is_digidoc = false,
      .is_unknown = true,
      .message = rust::String::lossy("Unknown C++ exception"),

      // Rest of the field values are stubs and meaningless
      .code = digidoc::ExceptionCode::General,
      .file = rust::String{},
      .line = 0,
      .causes = {},
  }
                         .to_json());
}

auto serialize_ffi_exception(const char* message) -> std::string {
  return std::string(ExceptionData{
      .is_ffi = true,
      .is_digidoc = false,
      .is_unknown = false,
      .message = rust::String::lossy(message),

      // Rest of the field values are stubs and meaningless
      .code = digidoc::ExceptionCode::General,
      .file = rust::String{},
      .line = 0,
      .causes = {},
  }
                         .to_json());
}

}  // namespace digidoc_rs::exception
