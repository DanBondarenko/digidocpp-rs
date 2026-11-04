#![allow(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::indexing_slicing,
        unreachable_pub,
    )
)]
mod common;

use crate::common::{conf_for_testing, dir_with_fixtures};
use digidocpp::library::{Access, AccessError, acquire, try_acquire};
use digidocpp::{Container, Initialized, Library, Uninitialized};
use rstest::rstest;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

#[rstest]
#[case::uninitialized(|_: &Path| Access::Uninitialized(Library::<Uninitialized>::acquire().unwrap()))]
#[case::initialized(|directory: &Path| {
    let library = Library::<Uninitialized>::acquire().unwrap();
    let conf = conf_for_testing(&library, directory, "TSL.xml");
    Access::Initialized(common::initialize_with(library, conf))
})]
fn access_waits_until_handle_is_dropped(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml"])]
    directory: TempDir,
    #[case] acquire_access: fn(&Path) -> Access,
) {
    let library_access = acquire_access(directory.path());
    let (sender, receiver) = mpsc::channel();
    let waiter = thread::spawn(move || {
        sender.send(try_acquire().map(drop)).unwrap();
        sender.send(acquire().map(drop)).unwrap();
    });

    assert_eq!(
        // asserts try_acquire() is sent
        receiver.recv_timeout(Duration::from_secs(60)).unwrap(),
        Err(AccessError::WouldBlock)
    );

    // asserts acquire() is still waiting
    assert!(receiver.recv_timeout(Duration::from_millis(200)).is_err());

    // asserts same-thread re-entry causes special error
    assert_eq!(acquire().unwrap_err(), AccessError::AlreadyHeld);

    drop(library_access);

    // asserts acquire() is sent
    assert_eq!(
        receiver.recv_timeout(Duration::from_secs(60)).unwrap(),
        Ok(())
    );

    waiter.join().unwrap();
}

#[test]
fn panic_while_holding_handle_does_not_block_later_access() {
    let holder = thread::spawn(|| {
        let _library = Library::<Uninitialized>::acquire().unwrap();
        panic!("holder panics with handle");
    });
    assert!(holder.join().is_err());

    let _library = Library::<Uninitialized>::try_acquire().unwrap();
}

#[derive(Clone, Copy)]
enum Executor {
    Threads,
    TokioBlocking,
}

impl Executor {
    fn run_one(self, job: impl FnOnce() + Send + 'static) {
        self.run_concurrently(vec![Box::new(job)]);
    }

    fn run_concurrently(self, jobs: Vec<Box<dyn FnOnce() + Send>>) {
        match self {
            Self::Threads => {
                let handles: Vec<_> = jobs.into_iter().map(thread::spawn).collect();
                for handle in handles {
                    handle.join().unwrap();
                }
            }
            Self::TokioBlocking => {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(2)
                    .build()
                    .unwrap();
                runtime.block_on(async {
                    let handles: Vec<_> =
                        jobs.into_iter().map(tokio::task::spawn_blocking).collect();
                    for handle in handles {
                        handle.await.unwrap();
                    }
                });
            }
        }
    }
}

#[rstest]
#[case::os_threads(Executor::Threads)]
#[case::tokio_blocking(Executor::TokioBlocking)]
fn library_works_across_threads(
    #[from(dir_with_fixtures)]
    #[with(&["TSL.xml", "EE_T.xml"])]
    directory: TempDir,
    #[case] executor: Executor,
) {
    let directory_path = directory.path().to_owned();

    // Configure and initialize in a separate thread
    executor.run_one(move || {
        let mut library = Library::<Uninitialized>::acquire().unwrap();
        let conf = conf_for_testing(&library, &directory_path, "TSL.xml");
        library.configure(conf).unwrap();
        library.initialize().unwrap();
    });

    // Validate a container on 8 separate threads
    let users = (0..8)
        .map(|_| -> Box<dyn FnOnce() + Send> {
            Box::new(|| {
                let library = Library::<Initialized>::acquire().unwrap();
                let container =
                    Container::open(&library, common::fixture_path("tsl.asice")).unwrap();
                container.signatures().unwrap()[0].validate().unwrap();
            })
        })
        .collect();
    executor.run_concurrently(users);

    executor.run_one(|| {
        Library::<Initialized>::acquire()
            .unwrap()
            .terminate()
            .unwrap();
    });
}
