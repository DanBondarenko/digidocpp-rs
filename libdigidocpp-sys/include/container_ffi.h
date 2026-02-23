#pragma once
#include "rust/cxx.h"
#include "digidocpp/Container.h"

namespace digidoc_rs::container {
    rust::String media_type(const digidoc::Container &container);
    size_t data_files_size(const digidoc::Container &container);
    const digidoc::DataFile &data_file_at(const digidoc::Container &container, size_t index);
}