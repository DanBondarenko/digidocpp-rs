//! Library lifecycle and process-wide information.
//!
//! Upstream has these functions in the [`digidoc` namespace].
//!
//! [`digidoc` namespace]: https://open-eid.github.io/libdigidocpp/namespacemembers_func.html

/// Bridge for upstream `digidocpp/Container.h` lifecycle functions.
#[cxx::bridge]
#[allow(clippy::missing_safety_doc, reason = "cxx")]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("digidocpp/Container.h");

        /// Calls upstream `digidoc::terminate`.
        ///
        /// # Safety
        ///
        /// Call it once, when the process stops using libdigidocpp. Drop every library object
        /// before the call. Make no library call afterwards.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        unsafe fn terminate() -> Result<()>;
    }

    #[namespace = "digidoc_rs::init"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/init_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        /// Calls upstream `digidoc::initialize`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when a library fails to
        /// initialize or the TSL cannot be loaded.
        fn initialize_default() -> Result<()>;

        /// Calls upstream `digidoc::initialize`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when a library fails to
        /// initialize or the TSL cannot be loaded.
        fn initialize(app_info: &str) -> Result<()>;

        /// Calls upstream `digidoc::initialize`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws, for example when a library fails to
        /// initialize or the TSL cannot be loaded.
        fn initialize_with_user_agent(app_info: &str, user_agent: &str) -> Result<()>;

        /// Calls upstream `digidoc::version`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn version() -> Result<String>;

        /// Calls upstream `digidoc::appInfo`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn app_info() -> Result<String>;

        /// Calls upstream `digidoc::userAgent`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn user_agent() -> Result<String>;
    }
}
