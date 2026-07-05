#[cxx::bridge]
pub mod ffi {
    #[namespace = "digidoc"]
    #[allow(missing_debug_implementations)]
    unsafe extern "C++" {
        include!("digidocpp/Container.h");

        type Container;
        type DataFile = crate::data_file::ffi::DataFile;
        type Signature = crate::signature::ffi::Signature;

        #[Self = "Container"]
        fn openPtr(path: &CxxString) -> Result<UniquePtr<Container>>;

        #[Self = "Container"]
        fn createPtr(path: &CxxString) -> Result<UniquePtr<Container>>;
    }

    #[namespace = "digidoc_rs::container"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("libdigidocpp-sys/include/container_ffi.h");

        fn media_type(container: &Container) -> Result<String>;
        fn data_files_size(container: &Container) -> Result<usize>;
        fn data_file_at(container: &Container, index: usize) -> Result<&DataFile>;
        fn signatures_size(container: &Container) -> Result<usize>;
        fn signature_at(container: &Container, index: usize) -> Result<&Signature>;
        fn add_data_file(
            container: Pin<&mut Container>,
            path: &str,
            media_type: &str,
        ) -> Result<()>;
        fn remove_data_file(container: Pin<&mut Container>, index: u32) -> Result<()>;
    }
}
