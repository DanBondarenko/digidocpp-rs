#pragma once

#include <cstdint>
#include <functional>
#include <vector>

#include "digidocpp/crypto/X509Cert.h"
#include "rust/cxx.h"

namespace digidoc_rs::util {

template <typename T, typename Container, typename F>
auto to_rust_vec(const Container& values, F convert) -> rust::Vec<T> {
  rust::Vec<T> out;
  out.reserve(values.size());
  for (const auto& value : values) {
    out.push_back(convert(value));
  }
  return out;
}

template <typename T, typename Container>
auto to_rust_vec(const Container& values) -> rust::Vec<T> {
  return to_rust_vec<T>(values,
                        [](const auto& value) -> T { return T{value}; });
}

template <typename T>
auto to_rust_vec(const std::vector<T>& values) -> rust::Vec<T> {
  return to_rust_vec<T>(values, std::identity{});
}

inline auto to_rust_der(const digidoc::X509Cert& cert) -> rust::Vec<uint8_t> {
  const std::vector<unsigned char> der = cert;
  return to_rust_vec(der);
}

}  // namespace digidoc_rs::util
