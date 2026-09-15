use std::fmt::{self, Debug, Formatter};

#[cxx::bridge]
#[allow(clippy::exhaustive_structs)]
pub mod ffi {
    #[namespace = "digidoc_rs::container"]
    #[derive(Debug)]
    struct ExtendedValidity {
        container: UniquePtr<Container>,
        extended_count: usize,
    }

    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/Container.h");

        type Container;
        type DataFile = crate::data_file::ffi::DataFile;
        type Signature = crate::signature::ffi::Signature;
        type Signer = crate::signer::ffi::Signer;
    }

    #[namespace = "digidoc_rs::stream"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/src/stream.rs.h");

        type Source = crate::stream::Source;
    }

    #[namespace = "digidoc_rs::container"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("libdigidocpp-sys/include/container_ffi.h");

        fn open(path: &str) -> Result<UniquePtr<Container>>;
        fn create(path: &str) -> Result<UniquePtr<Container>>;
        fn media_type(container: &Container) -> Result<String>;
        fn data_files_size(container: &Container) -> Result<usize>;
        fn data_file_at(container: &Container, index: usize) -> Result<&DataFile>;
        fn signatures_size(container: &Container) -> Result<usize>;
        fn signature_at(container: &Container, index: usize) -> Result<&Signature>;
        fn signature_at_mut(
            container: Pin<&mut Container>,
            index: usize,
        ) -> Result<Pin<&mut Signature>>;
        fn remove_signature(container: Pin<&mut Container>, index: u32) -> Result<()>;
        fn extend_container_validity(
            doc: Pin<&mut Container>,
            signer: Pin<&mut Signer>,
        ) -> Result<ExtendedValidity>;
        fn sign<'a>(
            container: Pin<&'a mut Container>,
            signer: Pin<&mut Signer>,
        ) -> Result<Pin<&'a mut Signature>>;
        fn prepare_signature<'a>(
            container: Pin<&'a mut Container>,
            signer: Pin<&mut Signer>,
        ) -> Result<Pin<&'a mut Signature>>;
        fn add_data_file(
            container: Pin<&mut Container>,
            path: &str,
            media_type: &str,
        ) -> Result<()>;
        fn remove_data_file(container: Pin<&mut Container>, index: u32) -> Result<()>;
        fn add_ades_signature(container: Pin<&mut Container>, signature: &[u8]) -> Result<()>;
        fn save(container: Pin<&mut Container>, path: &str) -> Result<()>;
        fn add_data_file_from_reader(
            container: Pin<&mut Container>,
            name: &str,
            media_type: &str,
            reader: Box<Source>,
        ) -> Result<()>;
    }
}

impl Debug for ffi::Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Container").finish_non_exhaustive()
    }
}
