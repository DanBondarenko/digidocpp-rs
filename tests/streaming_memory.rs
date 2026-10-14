#![allow(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        unreachable_pub,
        clippy::indexing_slicing,
        clippy::used_underscore_binding,
    )
)]
mod common;

use crate::common::{MEDIA_TYPE_OCTET_STREAM, create_empty_asice_container, init_with_valid_tsl};
use digidocpp::{Container, DigestAlgorithm};
use rstest::rstest;
use serial_test::serial;
use std::cell::Cell;
use std::error::Error as StdError;
use std::io::{self, Read, Seek, SeekFrom};
use std::rc::Rc;
use tempfile::TempDir;

// KEEP THIS TEST IN A SEPARATE FILE: memory usage is measured per process

fn resident_memory() -> usize {
    memory_stats::memory_stats()
        .expect("the platform should report memory use")
        .physical_mem
}

/// behaves as if reading `DATA_SIZE` zero bytes while recording memory usage
struct ZerosReader {
    position: u64,
    peak_memory: Rc<Cell<usize>>,
}

impl ZerosReader {
    const SIZE: u64 = 512 * 1024 * 1024;
    // SHA for zeroes
    const SHA256: &str = "9acca8e8c22201155389f65abbf6bc9723edc7384ead80503839f49dcc56d767";
}

impl Read for ZerosReader {
    #[expect(clippy::as_conversions)]
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.peak_memory.update(|peak| peak.max(resident_memory()));

        let count = io::repeat(0)
            .take(Self::SIZE.saturating_sub(self.position))
            .read(buffer)?;
        self.position += count as u64;
        Ok(count)
    }
}

impl Seek for ZerosReader {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        let target = match position {
            SeekFrom::Start(offset) => Some(offset),
            SeekFrom::End(offset) => Self::SIZE.checked_add_signed(offset),
            SeekFrom::Current(offset) => self.position.checked_add_signed(offset),
        };
        self.position = target.ok_or_else(|| io::Error::other("seek before the start"))?;
        Ok(self.position)
    }
}

#[rstest]
#[serial]
fn streams_large_file_without_holding_in_memory(
    #[from(init_with_valid_tsl)] _temp_dir: TempDir,
    #[from(create_empty_asice_container)] mut container: Container,
) -> Result<(), Box<dyn StdError>> {
    const MAX_MEMORY_GROWTH: usize = 64 * 1024 * 1024;

    struct CountingWriter {
        delivered: u64,
    }

    impl io::Write for CountingWriter {
        #[expect(clippy::as_conversions)]
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            self.delivered += data.len() as u64;
            Ok(data.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let baseline_memory = resident_memory();
    let peak_memory: Rc<Cell<usize>> = Rc::default();
    let memory_growth = || peak_memory.get().saturating_sub(baseline_memory);
    container.add_data_file_from_reader(
        "large.bin",
        MEDIA_TYPE_OCTET_STREAM,
        ZerosReader {
            position: 0,
            peak_memory: Rc::clone(&peak_memory),
        },
    )?;
    let data_file = &container.data_files()?[0];
    let mut writer = CountingWriter { delivered: 0 };

    data_file.write_to(&mut writer)?;

    assert_eq!(writer.delivered, ZerosReader::SIZE);
    assert!(
        memory_growth() < MAX_MEMORY_GROWTH,
        "memory grew by {} bytes",
        memory_growth()
    );

    let digest = hex::encode(data_file.calc_digest(DigestAlgorithm::Sha256)?);

    assert_eq!(digest, ZerosReader::SHA256);
    assert!(
        memory_growth() < MAX_MEMORY_GROWTH,
        "memory grew by {} bytes",
        memory_growth()
    );

    Ok(())
}
