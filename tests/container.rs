#![allow(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
        clippy::indexing_slicing,
        clippy::used_underscore_binding,
    )
)]
mod common;

use crate::common::{
    MEDIA_TYPE_TEXT_PLAIN, TestEnv, assert_general_digidoc_err, assert_internal_err,
    conf_for_testing, create_unsigned_asice_with_data_file, dir_with_fixtures, init_with_valid_tsl,
};
use digidocpp::algorithm::{SignatureAlgorithm, SignatureProfile};
use digidocpp::container::ContainerType;
use digidocpp::{Container, Error, Initialized, Library, Uninitialized};
use httpmock::{Mock, MockServer, Then};
use rstest::{fixture, rstest};
use serde_json::json;
use std::error::Error as StdError;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[rstest]
#[case::asice_with_expired_certificates("test.asice", ContainerType::AsicE)]
#[case::asics("test.asics", ContainerType::AsicS)]
#[case::asics_with_metadata("test-meta.asics", ContainerType::AsicS)]
#[case::asics_with_invalid_timestamp_token("test-invalidts.asics", ContainerType::AsicS)]
#[case::asics_with_zero_size_contents("test-tera-empty.asics", ContainerType::AsicS)]
#[case::asics_with_large_contents("test-tera.asics", ContainerType::AsicS)]
#[case::asice_with_subfolder_contents("asice-path.asice", ContainerType::AsicE)]
#[case::asice_with_forged_signature("forged_lt.asice", ContainerType::AsicE)]
#[case::asice_for_tsl_validation("tsl.asice", ContainerType::AsicE)]
fn opens_containers(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
    #[case] fixture: &str,
    #[case] container_type: ContainerType,
) -> Result<(), Box<dyn StdError>> {
    let container = Container::open(&library, common::fixture_path(fixture))?;

    assert_eq!(container.media_type()?, container_type);
    Ok(())
}

#[rstest]
#[case::invalid_mimetype("test-invalid.asics")]
#[case::subfolder("asics-subfolder.asics")]
fn fails_opening_invalid_containers(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
    #[case] name: &str,
) {
    let result = Container::open(&library, common::fixture_path(name));

    assert_general_digidoc_err!(result);
}

#[rstest]
fn creates_empty_asice_container(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let container = Container::create(&library, "new.asice")?;

    assert_eq!(container.media_type()?, ContainerType::AsicE);
    Ok(())
}

#[rstest]
#[case::open(
    |library: &Library<Initialized>, path: &Path| Container::open(library, path).map(|_| ())
)]
#[case::create(
    |library: &Library<Initialized>, path: &Path| Container::create(library, path).map(|_| ())
)]
#[case::add_data_file(|library: &Library<Initialized>, path: &Path| {
    Container::create(library, "new.asice")?.add_data_file(path, MEDIA_TYPE_TEXT_PLAIN)
})]
#[case::save_as(
    |library: &Library<Initialized>, path: &Path| Container::create(library, "new.asice")?.save_as(path)
)]
fn rejects_non_utf8_path(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
    #[case] operation: fn(&Library<Initialized>, &Path) -> Result<(), Error>,
) {
    let result = operation(&library, &common::non_utf8_path());

    assert!(assert_internal_err!(result).contains("UTF-8"));
}

#[rstest]
fn rejects_empty_save_path(
    #[from(init_with_valid_tsl)] TestEnv { library, .. }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut container = Container::create(&library, "new.asice")?;

    let result = container.save_as("");

    assert_eq!(assert_internal_err!(result), "path is empty");
    Ok(())
}

#[rstest]
fn saves_container_to_given_path(
    #[from(init_with_valid_tsl)] TestEnv { library, directory }: TestEnv,
) -> Result<(), Box<dyn StdError>> {
    let mut container =
        create_unsigned_asice_with_data_file(&library, directory.path(), "default.asice");
    let new_path = directory.path().join("copy.asice");

    container.save_as(&new_path)?;

    let saved = Container::open(&library, new_path)?;
    assert_eq!(saved.data_files()?[0].file_name()?, "test1.txt");
    Ok(())
}

struct ValidationServiceEnv {
    server: MockServer,
    library: Library<Initialized>,
    _temp_dir: TempDir,
    ddoc: PathBuf,
}

impl ValidationServiceEnv {
    fn with_mock(
        &self,
        response: impl FnOnce(Then) -> Then,
        callback: impl FnOnce(&Library<Initialized>, &Path),
    ) -> Mock<'_> {
        let mock = self.server.mock(|when, then| {
            when.any_request();
            response(then);
        });

        callback(&self.library, &self.ddoc);

        mock
    }

    fn with_invalid_response(
        &self,
        callback: impl FnOnce(&Library<Initialized>, &Path),
    ) -> Mock<'_> {
        self.with_mock(|then| then.status(500), callback)
    }

    fn with_valid_response(&self, callback: impl FnOnce(&Library<Initialized>, &Path)) -> Mock<'_> {
        let valid_validation_response = json!({
            "validationReport": {
                "validationConclusion": {
                    "signatures": [{
                        "id": "S0",
                        "claimedSigningTime": "2020-01-01T00:00:00Z",
                        "signatureFormat": "DIGIDOC_XML_1.3",
                        "indication": "TOTAL-PASSED",
                        "signedBy": "SIGNER,TEST",
                        "signatureMethod": SignatureAlgorithm::RsaSha256.as_ref()
                    }]
                }
            }
        });

        self.with_mock(
            |then| then.status(200).json_body(valid_validation_response),
            callback,
        )
    }
}

fn open_container_counting_calls<'lib>(
    library: &'lib Library<Initialized>,
    ddoc: &Path,
    consent_answer: Option<bool>,
) -> (Result<Container<'lib>, Error>, usize) {
    let mut consent_calls = 0_usize;

    let result = consent_answer.map_or_else(
        || Container::open(library, ddoc),
        |answer| {
            Container::open_requiring_online_validation_consent(library, ddoc, || {
                consent_calls += 1;
                answer
            })
        },
    );

    (result, consent_calls)
}

#[fixture]
fn validation_service_env() -> ValidationServiceEnv {
    let server = MockServer::start();

    let temp_dir = dir_with_fixtures(&["TSL.xml", "EE_T.xml", "test.ddoc"]);
    let library = Library::<Uninitialized>::acquire().unwrap();
    let mut conf = conf_for_testing(&library, temp_dir.path(), "TSL.xml");
    conf.set_verify_service_uri(server.url("/validate"))
        .unwrap();
    let library = common::initialize_with(library, conf);
    let ddoc = temp_dir.path().join("test.ddoc");

    ValidationServiceEnv {
        server,
        library,
        _temp_dir: temp_dir,
        ddoc,
    }
}

#[rstest]
#[case::declined(Some(false), 1, 0, "Online validation disabled")]
#[case::granted(Some(true), 1, 1, "Failed to send request to SiVa")]
#[case::omitted(None, 0, 1, "Failed to send request to SiVa")]
fn request_is_sent_unless_consent_declined(
    validation_service_env: ValidationServiceEnv,
    #[case] consent_answer: Option<bool>,
    #[case] expected_consent_calls: usize,
    #[case] expected_requests_count: usize,
    #[case] expected_error_message: &str,
) {
    let mock = validation_service_env.with_invalid_response(|library, path| {
        let (result, consent_calls) = open_container_counting_calls(library, path, consent_answer);

        let error = assert_general_digidoc_err!(result);
        assert_eq!(error.message(), expected_error_message);
        assert_eq!(consent_calls, expected_consent_calls);
    });

    assert_eq!(mock.calls(), expected_requests_count);
}

#[rstest]
#[case::granted(Some(true), 1)]
#[case::omitted(None, 0)]
fn granted_or_omitted_consent_opens_container_validated_by_service(
    validation_service_env: ValidationServiceEnv,
    #[case] consent_answer: Option<bool>,
    #[case] expected_consent_calls: usize,
) {
    let mock = validation_service_env.with_valid_response(|library, path| {
        let (result, consent_calls) = open_container_counting_calls(library, path, consent_answer);
        assert_eq!(consent_calls, expected_consent_calls);

        let container = result.unwrap();
        assert_eq!(container.media_type().unwrap(), ContainerType::Ddoc);

        let data_files = container.data_files().unwrap();
        assert_eq!(data_files.len(), 1);
        assert_eq!(data_files[0].file_name().unwrap(), "test1.txt");

        let signatures = container.signatures().unwrap();
        assert_eq!(signatures.len(), 1);
        assert_eq!(signatures[0].id().unwrap(), "S0");
        assert_eq!(
            signatures[0].profile().unwrap(),
            SignatureProfile::Other(String::from("DIGIDOC_XML_1.3"))
        );
        signatures[0].validate().unwrap();
    });

    assert_eq!(mock.calls(), 1);
}

#[rstest]
fn panicking_consent_resumes_panic_and_sends_nothing(validation_service_env: ValidationServiceEnv) {
    let mock = validation_service_env.with_valid_response(|library, path| {
        let mut consent_calls = 0_usize;

        let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
            Container::open_requiring_online_validation_consent(library, path, || {
                consent_calls += 1;
                panic!("scripted panic")
            })
        }));

        let payload = outcome.err().unwrap();
        assert_eq!(payload.downcast_ref::<&str>(), Some(&"scripted panic"));
        assert_eq!(consent_calls, 1);
    });

    assert_eq!(mock.calls(), 0);
}
