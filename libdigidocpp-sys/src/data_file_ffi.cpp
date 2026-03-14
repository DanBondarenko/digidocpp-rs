#include "libdigidocpp-sys/include/data_file_ffi.h"

#include <cstdint>
#include <string>

#include "digidocpp/DataFile.h"
#include "rust/cxx.h"

namespace digidoc_rs::data_file {
auto id(const digidoc::DataFile& data_file) -> rust::String {
  return data_file.id();
}

auto file_name(const digidoc::DataFile& data_file) -> rust::String {
  return data_file.fileName();
}

auto file_size(const digidoc::DataFile& data_file) -> uint64_t {
  return data_file.fileSize();
}

auto calc_digest(const digidoc::DataFile& file, const rust::Str method)
    -> rust::Vec<uint8_t> {
  const auto digest = file.calcDigest(std::string(method));
  rust::Vec<uint8_t> out;
  out.reserve(digest.size());
  for (const unsigned char byte : digest) {
    out.push_back(byte);
  }
  return out;
}
}  // namespace digidoc_rs::data_file