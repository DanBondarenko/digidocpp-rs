#include "libdigidocpp-sys/include/container_ffi.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "digidocpp/Container.h"
#include "digidocpp/crypto/Signer.h"
#include "libdigidocpp-sys/include/stream_ffi.h"
#include "libdigidocpp-sys/src/container.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::container {
namespace {

class RustContainerOpenCb final : public digidoc::ContainerOpenCB {
 public:
  explicit RustContainerOpenCb(ContainerOpenCallback& callback)
      : callback_(callback) {}
  RustContainerOpenCb(const RustContainerOpenCb&) = delete;
  RustContainerOpenCb(RustContainerOpenCb&&) = delete;
  auto operator=(const RustContainerOpenCb&) -> RustContainerOpenCb& = delete;
  auto operator=(RustContainerOpenCb&&) -> RustContainerOpenCb& = delete;
  ~RustContainerOpenCb() override = default;

  [[nodiscard]] auto validateOnline() const -> bool override {
    return callback_.validate_online();
  }

 private:
  ContainerOpenCallback& callback_;
};

}  // namespace

auto open(const rust::Str path) -> std::unique_ptr<digidoc::Container> {
  return digidoc::Container::openPtr(std::string(path));
}

auto open_with_callback(const rust::Str path, ContainerOpenCallback& callback)
    -> std::unique_ptr<digidoc::Container> {
  RustContainerOpenCb adapter(callback);
  return digidoc::Container::openPtr(std::string(path), &adapter);
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

// NOLINTNEXTLINE(misc-const-correctness): forced by cxx
auto signature_at_mut(digidoc::Container& container, const size_t index)
    -> digidoc::Signature& {
  return *container.signatures().at(index);
}

auto remove_signature(digidoc::Container& container, const uint32_t index)
    -> void {
  container.removeSignature(index);
}

auto extend_container_validity(digidoc::Container& container, digidoc::Signer& signer)
    -> ExtendedValidity {
  size_t extended_count = 0;
  std::unique_ptr<digidoc::Container> nullable_container =
      digidoc::Container::extendContainerValidity(container, &signer, extended_count);
  return ExtendedValidity{
      .container = std::move(nullable_container),
      .extended_count = extended_count,
  };
}

auto sign(digidoc::Container& container, digidoc::Signer& signer)
    -> digidoc::Signature& {
  digidoc::Signature* const signature = container.sign(&signer);
  if (signature == nullptr) {
    throw std::runtime_error("signature is null");
  }
  return *signature;
}

auto prepare_signature(digidoc::Container& container, digidoc::Signer& signer)
    -> digidoc::Signature& {
  digidoc::Signature* const signature = container.prepareSignature(&signer);
  if (signature == nullptr) {
    throw std::runtime_error("signature is null");
  }
  return *signature;
}

auto add_data_file(digidoc::Container& container, const rust::Str path,
                   const rust::Str media_type) -> void {
  container.addDataFile(std::string(path), std::string(media_type));
}

auto remove_data_file(digidoc::Container& container, const uint32_t index)
    -> void {
  container.removeDataFile(index);
}

auto add_ades_signature(digidoc::Container& container,
                        const rust::Slice<const uint8_t> signature) -> void {
  container.addAdESSignature(
      std::vector<unsigned char>(signature.begin(), signature.end()));
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
