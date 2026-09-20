#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::redundant_pub_crate,
        clippy::indexing_slicing
    )
)]

use libdigidocpp_sys::stream::{Sink, Source, call_then_deliver_stream_failure, ffi};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io::{self, Cursor, ErrorKind, Read, Seek, SeekFrom, Write};

const BUFFER_SIZE: usize = 64 * 1024;
const BOUNDARY_SIZES: [usize; 6] = [
    0,
    1,
    BUFFER_SIZE - 1,
    BUFFER_SIZE,
    BUFFER_SIZE + 1,
    2 * BUFFER_SIZE + 1,
];

fn random_bytes(size: usize) -> Vec<u8> {
    let mut bytes = vec![0; size];
    StdRng::seed_from_u64(0).fill_bytes(&mut bytes);
    bytes
}

#[test]
fn writes_exact_bytes() {
    #[derive(Default)]
    struct RecordingWriter {
        bytes: Vec<u8>,
        chunk_sizes: Vec<usize>,
    }

    impl Write for RecordingWriter {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            self.chunk_sizes.push(data.len());
            self.bytes.extend_from_slice(data);
            Ok(data.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    for size in BOUNDARY_SIZES.into_iter().chain([10 * 1024 * 1024]) {
        let expected = random_bytes(size);
        let mut writer = RecordingWriter::default();
        let mut sink = Sink::new(&mut writer);

        let result =
            call_then_deliver_stream_failure(|| ffi::copy_bytes_through_sink(&mut sink, &expected))
                .unwrap_or_else(|error| panic!("size {size}: {error}"));

        result.unwrap();
        assert_eq!(writer.bytes, expected, "{size}");
        assert!(
            writer.chunk_sizes.iter().all(|chunk| *chunk <= BUFFER_SIZE),
            "{size}"
        );
        assert!(
            writer.chunk_sizes.len() <= size / 4096 + 1,
            "{} writer calls (size{size})",
            writer.chunk_sizes.len()
        );
    }
}

#[test]
fn stops_at_a_writer_failure_and_reports_the_original_error() {
    struct FailingWriter {
        calls: usize,
        failing_call: usize,
    }

    impl Write for FailingWriter {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            if self.calls == self.failing_call {
                Err(io::Error::new(ErrorKind::BrokenPipe, "scripted failure"))
            } else {
                Ok(data.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let size_and_failing_call = [(200_000, 1), (400_000, 4), (10, 1)];

    for (size, failing_call) in size_and_failing_call {
        let mut writer = FailingWriter {
            calls: 0,
            failing_call,
        };
        let mut sink = Sink::new(&mut writer);

        let io_error = call_then_deliver_stream_failure(|| {
            ffi::copy_bytes_through_sink(&mut sink, &random_bytes(size))
        })
        .expect_err(&format!("{size}"));

        assert_eq!(io_error.kind(), ErrorKind::BrokenPipe, "{size}");
        assert_eq!(io_error.to_string(), "scripted failure", "{size}");
        assert_eq!(writer.calls, failing_call, "{size}");
    }
}

#[test]
fn reads_exact_bytes_at_buffer_boundaries() {
    struct ShortReader {
        inner: Cursor<Vec<u8>>,
    }

    impl Read for ShortReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let limit = buffer.len().min(3);
            self.inner.read(&mut buffer[..limit])
        }
    }

    impl Seek for ShortReader {
        fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
            self.inner.seek(position)
        }
    }

    for size in BOUNDARY_SIZES {
        let expected = random_bytes(size);

        for chunk_size in [1, 4096, 70_000] {
            let source = Source::new(Cursor::new(expected.clone()));

            let bytes = ffi::read_all_from_source(Box::new(source), chunk_size).unwrap();

            assert_eq!(bytes, expected, "{size}");
        }

        let source = Source::new(Cursor::new(expected.clone()));
        let bytes = ffi::copy_source_via_rdbuf(Box::new(source)).unwrap();
        assert_eq!(bytes, expected, "{size}");

        let source = Source::new(ShortReader {
            inner: Cursor::new(expected.clone()),
        });
        let bytes = ffi::read_all_from_source(Box::new(source), 4096).unwrap();
        assert_eq!(bytes, expected, "{size}");
    }
}

#[test]
fn retries_a_read_that_was_interrupted() {
    struct InterruptedOnFirstRead {
        inner: Cursor<Vec<u8>>,
        interrupted: bool,
    }

    impl Read for InterruptedOnFirstRead {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if !self.interrupted {
                self.interrupted = true;
                return Err(io::Error::from(ErrorKind::Interrupted));
            }
            self.inner.read(buffer)
        }
    }

    impl Seek for InterruptedOnFirstRead {
        fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
            self.inner.seek(position)
        }
    }

    let expected = random_bytes(1000);
    let reader = InterruptedOnFirstRead {
        inner: Cursor::new(expected.clone()),
        interrupted: false,
    };

    let bytes = ffi::read_all_from_source(Box::new(Source::new(reader)), 4096).unwrap();

    assert_eq!(bytes, expected);
}
