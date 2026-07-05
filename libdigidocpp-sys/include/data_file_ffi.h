#pragma once
#include <cstdint>

#include "digidocpp/DataFile.h"
#include "rust/cxx.h"

namespace digidoc_rs::data_file {
auto id(const digidoc::DataFile& data_file) -> rust::String;
auto media_type(const digidoc::DataFile& data_file) -> rust::String;
auto file_name(const digidoc::DataFile& data_file) -> rust::String;
auto file_size(const digidoc::DataFile& data_file) -> uint64_t;
auto calc_digest(const digidoc::DataFile& file, rust::Str method)
    -> rust::Vec<uint8_t>;
auto save_as(const digidoc::DataFile& data_file, rust::Str path) -> void;
}  // namespace digidoc_rs::data_file