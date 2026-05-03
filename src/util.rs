#![allow(clippy::redundant_pub_crate, reason = "Internal utility module")]

use crate::Error;
use std::path::Path;

pub(crate) fn path_to_str(path: &Path) -> Result<&str, Error> {
    path.to_str()
        .ok_or_else(|| Error::Internal(format!("path is not valid UTF-8: {}", path.display())))
}
