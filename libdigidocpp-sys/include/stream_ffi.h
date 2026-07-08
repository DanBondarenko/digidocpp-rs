#pragma once

#include <functional>
#include <istream>
#include <memory>
#include <ostream>

#include "libdigidocpp-sys/src/stream.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::stream {

// provides ostream that minimizes ffi calls and rethrows rust errors/panics
auto write_through(Sink& sink,
                   const std::function<void(std::ostream&)>& produce) -> void;

// provides istream that minimizes ffi calls and rethrows rust errors/panics
auto make_input_stream(rust::Box<Source> reader)
    -> std::unique_ptr<std::istream>;

}  // namespace digidoc_rs::stream
