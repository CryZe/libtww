use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mutex, MutexGuard};
use system::OS;

pub struct Condvar {
    inner: Box<[u8]>,
    mutex: AtomicUsize,
}

impl Condvar {
    pub fn new() -> Condvar {
        let mut cond = OS::allocate_cond();
        OS::init_cond(cond.as_mut_ptr());

        Condvar {
            inner: cond,
            mutex: AtomicUsize::new(0),
        }
    }

    pub fn notify(&self) {
        OS::signal_cond(self.inner.as_ptr());
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        {
            let lock = mutex::guard_lock(&guard);
            self.verify(lock);
            OS::wait_cond(self.inner.as_ptr(), lock.as_ptr());
        }
        guard
    }

    fn verify(&self, mutex: &[u8]) {
        let addr = mutex.as_ptr() as usize;
        match self.mutex.compare_and_swap(0, addr, Ordering::SeqCst) {
            // If we got out 0, then we have successfully bound the mutex to
            // this cvar.
            0 => {}

            // If we get out a value that's the same as `addr`, then someone
            // already beat us to the punch.
            n if n == addr => {}

            // Anything else and we're using more than one mutex on this cvar,
            // which is currently disallowed.
            _ => panic!("attempted to use a condition variable with two mutexes"),
        }
    }
}

impl Default for Condvar {
    fn default() -> Condvar {
        Condvar::new()
    }
}
