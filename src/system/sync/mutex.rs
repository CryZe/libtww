use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use system::OS;
use super::LockResult;

pub struct Mutex<T: ?Sized> {
    mutex: Box<[u8]>,
    data: UnsafeCell<T>,
}

#[must_use]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    mutex: &'a Mutex<T>,
}

pub fn guard_lock<'a, T: ?Sized + 'a>(guard: &'a MutexGuard<'a, T>) -> &'a [u8] {
    &guard.mutex.mutex
}

impl<'a, T: ?Sized> !Send for MutexGuard<'a, T> {}

impl<'mutex, T: ?Sized> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'mutex, T: ?Sized> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        OS::unlock_mutex(self.mutex.mutex.as_ptr());
    }
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
        let mut mutex = OS::allocate_mutex();
        OS::init_mutex(mutex.as_mut_ptr());

        Mutex {
            mutex: mutex,
            data: UnsafeCell::new(t),
        }
    }

    pub fn into_inner(self) -> T {
        unsafe { self.data.into_inner() }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<T> {
        OS::lock_mutex(self.mutex.as_ptr());
        MutexGuard { mutex: self }
    }

    pub fn try_lock(&self) -> LockResult<MutexGuard<T>> {
        let locked = OS::try_lock_mutex(self.mutex.as_ptr());
        if locked {
            Ok(MutexGuard { mutex: self })
        } else {
            Err(())
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}