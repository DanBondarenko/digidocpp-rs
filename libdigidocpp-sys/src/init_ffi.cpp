#include "libdigidocpp-sys/include/init_ffi.h"
#include "digidocpp/Container.h"
#include "rust/cxx.h"

namespace digidoc_rs {

rust::String version() {
    return digidoc::version();
}

rust::String app_info() {
    return digidoc::appInfo();
}

rust::String user_agent() {
    return digidoc::userAgent();
}

void initialize_default() {
    digidoc::initialize();
}

void initialize(const rust::Str app_info) {
    digidoc::initialize(std::string(app_info));
}

void initialize_with_user_agent(const rust::Str app_info, const rust::Str user_agent) {
    digidoc::initialize(std::string(app_info), std::string(user_agent));
}

}