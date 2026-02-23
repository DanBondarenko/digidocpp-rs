#include "rust/cxx.h"
#include "digidocpp/DataFile.h"

namespace digidoc_rs::data_file {
    rust::String id(const digidoc::DataFile &data_file) {
        return data_file.id();
    }

    rust::String file_name(const digidoc::DataFile &data_file) {
        return data_file.fileName();
    }

    uint64_t file_size(const digidoc::DataFile &data_file) {
        return data_file.fileSize();
    }

    rust::Vec<uint8_t> calc_digest(const digidoc::DataFile &file, const rust::Str method) {
      auto digest = file.calcDigest(std::string(method));
      rust::Vec<uint8_t> out;
      out.reserve(digest.size());
      for (unsigned char byte : digest) {
        out.push_back(byte);
      }
      return out;
    }
}