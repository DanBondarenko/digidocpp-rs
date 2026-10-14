//! Build-script helpers that locate a [libdigidocpp] installation.
//!
//! [`Install::discover`] finds the header, library and framework directories. This crate does not
//! compile or link anything itself.
//!
//! # Examples
//!
//! ```no_run
//! // build.rs
//! use libdigidocpp_build::{Install, Kind};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let install = Install::discover()?;
//!
//!     for dir in install.lib_dirs() {
//!         let dir = dir.to_str().ok_or("invalid UTF-8")?;
//!         println!("cargo:rustc-link-search=native={dir}");
//!     }
//!     for dir in install.framework_dirs() {
//!         let dir = dir.to_str().ok_or("invalid UTF-8")?;
//!         println!("cargo:rustc-link-search=framework={dir}");
//!     }
//!     match install.kind() {
//!         Kind::Framework => println!("cargo:rustc-link-lib=framework=digidocpp"),
//!         _ => println!("cargo:rustc-link-lib=dylib=digidocpp"),
//!     }
//!     install.emit_rpath()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Overview
//!
//! - [`Install::discover`] searches for libdigidocpp. Environment variables override the search.
//!   See its docs for the rules. Prints `cargo:rerun-if-env-changed` for each used variable.
//!   [`Install::emit_rpath`] prints the directories the loader needs at run time. Prints nothing if
//!   target is Windows. Downstream crates may need to call this in `build.rs`.
//! - [`Install::include_dirs`], [`Install::lib_dirs`] and [`Install::framework_dirs`] return the
//!   directories for the compiler and linker. Only the ones for the found [`Kind`] are non-empty.
//! - [`Install::runtime_dirs`] returns the directories the loader needs at run time.
//! - [`Install::kind`] returns [`Kind`]. A framework and a plain library need different link
//!   directives.
//!
//! # See also
//!
//! Looking for the bindings? See [`digidocpp`](https://docs.rs/digidocpp).
//!
//! [libdigidocpp]: https://github.com/open-eid/libdigidocpp
#![expect(clippy::print_stdout)]

use std::env;
use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};

const FRAMEWORK_NAME: &str = "digidocpp.framework";
const DEFAULT_FRAMEWORK_DIR: &str = "/Library/Frameworks";
const VARIABLES: [&str; 6] = [
    "DIGIDOCPP_FRAMEWORK_DIR",
    "DIGIDOCPP_INCLUDE_DIR",
    "DIGIDOCPP_LIB_DIR",
    "DIGIDOCPP_DIR",
    "ProgramFiles",
    "ProgramFiles(x86)",
];

/// A located libdigidocpp installation.
///
/// Get one with [`Install::discover`] from a build script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Install {
    kind: Kind,
    include_dirs: Vec<PathBuf>,
    lib_dirs: Vec<PathBuf>,
    framework_dirs: Vec<PathBuf>,
}

/// How libdigidocpp is installed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Kind {
    /// Apple framework (`digidocpp.framework`).
    Framework,
    /// Shared library with header directory.
    Library,
}

impl Install {
    /// Finds libdigidocpp installation on build machine.
    ///
    /// Standard install needs no configuration. Environment variables are optional overrides.
    ///
    /// # Lookup
    ///
    /// 1. Overrides. Used when set, skip all defaults:
    ///    - `DIGIDOCPP_INCLUDE_DIR` and `DIGIDOCPP_LIB_DIR`. Set both. First is directory that
    ///      contains `digidocpp/`.
    ///    - `DIGIDOCPP_FRAMEWORK_DIR`: directory that contains `digidocpp.framework` (Apple
    ///      targets).
    ///    - `DIGIDOCPP_DIR`: install root (Windows targets, ignored elsewhere).
    /// 2. Defaults:
    ///    - Linux: `pkg-config libdigidocpp`.
    ///    - macOS: `/Library/Frameworks/digidocpp.framework`, then `pkg-config libdigidocpp`.
    ///    - Windows: `%ProgramFiles%\libdigidocpp`. On ARM64 it is `libdigidocpp-arm64`. On x86
    ///      it is under `%ProgramFiles(x86)%`.
    ///
    ///    If `pkg-config` binary is absent (not when it just fails to find libdigidocpp), Linux
    ///    and macOS search `/usr/local` and `/usr` (Linux) or `/opt/homebrew` and `/usr/local`
    ///    (macOS). Not done when cross-compiling.
    ///
    /// Reads target from `CARGO_CFG_TARGET_*`. Prints `cargo:rerun-if-env-changed` for each
    /// variable it uses.
    ///
    /// # Errors
    /// Returns an error when no installation is found, or when overrides conflict (for example
    /// only one of include and lib directory is set).
    pub fn discover() -> Result<Self, Box<dyn Error>> {
        for name in VARIABLES {
            println!("cargo:rerun-if-env-changed={name}");
        }
        let target = Target::from_cargo_cfg(|name| env::var(name).ok())?;
        Self::resolve(Settings::from_env(), &target, probe_pkg_config)
    }

    /// Returns how libdigidocpp is installed.
    #[must_use]
    pub const fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns libdigidocpp header directories.
    ///
    /// Empty unless the kind is [`Kind::Library`].
    #[must_use]
    pub fn include_dirs(&self) -> &[PathBuf] {
        &self.include_dirs
    }

    /// Returns libdigidocpp link directories.
    ///
    /// Empty unless the kind is [`Kind::Library`].
    #[must_use]
    pub fn lib_dirs(&self) -> &[PathBuf] {
        &self.lib_dirs
    }

    /// Returns framework directories that contain `digidocpp.framework`.
    ///
    /// Empty unless the kind is [`Kind::Framework`].
    #[must_use]
    pub fn framework_dirs(&self) -> &[PathBuf] {
        &self.framework_dirs
    }

    /// Returns directories the dynamic loader needs to find the library at run time.
    #[must_use]
    pub fn runtime_dirs(&self) -> &[PathBuf] {
        match self.kind {
            Kind::Framework => &self.framework_dirs,
            Kind::Library => &self.lib_dirs,
        }
    }

    /// Emits rpath for each directory needed to find the library at run time.
    ///
    /// Prints nothing on Windows, otherwise `cargo:rustc-link-arg=-Wl,-rpath,<dir>`.
    ///
    /// Directories are paths on build machine. They are useless for binary that runs elsewhere,
    /// so skip this call when cross-compiling for such target.
    ///
    /// # Errors
    /// Returns an error upon invalid UTF-8 or missing target variables.
    pub fn emit_rpath(&self) -> Result<(), Box<dyn Error>> {
        let target = Target::from_cargo_cfg(|name| env::var(name).ok())?;
        for dir in self.rpath_dirs(&target) {
            let path = path_to_str(dir)?;
            println!("cargo:rustc-link-arg=-Wl,-rpath,{path}");
        }
        Ok(())
    }

    fn rpath_dirs(&self, target: &Target) -> Vec<&Path> {
        if target.family() == Family::Windows {
            return Vec::new();
        }
        match self.kind {
            Kind::Framework => self.framework_dirs.iter().map(PathBuf::as_path).collect(),
            Kind::Library => self
                .lib_dirs
                .iter()
                .map(PathBuf::as_path)
                .filter(|dir| !is_system_dir(dir))
                .collect(),
        }
    }

    fn resolve(
        settings: Settings,
        target: &Target,
        pkg_config: impl FnOnce() -> Result<Self, pkg_config::Error>,
    ) -> Result<Self, Box<dyn Error>> {
        let family = target.family();
        let Settings {
            include_dir,
            lib_dir,
            framework_dir,
            install_dir,
            program_files,
            program_files_x86,
            default_framework_dir,
            cross_compiling,
        } = settings;

        match (include_dir, lib_dir, framework_dir) {
            (Some(include_dir), Some(lib_dir), None) => {
                library_install(family, include_dir, lib_dir)
            }
            (None, None, Some(framework_dir)) => {
                if family == Family::Apple {
                    framework_install(framework_dir)
                } else {
                    Err("DIGIDOCPP_FRAMEWORK_DIR only supported for Apple targets".into())
                }
            }
            (None, None, None) => match family {
                Family::Unix => pkg_config_install(target, cross_compiling, pkg_config),
                Family::Windows => {
                    windows_install(target, install_dir, program_files, program_files_x86)
                }
                Family::Apple => match default_framework_dir {
                    Some(dir) if target.os == "macos" && dir.join(FRAMEWORK_NAME).is_dir() => {
                        framework_install(dir)
                    }
                    _ => pkg_config_install(target, cross_compiling, pkg_config),
                },
            },
            _ => Err("Invalid combination of vars. Set either DIGIDOCPP_INCLUDE_DIR and DIGIDOCPP_LIB_DIR or DIGIDOCPP_FRAMEWORK_DIR.".into()),
        }
    }
}

fn path_to_str(path: &Path) -> Result<&str, Box<dyn Error>> {
    path.to_str()
        .ok_or_else(|| format!("invalid UTF-8: {}", path.display()).into())
}

fn is_system_dir(dir: &Path) -> bool {
    ["/lib", "/lib64", "/usr/lib", "/usr/lib64"]
        .iter()
        .any(|system_dir| dir.starts_with(system_dir))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Apple,
    Windows,
    Unix,
}

#[derive(Debug, Clone, PartialEq)]
struct Target {
    os: String,
    arch: String,
    vendor: String,
}

impl Target {
    fn from_cargo_cfg(env: impl Fn(&str) -> Option<String>) -> Result<Self, Box<dyn Error>> {
        let required = |name: &str| env(name).ok_or_else(|| format!("{name} not set"));
        Ok(Self {
            os: required("CARGO_CFG_TARGET_OS")?,
            arch: required("CARGO_CFG_TARGET_ARCH")?,
            vendor: env("CARGO_CFG_TARGET_VENDOR").unwrap_or_default(),
        })
    }

    fn family(&self) -> Family {
        if self.vendor == "apple" {
            Family::Apple
        } else if self.os == "windows" {
            Family::Windows
        } else {
            Family::Unix
        }
    }

    fn fallback_candidates(&self) -> Vec<(PathBuf, PathBuf)> {
        let prefixes: &[&str] = match self.family() {
            Family::Apple => &["/opt/homebrew", "/usr/local"],
            Family::Unix => &["/usr/local", "/usr"],
            Family::Windows => &[],
        };
        let lib_subdirs: &[&str] = match (self.os.as_str(), self.arch.as_str()) {
            ("linux", "x86_64") => &["lib/x86_64-linux-gnu", "lib", "lib64"],
            ("linux", "aarch64") => &["lib/aarch64-linux-gnu", "lib", "lib64"],
            ("linux", _) => &["lib", "lib64"],
            _ => &["lib"],
        };
        prefixes
            .iter()
            .flat_map(|prefix| {
                lib_subdirs.iter().map(move |subdir| {
                    (
                        Path::new(prefix).join("include"),
                        Path::new(prefix).join(subdir),
                    )
                })
            })
            .collect()
    }
}

#[derive(Debug, Default)]
struct Settings {
    include_dir: Option<PathBuf>,
    lib_dir: Option<PathBuf>,
    framework_dir: Option<PathBuf>,
    install_dir: Option<PathBuf>,
    program_files: Option<PathBuf>,
    program_files_x86: Option<PathBuf>,
    default_framework_dir: Option<PathBuf>,
    cross_compiling: bool,
}

impl Settings {
    fn from_env() -> Self {
        fn path(name: &str) -> Option<PathBuf> {
            env::var_os(name).map(PathBuf::from)
        }
        Self {
            include_dir: path("DIGIDOCPP_INCLUDE_DIR"),
            lib_dir: path("DIGIDOCPP_LIB_DIR"),
            framework_dir: path("DIGIDOCPP_FRAMEWORK_DIR"),
            install_dir: path("DIGIDOCPP_DIR"),
            program_files: path("ProgramFiles"),
            program_files_x86: path("ProgramFiles(x86)"),
            default_framework_dir: Some(PathBuf::from(DEFAULT_FRAMEWORK_DIR)),
            cross_compiling: env::var_os("HOST") != env::var_os("TARGET"),
        }
    }
}

fn probe_pkg_config() -> Result<Install, pkg_config::Error> {
    let library = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libdigidocpp")?;

    Ok(Install {
        kind: Kind::Library,
        include_dirs: library.include_paths,
        lib_dirs: library.link_paths,
        framework_dirs: library.framework_paths,
    })
}

fn require_file(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.is_file() {
        Ok(())
    } else {
        let path = path_to_str(path)?;
        Err(format!("{path} not found").into())
    }
}

fn framework_install(dir: PathBuf) -> Result<Install, Box<dyn Error>> {
    let framework = dir.join(FRAMEWORK_NAME);
    if !framework.is_dir() {
        let path = path_to_str(&framework)?;
        return Err(format!("{path} not found").into());
    }
    require_file(&framework.join("Headers").join("Container.h"))?;
    require_file(&framework.join("digidocpp"))?;
    Ok(Install {
        kind: Kind::Framework,
        include_dirs: Vec::new(),
        lib_dirs: Vec::new(),
        framework_dirs: vec![dir],
    })
}

fn library_install(
    family: Family,
    include_dir: PathBuf,
    lib_dir: PathBuf,
) -> Result<Install, Box<dyn Error>> {
    require_file(&include_dir.join("digidocpp").join("Container.h"))?;

    let file_name = match family {
        Family::Windows => "digidocpp.lib",
        Family::Apple => "libdigidocpp.dylib",
        Family::Unix => "libdigidocpp.so",
    };
    require_file(&lib_dir.join(file_name))?;

    Ok(Install {
        kind: Kind::Library,
        include_dirs: vec![include_dir],
        lib_dirs: vec![lib_dir],
        framework_dirs: Vec::new(),
    })
}

fn pkg_config_install(
    target: &Target,
    cross_compiling: bool,
    pkg_config: impl FnOnce() -> Result<Install, pkg_config::Error>,
) -> Result<Install, Box<dyn Error>> {
    match pkg_config() {
        Ok(install) => Ok(install),
        Err(reason @ pkg_config::Error::Command { .. }) if cross_compiling => Err(not_found(
            target,
            format!("{reason}; fallback search skipped when cross-compiling"),
        )),
        Err(reason @ pkg_config::Error::Command { .. }) => fallback_install(target)
            .ok_or_else(|| not_found(target, format!("{reason}; fallback found nothing"))),
        Err(reason) => Err(not_found(target, reason)),
    }
}

fn fallback_install(target: &Target) -> Option<Install> {
    target
        .fallback_candidates()
        .into_iter()
        .find_map(|(include_dir, lib_dir)| {
            library_install(target.family(), include_dir, lib_dir).ok()
        })
}

fn windows_install(
    target: &Target,
    install_dir: Option<PathBuf>,
    program_files: Option<PathBuf>,
    program_files_x86: Option<PathBuf>,
) -> Result<Install, Box<dyn Error>> {
    let (program_files, folder) = match target.arch.as_str() {
        "x86" => (program_files_x86, "libdigidocpp"),
        "aarch64" => (program_files, "libdigidocpp-arm64"),
        _ => (program_files, "libdigidocpp"),
    };
    install_dir
        .or_else(|| {
            program_files
                .map(|program_files| program_files.join(folder))
                .filter(|root| root.is_dir())
        })
        .map_or_else(
            || {
                Err(not_found(
                    target,
                    "DIGIDOCPP_DIR not set, default install folder absent",
                ))
            },
            |root| library_install(Family::Windows, root.join("include"), root),
        )
}

fn not_found(target: &Target, reason: impl Display) -> Box<dyn Error> {
    let mut hints = vec!["DIGIDOCPP_INCLUDE_DIR and DIGIDOCPP_LIB_DIR"];
    match target.family() {
        Family::Apple => hints.push("DIGIDOCPP_FRAMEWORK_DIR"),
        Family::Windows => hints.push("DIGIDOCPP_DIR"),
        Family::Unix => {}
    }
    format!(
        "libdigidocpp not found for target {}: {reason}\nSet one of:\n  - {}",
        target.os,
        hints.join("\n  - ")
    )
    .into()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;

    use rstest::{fixture, rstest};
    use tempfile::TempDir;

    fn touch(path: impl AsRef<Path>) {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, "").unwrap();
    }

    impl Target {
        fn new(os: &str, arch: &str, vendor: &str) -> Self {
            Self {
                os: os.to_owned(),
                arch: arch.to_owned(),
                vendor: vendor.to_owned(),
            }
        }

        fn macos() -> Self {
            Self::new("macos", "aarch64", "apple")
        }

        fn ios() -> Self {
            Self::new("ios", "aarch64", "apple")
        }

        fn linux() -> Self {
            Self::new("linux", "x86_64", "unknown")
        }

        fn windows(arch: &str) -> Self {
            Self::new("windows", arch, "pc")
        }
    }

    impl Install {
        fn library(lib_dirs: &[&str]) -> Self {
            Self {
                kind: Kind::Library,
                include_dirs: Vec::new(),
                lib_dirs: lib_dirs.iter().map(PathBuf::from).collect(),
                framework_dirs: Vec::new(),
            }
        }

        fn framework(dir: &Path) -> Self {
            Self {
                kind: Kind::Framework,
                include_dirs: Vec::new(),
                lib_dirs: Vec::new(),
                framework_dirs: vec![dir.to_path_buf()],
            }
        }

        fn probe_failed() -> Result<Self, pkg_config::Error> {
            Err(pkg_config::Error::EnvNoPkgConfig("probe failed".to_owned()))
        }

        #[expect(clippy::unnecessary_wraps)]
        fn mock_library() -> Result<Self, pkg_config::Error> {
            Ok(Self::library(&["/usr/local/lib"]))
        }
    }

    #[fixture]
    fn temp_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    #[fixture]
    fn temp_dir_with_framework(temp_dir: TempDir) -> TempDir {
        touch(
            temp_dir
                .path()
                .join(FRAMEWORK_NAME)
                .join("Headers/Container.h"),
        );
        touch(temp_dir.path().join(FRAMEWORK_NAME).join("digidocpp"));
        temp_dir
    }

    #[fixture]
    fn temp_dir_with_header(temp_dir: TempDir) -> TempDir {
        touch(temp_dir.path().join("include/digidocpp/Container.h"));
        temp_dir
    }

    #[rstest]
    #[case::framework_linux(
        Target::linux(),
        Settings {
            framework_dir: Some("/fw".into()),
            ..Settings::default()
        },
        || panic!(),
        "only supported for Apple targets"
    )]
    #[case::lib_only(
        Target::macos(),
        Settings {
            lib_dir: Some("/lib".into()),
            ..Settings::default()
        },
        || panic!(),
        "Invalid combination of vars"
    )]
    #[case::include_only(
        Target::macos(),
        Settings {
            include_dir: Some("/inc".into()),
            ..Settings::default()
        },
        || panic!(),
        "Invalid combination of vars"
    )]
    #[case::framework_and_include(
        Target::macos(),
        Settings {
            include_dir: Some("/inc".into()),
            framework_dir: Some("/fw".into()),
            ..Settings::default()
        },
        || panic!(),
        "Invalid combination of vars"
    )]
    #[case::framework_and_lib(
        Target::macos(),
        Settings {
            lib_dir: Some("/lib".into()),
            framework_dir: Some("/fw".into()),
            ..Settings::default()
        },
        || panic!(),
        "Invalid combination of vars"
    )]
    #[case::framework_and_lib_and_include(
        Target::macos(),
        Settings {
            include_dir: Some("/inc".into()),
            lib_dir: Some("/lib".into()),
            framework_dir: Some("/fw".into()),
            ..Settings::default()
        },
        || panic!(),
        "Invalid combination of vars"
    )]
    #[case::no_pkg_config_on_windows(
        Target::windows("x86_64"),
        Settings::default(),
        || panic!(),
        "DIGIDOCPP_DIR not set"
    )]
    #[case::offers_include_and_lib(
        Target::ios(),
        Settings::default(),
        Install::probe_failed,
        "DIGIDOCPP_INCLUDE_DIR and DIGIDOCPP_LIB_DIR"
    )]
    #[case::offers_framework(
        Target::ios(),
        Settings::default(),
        Install::probe_failed,
        "DIGIDOCPP_FRAMEWORK_DIR"
    )]
    #[case::probe_failed(
        Target::ios(),
        Settings::default(),
        Install::probe_failed,
        "probe failed"
    )]
    #[case::cross_compiling(
        Target::linux(),
        Settings {
            cross_compiling: true,
            ..Settings::default()
        },
        || Err(pkg_config::Error::Command {
            command: "pkg-config".to_owned(),
            cause: std::io::Error::from(std::io::ErrorKind::NotFound),
        }),
        "cross-compiling"
    )]
    fn rejects_resolution(
        #[case] target: Target,
        #[case] settings: Settings,
        #[case] pkg_config: fn() -> Result<Install, pkg_config::Error>,
        #[case] expected_text: &str,
    ) {
        let result = Install::resolve(settings, &target, pkg_config);
        let text = result.unwrap_err().to_string();
        assert!(text.contains(expected_text), "{text}");
    }

    #[rstest]
    #[case::linux(Target::linux())]
    #[case::ios(Target::ios())]
    fn uses_pkg_config(#[case] target: Target) -> Result<(), Box<dyn Error>> {
        let install = Install::resolve(Settings::default(), &target, Install::mock_library)?;
        assert_eq!(install, Install::library(&["/usr/local/lib"]));
        Ok(())
    }

    #[test]
    fn skips_fallback_when_pkg_config_runs_and_fails() {
        let result = Install::resolve(Settings::default(), &Target::linux(), Install::probe_failed);
        let text = result.unwrap_err().to_string();
        assert!(!text.contains("fallback"), "{text}");
    }

    #[rstest]
    #[case::linux_x86_64(Target::linux(), &[
        ("/usr/local/include", "/usr/local/lib/x86_64-linux-gnu"),
        ("/usr/local/include", "/usr/local/lib"),
        ("/usr/local/include", "/usr/local/lib64"),
        ("/usr/include", "/usr/lib/x86_64-linux-gnu"),
        ("/usr/include", "/usr/lib"),
        ("/usr/include", "/usr/lib64"),
    ])]
    #[case::linux_other_arch(Target::new("linux", "riscv64", "unknown"), &[
        ("/usr/local/include", "/usr/local/lib"),
        ("/usr/local/include", "/usr/local/lib64"),
        ("/usr/include", "/usr/lib"),
        ("/usr/include", "/usr/lib64"),
    ])]
    #[case::macos(Target::macos(), &[
        ("/opt/homebrew/include", "/opt/homebrew/lib"),
        ("/usr/local/include", "/usr/local/lib"),
    ])]
    fn lists_fallback_candidates(#[case] target: Target, #[case] expected: &[(&str, &str)]) {
        let expected: Vec<_> = expected
            .iter()
            .map(|(include_dir, lib_dir)| (PathBuf::from(include_dir), PathBuf::from(lib_dir)))
            .collect();
        assert_eq!(target.fallback_candidates(), expected);
    }

    #[rstest]
    #[case::default_framework(Target::macos(), |dir| Settings {
        default_framework_dir: Some(dir),
        ..Settings::default()
    })]
    #[case::ios_framework(Target::ios(), |dir| Settings {
        framework_dir: Some(dir),
        ..Settings::default()
    })]
    fn finds_framework(
        temp_dir_with_framework: TempDir,
        #[case] target: Target,
        #[case] settings: fn(PathBuf) -> Settings,
    ) -> Result<(), Box<dyn Error>> {
        let settings = settings(temp_dir_with_framework.path().to_path_buf());

        let install = Install::resolve(settings, &target, Install::probe_failed)?;
        assert_eq!(install, Install::framework(temp_dir_with_framework.path()));
        Ok(())
    }

    #[rstest]
    #[case::empty_bundle(&[], "Container.h not found")]
    #[case::missing_header(&["digidocpp"], "Container.h not found")]
    #[case::missing_binary(&["Headers/Container.h"], "digidocpp not found")]
    fn rejects_invalid_framework(
        temp_dir: TempDir,
        #[case] present_files: &[&str],
        #[case] expected_text: &str,
    ) -> Result<(), Box<dyn Error>> {
        fs::create_dir(temp_dir.path().join(FRAMEWORK_NAME))?;
        for relative in present_files {
            touch(temp_dir.path().join(FRAMEWORK_NAME).join(relative));
        }
        let settings = Settings {
            framework_dir: Some(temp_dir.path().to_path_buf()),
            ..Settings::default()
        };

        let result = Install::resolve(settings, &Target::ios(), Install::probe_failed);
        let text = result.unwrap_err().to_string();
        assert!(text.contains(expected_text), "{text}");
        Ok(())
    }

    #[rstest]
    fn var_overrides_default_framework(
        temp_dir_with_header: TempDir,
    ) -> Result<(), Box<dyn Error>> {
        let root = temp_dir_with_header.path();
        touch(root.join("lib").join("libdigidocpp.dylib"));
        touch(
            root.join("default")
                .join(FRAMEWORK_NAME)
                .join("Headers/Container.h"),
        );
        touch(root.join("default").join(FRAMEWORK_NAME).join("digidocpp"));
        let settings = Settings {
            include_dir: Some(root.join("include")),
            lib_dir: Some(root.join("lib")),
            default_framework_dir: Some(root.join("default")),
            ..Settings::default()
        };

        let install = Install::resolve(settings, &Target::macos(), Install::probe_failed)?;
        assert_eq!(
            (install.kind(), install.lib_dirs()),
            (Kind::Library, &[root.join("lib")][..]),
        );
        Ok(())
    }

    #[rstest]
    #[case::empty_install(&[], "Container.h not found")]
    #[case::missing_header(&["lib/libdigidocpp.so"], "Container.h not found")]
    #[case::missing_library(&["include/digidocpp/Container.h"], "libdigidocpp.so not found")]
    fn rejects_invalid_library(
        temp_dir: TempDir,
        #[case] present_files: &[&str],
        #[case] expected_text: &str,
    ) {
        for relative in present_files {
            touch(temp_dir.path().join(relative));
        }
        let settings = Settings {
            include_dir: Some(temp_dir.path().join("include")),
            lib_dir: Some(temp_dir.path().join("lib")),
            ..Settings::default()
        };

        let result = Install::resolve(settings, &Target::linux(), Install::probe_failed);
        let text = result.unwrap_err().to_string();
        assert!(text.contains(expected_text), "{text}");
    }

    #[rstest]
    #[case::x86_64("x86_64", "pf/libdigidocpp")]
    #[case::aarch64("aarch64", "pf/libdigidocpp-arm64")]
    #[case::x86("x86", "pf86/libdigidocpp")]
    fn architecture_determines_default_windows(
        temp_dir: TempDir,
        #[case] arch: &str,
        #[case] folder: &str,
    ) -> Result<(), Box<dyn Error>> {
        let root = temp_dir.path();
        touch(root.join(folder).join("include/digidocpp/Container.h"));
        touch(root.join(folder).join("digidocpp.lib"));
        let settings = Settings {
            program_files: Some(root.join("pf")),
            program_files_x86: Some(root.join("pf86")),
            ..Settings::default()
        };

        let install = Install::resolve(settings, &Target::windows(arch), Install::probe_failed)?;
        assert_eq!(install.lib_dirs(), &[root.join(folder)][..],);
        Ok(())
    }

    #[rstest]
    #[case::windows(Install::library(&["/opt/lib"]), Target::windows("x86_64"), &[])]
    #[case::linux(
        Install::library(&[
            "/usr/lib/x86_64-linux-gnu",
            "/usr/lib64",
            "/usr/local/lib",
            "/opt/lib",
        ]),
        Target::linux(),
        &["/usr/local/lib", "/opt/lib"]
    )]
    #[case::apple(
        Install::framework(Path::new("/Library/Frameworks")),
        Target::macos(),
        &["/Library/Frameworks"]
    )]
    fn target_determines_rpath(
        #[case] install: Install,
        #[case] target: Target,
        #[case] expected: &[&str],
    ) {
        let expected: Vec<_> = expected.iter().map(Path::new).collect();
        assert_eq!(install.rpath_dirs(&target), expected);
    }
}
