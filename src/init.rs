use crate::Error;
use libdigidocpp_sys::init::ffi;

pub fn initialize_default() -> Result<(), Error> {
    Ok(ffi::initialize_default()?)
}

pub fn initialize_with_user_agent(app_info: &str, user_agent: &str) -> Result<(), Error> {
    Ok(ffi::initialize_with_user_agent(app_info, user_agent)?)
}

pub fn terminate() -> Result<(), Error> {
    Ok(ffi::terminate()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serial_test::serial;
    use std::error::Error;

    #[test]
    #[serial]
    fn gets_library_info() -> Result<(), Box<dyn Error>> {
        initialize_default()?;

        let version = ffi::version()?;
        let app_info = ffi::app_info()?;
        let user_agent = ffi::user_agent()?;

        assert!(!version.is_empty());
        assert!(!app_info.is_empty());
        assert!(!user_agent.is_empty());

        Ok(())
    }

    #[test]
    #[serial]
    fn sets_and_gets_library_info() -> Result<(), Box<dyn Error>> {
        let expected_app_info = "expected app info";
        let expected_user_agent = "expected user agent";

        initialize_with_user_agent(expected_app_info, expected_user_agent)?;

        let app_info = ffi::app_info()?;
        let user_agent = ffi::user_agent()?;

        assert_eq!(app_info, expected_app_info);
        assert_eq!(user_agent, expected_user_agent);

        Ok(())
    }
}
