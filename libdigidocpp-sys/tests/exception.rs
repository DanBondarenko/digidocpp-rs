#![allow(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

use libdigidocpp_sys::exception::ffi;

fn exception_data(result: Result<(), cxx::Exception>) -> ffi::ExceptionData {
    ffi::ExceptionData::from(result.unwrap_err())
}

#[test]
fn keeps_a_valid_message_code_file_and_line() {
    let data = exception_data(ffi::throw_digidoc_exception(b"plain message"));

    assert!(!data.is_ffi);
    assert!(data.is_digidoc);
    assert!(!data.is_unknown);
    assert_eq!(data.message, "plain message");
    assert_eq!(data.code, ffi::ExceptionCode::General);
    assert_eq!(data.file, "test.cpp");
    assert_eq!(data.line, 1);
}

#[test]
fn replaces_invalid_utf8_in_a_digidoc_exception() {
    let data = exception_data(ffi::throw_digidoc_exception(b"before \xff after"));

    assert!(!data.is_ffi);
    assert!(data.is_digidoc);
    assert!(!data.is_unknown);
    assert_eq!(data.message, "before \u{fffd} after");
    assert_eq!(data.code, ffi::ExceptionCode::General);
    assert_eq!(data.file, "test.cpp");
    assert_eq!(data.line, 1);
}

#[test]
fn replaces_invalid_utf8_in_a_std_exception() {
    let data = exception_data(ffi::throw_std_exception(b"before \xff after"));

    assert!(!data.is_ffi);
    assert!(!data.is_digidoc);
    assert!(data.is_unknown);
    assert_eq!(data.message, "before \u{fffd} after");
    assert!(data.file.is_empty());
    assert!(data.causes.is_empty());
}

#[test]
fn turns_a_non_std_exception_into_a_generic_error() {
    let data = exception_data(ffi::throw_non_std_exception());

    assert!(!data.is_ffi);
    assert!(!data.is_digidoc);
    assert!(data.is_unknown);
    assert_eq!(data.message, "Unknown C++ exception");
    assert_eq!(data.code, ffi::ExceptionCode::General);
    assert!(data.file.is_empty());
    assert!(data.causes.is_empty());
}

#[test]
fn lists_causes_in_depth_first_order_with_depths() {
    let data = exception_data(ffi::throw_digidoc_exception_with_causes());

    assert!(!data.is_ffi);
    assert!(data.is_digidoc);
    assert!(!data.is_unknown);
    assert_eq!(data.code, ffi::ExceptionCode::General);
    assert_eq!(data.message, "root");
    assert_eq!(data.file, "root.cpp");
    assert_eq!(data.line, 1);

    let [first_cause, second_cause, third_cause] = &data.causes[..] else {
        panic!("Expected three causes")
    };

    assert_eq!(first_cause.message.as_str(), "cause a");
    assert_eq!(first_cause.file.as_str(), "a.cpp");
    assert_eq!(first_cause.line, 2);
    assert_eq!(first_cause.depth, 0);

    assert_eq!(second_cause.message.as_str(), "cause c");
    assert_eq!(second_cause.file.as_str(), "c.cpp");
    assert_eq!(second_cause.line, 3);
    assert_eq!(second_cause.depth, 1);

    assert_eq!(third_cause.message.as_str(), "cause b");
    assert_eq!(third_cause.file.as_str(), "b.cpp");
    assert_eq!(third_cause.line, 4);
    assert_eq!(third_cause.depth, 0);
}
