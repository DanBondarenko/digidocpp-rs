#![expect(clippy::redundant_pub_crate, reason = "Internal utility module")]

use crate::Error;
use std::path::Path;

pub(crate) fn path_to_str(path: &Path) -> Result<&str, Error> {
    path.to_str()
        .ok_or_else(|| Error::Internal(format!("path is not valid UTF-8: {}", path.display())))
}

macro_rules! library_variant_doc {
    () => {
        " Returns [`Error::Library`] when libdigidocpp reports a failure."
    };
}

pub(crate) use library_variant_doc;

macro_rules! ffi_variant_doc {
    () => {
        " Returns [`Error::Ffi`] when the C++ exception could not be converted."
    };
}

pub(crate) use ffi_variant_doc;

macro_rules! library_and_ffi_variants_doc {
    () => {
        concat!(
            $crate::util::library_variant_doc!(),
            $crate::util::ffi_variant_doc!()
        )
    };
}

pub(crate) use library_and_ffi_variants_doc;

macro_rules! io_variant_doc {
    () => {
        " Returns [`Error::Io`] when a reader or writer fails."
    };
}

pub(crate) use io_variant_doc;

macro_rules! library_io_and_ffi_variants_doc {
    () => {
        concat!(
            $crate::util::library_variant_doc!(),
            $crate::util::io_variant_doc!(),
            $crate::util::ffi_variant_doc!()
        )
    };
}

pub(crate) use library_io_and_ffi_variants_doc;

macro_rules! reader_or_writer_panic_doc {
    () => {
        " Resumes the panic of a reader or writer that panicked during the call."
    };
}

pub(crate) use reader_or_writer_panic_doc;
