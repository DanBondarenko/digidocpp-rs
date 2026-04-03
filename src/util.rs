#![allow(clippy::redundant_pub_crate, reason = "Internal utility module")]

use std::fmt::Write;

pub(crate) fn hex_encode(bytes: impl IntoIterator<Item = u8>) -> String {
    bytes.into_iter().fold(String::new(), |mut output, byte| {
        let _ = write!(output, "{byte:02x}");
        output
    })
}
