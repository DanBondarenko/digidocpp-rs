use std::fmt::{Debug, Formatter};

/// A DER-encoded X.509 certificate.
///
/// The bytes are not parsed.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DerCert(pub(crate) Vec<u8>);

impl DerCert {
    pub(crate) fn from_ffi(bytes: Vec<u8>) -> Option<Self> {
        let is_filled = !bytes.is_empty();
        is_filled.then_some(Self(bytes))
    }

    /// Returns the DER bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for DerCert {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Debug for DerCert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DerCert")
            .field(&format_args!("{} bytes", self.0.len()))
            .finish()
    }
}

impl AsRef<[u8]> for DerCert {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
