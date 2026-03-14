#include "libdigidocpp-sys/include/init_ffi.h"

#include <string>

#include "digidocpp/Container.h"
#include "rust/cxx.h"

namespace digidoc_rs::init {
auto version() -> rust::String { return digidoc::version(); }

auto app_info() -> rust::String { return digidoc::appInfo(); }

auto user_agent() -> rust::String { return digidoc::userAgent(); }

auto initialize_default() -> void { digidoc::initialize(); }

auto initialize(const rust::Str app_info) -> void {
  digidoc::initialize(std::string(app_info));
}

auto initialize_with_user_agent(const rust::Str app_info,
                                const rust::Str user_agent) -> void {
  digidoc::initialize(std::string(app_info), std::string(user_agent));
}
}  // namespace digidoc_rs::init
