use anyhow::Result;
use rstest::rstest;
use serde_json::Value;
use serial_test::serial;
use std::fs;
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use ureq::unversioned::multipart::{Form, Part};

#[derive(Clone, Copy)]
enum TslSource {
    Fixture(&'static str),
    #[cfg(feature = "_online_tests")]
    Download,
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn start_process(conf_path: &Path, port: u16) -> Result<Child> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_digidocpp-example-server"));
    command
        .env("PORT", port.to_string())
        .env("CONF", conf_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    Ok(command.spawn()?)
}

struct TestServer {
    process: Child,
    address: SocketAddr,
    _directory: TempDir,
}

impl TestServer {
    fn start(tsl_source: TslSource) -> Result<Self> {
        let directory = TempDir::new()?;
        let tsl_fixture = match tsl_source {
            TslSource::Fixture(name) => Some(name),
            #[cfg(feature = "_online_tests")]
            TslSource::Download => None,
        };
        let is_online = tsl_fixture.is_none();
        let conf_path = directory.path().join("test.conf");
        let cache_path = directory.path().display();
        let conf_content = format!(
            r#"<configuration>
                   <param name="tsl.autoupdate">{is_online}</param>
                   <param name="tsl.onlineDigest">{is_online}</param>
                   <param name="tsl.cache">{cache_path}</param>
               </configuration>"#,
        );

        fs::write(&conf_path, conf_content)?;
        if let Some(name) = tsl_fixture {
            fs::copy(fixture_path(name), directory.path().join("EE_T.xml"))?;
        }

        let port = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?
            .local_addr()?
            .port();
        let process = start_process(conf_path.as_path(), port)?;
        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
        let deadline = Instant::now() + Duration::from_secs(30);

        while TcpStream::connect(address).is_err() {
            assert!(Instant::now() < deadline);
            sleep(Duration::from_millis(100));
        }

        Ok(Self {
            process,
            address,
            _directory: directory,
        })
    }

    fn post_form(&self, path: &str, form: Form) -> Result<Vec<u8>, ureq::Error> {
        ureq::post(format!("http://{}{path}", self.address))
            .config()
            .timeout_global(Some(Duration::from_secs(60)))
            .build()
            .send(form)?
            .body_mut()
            .read_to_vec()
    }

    fn validate(&self, container: &[u8]) -> Result<Value> {
        let report = self.post_form(
            "/validate",
            Form::new().part("file", Part::bytes(container)),
        )?;

        Ok(serde_json::from_slice(&report)?)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

#[cfg(feature = "_online_tests")]
#[rstest]
#[case::p12("/sign/p12")]
#[case::external_service("/sign/external")]
#[serial]
fn signs_with_key_from_p12_file(#[case] endpoint: &str) -> Result<()> {
    let server = TestServer::start(TslSource::Download)?;
    let file = Part::bytes(b"hello")
        .file_name("hello.txt")
        .mime_str("text/plain")?;

    let signed_container = server.post_form(endpoint, Form::new().part("file", file))?;
    let report = server.validate(&signed_container)?;

    assert_eq!(report["is_valid"], true);
    assert_eq!(report["signatures"][0]["is_valid"], true);
    assert_eq!(
        report["signatures"][0]["signed_by"],
        "O’CONNEŽ-ŠUSLIK TESTNUMBER,MARY ÄNN,60001016970"
    );
    Ok(())
}

#[cfg(feature = "_online_tests")]
#[rstest]
#[serial]
fn extends_signature_in_place() -> Result<()> {
    let server = TestServer::start(TslSource::Download)?;
    let file = Part::bytes(b"greetings")
        .file_name("greeting.txt")
        .mime_str("text/plain")?;

    let signed_container = server.post_form("/sign/p12", Form::new().part("file", file))?;
    let extended_container = server.post_form(
        "/extend",
        Form::new().part("file", Part::bytes(&signed_container)),
    )?;
    let report = server.validate(&extended_container)?;

    assert_eq!(report["is_valid"], true);
    assert_eq!(report["signatures"][0]["is_valid"], true);
    Ok(())
}

#[rstest]
#[case::signed_by_person(
    TslSource::Fixture("EE_T.xml"),
    "tsl.asice",
    true,
    "Valid",
    "MÄNNIK,MARI-LIIS,47101010033",
    "2016-11-28T13:46:43Z"
)]
#[case::signed_by_person_without_qualification(
    TslSource::Fixture("EE_T-no_QCSD.xml"),
    "tsl.asice",
    true,
    "NonQSCD",
    "MÄNNIK,MARI-LIIS,47101010033",
    "2016-11-28T13:46:43Z"
)]
#[case::time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test.asics",
    true,
    "Valid",
    "DEMO of SK TSA 2014",
    "2016-11-02T11:07:45Z"
)]
#[case::invalid_time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test-invalidts.asics",
    false,
    "Invalid",
    "DEMO of SK TSA 2014",
    "2016-11-02T11:07:45Z"
)]
#[cfg_attr(
    feature = "_online_tests",
    case::signed_by_person_online(
        TslSource::Download,
        "tsl.asice",
        true,
        "Valid",
        "MÄNNIK,MARI-LIIS,47101010033",
        "2016-11-28T13:46:43Z"
    )
)]
#[serial]
fn validates_fixture_container(
    #[case] tsl_source: TslSource,
    #[case] fixture_name: &str,
    #[case] is_valid: bool,
    #[case] status: &str,
    #[case] signed_by: &str,
    #[case] signed_at: &str,
) -> Result<()> {
    let server = TestServer::start(tsl_source)?;
    let container = fs::read(fixture_path(fixture_name))?;

    let report = server.validate(&container)?;

    assert_eq!(report["is_valid"], is_valid);
    assert_eq!(report["signatures"][0]["is_valid"], is_valid);
    assert_eq!(report["signatures"][0]["status"], status);
    assert_eq!(report["signatures"][0]["signed_by"], signed_by);
    assert_eq!(report["signatures"][0]["signed_at"], signed_at);
    Ok(())
}

#[rstest]
#[case::signed_by_person(
    TslSource::Fixture("EE_T.xml"),
    "tsl.asice",
    "application/vnd.etsi.asic-e+zip",
    "test.txt",
    4,
    true,
    "MÄNNIK,MARI-LIIS,47101010033"
)]
#[case::time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test.asics",
    "application/vnd.etsi.asic-s+zip",
    "test1.txt",
    5,
    true,
    "DEMO of SK TSA 2014"
)]
#[case::invalid_time_stamp(
    TslSource::Fixture("EE_T.xml"),
    "test-invalidts.asics",
    "application/vnd.etsi.asic-s+zip",
    "test2.txt",
    10,
    false,
    "DEMO of SK TSA 2014"
)]
#[cfg_attr(
    feature = "_online_tests",
    case::signed_by_person_online(
        TslSource::Download,
        "tsl.asice",
        "application/vnd.etsi.asic-e+zip",
        "test.txt",
        4,
        true,
        "MÄNNIK,MARI-LIIS,47101010033"
    )
)]
#[serial]
fn shows_container_info(
    #[case] tsl_source: TslSource,
    #[case] fixture_name: &str,
    #[case] container_type: &str,
    #[case] file_name: &str,
    #[case] size_bytes: u64,
    #[case] is_valid: bool,
    #[case] signed_by: &str,
) -> Result<()> {
    let server = TestServer::start(tsl_source)?;
    let container = fs::read(fixture_path(fixture_name))?;

    let response = server.post_form("/info", Form::new().part("file", Part::bytes(&container)))?;
    let info: Value = serde_json::from_slice(&response)?;

    assert_eq!(info["container_type"], container_type);
    assert_eq!(info["data_files"][0]["name"], file_name);
    assert_eq!(info["data_files"][0]["size_bytes"], size_bytes);
    assert_eq!(info["signatures"][0]["is_valid"], is_valid);
    assert_eq!(info["signatures"][0]["signed_by"], signed_by);
    Ok(())
}
