use std::env;
use std::error::Error;
use std::fmt::Display;

use libdigidocpp_build::{Install, Kind, path_to_str};

const BRIDGES: [&str; 8] = [
    "src/init.rs",
    "src/conf.rs",
    "src/exception.rs",
    "src/container.rs",
    "src/data_file.rs",
    "src/signature.rs",
    "src/signer.rs",
    "src/stream.rs",
];

const SOURCES: [&str; 8] = [
    "src/init_ffi.cpp",
    "src/conf_ffi.cpp",
    "src/exception_ffi.cpp",
    "src/container_ffi.cpp",
    "src/data_file_ffi.cpp",
    "src/signature_ffi.cpp",
    "src/signer_ffi.cpp",
    "src/stream_ffi.cpp",
];

fn main() -> Result<(), Box<dyn Error>> {
    let install = Install::discover()?;

    compile_bridge(&install)?;
    emit_link_directives(&install)?;
    install.emit_rpath()?;
    emit_rerun_directives();

    Ok(())
}

fn compile_bridge(install: &Install) -> Result<(), Box<dyn Error>> {
    let mut build = cxx_build::bridges(BRIDGES);
    build
        .std("c++20")
        .files(SOURCES)
        .includes(install.include_dirs());

    if env::var_os("CARGO_FEATURE_TEST_UTIL").is_some() {
        build.file("src/test_util.cpp");
    }

    for framework_dir in install.framework_dirs() {
        build.flag(format!("-F{}", path_to_str(framework_dir)?));
    }

    if is_windows_msvc_target() {
        // libdigidocpp checks WIN32
        build.define("WIN32", None).flag("/EHsc");
    }

    build.compile("libdigidocpp-sys");
    Ok(())
}

fn is_windows_msvc_target() -> bool {
    env::var("CARGO_CFG_TARGET_OS").is_ok_and(|target_os| target_os == "windows")
        && env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|target_env| target_env == "msvc")
}

fn emit_link_directives(install: &Install) -> Result<(), Box<dyn Error>> {
    for framework_dir in install.framework_dirs() {
        emit(format_args!(
            "cargo:rustc-link-search=framework={}",
            path_to_str(framework_dir)?
        ));
    }
    for lib_dir in install.lib_dirs() {
        emit(format_args!(
            "cargo:rustc-link-search=native={}",
            path_to_str(lib_dir)?
        ));
    }

    match install.kind() {
        Kind::Framework => emit("cargo:rustc-link-lib=framework=digidocpp"),
        _ => emit("cargo:rustc-link-lib=dylib=digidocpp"),
    }
    Ok(())
}

fn emit_rerun_directives() {
    emit("cargo:rerun-if-changed=src");
    emit("cargo:rerun-if-changed=include");
}

fn emit(directive: impl Display) {
    println!("{directive}");
}
