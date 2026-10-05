// these tests use real APIs over network
use anyhow::Result;
use rstest::{fixture, rstest};
use serde_json::Value;
use serial_test::serial;
use std::fs;
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};
use ureq::unversioned::multipart::{Form, Part};

struct TestServer {
    process: Child,
    address: SocketAddr,
}

impl TestServer {
    fn start() -> Result<Self> {
        let port = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?
            .local_addr()?
            .port();
        let process = Command::new(env!("CARGO_BIN_EXE_digidocpp-example-server"))
            .env("PORT", port.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
        let server = Self { process, address };
        let deadline = Instant::now() + Duration::from_secs(30);
        while TcpStream::connect(address).is_err() {
            assert!(Instant::now() < deadline);
            sleep(Duration::from_millis(100));
        }

        Ok(server)
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

#[fixture]
fn server() -> TestServer {
    TestServer::start().unwrap()
}

#[rstest]
#[case::p12("/sign/p12")]
#[case::external_service("/sign/external")]
#[serial]
fn signs_with_key_from_p12_file(server: TestServer, #[case] endpoint: &str) -> Result<()> {
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

#[rstest]
#[serial]
fn extends_signature_in_place(server: TestServer) -> Result<()> {
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
    "tsl.asice",
    true,
    "MÄNNIK,MARI-LIIS,47101010033",
    "2016-11-28T13:46:43Z"
)]
#[case::time_stamp("test.asics", true, "DEMO of SK TSA 2014", "2016-11-02T11:07:45Z")]
#[case::invalid_time_stamp(
    "test-invalidts.asics",
    false,
    "DEMO of SK TSA 2014",
    "2016-11-02T11:07:45Z"
)]
#[serial]
fn validates_fixture_container(
    server: TestServer,
    #[case] fixture: &str,
    #[case] is_valid: bool,
    #[case] signed_by: &str,
    #[case] signed_at: &str,
) -> Result<()> {
    let container = fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(fixture),
    )?;

    let report = server.validate(&container)?;

    assert_eq!(report["is_valid"], is_valid);
    assert_eq!(report["signatures"][0]["is_valid"], is_valid);
    assert_eq!(report["signatures"][0]["signed_by"], signed_by);
    assert_eq!(report["signatures"][0]["signed_at"], signed_at);
    Ok(())
}

#[rstest]
#[case::signed_by_person(
    "tsl.asice",
    "application/vnd.etsi.asic-e+zip",
    "test.txt",
    4,
    true,
    "MÄNNIK,MARI-LIIS,47101010033"
)]
#[case::time_stamp(
    "test.asics",
    "application/vnd.etsi.asic-s+zip",
    "test1.txt",
    5,
    true,
    "DEMO of SK TSA 2014"
)]
#[case::invalid_time_stamp(
    "test-invalidts.asics",
    "application/vnd.etsi.asic-s+zip",
    "test2.txt",
    10,
    false,
    "DEMO of SK TSA 2014"
)]
#[serial]
fn shows_container_info(
    server: TestServer,
    #[case] fixture: &str,
    #[case] container_type: &str,
    #[case] file_name: &str,
    #[case] size_bytes: u64,
    #[case] is_valid: bool,
    #[case] signed_by: &str,
) -> Result<()> {
    let container = fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(fixture),
    )?;

    let response = server.post_form("/info", Form::new().part("file", Part::bytes(&container)))?;
    let info: Value = serde_json::from_slice(&response)?;

    assert_eq!(info["container_type"], container_type);
    assert_eq!(info["data_files"][0]["name"], file_name);
    assert_eq!(info["data_files"][0]["size_bytes"], size_bytes);
    assert_eq!(info["signatures"][0]["is_valid"], is_valid);
    assert_eq!(info["signatures"][0]["signed_by"], signed_by);
    Ok(())
}
