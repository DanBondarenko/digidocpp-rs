#pragma once
#include "rust/cxx.h"
#include "digidocpp/DataFile.h"

namespace digidoc_rs::data_file {
    rust::String id(const digidoc::DataFile &data_file);
    rust::String file_name(const digidoc::DataFile &data_file);
    uint64_t file_size(const digidoc::DataFile &data_file);
    rust::Vec<uint8_t> calc_digest(const digidoc::DataFile &file, const rust::Str method);
}