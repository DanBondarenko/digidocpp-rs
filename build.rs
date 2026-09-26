use std::error::Error;

use libdigidocpp_build::Install;

fn main() -> Result<(), Box<dyn Error>> {
    Install::discover()?.emit_rpath()?;
    Ok(())
}
