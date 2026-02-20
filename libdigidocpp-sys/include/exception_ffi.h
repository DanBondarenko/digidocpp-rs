#pragma once
#include "digidocpp/Exception.h"
#include "rust/cxx.h"

namespace digidoc {
    using ExceptionCode = Exception::ExceptionCode;
}

namespace digidoc_rs::exception {
    std::string serialize(const digidoc::Exception &e);
    inline constexpr std::string_view kMagic = "DDE1";
    rust::String magic();
}

namespace rust::behavior {

template <typename Try, typename Fail>
static void trycatch(Try &&func, Fail &&fail) noexcept try {
    func();
} catch (const digidoc::Exception &e) {
    fail(digidoc_rs::exception::serialize(e).c_str());
} catch (const std::exception &e) {
    fail(e.what());
}

}