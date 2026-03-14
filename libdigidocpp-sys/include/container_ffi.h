#pragma once
#include <cstddef>

#include "digidocpp/Container.h"
#include "rust/cxx.h"

namespace digidoc_rs::container {
auto media_type(const digidoc::Container& container) -> rust::String;
auto data_files_size(const digidoc::Container& container) -> size_t;
auto data_file_at(const digidoc::Container& container, size_t index)
    -> const digidoc::DataFile&;
}  // namespace digidoc_rs::container