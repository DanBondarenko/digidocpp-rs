use crate::Error;
use libdigidocpp_sys::init::ffi;

pub fn version() -> Result<String, Error> {
    Ok(ffi::version()?)
}

pub fn app_info() -> Result<String, Error> {
    Ok(ffi::app_info()?)
}

pub fn user_agent() -> Result<String, Error> {
    Ok(ffi::user_agent()?)
}

/// Initializes the libraries and loads the TSL.
///
/// Call it before any `Container` function. It installs a default configuration when none is
/// installed. To use another configuration, call [`init_conf`](crate::init_conf) first. Calling
/// it again reloads the TSL. Do not call it after [`terminate`].
pub fn initialize() -> Result<(), Error> {
    Ok(ffi::initialize_default()?)
}

pub fn initialize_with_app_info(app_info: &str) -> Result<(), Error> {
    Ok(ffi::initialize(app_info)?)
}

pub fn initialize_with_user_agent(app_info: &str, user_agent: &str) -> Result<(), Error> {
    Ok(ffi::initialize_with_user_agent(app_info, user_agent)?)
}

/// Deletes the installed configuration and shuts the libraries down.
///
/// Call it once, when the process stops using libdigidocpp. Afterwards, parsing a certificate
/// fails, and calling [`initialize`] crashes the process.
pub fn terminate() -> Result<(), Error> {
    Ok(ffi::terminate()?)
}
