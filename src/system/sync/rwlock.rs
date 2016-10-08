use super::{Condvar, Mutex, LockResult};
use std::cell::UnsafeCell;
use std::marker;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Reading(usize),
    Writing,
}

use self::State::*;

pub struct RwLock<T: ?Sized> {
    state: Mutex<State>,
    condvar: Condvar,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send + Sync> Send for RwLock<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

#[must_use]
pub struct RwLockReadGuard<'a, T: ?Sized + 'a> {
    lock: &'a RwLock<T>,
}

impl<'a, T: ?Sized> !marker::Send for RwLockReadGuard<'a, T> {}

#[must_use]
pub struct RwLockWriteGuard<'a, T: ?Sized + 'a> {
    lock: &'a RwLock<T>,
}

impl<'a, T: ?Sized> !marker::Send for RwLockWriteGuard<'a, T> {}

impl<T> RwLock<T> {
    pub fn new(t: T) -> RwLock<T> {
        RwLock {
            state: Mutex::new(Reading(0)),
            condvar: Condvar::new(),
            data: UnsafeCell::new(t),
        }
    }
}

impl<T: ?Sized> RwLock<T> {
    pub fn read(&self) -> RwLockReadGuard<T> {
        let mut state = self.state.lock();
        loop {
            if let Reading(ref mut x) = *state {
                *x += 1;
                return RwLockReadGuard::new(self);
            }
            state = self.condvar.wait(state);
        }
    }

    pub fn try_read(&self) -> LockResult<RwLockReadGuard<T>> {
        let mut state = self.state.lock();
        if let Reading(ref mut x) = *state {
            *x += 1;
            Ok(RwLockReadGuard::new(self))
        } else {
            Err(())
        }
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        let mut state = self.state.lock();
        loop {
            if *state == Reading(0) {
                *state = Writing;
                return RwLockWriteGuard::new(self);
            }
            state = self.condvar.wait(state);
        }
    }

    pub fn try_write(&self) -> LockResult<RwLockWriteGuard<T>> {
        let mut state = self.state.lock();
        if *state == Reading(0) {
            *state = Writing;
            Ok(RwLockWriteGuard::new(self))
        } else {
            Err(())
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}

impl<T: Default> Default for RwLock<T> {
    fn default() -> RwLock<T> {
        RwLock::new(Default::default())
    }
}

impl<'rwlock, T: ?Sized> RwLockReadGuard<'rwlock, T> {
    fn new(lock: &'rwlock RwLock<T>) -> Self {
        RwLockReadGuard { lock: lock }
    }
}

impl<'rwlock, T: ?Sized> RwLockWriteGuard<'rwlock, T> {
    fn new(lock: &'rwlock RwLock<T>) -> Self {
        RwLockWriteGuard { lock: lock }
    }
}

impl<'rwlock, T: ?Sized> Deref for RwLockReadGuard<'rwlock, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'rwlock, T: ?Sized> Deref for RwLockWriteGuard<'rwlock, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'rwlock, T: ?Sized> DerefMut for RwLockWriteGuard<'rwlock, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for RwLockReadGuard<'a, T> {
    fn drop(&mut self) {
        let new_count;
        {
            let mut state = self.lock.state.lock();
            if let Reading(ref mut x) = *state {
                new_count = *x - 1;
                *x = new_count;
            } else {
                panic!("Writer is active while reading.");
            }
        }
        if new_count == 0 {
            // Notify a potential writer.
            self.lock.condvar.notify();
        }
    }
}

impl<'a, T: ?Sized> Drop for RwLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        {
            let mut state = self.lock.state.lock();
            if *state == Writing {
                *state = Reading(0);
            } else {
                panic!("Readers are active while writing.");
            }
        }
        self.lock.condvar.notify();
    }
}