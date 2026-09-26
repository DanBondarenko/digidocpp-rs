use std::any::Any;
use std::cell::{Cell, RefCell};
use std::fmt::{self, Debug, Formatter};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::panic::{self, AssertUnwindSafe};

#[cxx::bridge]
pub mod ffi {
    #[namespace = "digidoc_rs::stream"]
    extern "Rust" {
        #[derive(ExternType)]
        type Sink<'a>;
        #[derive(ExternType)]
        type Source;

        fn write(self: &mut Sink<'_>, data: &[u8]) -> Result<()>;

        fn read(self: &Source, buffer: &mut [u8]) -> Result<usize>;
        fn seek_from_start(self: &Source, offset: u64) -> Result<u64>;
        fn seek_from_end(self: &Source, offset: i64) -> Result<u64>;
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

        fn copy_bytes_through_sink(sink: &mut Sink<'_>, bytes: &[u8]) -> Result<()>;
        fn read_all_from_source(source: Box<Source>, chunk_size: usize) -> Result<Vec<u8>>;
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

/// performs call while checking stored error/panic: errors are returned, panics resumed
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

pub struct Sink<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> Sink<'a> {
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

pub struct Source {
    // guard against reentrant reader calls
    reader: RefCell<Box<dyn ReadSeek>>,
    failed: Cell<bool>,
}

impl Source {
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
