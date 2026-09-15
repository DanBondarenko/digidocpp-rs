use std::fmt::{self, Debug, Formatter};

#[cxx::bridge]
#[allow(clippy::exhaustive_structs)]
#[allow(clippy::exhaustive_enums)]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/DataFile.h");
        type DataFile;
    }

    #[namespace = "digidoc_rs::stream"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/src/stream.rs.h");

        type Sink<'a> = crate::stream::Sink<'a>;
    }

    #[namespace = "digidoc_rs::data_file"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/data_file_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        fn id(data_file: &DataFile) -> Result<String>;
        fn media_type(data_file: &DataFile) -> Result<String>;
        fn file_name(data_file: &DataFile) -> Result<String>;
        fn file_size(data_file: &DataFile) -> Result<u64>;
        fn calc_digest(data_file: &DataFile, method: &str) -> Result<Vec<u8>>;
        fn save_as(data_file: &DataFile, path: &str) -> Result<()>;
        fn write_to(data_file: &DataFile, sink: &mut Sink<'_>) -> Result<()>;
    }
}

impl Debug for ffi::DataFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataFile").finish_non_exhaustive()
    }
}
