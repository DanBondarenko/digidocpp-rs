use cxx::{Exception, UniquePtr};
use ffi::Container;
use std::any::Any;
use std::fmt::{self, Debug, Formatter};
use std::panic::{self, AssertUnwindSafe};

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
    extern "Rust" {
        type ContainerOpenCallback<'a>;

        fn validate_online(self: &mut ContainerOpenCallback<'_>) -> bool;
    }

    #[namespace = "digidoc_rs::container"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("libdigidocpp-sys/include/container_ffi.h");

        fn open(path: &str) -> Result<UniquePtr<Container>>;
        fn open_with_callback(
            path: &str,
            callback: &mut ContainerOpenCallback<'_>,
        ) -> Result<UniquePtr<Container>>;
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
            container: Pin<&mut Container>,
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

pub struct ContainerOpenCallback<'a> {
    consent: &'a mut dyn FnMut() -> bool,
    panic_payload: Option<Box<dyn Any + Send>>,
}

impl<'a> ContainerOpenCallback<'a> {
    fn new(consent: &'a mut dyn FnMut() -> bool) -> Self {
        Self {
            consent,
            panic_payload: None,
        }
    }

    fn resume_stored_panic(self) {
        if let Some(payload) = self.panic_payload {
            panic::resume_unwind(payload);
        }
    }

    fn validate_online(&mut self) -> bool {
        if self.panic_payload.is_some() {
            return false;
        }

        match panic::catch_unwind(AssertUnwindSafe(&mut *self.consent)) {
            Ok(consented) => consented,
            Err(payload) => {
                self.panic_payload = Some(payload);
                false
            }
        }
    }
}

impl Debug for ContainerOpenCallback<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContainerOpenCallback")
            .finish_non_exhaustive()
    }
}

pub fn open_with_callback_resuming_panic(
    path: &str,
    consent: &mut dyn FnMut() -> bool,
) -> Result<UniquePtr<Container>, Exception> {
    let mut callback = ContainerOpenCallback::new(consent);
    let result = ffi::open_with_callback(path, &mut callback);
    callback.resume_stored_panic();

    result
}

impl Debug for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Container").finish_non_exhaustive()
    }
}
