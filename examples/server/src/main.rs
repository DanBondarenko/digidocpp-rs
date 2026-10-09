use anyhow::Context;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use libdigidocpp::{
    Algorithm, Conf, Container, DataFile, DerCert, ExternalPartialSigner, PartialSigner,
    Pkcs12Signer, Signature, Signer, ValidatorStatus, ValidityExtension,
};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::{env, fs, io};
use tempfile::NamedTempFile;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_library()?;

    let app = Router::new()
        .route("/validate", post(validate))
        .route("/info", post(info))
        .route("/store_files", post(store_files))
        .route("/extend", post(extend))
        .route("/sign/p12", post(sign_with_p12))
        .route("/sign/external", post(sign_with_external_service));
    let port = Setting::Port.val()?.parse()?;
    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, port)).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;

    Ok(libdigidocpp::terminate()?)
}

fn init_library() -> anyhow::Result<()> {
    let mut conf = Setting::Conf
        .val_opt()
        .map_or_else(Conf::new, Conf::from_file)?;

    conf.set_tsl_url(Setting::TslUrl.val()?)?;
    conf.clear_tsl_certs();
    conf.add_tsl_cert(fs::read(Setting::TslCert.val()?)?)?;
    conf.set_ts_url(Setting::TimeStampUrl.val()?)?;

    libdigidocpp::init_conf(conf)?;
    Ok(libdigidocpp::initialize()?)
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
struct SignatureValidity {
    is_valid: bool,
    status: String,
    diagnostics: String,
    signed_by: String,
    signed_at: String,
}

impl SignatureValidity {
    fn of(signature: &Signature<'_>) -> Result<Self, AppError> {
        let result = signature.validator_result()?;
        let status = result.status().unwrap_or(ValidatorStatus::Invalid);

        Ok(Self {
            is_valid: matches!(
                status,
                ValidatorStatus::Valid | ValidatorStatus::Warning | ValidatorStatus::NonQSCD
            ),
            status: format!("{status:?}"),
            diagnostics: result.into_diagnostics(),
            signed_by: signature.signed_by()?,
            signed_at: signature.trusted_signing_time()?,
        })
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
struct ValidateResponse {
    is_valid: bool,
    signatures: Vec<SignatureValidity>,
}

#[derive(TryFromMultipart)]
struct ContainerRequest {
    file: NamedTempFile,
}

async fn validate(
    TypedMultipart(request): TypedMultipart<ContainerRequest>,
) -> Result<Json<ValidateResponse>, AppError> {
    let path = request.file.into_temp_path();
    let container = Container::open(&path)?;
    let signatures = container
        .signatures()?
        .iter()
        .map(SignatureValidity::of)
        .collect::<Result<Vec<_>, _>>()?;
    let is_valid = signatures.iter().all(|signature| signature.is_valid);

    Ok(Json(ValidateResponse {
        is_valid,
        signatures,
    }))
}

#[derive(Serialize)]
struct DataFileInfo {
    name: String,
    size_bytes: u64,
}

#[derive(Serialize)]
struct InfoResponse {
    container_type: String,
    data_files: Vec<DataFileInfo>,
    signatures: Vec<SignatureValidity>,
}

async fn info(
    TypedMultipart(request): TypedMultipart<ContainerRequest>,
) -> Result<Json<InfoResponse>, AppError> {
    let path = request.file.into_temp_path();
    let container = Container::open(&path)?;

    let data_files = container
        .data_files()?
        .iter()
        .map(|data_file| {
            Ok(DataFileInfo {
                name: data_file.file_name()?,
                size_bytes: data_file.file_size()?,
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    let signatures = container
        .signatures()?
        .iter()
        .map(SignatureValidity::of)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(InfoResponse {
        container_type: container.media_type()?.to_string(),
        data_files,
        signatures,
    }))
}

async fn store_files(
    TypedMultipart(request): TypedMultipart<ContainerRequest>,
) -> Result<(), AppError> {
    let path = request.file.into_temp_path();
    let container = Container::open(&path)?;

    fn store_file(file: &DataFile) -> Result<(), AppError> {
        // this fn body is placeholder, replace this with your use case
        let _name = file.file_name()?;
        let _media_type = file.media_type()?;
        Ok(file.write_to(io::sink())?)
    }
    for file in container.data_files()? {
        store_file(&file)?;
    }

    Ok(())
}

async fn extend(
    TypedMultipart(request): TypedMultipart<ContainerRequest>,
) -> Result<Vec<u8>, AppError> {
    let path = request.file.into_temp_path();
    let mut container = Container::open(&path)?;
    let mut signer = ExternalPartialSigner::new(Vec::new())?;

    let mut extended_container = match container.extend_container_validity(&mut signer)? {
        ValidityExtension::InPlace { .. } => container,
        ValidityExtension::Wrapped(wrapper) => wrapper,
    };
    let extended_path = NamedTempFile::new()?.into_temp_path();
    extended_container.save_as(&extended_path)?;

    Ok(fs::read(&extended_path)?)
}

#[derive(TryFromMultipart)]
struct SignRequest {
    file: FieldData<NamedTempFile>,
}

async fn sign_with_p12(
    TypedMultipart(request): TypedMultipart<SignRequest>,
) -> Result<Vec<u8>, AppError> {
    let FieldData { metadata, contents } = request.file;
    let path = NamedTempFile::with_suffix(".asice")?.into_temp_path();
    let mut container = Container::create(&path)?;

    container.add_data_file_from_reader(
        &metadata.file_name.context("no file name")?,
        &metadata.content_type.context("no media type")?,
        contents,
    )?;

    let mut signer = Pkcs12Signer::new(Setting::P12File.val()?, Setting::P12Password.val()?)?;
    container.sign(&mut signer)?;
    container.save()?;

    Ok(fs::read(&path)?)
}

async fn sign_with_external_service(
    TypedMultipart(request): TypedMultipart<SignRequest>,
) -> Result<Vec<u8>, AppError> {
    let FieldData { metadata, contents } = request.file;
    let path = NamedTempFile::with_suffix(".asice")?.into_temp_path();
    let mut container = Container::create(&path)?;
    container.add_data_file_from_reader(
        &metadata.file_name.context("no file name")?,
        &metadata.content_type.context("no media type")?,
        contents,
    )?;

    // get signing certificate from somewhere:
    // - ID-card signers: this would probably be an endpoint parameter
    // - mobile-ID: this would be requested from the mobile ID service provider
    // - organization e-seals: this would come from the environment (env-var, file, etc)
    fn mock_external_cert() -> Result<DerCert, AppError> {
        // this fn body is placeholder, replace this with your use case
        let signer = Pkcs12Signer::new(Setting::P12File.val()?, Setting::P12Password.val()?)?;
        Ok(signer.cert()?.context("no certificate in P12 file")?)
    }
    let cert = mock_external_cert()?;

    let mut signer = ExternalPartialSigner::new(cert)?;
    let mut signature = container.prepare_signature(&mut signer)?;
    let digest = signature.as_signature().data_to_sign()?;

    // ask a service to sign the bytes:
    // - organization e-seals: HSM or signing service answers at once
    // - mobile ID, ID-card signers: user signs on own device, flow splits in two requests
    fn mock_external_sign(method: Algorithm, digest: &[u8]) -> Result<Vec<u8>, AppError> {
        // this fn body is placeholder, replace this with your use case
        let signer = Pkcs12Signer::new(Setting::P12File.val()?, Setting::P12Password.val()?)?;
        Ok(signer.sign(method, digest)?)
    }
    let sig_value = mock_external_sign(signature.as_signature().signature_method()?, &digest)?;

    signature.set_signature_value(&sig_value)?;
    signature.extend_signature_profile(&mut signer)?;
    container.save()?;

    Ok(fs::read(&path)?)
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // appropriate status codes out of scope for the example
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#}", self.0)).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

#[derive(
    Debug, Hash, Copy, Clone, Eq, PartialEq, strum::AsRefStr, strum::Display, strum::EnumString,
)]
enum Setting {
    #[strum(serialize = "PORT")]
    Port,
    #[strum(serialize = "TSL_URL")]
    TslUrl,
    #[strum(serialize = "TIME_STAMP_URL")]
    TimeStampUrl,
    #[strum(serialize = "TSL_CERT")]
    TslCert,
    #[strum(serialize = "P12_FILE")]
    P12File,
    #[strum(serialize = "P12_PASSWORD")]
    P12Password,
    #[strum(serialize = "CONF")]
    Conf,
}

impl Setting {
    fn val(self) -> anyhow::Result<String> {
        env::var(self.as_ref()).or_else(|_| self.default())
    }

    fn val_opt(self) -> Option<String> {
        self.val().ok()
    }

    fn default(self) -> anyhow::Result<String> {
        let value = match self {
            Self::Port => "3000",
            Self::TslUrl => "https://open-eid.github.io/test-TL/EE_T.xml",
            Self::TimeStampUrl => "http://tsa.demo.sk.ee/tsa",
            Self::TslCert => concat!(env!("CARGO_MANIFEST_DIR"), "/demo/test-tsl.der"),
            Self::P12File => concat!(env!("CARGO_MANIFEST_DIR"), "/demo/test-signer.p12"),
            Self::P12Password => "1234",
            _ => anyhow::bail!("No default"),
        };

        Ok(String::from(value))
    }
}
