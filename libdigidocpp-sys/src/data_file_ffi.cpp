#include "libdigidocpp-sys/include/data_file_ffi.h"

#include <cstdint>
#include <string>

#include "digidocpp/DataFile.h"
#include "libdigidocpp-sys/include/ffi_util.h"
#include "rust/cxx.h"

namespace digidoc_rs::data_file {
auto id(const digidoc::DataFile& data_file) -> rust::String {
  return data_file.id();
}

auto media_type(const digidoc::DataFile& data_file) -> rust::String {
  return data_file.mediaType();
}

auto file_name(const digidoc::DataFile& data_file) -> rust::String {
  return data_file.fileName();
}

auto file_size(const digidoc::DataFile& data_file) -> uint64_t {
  return data_file.fileSize();
}

auto calc_digest(const digidoc::DataFile& file, const rust::Str method)
    -> rust::Vec<uint8_t> {
  return util::to_rust_vec(file.calcDigest(std::string(method)));
}

auto save_as(const digidoc::DataFile& data_file, const rust::Str path) -> void {
  data_file.saveAs(std::string(path));
}
}  // namespace digidoc_rs::data_file