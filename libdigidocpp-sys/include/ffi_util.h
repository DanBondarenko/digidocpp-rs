#pragma once

#include <functional>
#include <vector>

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

template <typename T>
auto to_rust_vec(const std::vector<T>& values) -> rust::Vec<T> {
  return to_rust_vec<T>(values, std::identity{});
}

}  // namespace digidoc_rs::util
