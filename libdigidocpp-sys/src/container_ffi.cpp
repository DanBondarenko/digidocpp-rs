#include "libdigidocpp-sys/include/container_ffi.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <utility>

#include "digidocpp/Container.h"
#include "libdigidocpp-sys/include/stream_ffi.h"
#include "rust/cxx.h"

namespace digidoc_rs::container {
auto open(const rust::Str path) -> std::unique_ptr<digidoc::Container> {
  return digidoc::Container::openPtr(std::string(path));
}

auto create(const rust::Str path) -> std::unique_ptr<digidoc::Container> {
  return digidoc::Container::createPtr(std::string(path));
}

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

auto signatures_size(const digidoc::Container& container) -> size_t {
  return container.signatures().size();
}

auto signature_at(const digidoc::Container& container, const size_t index)
    -> const digidoc::Signature& {
  return *container.signatures().at(index);
}

auto add_data_file(digidoc::Container& container, const rust::Str path,
                   const rust::Str media_type) -> void {
  container.addDataFile(std::string(path), std::string(media_type));
}

auto remove_data_file(digidoc::Container& container, const uint32_t index)
    -> void {
  container.removeDataFile(index);
}

auto save(digidoc::Container& container, const rust::Str path) -> void {
  container.save(std::string(path));
}

auto add_data_file_from_reader(digidoc::Container& container,
                               const rust::Str name, const rust::Str media_type,
                               rust::Box<stream::Source> reader) -> void {
  container.addDataFile(stream::make_input_stream(std::move(reader)),
                        std::string(name), std::string(media_type));
}
}  // namespace digidoc_rs::container
