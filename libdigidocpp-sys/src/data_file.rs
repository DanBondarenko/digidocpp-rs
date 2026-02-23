#[cxx::bridge]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/DataFile.h");
        type DataFile;
    }

    #[namespace = "digidoc_rs::data_file"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/data_file_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        fn id(data_file: &DataFile) -> Result<String>;
        fn file_name(data_file: &DataFile) -> Result<String>;
        fn file_size(data_file: &DataFile) -> Result<u64>;
        fn calc_digest(data_file: &DataFile, method: &str) -> Result<Vec<u8>>;
    }
}