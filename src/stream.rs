#![expect(clippy::redundant_pub_crate, reason = "Internal module")]

use crate::Error;
use libdigidocpp_sys::stream;

/// runs C++ that may call back into rust `Read`/`Write`, carries errors, resumes panics
pub(crate) fn call_then_deliver_stream_failure<T>(
    ffi_call: impl FnOnce() -> Result<T, cxx::Exception>,
) -> Result<T, Error> {
    Ok(stream::call_then_deliver_stream_failure(ffi_call).map_err(Error::Io)??)
}
