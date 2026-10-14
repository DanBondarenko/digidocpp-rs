//! Build script: emits the run-time search path of libdigidocpp.

use std::env;
use std::error::Error;

use libdigidocpp_build::Install;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if env::var_os("DOCS_RS").is_some() {
        return Ok(());
    }

    // Emit rpath in case `libdigidocpp` library is installed to a custom location
    Install::discover()?.emit_rpath()?;
    Ok(())
}
