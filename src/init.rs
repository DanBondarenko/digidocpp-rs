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

/// # Safety
///
/// This may only be called once. Drop every library object before the call. After this call, no
/// part of the API may be used, nothing may be called. Failure to do say may result in crashing the
/// process. This is upstream `libdigidocpp` behaviour.
pub unsafe fn terminate() -> Result<(), Error> {
    // SAFETY: caller upholds contract of this function
    Ok(unsafe { ffi::terminate() }?)
}
