pub mod mpsc;
mod barrier;
mod condvar;
mod mutex;
mod once;
mod rwlock;

pub use self::barrier::{Barrier, BarrierWaitResult};
pub use self::condvar::Condvar;
pub use self::mutex::{Mutex, MutexGuard};
pub use self::once::{Once, OnceState, ONCE_INIT};
pub use self::rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use core::sync::atomic;
pub use alloc::arc::*;

pub type LockResult<T> = ::std::result::Result<T, ()>;