#pragma once
#include <cstddef>
#include <cstdint>
#include <memory>

#include "digidocpp/Container.h"
#include "rust/cxx.h"

namespace digidoc_rs::stream {
struct Source;
}  // namespace digidoc_rs::stream

namespace digidoc_rs::container {
auto open(rust::Str path) -> std::unique_ptr<digidoc::Container>;
auto create(rust::Str path) -> std::unique_ptr<digidoc::Container>;
auto media_type(const digidoc::Container& container) -> rust::String;
auto data_files_size(const digidoc::Container& container) -> size_t;
auto data_file_at(const digidoc::Container& container, size_t index)
    -> const digidoc::DataFile&;
auto signatures_size(const digidoc::Container& container) -> size_t;
auto signature_at(const digidoc::Container& container, size_t index)
    -> const digidoc::Signature&;
auto add_data_file(digidoc::Container& container, rust::Str path,
                   rust::Str media_type) -> void;
auto remove_data_file(digidoc::Container& container, uint32_t index) -> void;
auto save(digidoc::Container& container, rust::Str path) -> void;
auto add_data_file_from_reader(digidoc::Container& container, rust::Str name,
                               rust::Str media_type,
                               rust::Box<stream::Source> reader) -> void;
}  // namespace digidoc_rs::container
