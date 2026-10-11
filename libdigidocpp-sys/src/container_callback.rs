use crate::container::ffi::Container;
use cxx::{Exception, UniquePtr};
use std::any::Any;
use std::fmt::{self, Debug, Formatter};
use std::panic::{self, AssertUnwindSafe};

#[cxx::bridge]
#[allow(unreachable_pub)]
mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("digidocpp/Container.h");

        type Container = crate::container::ffi::Container;
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

        fn open_with_callback(
            path: &str,
            callback: &mut ContainerOpenCallback<'_>,
        ) -> Result<UniquePtr<Container>>;
    }
}

struct ContainerOpenCallback<'a> {
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
