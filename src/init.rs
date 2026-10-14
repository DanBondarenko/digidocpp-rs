use crate::Error;
use crate::util::{ffi_variant_doc, library_and_ffi_variants_doc, library_variant_doc};
use libdigidocpp_sys::init::ffi;

/// Returns the libdigidocpp version.
///
/// # Errors
#[doc = library_and_ffi_variants_doc!()]
pub fn version() -> Result<String, Error> {
    Ok(ffi::version()?)
}

/// Returns the application name given to `initialize*`.
///
/// This can be set with `initialize*` functions. Defaults to `"libdigidocpp"`. libdigidocpp writes
/// this into the comments of containers it saves.
///
/// # Errors
#[doc = library_and_ffi_variants_doc!()]
pub fn app_info() -> Result<String, Error> {
    Ok(ffi::app_info()?)
}

/// Returns the user agent sent to online services (OCSP, time-stamping).
///
/// This can be set with `initialize*` functions. Defaults to `"libdigidocpp"`.
///
/// # Errors
#[doc = library_and_ffi_variants_doc!()]
pub fn user_agent() -> Result<String, Error> {
    Ok(ffi::user_agent()?)
}

/// Initializes the libraries and loads the TSL.
///
/// Call this before calling any of the [`Container`](crate::Container) API.
/// If you wish to configure libdigidocpp, you must call [`init_conf`](crate::init_conf) before
/// calling this. Otherwise, installs a default configuration.
///
/// Any repeated calls reload the TSL.
///
/// # Errors
#[doc = library_variant_doc!()]
/// For example, libdigidocpp fails when a dependent library does not initialize or the TSL cannot
/// be loaded (network failure, expired list or untrusted signature).
#[doc = ffi_variant_doc!()]
pub fn initialize() -> Result<(), Error> {
    Ok(ffi::initialize_default()?)
}

/// Initializes the libraries and loads the TSL, saving `app_info` as the application name.
///
/// Call this before calling any of the [`Container`](crate::Container) API.
/// If you wish to configure libdigidocpp, you must call [`init_conf`](crate::init_conf) before
/// calling this. Otherwise, installs a default configuration.
///
/// `app_info` is written into container comments. Any repeated calls reload the TSL.
///
/// # Errors
#[doc = library_variant_doc!()]
/// For example, libdigidocpp fails when a dependent library does not initialize or the TSL cannot
/// be loaded (network failure, expired list or untrusted signature).
#[doc = ffi_variant_doc!()]
pub fn initialize_with_app_info(app_info: &str) -> Result<(), Error> {
    Ok(ffi::initialize(app_info)?)
}

/// Initializes the libraries and loads the TSL, saving `app_info` as the application name and
/// `user_agent` as the user agent.
///
/// Call this before calling any of the [`Container`](crate::Container) API.
/// If you wish to configure libdigidocpp, you must call [`init_conf`](crate::init_conf) before
/// calling this. Otherwise, installs a default configuration.
///
/// `app_info` is written into container comments. `user_agent` is sent to online services. Any
/// repeated calls reload the TSL.
///
/// # Errors
#[doc = library_variant_doc!()]
/// For example, libdigidocpp fails when a dependent library does not initialize or the TSL cannot
/// be loaded (network failure, expired list or untrusted signature).
#[doc = ffi_variant_doc!()]
pub fn initialize_with_user_agent(app_info: &str, user_agent: &str) -> Result<(), Error> {
    Ok(ffi::initialize_with_user_agent(app_info, user_agent)?)
}

/// Deletes the installed configuration and shuts the libraries down.
///
/// # Safety
///
/// This may only be called once. Drop every library object, such as a
/// [`Container`](crate::Container), before the call. After this call, no part of the API may be
/// used, nothing may be called. Failure to do say may result in crashing the process. This is
/// upstream `libdigidocpp` behaviour.
///
/// # Errors
#[doc = library_and_ffi_variants_doc!()]
pub unsafe fn terminate() -> Result<(), Error> {
    // SAFETY: caller upholds contract of this function
    Ok(unsafe { ffi::terminate() }?)
}
