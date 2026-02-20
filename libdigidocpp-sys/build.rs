use std::env;

fn main() {
    let mut build = cxx_build::bridges([
        "src/init.rs",
        "src/exception.rs",
    ]);
    build
        .std("c++20")
        .file("src/init_ffi.cpp")
        .file("src/exception_ffi.cpp");

    println!("cargo:rerun-if-env-changed=DIGIDOCPP_FRAMEWORK_DIR");

    let framework_dir = env::var("DIGIDOCPP_FRAMEWORK_DIR")
        .expect("set DIGIDOCPP_FRAMEWORK_DIR to the folder holding digidocpp.framework");

    build.flag(&format!("-F{framework_dir}"));
    build.compile("libdigidocpp-sys");

    println!("cargo:rustc-link-search=framework={framework_dir}");
    println!("cargo:rustc-link-lib=framework=digidocpp");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{framework_dir}");

    println!("cargo:rerun-if-changed=src/exception_ffi.cpp");
    println!("cargo:rerun-if-changed=include/exception_ffi.h");
    println!("cargo:rerun-if-changed=src/init_ffi.cpp");
    println!("cargo:rerun-if-changed=include/init_ffi.h");
}
