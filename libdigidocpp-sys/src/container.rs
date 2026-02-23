#[cxx::bridge]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/Container.h");

        type Container;
        type DataFile = crate::data_file::ffi::DataFile;

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
    }
}
