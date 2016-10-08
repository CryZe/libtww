use system::OS;

mod manager;
pub mod scoped;

// #[macro_use]mod local;
// pub use self::local::{LocalKey, LocalKeyState};

/// Cooperatively gives up a timeslice to the OS scheduler.
pub fn yield_now() {
    OS::yield_thread()
}

/// Blocks unless or until the current thread's token is made available.
///
/// Every thread is equipped with some basic low-level blocking support, via
/// the `park()` function and the [`unpark()`][unpark] method. These can be
/// used as a more CPU-efficient implementation of a spinlock.
///
/// [unpark]: struct.Thread.html#method.unpark
///
/// The API is typically used by acquiring a handle to the current thread,
/// placing that handle in a shared data structure so that other threads can
/// find it, and then parking (in a loop with a check for the token actually
/// being acquired).
///
/// A call to `park` does not guarantee that the thread will remain parked
/// forever, and callers should be prepared for this possibility.
///
/// See the [module documentation][thread] for more detail.
///
/// [thread]: index.html
// The implementation currently uses the trivial strategy of a Mutex+Condvar
// with wakeup flag, which does not actually allow spurious wakeups. In the
// future, this will be implemented in a more efficient way, perhaps along the lines of
//   http://cr.openjdk.java.net/~stefank/6989984.1/raw_files/new/src/os/linux/vm/os_linux.cpp
// or futuxes, and in either case may allow spurious wakeups.
pub fn park() {
    OS::suspend_thread(OS::get_current_thread());
}

pub use self::manager::{collect, current, JoinHandle, Thread, spawn};