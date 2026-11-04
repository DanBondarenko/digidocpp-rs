#![allow(missing_docs)]

use digidocpp::conf::Conf;
use digidocpp::library::{Access, AccessError};
use digidocpp::signature::SignatureMut;
use digidocpp::signer::{ExternalPartialSigner, Pkcs11Signer, Pkcs12Signer};
use digidocpp::{Container, DataFile, Error, Initialized, Library, Signature, Uninitialized};
use static_assertions::{assert_impl_all, assert_not_impl_any};

assert_not_impl_any!(Library<Uninitialized>: Send, Sync);
assert_not_impl_any!(Library<Initialized>: Send, Sync);
assert_not_impl_any!(Access: Send, Sync);
assert_not_impl_any!(Container<'static>: Send, Sync);
assert_not_impl_any!(Signature<'static>: Send, Sync);
assert_not_impl_any!(SignatureMut<'static>: Send, Sync);
assert_not_impl_any!(DataFile<'static>: Send, Sync);
assert_not_impl_any!(Pkcs11Signer<'static>: Send, Sync);
assert_not_impl_any!(Pkcs12Signer<'static>: Send, Sync);
assert_not_impl_any!(ExternalPartialSigner<'static>: Send, Sync);
assert_not_impl_any!(Conf: Send, Sync);

assert_impl_all!(Error: Send, Sync);
assert_impl_all!(AccessError: Send, Sync);
