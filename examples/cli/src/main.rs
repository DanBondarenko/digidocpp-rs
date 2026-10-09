use anyhow::{Context, Result, anyhow, ensure};
use bytesize::ByteSize;
use clap::{Args, Parser, Subcommand};
#[cfg(not(windows))]
use libdigidocpp::Pkcs11Signer;
#[cfg(windows)]
use libdigidocpp::WinSigner;
use libdigidocpp::{
    Conf, Container, ContainerType, ExceptionCode, ExternalPartialSigner, PartialSigner, Signature,
    ValidatorResult, ValidatorStatus, ValidityExtension,
};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

#[derive(Parser)]
#[command(about = "Sign files, validate/extend digital signatures")]
struct Cli {
    #[command(flatten)]
    library: LibraryOptions,
    #[command(subcommand)]
    command: Command,
}

#[derive(Args)]
struct LibraryOptions {
    /// libdigidocpp config file, by default searches system directories
    #[arg(long, env = "CONF")]
    conf: Option<PathBuf>,
    /// use library production defaults instead of SK test environment
    #[arg(long, conflicts_with_all = ["tsl_url", "tsl_cert", "time_stamp_url"])]
    production: bool,
    /// TSL url or file
    #[arg(
        long,
        env = "TSL_URL",
        default_value = "https://open-eid.github.io/test-TL/EE_T.xml"
    )]
    tsl_url: String,
    /// DER certificate that signed TSL
    #[arg(
        long,
        env = "TSL_CERT",
        default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/demo/test-tsl.der")
    )]
    tsl_cert: PathBuf,
    /// timestamp service
    #[arg(
        long,
        global = true,
        env = "TIME_STAMP_URL",
        default_value = "http://tsa.demo.sk.ee/tsa"
    )]
    time_stamp_url: String,
}

#[derive(Subcommand)]
enum Command {
    /// validate signatures of container
    Validate { file: PathBuf },
    /// extend container validity by adding timestamps to signatures
    Extend {
        file: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// sign using ID-card
    Sign {
        file: PathBuf,
        #[arg(long)]
        output: PathBuf,
        /// PKCS#11 driver of ID-card
        #[arg(long)]
        pkcs11_driver: Option<PathBuf>,
    },
    /// show information about container
    Info { file: PathBuf },
    /// extract container's data files
    Extract {
        file: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    configure_library(&cli.library)?;
    libdigidocpp::initialize()?;
    let result = run(&cli.command);
    libdigidocpp::terminate()?;
    result
}

fn configure_library(options: &LibraryOptions) -> Result<()> {
    let mut conf = options
        .conf
        .as_deref()
        .map_or_else(Conf::new, Conf::from_file)?;

    if !options.production {
        conf.set_tsl_url(&options.tsl_url)?;
        conf.clear_tsl_certs();
        conf.add_tsl_cert(fs::read(&options.tsl_cert)?)?;
        conf.set_ts_url(&options.time_stamp_url)?;
    }

    Ok(libdigidocpp::init_conf(conf)?)
}

fn run(command: &Command) -> Result<ExitCode> {
    match command {
        Command::Validate { file } => validate(file),
        Command::Extend { file, output } => {
            extend(file, output)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Sign {
            file,
            output,
            pkcs11_driver,
        } => {
            sign(file, output, pkcs11_driver.as_deref())?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Info { file } => {
            info(file)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Extract { file, output } => {
            extract(file, output)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn validate(container_path: &Path) -> Result<ExitCode> {
    let container = Container::open(container_path)?;
    let signatures = container.signatures()?;
    let results = signatures
        .iter()
        .map(Signature::validator_result)
        .collect::<Result<Vec<_>, _>>()?;

    for (signature, result) in signatures.iter().zip(&results) {
        let id = signature.id()?;
        let signed_by = signature.signed_by()?;
        let signed_at = signature.trusted_signing_time()?;
        let diagnostics = result.diagnostics().trim_end();
        let validity = match result.status() {
            Some(ValidatorStatus::Valid) => "signature is valid",
            Some(ValidatorStatus::Warning) => "signature is valid, but has warnings",
            Some(ValidatorStatus::NonQSCD) => "signature is valid, but not qualified",
            _ => "signature is not valid",
        };

        println!("Signature {id}");
        println!("  validity:    {validity}");
        println!("  signed by:   {signed_by}");
        println!("  signed at:   {signed_at}");
        if !diagnostics.is_empty() {
            println!("  diagnostics: {diagnostics}");
        }
    }

    if results
        .iter()
        .all(|result| result.status() == Some(ValidatorStatus::Valid))
    {
        println!("All signatures are valid.");
        Ok(ExitCode::SUCCESS)
    } else if results.iter().all(is_considered_valid) {
        println!("All signatures are valid, but some have warnings or are not qualified.");
        Ok(ExitCode::SUCCESS)
    } else {
        println!("Invalid signatures detected.");
        Ok(ExitCode::FAILURE)
    }
}

fn extend(path: &Path, output: &Path) -> Result<()> {
    let mut container = Container::open(path)?;
    let mut signer = ExternalPartialSigner::new(Vec::new())?;

    let (mut container, message) = match container.extend_container_validity(&mut signer)? {
        ValidityExtension::InPlace {
            extended_count: count,
        } => {
            let message = format!("Extended {count} signature(s), saved {}", output.display());
            (container, message)
        }
        ValidityExtension::Wrapped(container) => {
            let message = format!("Wrapped {} into {}", path.display(), output.display());
            (container, message)
        }
    };

    ensure!(
        is_extension_valid(output, container.media_type()?),
        "invalid extension"
    );
    container.save_as(output)?;

    println!("{message}");
    Ok(())
}

#[cfg(windows)]
fn signer(_: Option<&Path>) -> Result<WinSigner> {
    let mut signer = WinSigner::new()?;
    signer.set_select_first(true)?;
    Ok(signer)
}

#[cfg(not(windows))]
fn signer(pkcs11_driver: Option<&Path>) -> Result<Pkcs11Signer> {
    let signer = pkcs11_driver.map_or_else(Pkcs11Signer::new, Pkcs11Signer::with_driver)?;
    Ok(signer)
}

fn sign(file_path: &Path, output: &Path, pkcs11_driver: Option<&Path>) -> Result<()> {
    ensure!(
        is_extension_valid(output, ContainerType::AsicE),
        "invalid extension"
    );
    let mut container = Container::create(output)?;

    let mime = mime_guess::from_path(file_path).first_or_octet_stream();
    container.add_data_file(file_path, mime.essence_str())?;

    let mut signer = signer(pkcs11_driver)?;
    let certificate = signer.cert()?.context("no cert")?;
    let (_, parsed) = x509_parser::parse_x509_certificate(certificate.as_ref())?;
    println!("Signing as {}", parsed.subject());

    let pin = match env::var("PIN2") {
        Ok(pin) => pin,
        Err(_) => rpassword::prompt_password("PIN2: ")?,
    };
    ensure!(is_valid_pin2(&pin), "invalid PIN (5-12 digits)");

    signer.set_pin(pin)?;
    container.sign(&mut signer).map_err(|error| {
        if error.has_code(ExceptionCode::PINIncorrect) {
            anyhow!("PIN rejected by card")
        } else {
            error.into()
        }
    })?;

    container.save()?;
    println!("Saved {}", output.display());

    Ok(())
}

fn info(container_path: &Path) -> Result<()> {
    let container = Container::open(container_path)?;

    println!("type: {:?}", container.media_type()?);

    for data_file in container.data_files()? {
        let name = data_file.file_name()?;
        let size = ByteSize::b(data_file.file_size()?).display().iec_short();

        println!("file: {name} ({size})");
    }

    for signature in container.signatures()? {
        let signed_by = signature.signed_by()?;
        let signed_at = signature.trusted_signing_time()?;
        let validity = match signature.validator_result()?.status() {
            Some(ValidatorStatus::Valid) => "valid",
            Some(ValidatorStatus::Warning) => "valid with warnings",
            Some(ValidatorStatus::NonQSCD) => "valid, but unqualified",
            _ => "invalid",
        };

        println!("signature: {signed_by} at {signed_at} ({validity})");
    }

    Ok(())
}

fn is_considered_valid(result: &ValidatorResult) -> bool {
    matches!(
        result.status(),
        Some(ValidatorStatus::Valid | ValidatorStatus::Warning | ValidatorStatus::NonQSCD)
    )
}

fn extract(container_path: &Path, output: &Path) -> Result<()> {
    let container = Container::open(container_path)?;
    fs::create_dir(output)?;

    for data_file in container.data_files()? {
        let file_path = output.join(data_file.file_name()?);
        data_file.save_as(&file_path)?;

        println!("Extracted {}", file_path.display());
    }

    Ok(())
}

fn is_valid_pin2(pin: &str) -> bool {
    (5..=12).contains(&pin.len()) && pin.chars().all(|character| character.is_ascii_digit())
}

fn is_extension_valid(path: &Path, container_type: ContainerType) -> bool {
    let allowed = match container_type {
        ContainerType::AsicE => &["asice", "sce", "bdoc"],
        ContainerType::AsicS => &["asics", "scs"][..],
        _ => return false,
    };

    path.extension()
        .and_then(OsStr::to_str)
        .map(str::to_lowercase)
        .is_some_and(|extension| allowed.contains(&extension.as_str()))
}
