#pragma once
#include "digidocpp/Exception.h"
#include "rust/cxx.h"

namespace digidoc {
    using ExceptionCode = Exception::ExceptionCode;
}

namespace digidoc_rs {
    std::string serialize_exception(const digidoc::Exception &e);
    inline constexpr std::string_view kExceptionMagic = "DDE1";
    rust::String exception_magic();
}

namespace rust::behavior {

template <typename Try, typename Fail>
static void trycatch(Try &&func, Fail &&fail) noexcept try {
    func();
} catch (const digidoc::Exception &e) {
    fail(digidoc_rs::serialize_exception(e).c_str());
} catch (const std::exception &e) {
    fail(e.what());
}

}