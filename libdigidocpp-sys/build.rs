use std::env;

fn main() {
    let mut build = cxx_build::bridges([
        "src/init.rs",
        "src/conf.rs",
        "src/exception.rs",
        "src/container.rs",
        "src/data_file.rs",
        "src/signature.rs",
    ]);
    build
        .std("c++20")
        .file("src/init_ffi.cpp")
        .file("src/conf_ffi.cpp")
        .file("src/exception_ffi.cpp")
        .file("src/container_ffi.cpp")
        .file("src/data_file_ffi.cpp")
        .file("src/signature_ffi.cpp");

    let test_util_feature = env::var_os("CARGO_FEATURE_TEST_UTIL").is_some();

    if test_util_feature {
        build.file("src/test_util.cpp");
    }

    println!("cargo:rerun-if-env-changed=DIGIDOCPP_FRAMEWORK_DIR");

    #[expect(
        clippy::expect_used,
        reason = "Specifying ligdigidocpp location is required for dynamic linking"
    )]
    let framework_dir = env::var("DIGIDOCPP_FRAMEWORK_DIR")
        .expect("set DIGIDOCPP_FRAMEWORK_DIR to the folder holding digidocpp.framework");

    build.flag(format!("-F{framework_dir}"));
    build.compile("libdigidocpp-sys");

    println!("cargo:rustc-link-search=framework={framework_dir}");
    println!("cargo:rustc-link-lib=framework=digidocpp");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{framework_dir}");

    println!("cargo:rerun-if-changed=include/ffi_util.h");
    println!("cargo:rerun-if-changed=src/init_ffi.cpp");
    println!("cargo:rerun-if-changed=include/init_ffi.h");
    println!("cargo:rerun-if-changed=src/conf_ffi.cpp");
    println!("cargo:rerun-if-changed=include/conf_ffi.h");
    println!("cargo:rerun-if-changed=src/exception_ffi.cpp");
    println!("cargo:rerun-if-changed=include/exception_ffi.h");
    println!("cargo:rerun-if-changed=src/container_ffi.cpp");
    println!("cargo:rerun-if-changed=include/container_ffi.h");
    println!("cargo:rerun-if-changed=src/data_file_ffi.cpp");
    println!("cargo:rerun-if-changed=include/data_file_ffi.h");
    println!("cargo:rerun-if-changed=src/signature_ffi.cpp");
    println!("cargo:rerun-if-changed=include/signature_ffi.h");

    if test_util_feature {
        println!("cargo:rerun-if-changed=src/test_util.cpp");
        println!("cargo:rerun-if-changed=include/test_util.h");
    }
}
