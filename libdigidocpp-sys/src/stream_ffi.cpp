#include "libdigidocpp-sys/include/stream_ffi.h"

#include <cstddef>
#include <cstdint>
#include <exception>
#include <functional>
#include <ios>
#include <istream>
#include <memory>
#include <ostream>
#include <span>
#include <streambuf>
#include <utility>
#include <vector>

#include "libdigidocpp-sys/src/stream.rs.h"
#include "rust/cxx.h"

namespace digidoc_rs::stream {
namespace {

// bytes before making ffi call
constexpr std::size_t kBufferSize = std::size_t{64} * 1024;

// buffers bytes before forwarding to rust to minimize ffi calls
// rust errors/panics are stored and rethrown
class WriterBuf final : public std::streambuf {
 public:
  explicit WriterBuf(Sink& writer) : writer_(writer), buffer_(kBufferSize) {
    reset_put_area();
  }
  WriterBuf(const WriterBuf&) = delete;
  WriterBuf(WriterBuf&&) = delete;
  auto operator=(const WriterBuf&) -> WriterBuf& = delete;
  auto operator=(WriterBuf&&) -> WriterBuf& = delete;
  ~WriterBuf() override = default;

  void rethrow_failure() const {
    if (failure_) {
      std::rethrow_exception(failure_);
    }
  }

 protected:
  auto overflow(int_type character) -> int_type override {
    if (!flush()) {
      return traits_type::eof();
    }
    if (!traits_type::eq_int_type(character, traits_type::eof())) {
      *pptr() = traits_type::to_char_type(character);
      pbump(1);
    }
    return traits_type::not_eof(character);
  }

  auto sync() -> int override { return flush() ? 0 : -1; }

 private:
  void reset_put_area() {
    setp(buffer_.data(), std::to_address(buffer_.end()));
  }

  auto flush() -> bool {
    if (failure_) {
      return false;
    }
    const auto pending = static_cast<std::size_t>(pptr() - pbase());
    if (pending == 0) {
      return true;
    }
    try {
      // NOLINTNEXTLINE(cppcoreguidelines-pro-type-reinterpret-cast)
      const auto* bytes = reinterpret_cast<const std::uint8_t*>(pbase());
      writer_.write(rust::Slice<const std::uint8_t>(bytes, pending));
    } catch (...) {
      failure_ = std::current_exception();
      return false;
    }
    reset_put_area();
    return true;
  }

  Sink& writer_;
  std::vector<char> buffer_;
  std::exception_ptr failure_;
};

// buffers bytes before forwarding to rust to minimize ffi calls
// rust errors/panics are stored and rethrown
class ReaderBuf final : public std::streambuf {
 public:
  explicit ReaderBuf(rust::Box<Source> reader)
      : reader_(std::move(reader)), buffer_(kBufferSize) {}

 protected:
  auto underflow() -> int_type override {
    if (gptr() < egptr()) {
      return traits_type::to_int_type(*gptr());
    }
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-reinterpret-cast)
    auto* bytes = reinterpret_cast<std::uint8_t*>(buffer_.data());
    const std::size_t count =
        reader_->read(rust::Slice<std::uint8_t>(bytes, buffer_.size()));
    if (count == 0) {
      return traits_type::eof();
    }
    const std::span<char> filled = std::span(buffer_).first(count);
    setg(buffer_.data(), buffer_.data(), std::to_address(filled.end()));
    return traits_type::to_int_type(*gptr());
  }

  auto seekoff(off_type offset, std::ios_base::seekdir direction,
               std::ios_base::openmode mode) -> pos_type override {
    if ((mode & std::ios_base::in) == 0) {
      return {off_type{-1}};
    }
    const auto buffered = static_cast<off_type>(egptr() - gptr());
    if (direction == std::ios_base::cur && offset == 0) {
      return {static_cast<off_type>(reader_->seek_from_current(0)) - buffered};
    }
    std::uint64_t position = 0;
    if (direction == std::ios_base::beg) {
      position = reader_->seek_from_start(static_cast<std::uint64_t>(offset));
    } else if (direction == std::ios_base::end) {
      position = reader_->seek_from_end(offset);
    } else {
      position = reader_->seek_from_current(offset - buffered);
    }
    setg(buffer_.data(), buffer_.data(), buffer_.data());
    return {static_cast<off_type>(position)};
  }

  auto seekpos(pos_type position, std::ios_base::openmode mode)
      -> pos_type override {
    return seekoff(position, std::ios_base::beg, mode);
  }

 private:
  rust::Box<Source> reader_;
  std::vector<char> buffer_;
};

class OwnedIStream final : public std::istream {
 public:
  explicit OwnedIStream(rust::Box<Source> reader)
      : std::istream(nullptr), buffer_(std::move(reader)) {
    rdbuf(&buffer_);
    exceptions(std::ios::badbit);
  }

 private:
  ReaderBuf buffer_;
};

}  // namespace

auto write_through(Sink& sink,
                   const std::function<void(std::ostream&)>& produce) -> void {
  WriterBuf buffer(sink);
  std::ostream output(&buffer);
  produce(output);
  buffer.pubsync();
  buffer.rethrow_failure();
}

auto make_input_stream(rust::Box<Source> reader)
    -> std::unique_ptr<std::istream> {
  return std::make_unique<OwnedIStream>(std::move(reader));
}

}  // namespace digidoc_rs::stream
