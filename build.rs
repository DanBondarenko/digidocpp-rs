use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=DIGIDOCPP_FRAMEWORK_DIR");

    #[expect(
        clippy::expect_used,
        reason = "Specifying ligdigidocpp location is required for dynamic linking"
    )]
    let framework_dir = env::var("DIGIDOCPP_FRAMEWORK_DIR")
        .expect("set DIGIDOCPP_FRAMEWORK_DIR to the folder holding digidocpp.framework");

    println!("cargo:rustc-link-arg=-Wl,-rpath,{framework_dir}");
}
