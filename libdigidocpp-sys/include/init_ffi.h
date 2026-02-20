#pragma once
#include <string>
#include "rust/cxx.h"

namespace digidoc_rs {
    rust::String version();
    rust::String app_info();
    rust::String user_agent();
    void initialize_default();
    void initialize(const rust::Str app_info);
    void initialize_with_user_agent(const rust::Str app_info, const rust::Str user_agent);
}