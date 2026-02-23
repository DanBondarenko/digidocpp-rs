#include "libdigidocpp-sys/include/container_ffi.h"
#include "digidocpp/Container.h"
#include "rust/cxx.h"

namespace digidoc_rs::container {
    rust::String media_type(const digidoc::Container &container) {
        return container.mediaType();
    }

    size_t data_files_size(const digidoc::Container &container) {
        return container.dataFiles().size();
    }

    const digidoc::DataFile &data_file_at(const digidoc::Container &container, size_t index) {
        return *container.dataFiles().at(index);
    }
}
