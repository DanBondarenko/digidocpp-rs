#include "libdigidocpp-sys/include/test_util.h"

#include <cstddef>
#include <cstdint>
#include <ios>
#include <ostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "digidocpp/Exception.h"
#include "libdigidocpp-sys/include/ffi_util.h"
#include "libdigidocpp-sys/include/stream_ffi.h"
#include "libdigidocpp-sys/src/exception.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::exception {
namespace {
struct UnrelatedException {};

auto to_std_string(const rust::Slice<const std::uint8_t> bytes) -> std::string {
  return {bytes.begin(), bytes.end()};
}

auto make_digidoc_exception_with_causes() -> digidoc::Exception {
  digidoc::Exception root("root.cpp", 1, "root");
  root.addCause(digidoc::Exception("a.cpp", 2, "cause a",
                                   digidoc::Exception("c.cpp", 3, "cause c")));
  root.addCause(digidoc::Exception("b.cpp", 4, "cause b"));
  return root;
}

auto make_digidoc_exception_with_invalid_code() -> digidoc::Exception {
  digidoc::Exception exception("root.cpp", 1, "root");
  constexpr int kUnknownExceptionCode = 999;
  // NOLINTNEXTLINE(clang-analyzer-optin.core.EnumCastOutOfRange)
  exception.setCode(static_cast<digidoc::ExceptionCode>(kUnknownExceptionCode));
  return exception;
}
}  // namespace

auto throw_digidoc_exception(const rust::Slice<const std::uint8_t> message)
    -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw digidoc::Exception("test.cpp", 1, to_std_string(message));
}

auto throw_std_exception(const rust::Slice<const std::uint8_t> message)
    -> void {
  throw std::runtime_error(to_std_string(message));
}

auto throw_non_std_exception() -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw UnrelatedException();
}

auto throw_digidoc_exception_with_causes() -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw make_digidoc_exception_with_causes();
}

auto throw_digidoc_exception_with_invalid_code() -> void {
  // NOLINTNEXTLINE(bugprone-std-exception-baseclass)
  throw make_digidoc_exception_with_invalid_code();
}

}  // namespace digidoc_rs::exception

namespace digidoc_rs::stream {
namespace {
auto to_bytes(const std::string& text) -> rust::Vec<std::uint8_t> {
  const std::vector<std::uint8_t> bytes(text.begin(), text.end());
  return util::to_rust_vec(bytes);
}
}  // namespace

auto copy_bytes_through_sink(Sink& sink,
                             const rust::Slice<const std::uint8_t> bytes)
    -> void {
  const std::istringstream source(std::string(bytes.begin(), bytes.end()));
  write_through(sink, [&source](std::ostream& output) -> void {
    output << source.rdbuf();
  });
}

auto read_all_from_source(rust::Box<Source> source,
                          const std::size_t chunk_size)
    -> rust::Vec<std::uint8_t> {
  const auto input = make_input_stream(std::move(source));
  std::string collected;
  std::string chunk(chunk_size, '\0');
  while (*input) {
    input->read(chunk.data(), static_cast<std::streamsize>(chunk_size));
    collected.append(chunk, 0, static_cast<std::size_t>(input->gcount()));
  }
  return to_bytes(collected);
}

auto copy_source_via_rdbuf(rust::Box<Source> source)
    -> rust::Vec<std::uint8_t> {
  const auto input = make_input_stream(std::move(source));
  std::ostringstream output;
  output << input->rdbuf();
  return to_bytes(output.str());
}

}  // namespace digidoc_rs::stream
