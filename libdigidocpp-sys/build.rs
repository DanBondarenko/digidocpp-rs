use std::env;

fn main() {
    let mut build = cxx_build::bridge("src/lib.rs");
    build.std("c++20");

    println!("cargo:rerun-if-env-changed=DIGIDOCPP_FRAMEWORK_DIR");

    let framework_dir = env::var("DIGIDOCPP_FRAMEWORK_DIR")
        .expect("set DIGIDOCPP_FRAMEWORK_DIR to the folder holding digidocpp.framework");

    build.flag(&format!("-F{framework_dir}"));
    build.compile("libdigidocpp-sys");

    println!("cargo:rustc-link-search=framework={framework_dir}");
    println!("cargo:rustc-link-lib=framework=digidocpp");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{framework_dir}");
}
