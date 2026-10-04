fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Emit rpath in case `libdigidocpp` library is installed to a custom location
    libdigidocpp_build::Install::discover()?.emit_rpath()
}
