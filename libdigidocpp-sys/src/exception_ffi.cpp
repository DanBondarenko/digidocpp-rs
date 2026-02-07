#include "libdigidocpp-sys/include/exception_ffi.h"
#include <format>
#include "rust/cxx.h"

namespace digidoc_rs {

rust::String exception_magic() {
    return rust::String(std::string(kExceptionMagic));
}

std::string serialize_exception(const digidoc::Exception &e) {
    std::string file = e.file();

    return std::format(
        "{}{:08x}{:08x}{:08x}{}{}",
        kExceptionMagic,
        static_cast<uint32_t>(e.code()),
        static_cast<uint32_t>(e.line()),
        static_cast<uint32_t>(file.size()),
        file,
        e.msg()
    );
}

}

