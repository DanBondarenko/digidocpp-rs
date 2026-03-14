#include "libdigidocpp-sys/include/exception_ffi.h"

#include <cstdint>
#include <format>
#include <string>

#include "digidocpp/Exception.h"
#include "rust/cxx.h"

namespace digidoc_rs::exception {

auto magic() -> rust::String { return {std::string(kMagic)}; }

auto serialize(const digidoc::Exception& exception) -> std::string {
  std::string file = exception.file();

  return std::format("{}{:08x}{:08x}{:08x}{}{}", kMagic,
                     static_cast<uint32_t>(exception.code()),
                     static_cast<uint32_t>(exception.line()),
                     static_cast<uint32_t>(file.size()), file, exception.msg());
}

}  // namespace digidoc_rs::exception
