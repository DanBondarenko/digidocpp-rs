#include "libdigidocpp-sys/include/container_ffi.h"

#include <cstddef>

#include "digidocpp/Container.h"
#include "rust/cxx.h"

namespace digidoc_rs::container {
auto media_type(const digidoc::Container& container) -> rust::String {
  return container.mediaType();
}

auto data_files_size(const digidoc::Container& container) -> size_t {
  return container.dataFiles().size();
}

auto data_file_at(const digidoc::Container& container, const size_t index)
    -> const digidoc::DataFile& {
  return *container.dataFiles().at(index);
}
}  // namespace digidoc_rs::container
