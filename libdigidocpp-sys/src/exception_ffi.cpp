#include "libdigidocpp-sys/include/exception_ffi.h"
#include <format>
#include "rust/cxx.h"

namespace digidoc_rs::exception {

rust::String magic() {
    return rust::String(std::string(kMagic));
}

std::string serialize(const digidoc::Exception &e) {
    std::string file = e.file();

    return std::format(
        "{}{:08x}{:08x}{:08x}{}{}",
        kMagic,
        static_cast<uint32_t>(e.code()),
        static_cast<uint32_t>(e.line()),
        static_cast<uint32_t>(file.size()),
        file,
        e.msg()
    );
}

}

