#include "libdigidocpp-sys/include/exception_ffi.h"

#include <exception>
#include <string>

#include "digidocpp/Exception.h"
#include "libdigidocpp-sys/src/exception.rs.h"

namespace digidoc_rs::exception {

auto serialize_exception(const digidoc::Exception& exception) noexcept
    -> std::string {
  return std::string(ExceptionData{
      .code = exception.code(),
      .message = exception.msg(),
      .file = exception.file(),
      .line = exception.line(),
      .is_file_and_line_determined = true,
  }
                         .to_json());
}

auto serialize_exception(const std::exception& exception) noexcept
    -> std::string {
  return std::string(ExceptionData{
      .code = digidoc::ExceptionCode::General,
      .message = exception.what(),
      .file = "",
      .line = 0,
      .is_file_and_line_determined = false,
  }
                         .to_json());
}

}  // namespace digidoc_rs::exception
