mod init;
mod container;
mod data_file;

pub use init::*;
pub use container::*;
pub use data_file::*;

use libdigidocpp_sys::exception::FfiException;

fn fallible<F, R>(func: F) -> Result<R, FfiException>
where F: FnOnce() -> Result<R, cxx::Exception> {
    func().map_err(FfiException::from)
}