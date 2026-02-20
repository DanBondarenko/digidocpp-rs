#[cxx::bridge]
pub mod ffi {
    #[namespace = "digidoc"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
        include!("digidocpp/Container.h");

        fn terminate() -> Result<()>;
    }

    #[namespace = "digidoc_rs::init"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/init_ffi.h");
        include!("libdigidocpp-sys/include/exception_ffi.h");

        fn initialize_default() -> Result<()>;
        fn initialize(app_info: &str) -> Result<()>;
        fn initialize_with_user_agent(app_info: &str, user_agent: &str) -> Result<()>;
        fn version() -> Result<String>;
        fn app_info() -> Result<String>;
        fn user_agent() -> Result<String>;
    }
}