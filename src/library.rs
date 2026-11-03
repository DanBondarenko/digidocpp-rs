//! Thread-safe access to libdigidocpp and its lifecycle
//!
//! The libdigidocpp library has global mutable state and no locks. Two calls at same time are
//! unsafe, even on unrelated containers. This module puts a lock in front of library access,
//! ensuring only a single thread has access at a time.
//!
//! The library moves through phases irreversibly throughout its lifecycle:
//! [`Uninitialized`] -> [`Initialized`] -> terminated.
//! Dropping handle only releases lock, phase stays. Next [`acquire`] reports current phase.
//! [`Library<Initialized>::terminate`] ends lifecycle permanently.

use crate::Error;
use crate::conf::Conf;
use crate::util::{ffi_variant_doc, library_and_ffi_variants_doc, library_variant_doc};
use libdigidocpp_sys::init::ffi;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::{Mutex, MutexGuard, PoisonError, TryLockError};
use thiserror::Error as ThisError;

// Process-wide library state
static PHASE: Mutex<Phase> = Mutex::new(Phase::Uninitialized);

// Prevents deadlocks on same-thread reentrancy (`AccessError::AlreadyHeld`)
thread_local! {
    static THREAD_HOLDS_LIBRARY: Cell<bool> = const { Cell::new(false) };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Uninitialized,
    Initialized,
    Terminated,
}

struct PhaseGuard {
    phase: MutexGuard<'static, Phase>,
    _thread_bound: PhantomData<*const ()>,
}

impl PhaseGuard {
    fn lock() -> Result<Self, AccessError> {
        // Phase is plain data and each write is one store, so panic of a holder cannot corrupt it.
        Self::acquire_with(|| Ok(PHASE.lock().unwrap_or_else(PoisonError::into_inner)))
    }

    fn try_lock() -> Result<Self, AccessError> {
        Self::acquire_with(|| match PHASE.try_lock() {
            Ok(guard) => Ok(guard),
            Err(TryLockError::Poisoned(poisoned)) => Ok(poisoned.into_inner()),
            Err(TryLockError::WouldBlock) => Err(AccessError::WouldBlock),
        })
    }

    fn acquire_with(
        lock_phase: impl FnOnce() -> Result<MutexGuard<'static, Phase>, AccessError>,
    ) -> Result<Self, AccessError> {
        if THREAD_HOLDS_LIBRARY.get() {
            return Err(AccessError::AlreadyHeld);
        }

        let phase = lock_phase()?;
        THREAD_HOLDS_LIBRARY.set(true);

        Ok(Self {
            phase,
            _thread_bound: PhantomData,
        })
    }
}

impl Drop for PhaseGuard {
    fn drop(&mut self) {
        THREAD_HOLDS_LIBRARY.set(false);
    }
}

/// Library access in whatever state library is in right now.
#[derive(Debug)]
#[expect(clippy::exhaustive_enums)]
pub enum Access {
    /// Initialization pending (or failed earlier and can be retried).
    Uninitialized(Library<Uninitialized>),
    /// Library is ready for use.
    Initialized(Library<Initialized>),
}

impl Access {
    fn claim(guard: PhaseGuard) -> Result<Self, AccessError> {
        Ok(match *guard.phase {
            Phase::Terminated => return Err(AccessError::Terminated),
            Phase::Uninitialized => Self::Uninitialized(Library::new(guard)),
            Phase::Initialized => Self::Initialized(Library::new(guard)),
        })
    }
}

/// Represents cases where library access is not possible.
#[derive(ThisError, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AccessError {
    /// Another thread holds library access.
    #[error("library access is held by another thread")]
    WouldBlock,
    /// This thread holds library access already.
    #[error("this thread already holds library access")]
    AlreadyHeld,
    /// Library has been terminated.
    #[error("library was terminated")]
    Terminated,
    /// Library is not initialized yet.
    #[error("library is not initialized")]
    NotInitialized,
    /// Library is initialized already.
    #[error("library is already initialized")]
    AlreadyInitialized,
}

/// Blocks until this thread has exclusive library access.
///
/// Phase of the library persists between handles and threads.
///
/// # Errors
/// Returns [`AccessError::AlreadyHeld`] when calling thread holds handle already.
/// Returns [`AccessError::Terminated`] if the library has already been terminated.
pub fn acquire() -> Result<Access, AccessError> {
    Access::claim(PhaseGuard::lock()?)
}

/// Like [`acquire`], but never blocks.
///
/// # Errors
/// Returns [`AccessError::WouldBlock`] when another thread holds handle.
/// Returns [`AccessError::AlreadyHeld`] when calling thread holds handle already.
/// Returns [`AccessError::Terminated`] if the library has already been terminated.
pub fn try_acquire() -> Result<Access, AccessError> {
    Access::claim(PhaseGuard::try_lock()?)
}

/// Library is not initialized yet. Configure it with [`Library<Uninitialized>::configure`] and
/// initialize with [`Library<Uninitialized>::initialize`] (or sibling methods).
#[derive(Debug)]
#[non_exhaustive]
pub struct Uninitialized;

/// Library is initialized. Containers and signers can be used.
#[derive(Debug)]
#[non_exhaustive]
pub struct Initialized;

/// Exclusive access to libdigidocpp.
///
/// At most one handle library exists in whole process. Handle is `!Send` and `!Sync`, so it stays
/// on thread that acquired it.
///
/// Use [`acquire`] or [`try_acquire`] to get handle in whatever phase library is in. When phase is
/// known, [`Library<Uninitialized>::acquire`] and [`Library<Initialized>::acquire`] (and their
/// `try_acquire` siblings) return handle of that phase only. [`acquire`] blocks until previous
/// handle is dropped, [`try_acquire`] fails instead. Do not hold handle across `.await`. Async code
/// should call [`acquire`] inside `spawn_blocking`-style closure and do all library work there.
pub struct Library<State> {
    guard: PhaseGuard,
    _state: PhantomData<State>,
}

impl<State> Library<State> {
    const fn new(guard: PhaseGuard) -> Self {
        Self {
            guard,
            _state: PhantomData,
        }
    }

    fn claim(
        guard: PhaseGuard,
        expected: Phase,
        mismatch: AccessError,
    ) -> Result<Self, AccessError> {
        match *guard.phase {
            Phase::Terminated => Err(AccessError::Terminated),
            phase if phase == expected => Ok(Self::new(guard)),
            _ => Err(mismatch),
        }
    }

    /// Returns the libdigidocpp version.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn version(&self) -> Result<String, Error> {
        Ok(ffi::version()?)
    }

    /// Returns application name given to `initialize*`.
    ///
    /// Defaults to `"libdigidocpp"`. libdigidocpp writes this into comments of containers it
    /// saves.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn app_info(&self) -> Result<String, Error> {
        Ok(ffi::app_info()?)
    }

    /// Returns user agent sent to online services (OCSP, time-stamping).
    ///
    /// Set it with [`Library<Uninitialized>::initialize_with_user_agent`]. Defaults to `"libdigidocpp"`.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn user_agent(&self) -> Result<String, Error> {
        Ok(ffi::user_agent()?)
    }
}

impl Library<Uninitialized> {
    /// Blocks until this thread has exclusive access, requires library to be uninitialized.
    ///
    /// # Errors
    /// Returns [`AccessError::AlreadyInitialized`] when library is initalized.
    /// Returns [`AccessError::AlreadyHeld`] when calling thread holds handle already.
    /// Returns [`AccessError::Terminated`] after [`Library<Initialized>::terminate`].
    pub fn acquire() -> Result<Self, AccessError> {
        Self::claim(
            PhaseGuard::lock()?,
            Phase::Uninitialized,
            AccessError::AlreadyInitialized,
        )
    }

    /// Like [`Library<Uninitialized>::acquire`], but never blocks.
    ///
    /// # Errors
    /// Returns [`AccessError::WouldBlock`] when another thread holds handle.
    /// Other errors are same as for [`Library<Uninitialized>::acquire`].
    pub fn try_acquire() -> Result<Self, AccessError> {
        Self::claim(
            PhaseGuard::try_lock()?,
            Phase::Uninitialized,
            AccessError::AlreadyInitialized,
        )
    }

    /// Installs global configuration and drops previous one.
    ///
    /// Can be called until the library is initialized. Configuration takes effect on
    /// initialization. When it is never called, upstream installs its default (same as
    /// [`Conf::new`]) on initialization.
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn configure(&mut self, conf: Conf) -> Result<(), Error> {
        conf.install()
    }

    /// Initializes libraries and loads the TSL.
    ///
    /// On error library stays [`Uninitialized`] and handle is released. Call [`acquire`] again to
    /// retry, optionally after [`Library<Uninitialized>::configure`] with a fixed configuration.
    /// # Errors
    #[doc = library_variant_doc!()]
    /// For example, libdigidocpp fails when a dependent library does not initialize or the TSL
    /// cannot be loaded (network failure, expired list or untrusted signature).
    #[doc = ffi_variant_doc!()]
    pub fn initialize(mut self) -> Result<Library<Initialized>, Error> {
        ffi::initialize_default()?;
        *self.guard.phase = Phase::Initialized;

        Ok(Library::new(self.guard))
    }

    /// Initializes libraries and loads the TSL, saving `app_info` as application name.
    ///
    /// `app_info` is written into container comments. See [`Library<Uninitialized>::initialize`].
    ///
    /// # Errors
    /// Same as [`Library<Uninitialized>::initialize`].
    pub fn initialize_with_app_info(
        mut self,
        app_info: &str,
    ) -> Result<Library<Initialized>, Error> {
        ffi::initialize(app_info)?;
        *self.guard.phase = Phase::Initialized;

        Ok(Library::new(self.guard))
    }

    /// Initializes libraries and loads the TSL, saving `app_info` as application name and
    /// `user_agent` as user agent.
    ///
    /// `app_info` is written into container comments. `user_agent` is sent to online services.
    /// See [`Library<Uninitialized>::initialize`].
    ///
    /// # Errors
    /// Same as [`Library<Uninitialized>::initialize`].
    pub fn initialize_with_user_agent(
        mut self,
        app_info: &str,
        user_agent: &str,
    ) -> Result<Library<Initialized>, Error> {
        ffi::initialize_with_user_agent(app_info, user_agent)?;
        *self.guard.phase = Phase::Initialized;

        Ok(Library::new(self.guard))
    }
}

impl Library<Initialized> {
    /// Blocks until this thread has exclusive access, requires library to be initialized.
    ///
    /// # Errors
    /// Returns [`AccessError::NotInitialized`] when library is not initialized.
    /// Returns [`AccessError::AlreadyHeld`] when calling thread holds handle already.
    /// Returns [`AccessError::Terminated`] after [`Library<Initialized>::terminate`].
    pub fn acquire() -> Result<Self, AccessError> {
        Self::claim(
            PhaseGuard::lock()?,
            Phase::Initialized,
            AccessError::NotInitialized,
        )
    }

    /// Like [`Library<Initialized>::acquire`], but never blocks.
    ///
    /// # Errors
    /// Returns [`AccessError::WouldBlock`] when another thread holds handle.
    /// Other errors are same as for [`Library<Initialized>::acquire`].
    pub fn try_acquire() -> Result<Self, AccessError> {
        Self::claim(
            PhaseGuard::try_lock()?,
            Phase::Initialized,
            AccessError::NotInitialized,
        )
    }

    /// Deletes installed configuration and shuts libraries down.
    ///
    /// Termination is permanent for whole process, on every thread. Calling [`acquire`] or
    /// [`try_acquire`] after this returns [`AccessError::Terminated`].
    ///
    /// # Errors
    #[doc = library_and_ffi_variants_doc!()]
    pub fn terminate(mut self) -> Result<(), Error> {
        // SAFETY: This is guaranteed to run only once per process.
        let result = unsafe { ffi::terminate() };
        *self.guard.phase = Phase::Terminated;

        Ok(result?)
    }
}

impl<State> Debug for Library<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Library")
            .field("phase", &*self.guard.phase)
            .finish_non_exhaustive()
    }
}
