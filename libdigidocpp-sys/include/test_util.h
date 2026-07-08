#pragma once
#include <cstddef>
#include <cstdint>

#include "rust/cxx.h"

namespace digidoc_rs::exception {

auto throw_digidoc_exception(rust::Slice<const std::uint8_t> message) -> void;
auto throw_std_exception(rust::Slice<const std::uint8_t> message) -> void;
auto throw_non_std_exception() -> void;
auto throw_digidoc_exception_with_causes() -> void;
auto throw_digidoc_exception_with_invalid_code() -> void;

}  // namespace digidoc_rs::exception

namespace digidoc_rs::stream {

struct Sink;
struct Source;

auto copy_bytes_through_sink(Sink& sink, rust::Slice<const std::uint8_t> bytes)
    -> void;
auto read_all_from_source(rust::Box<Source> source, std::size_t chunk_size)
    -> rust::Vec<std::uint8_t>;
auto copy_source_via_rdbuf(rust::Box<Source> source) -> rust::Vec<std::uint8_t>;

}  // namespace digidoc_rs::stream
