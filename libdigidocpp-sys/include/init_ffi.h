#pragma once
#include "rust/cxx.h"

namespace digidoc_rs::init {
auto version() -> rust::String;
auto app_info() -> rust::String;
auto user_agent() -> rust::String;
void initialize_default();
void initialize(rust::Str app_info);
void initialize_with_user_agent(rust::Str app_info, rust::Str user_agent);
}  // namespace digidoc_rs::init