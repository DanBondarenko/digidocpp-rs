#pragma once
#include <cstddef>
#include <cstdint>
#include <memory>

#include "digidocpp/Container.h"
#include "digidocpp/crypto/Signer.h"
#include "rust/cxx.h"

namespace digidoc_rs::stream {
struct Source;
}  // namespace digidoc_rs::stream

namespace digidoc_rs::container {
struct ExtendedValidity;
struct ContainerOpenCallback;

auto open(rust::Str path) -> std::unique_ptr<digidoc::Container>;
auto open_with_callback(rust::Str path, ContainerOpenCallback& callback)
    -> std::unique_ptr<digidoc::Container>;
auto create(rust::Str path) -> std::unique_ptr<digidoc::Container>;
auto media_type(const digidoc::Container& container) -> rust::String;
auto data_files_size(const digidoc::Container& container) -> size_t;
auto data_file_at(const digidoc::Container& container, size_t index)
    -> const digidoc::DataFile&;
auto signatures_size(const digidoc::Container& container) -> size_t;
auto signature_at(const digidoc::Container& container, size_t index)
    -> const digidoc::Signature&;
auto signature_at_mut(digidoc::Container& container, size_t index)
    -> digidoc::Signature&;
auto remove_signature(digidoc::Container& container, uint32_t index) -> void;
auto extend_container_validity(digidoc::Container& container,
                               digidoc::Signer& signer) -> ExtendedValidity;
auto sign(digidoc::Container& container, digidoc::Signer& signer)
    -> digidoc::Signature&;
auto prepare_signature(digidoc::Container& container, digidoc::Signer& signer)
    -> digidoc::Signature&;
auto add_data_file(digidoc::Container& container, rust::Str path,
                   rust::Str media_type) -> void;
auto remove_data_file(digidoc::Container& container, uint32_t index) -> void;
auto add_ades_signature(digidoc::Container& container,
                        rust::Slice<const uint8_t> signature) -> void;
auto save(digidoc::Container& container, rust::Str path) -> void;
auto add_data_file_from_reader(digidoc::Container& container, rust::Str name,
                               rust::Str media_type,
                               rust::Box<stream::Source> reader) -> void;
}  // namespace digidoc_rs::container
