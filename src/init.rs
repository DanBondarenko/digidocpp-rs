use libdigidocpp_sys::exception::FfiException;
use libdigidocpp_sys::init::ffi;

pub fn initialize_default() -> Result<(), FfiException> {
    crate::fallible(ffi::initialize_default)
}

pub fn initialize_with_user_agent(app_info: &str, user_agent: &str) -> Result<(), FfiException> {
    crate::fallible(|| ffi::initialize_with_user_agent(app_info, user_agent))
}

pub fn terminate() -> Result<(), FfiException> {
    crate::fallible(ffi::terminate)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;
    use serial_test::serial;

    #[test]
    #[serial]
    fn gets_library_info() -> Result<(), Box<dyn Error>> {
        initialize_default()?;

        let version = dbg!(ffi::version()?);
        let app_info = dbg!(ffi::app_info()?);
        let user_agent = dbg!(ffi::user_agent()?);

        assert!(version.len() > 0);
        assert!(app_info.len() > 0);
        assert!(user_agent.len() > 0);

        Ok(())
    }

    #[test]
    #[serial]
    fn sets_and_gets_library_info() -> Result<(), Box<dyn Error>> {
        let expected_app_info = "expected app info";
        let expected_user_agent = "expected user agent";

        initialize_with_user_agent(expected_app_info, expected_user_agent)?;

        let app_info = dbg!(ffi::app_info()?);
        let user_agent = dbg!(ffi::user_agent()?);

        assert_eq!(app_info, expected_app_info);
        assert_eq!(user_agent, expected_user_agent);

        Ok(())
    }
}