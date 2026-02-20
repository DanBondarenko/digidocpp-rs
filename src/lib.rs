mod init;

pub use init::*;

use libdigidocpp_sys::FfiException;

fn fallible<F, R>(func: F) -> Result<R, FfiException>
where F: FnOnce() -> Result<R, cxx::Exception> {
    func().map_err(FfiException::from)
}