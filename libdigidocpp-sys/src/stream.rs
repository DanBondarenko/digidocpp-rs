//! Rust readers and writers exposed to C++ as `std::istream` and `std::ostream`.
//!
//! # Failures
//!
//! A reader or writer can fail with an `io::Error` or a panic. C++ stream code cannot carry
//! either one. So this module stores the first failure in a thread-local slot. The stream then
//! reports a plain error to C++.
//!
//! Wrap every C++ call that may use a stream in [`call_then_deliver_stream_failure`]. After the
//! call, it returns the stored `io::Error` or resumes the stored panic.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::fmt::{self, Debug, Formatter};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::panic::{self, AssertUnwindSafe};

/// Bridge for streams.
#[cxx::bridge]
pub mod ffi {
    #[namespace = "digidoc_rs::stream"]
    extern "Rust" {
        /// A Rust [`Write`](std::io::Write) that C++ sees as an `std::ostream`.
        #[derive(ExternType)]
        type Sink<'a>;

        /// A Rust [`Read`](std::io::Read) + [`Seek`](std::io::Seek) that C++ sees as an
        /// `std::istream`.
        #[derive(ExternType)]
        type Source;

        /// Writes all of `data`.
        fn write(self: &mut Sink<'_>, data: &[u8]) -> Result<()>;

        /// Reads into `buffer`.
        fn read(self: &Source, buffer: &mut [u8]) -> Result<usize>;

        /// Seeks to `offset` bytes from the start.
        fn seek_from_start(self: &Source, offset: u64) -> Result<u64>;

        /// Seeks relative to the end.
        fn seek_from_end(self: &Source, offset: i64) -> Result<u64>;

        /// Seeks relative to the current position.
        fn seek_from_current(self: &Source, offset: i64) -> Result<u64>;
    }

    #[namespace = "digidoc_rs::stream"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/exception_ffi.h");
    }

    impl Box<Source> {}

    #[cfg(feature = "_test_util")]
    #[namespace = "digidoc_rs::stream"]
    unsafe extern "C++" {
        include!("libdigidocpp-sys/include/test_util.h");

        /// Writes `bytes` through the C++ `ostream` of `sink`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn copy_bytes_through_sink(sink: &mut Sink<'_>, bytes: &[u8]) -> Result<()>;

        /// Reads `source` to the end through its C++ `istream`, `chunk_size` bytes at a time.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn read_all_from_source(source: Box<Source>, chunk_size: usize) -> Result<Vec<u8>>;

        /// Reads `source` to the end through the `rdbuf` of its C++ `istream`.
        ///
        /// # Errors
        /// Returns [`cxx::Exception`] when the C++ call throws.
        fn copy_source_via_rdbuf(source: Box<Source>) -> Result<Vec<u8>>;
    }
}

const EARLIER_FAILURE: &str = "the data source failed earlier and cannot be used again";
const REENTRANT_USE: &str = "the reader was used again while it was already in use";
const PANIC_MESSAGE: &str = "the reader or writer panicked";

enum StreamFailure {
    Panic(Box<dyn Any + Send>),
    Io(io::Error),
}

thread_local! {
    static STREAM_FAILURE: RefCell<Option<StreamFailure>> = const { RefCell::new(None) };
}

fn store_stream_failure(failure: StreamFailure) {
    STREAM_FAILURE.with_borrow_mut(|slot| {
        if slot.is_none() {
            *slot = Some(failure);
        }
    });
}

/// Runs `ffi_call`, then returns the stream failure it caused, if any.
///
/// # Errors
/// Returns the first `io::Error` that a [`Source`] or [`Sink`] hit during the call.
///
/// # Panics
///
/// Resumes the panic of a reader or writer that panicked during the call.
pub fn call_then_deliver_stream_failure<T>(ffi_call: impl FnOnce() -> T) -> Result<T, io::Error> {
    let result = ffi_call();
    match STREAM_FAILURE.with_borrow_mut(Option::take) {
        Some(StreamFailure::Panic(payload)) => panic::resume_unwind(payload),
        Some(StreamFailure::Io(error)) => Err(error),
        None => Ok(result),
    }
}

/// performs call while storing error/panic
fn run_and_store_failure<T>(callback: impl FnOnce() -> io::Result<T>) -> Result<T, String> {
    match panic::catch_unwind(AssertUnwindSafe(callback)) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            let message = error.to_string();
            store_stream_failure(StreamFailure::Io(error));
            Err(message)
        }
        Err(payload) => {
            store_stream_failure(StreamFailure::Panic(payload));
            Err(String::from(PANIC_MESSAGE))
        }
    }
}

/// A writer for C++, exposed as `std::ostream`.
pub struct Sink<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> Sink<'a> {
    /// Wraps `writer`.
    pub fn new(writer: &'a mut dyn Write) -> Self {
        Self { writer }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), String> {
        run_and_store_failure(|| self.writer.write_all(data))
    }
}

impl Debug for Sink<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sink").finish_non_exhaustive()
    }
}

trait ReadSeek: Read + Seek {}

impl<T: Read + Seek> ReadSeek for T {}

/// A reader for C++, exposed as `std::istream`.
///
/// After a failure it rejects all further calls. A reentrant call (the reader being used again
/// while it runs) also fails.
pub struct Source {
    // guard against reentrant reader calls
    reader: RefCell<Box<dyn ReadSeek>>,
    failed: Cell<bool>,
}

impl Source {
    /// Wraps `reader`.
    pub fn new(reader: impl Read + Seek + 'static) -> Self {
        Self {
            reader: RefCell::new(Box::new(reader)),
            failed: Cell::new(false),
        }
    }

    fn run_unless_failed_and_store_failure<T>(
        &self,
        reader_call: impl FnOnce(&mut dyn ReadSeek) -> io::Result<T>,
    ) -> Result<T, String> {
        if self.failed.get() {
            store_stream_failure(StreamFailure::Io(io::Error::other(EARLIER_FAILURE)));
            return Err(String::from(EARLIER_FAILURE));
        }

        let Ok(mut reader) = self.reader.try_borrow_mut() else {
            store_stream_failure(StreamFailure::Io(io::Error::other(REENTRANT_USE)));
            return Err(String::from(REENTRANT_USE));
        };
        let result = run_and_store_failure(|| reader_call(&mut **reader));
        self.failed.set(result.is_err());
        result
    }

    fn read(&self, buffer: &mut [u8]) -> Result<usize, String> {
        let capacity = buffer.len();
        self.run_unless_failed_and_store_failure(|reader| {
            let count = loop {
                match reader.read(buffer) {
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                    result => break result,
                }
            }?;
            if count > capacity {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("the reader returned {count} bytes for a buffer of {capacity} bytes"),
                ));
            }
            Ok(count)
        })
    }

    fn seek_from_start(&self, offset: u64) -> Result<u64, String> {
        self.seek(SeekFrom::Start(offset))
    }

    fn seek_from_end(&self, offset: i64) -> Result<u64, String> {
        self.seek(SeekFrom::End(offset))
    }

    fn seek_from_current(&self, offset: i64) -> Result<u64, String> {
        self.seek(SeekFrom::Current(offset))
    }

    fn seek(&self, position: SeekFrom) -> Result<u64, String> {
        self.run_unless_failed_and_store_failure(|reader| reader.seek(position))
    }
}

impl Debug for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Source").finish_non_exhaustive()
    }
}
