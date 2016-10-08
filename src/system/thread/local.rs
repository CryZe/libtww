// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Thread local storage

#![unstable(feature = "thread_local_internals", issue = "0")]

use std::cell::UnsafeCell;
use std::mem;

/// A thread local storage key which owns its contents.
///
/// This key uses the fastest possible implementation available to it for the
/// target platform. It is instantiated with the `thread_local!` macro and the
/// primary method is the `with` method.
///
/// The `with` method yields a reference to the contained value which cannot be
/// sent across threads or escape the given closure.
///
/// # Initialization and Destruction
///
/// Initialization is dynamically performed on the first call to `with()`
/// within a thread, and values support destructors which will be run when a
/// thread exits.
///
/// # Examples
///
/// ```
/// use std::cell::RefCell;
/// use std::thread;
///
/// thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
///
/// FOO.with(|f| {
///     assert_eq!(*f.borrow(), 1);
///     *f.borrow_mut() = 2;
/// });
///
/// // each thread starts out with the initial value of 1
/// thread::spawn(move|| {
///     FOO.with(|f| {
///         assert_eq!(*f.borrow(), 1);
///         *f.borrow_mut() = 3;
///     });
/// });
///
/// // we retain our original value of 2 despite the child thread
/// FOO.with(|f| {
///     assert_eq!(*f.borrow(), 2);
/// });
/// ```
///
/// # Platform-specific behavior
///
/// Note that a "best effort" is made to ensure that destructors for types
/// stored in thread local storage are run, but not all platforms can guarantee
/// that destructors will be run for all types in thread local storage. For
/// example, there are a number of known caveats where destructors are not run:
///
/// 1. On Unix systems when pthread-based TLS is being used, destructors will
///    not be run for TLS values on the main thread when it exits. Note that the
///    application will exit immediately after the main thread exits as well.
/// 2. On all platforms it's possible for TLS to re-initialize other TLS slots
///    during destruction. Some platforms ensure that this cannot happen
///    infinitely by preventing re-initialization of any slot that has been
///    destroyed, but not all platforms have this guard. Those platforms that do
///    not guard typically have a synthetic limit after which point no more
///    destructors are run.
/// 3. On OSX, initializing TLS during destruction of other TLS slots can
///    sometimes cancel *all* destructors for the current thread, whether or not
///    the slots have already had their destructors run or not.
#[stable(feature = "rust1", since = "1.0.0")]
pub struct LocalKey<T: 'static> {
    // This outer `LocalKey<T>` type is what's going to be stored in statics,
    // but actual data inside will sometimes be tagged with #[thread_local].
    // It's not valid for a true static to reference a #[thread_local] static,
    // so we get around that by exposing an accessor through a layer of function
    // indirection (this thunk).
    //
    // Note that the thunk is itself unsafe because the returned lifetime of the
    // slot where data lives, `'static`, is not actually valid. The lifetime
    // here is actually `'thread`!
    //
    // Although this is an extra layer of indirection, it should in theory be
    // trivially devirtualizable by LLVM because the value of `inner` never
    // changes and the constant should be readonly within a crate. This mainly
    // only runs into problems when TLS statics are exported across crates.
    inner: fn() -> Option<&'static UnsafeCell<Option<T>>>,

    // initialization routine to invoke to create a value
    init: fn() -> T,
}

/// Declare a new thread local storage key of type `std::thread::LocalKey`.
///
/// # Syntax
///
/// The macro wraps any number of static declarations and makes them thread local.
/// Each static may be public or private, and attributes are allowed. Example:
///
/// ```
/// use std::cell::RefCell;
/// thread_local! {
///     pub static FOO: RefCell<u32> = RefCell::new(1);
///
///     #[allow(unused)]
///     static BAR: RefCell<f32> = RefCell::new(1.0);
/// }
/// # fn main() {}
/// ```
///
/// See [LocalKey documentation](thread/struct.LocalKey.html) for more
/// information.
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable]
macro_rules! thread_local {
    // rule 0: empty (base case for the recursion)
    () => {};

    // rule 1: process multiple declarations where the first one is private
    ($(#[$attr:meta])* static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => (
        thread_local!($(#[$attr])* static $name: $t = $init); // go to rule 2
        thread_local!($($rest)*);
    );

    // rule 2: handle a single private declaration
    ($(#[$attr:meta])* static $name:ident: $t:ty = $init:expr) => (
        $(#[$attr])* static $name: $crate::thread::LocalKey<$t> =
            __thread_local_inner!($t, $init);
    );

    // rule 3: handle multiple declarations where the first one is public
    ($(#[$attr:meta])* pub static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => (
        thread_local!($(#[$attr])* pub static $name: $t = $init); // go to rule 4
        thread_local!($($rest)*);
    );

    // rule 4: handle a single public declaration
    ($(#[$attr:meta])* pub static $name:ident: $t:ty = $init:expr) => (
        $(#[$attr])* pub static $name: $crate::thread::LocalKey<$t> =
            __thread_local_inner!($t, $init);
    );
}

#[doc(hidden)]
#[unstable(feature = "thread_local_internals",
           reason = "should not be necessary",
           issue = "0")]
#[macro_export]
#[allow_internal_unstable]
macro_rules! __thread_local_inner {
    ($t:ty, $init:expr) => {{
        fn __init() -> $t { $init }

        fn __getit() -> $crate::option::Option<
            &'static $crate::cell::UnsafeCell<
                $crate::option::Option<$t>>>
        {
            #[thread_local]
            #[cfg(target_thread_local)]
            static __KEY: $crate::thread::__ElfLocalKeyInner<$t> =
                $crate::thread::__ElfLocalKeyInner::new();

            #[cfg(not(target_thread_local))]
            static __KEY: $crate::thread::__OsLocalKeyInner<$t> =
                $crate::thread::__OsLocalKeyInner::new();

            __KEY.get()
        }

        $crate::thread::LocalKey::new(__getit, __init)
    }}
}

/// Indicator of the state of a thread local storage key.
#[unstable(feature = "thread_local_state",
           reason = "state querying was recently added",
           issue = "27716")]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum LocalKeyState {
    /// All keys are in this state whenever a thread starts. Keys will
    /// transition to the `Valid` state once the first call to `with` happens
    /// and the initialization expression succeeds.
    ///
    /// Keys in the `Uninitialized` state will yield a reference to the closure
    /// passed to `with` so long as the initialization routine does not panic.
    Uninitialized,

    /// Once a key has been accessed successfully, it will enter the `Valid`
    /// state. Keys in the `Valid` state will remain so until the thread exits,
    /// at which point the destructor will be run and the key will enter the
    /// `Destroyed` state.
    ///
    /// Keys in the `Valid` state will be guaranteed to yield a reference to the
    /// closure passed to `with`.
    Valid,

    /// When a thread exits, the destructors for keys will be run (if
    /// necessary). While a destructor is running, and possibly after a
    /// destructor has run, a key is in the `Destroyed` state.
    ///
    /// Keys in the `Destroyed` states will trigger a panic when accessed via
    /// `with`.
    Destroyed,
}

impl<T: 'static> LocalKey<T> {
    #[doc(hidden)]
    #[unstable(feature = "thread_local_internals",
               reason = "recently added to create a key",
               issue = "0")]
    pub const fn new(inner: fn() -> Option<&'static UnsafeCell<Option<T>>>,
                     init: fn() -> T)
                     -> LocalKey<T> {
        LocalKey {
            inner: inner,
            init: init,
        }
    }

    /// Acquires a reference to the value in this TLS key.
    ///
    /// This will lazily initialize the value if this thread has not referenced
    /// this key yet.
    ///
    /// # Panics
    ///
    /// This function will `panic!()` if the key currently has its
    /// destructor running, and it **may** panic if the destructor has
    /// previously been run for this thread.
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn with<F, R>(&'static self, f: F) -> R
        where F: FnOnce(&T) -> R
    {
        unsafe {
            let slot = (self.inner)();
            let slot = slot.expect("cannot access a TLS value during or \
                                    after it is destroyed");
            f(match *slot.get() {
                Some(ref inner) => inner,
                None => self.init(slot),
            })
        }
    }

    unsafe fn init(&self, slot: &UnsafeCell<Option<T>>) -> &T {
        // Execute the initialization up front, *then* move it into our slot,
        // just in case initialization fails.
        let value = (self.init)();
        let ptr = slot.get();

        // note that this can in theory just be `*ptr = Some(value)`, but due to
        // the compiler will currently codegen that pattern with something like:
        //
        //      ptr::drop_in_place(ptr)
        //      ptr::write(ptr, Some(value))
        //
        // Due to this pattern it's possible for the destructor of the value in
        // `ptr` (e.g. if this is being recursively initialized) to re-access
        // TLS, in which case there will be a `&` and `&mut` pointer to the same
        // value (an aliasing violation). To avoid setting the "I'm running a
        // destructor" flag we just use `mem::replace` which should sequence the
        // operations a little differently and make this safe to call.
        mem::replace(&mut *ptr, Some(value));

        (*ptr).as_ref().unwrap()
    }

    /// Query the current state of this key.
    ///
    /// A key is initially in the `Uninitialized` state whenever a thread
    /// starts. It will remain in this state up until the first call to `with`
    /// within a thread has run the initialization expression successfully.
    ///
    /// Once the initialization expression succeeds, the key transitions to the
    /// `Valid` state which will guarantee that future calls to `with` will
    /// succeed within the thread.
    ///
    /// When a thread exits, each key will be destroyed in turn, and as keys are
    /// destroyed they will enter the `Destroyed` state just before the
    /// destructor starts to run. Keys may remain in the `Destroyed` state after
    /// destruction has completed. Keys without destructors (e.g. with types
    /// that are `Copy`), may never enter the `Destroyed` state.
    ///
    /// Keys in the `Uninitialized` state can be accessed so long as the
    /// initialization does not panic. Keys in the `Valid` state are guaranteed
    /// to be able to be accessed. Keys in the `Destroyed` state will panic on
    /// any call to `with`.
    #[unstable(feature = "thread_local_state",
               reason = "state querying was recently added",
               issue = "27716")]
    pub fn state(&'static self) -> LocalKeyState {
        unsafe {
            match (self.inner)() {
                Some(cell) => {
                    match *cell.get() {
                        Some(..) => LocalKeyState::Valid,
                        None => LocalKeyState::Uninitialized,
                    }
                }
                None => LocalKeyState::Destroyed,
            }
        }
    }
}

#[cfg(target_thread_local)]
#[doc(hidden)]
pub mod elf {
    use cell::{Cell, UnsafeCell};
    use intrinsics;
    use ptr;

    pub struct Key<T> {
        inner: UnsafeCell<Option<T>>,

        // Metadata to keep track of the state of the destructor. Remember that
        // these variables are thread-local, not global.
        dtor_registered: Cell<bool>,
        dtor_running: Cell<bool>,
    }

    unsafe impl<T> ::marker::Sync for Key<T> {}

    impl<T> Key<T> {
        pub const fn new() -> Key<T> {
            Key {
                inner: UnsafeCell::new(None),
                dtor_registered: Cell::new(false),
                dtor_running: Cell::new(false),
            }
        }

        pub fn get(&'static self) -> Option<&'static UnsafeCell<Option<T>>> {
            unsafe {
                if intrinsics::needs_drop::<T>() && self.dtor_running.get() {
                    return None;
                }
                self.register_dtor();
            }
            Some(&self.inner)
        }

        unsafe fn register_dtor(&self) {
            if !intrinsics::needs_drop::<T>() || self.dtor_registered.get() {
                return;
            }

            register_dtor(self as *const _ as *mut u8, destroy_value::<T>);
            self.dtor_registered.set(true);
        }
    }

    // Since what appears to be glibc 2.18 this symbol has been shipped which
    // GCC and clang both use to invoke destructors in thread_local globals, so
    // let's do the same!
    //
    // Note, however, that we run on lots older linuxes, as well as cross
    // compiling from a newer linux to an older linux, so we also have a
    // fallback implementation to use as well.
    //
    // Due to rust-lang/rust#18804, make sure this is not generic!
    #[cfg(target_os = "linux")]
    unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
        use mem;
        use libc;
        use sys_common::thread_local as os;

        extern "C" {
            #[linkage = "extern_weak"]
            static __dso_handle: *mut u8;
            #[linkage = "extern_weak"]
            static __cxa_thread_atexit_impl: *const libc::c_void;
        }
        if !__cxa_thread_atexit_impl.is_null() {
            type F = unsafe extern "C" fn(dtor: unsafe extern "C" fn(*mut u8),
                                          arg: *mut u8,
                                          dso_handle: *mut u8)
                                          -> libc::c_int;
            mem::transmute::<*const libc::c_void,
                             F>(__cxa_thread_atexit_impl)(dtor,
                                                          t,
                                                          &__dso_handle as *const _ as *mut _);
            return;
        }

        // The fallback implementation uses a vanilla OS-based TLS key to track
        // the list of destructors that need to be run for this thread. The key
        // then has its own destructor which runs all the other destructors.
        //
        // The destructor for DTORS is a little special in that it has a `while`
        // loop to continuously drain the list of registered destructors. It
        // *should* be the case that this loop always terminates because we
        // provide the guarantee that a TLS key cannot be set after it is
        // flagged for destruction.
        static DTORS: os::StaticKey = os::StaticKey::new(Some(run_dtors));
        type List = Vec<(*mut u8, unsafe extern "C" fn(*mut u8))>;
        if DTORS.get().is_null() {
            let v: Box<List> = box Vec::new();
            DTORS.set(Box::into_raw(v) as *mut u8);
        }
        let list: &mut List = &mut *(DTORS.get() as *mut List);
        list.push((t, dtor));

        unsafe extern "C" fn run_dtors(mut ptr: *mut u8) {
            while !ptr.is_null() {
                let list: Box<List> = Box::from_raw(ptr as *mut List);
                for &(ptr, dtor) in list.iter() {
                    dtor(ptr);
                }
                ptr = DTORS.get();
                DTORS.set(ptr::null_mut());
            }
        }
    }

    // OSX's analog of the above linux function is this _tlv_atexit function.
    // The disassembly of thread_local globals in C++ (at least produced by
    // clang) will have this show up in the output.
    #[cfg(target_os = "macos")]
    unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
        extern "C" {
            fn _tlv_atexit(dtor: unsafe extern "C" fn(*mut u8), arg: *mut u8);
        }
        _tlv_atexit(dtor, t);
    }

    pub unsafe extern "C" fn destroy_value<T>(ptr: *mut u8) {
        let ptr = ptr as *mut Key<T>;
        // Right before we run the user destructor be sure to flag the
        // destructor as running for this thread so calls to `get` will return
        // `None`.
        (*ptr).dtor_running.set(true);

        // The OSX implementation of TLS apparently had an odd aspect to it
        // where the pointer we have may be overwritten while this destructor
        // is running. Specifically if a TLS destructor re-accesses TLS it may
        // trigger a re-initialization of all TLS variables, paving over at
        // least some destroyed ones with initial values.
        //
        // This means that if we drop a TLS value in place on OSX that we could
        // revert the value to its original state halfway through the
        // destructor, which would be bad!
        //
        // Hence, we use `ptr::read` on OSX (to move to a "safe" location)
        // instead of drop_in_place.
        if cfg!(target_os = "macos") {
            ptr::read((*ptr).inner.get());
        } else {
            ptr::drop_in_place((*ptr).inner.get());
        }
    }
}

#[doc(hidden)]
pub mod os {
    use std::cell::{Cell, UnsafeCell};
    use std::marker;
    use std::ptr;
    use std::sys_common::thread_local::StaticKey as OsStaticKey;

    pub struct Key<T> {
        // OS-TLS key that we'll use to key off.
        os: OsStaticKey,
        marker: marker::PhantomData<Cell<T>>,
    }

    unsafe impl<T> std::marker::Sync for Key<T> {}

    struct Value<T: 'static> {
        key: &'static Key<T>,
        value: UnsafeCell<Option<T>>,
    }

    impl<T: 'static> Key<T> {
        pub const fn new() -> Key<T> {
            Key {
                os: OsStaticKey::new(Some(destroy_value::<T>)),
                marker: marker::PhantomData,
            }
        }

        pub fn get(&'static self) -> Option<&'static UnsafeCell<Option<T>>> {
            unsafe {
                let ptr = self.os.get() as *mut Value<T>;
                if !ptr.is_null() {
                    if ptr as usize == 1 {
                        return None;
                    }
                    return Some(&(*ptr).value);
                }

                // If the lookup returned null, we haven't initialized our own local
                // copy, so do that now.
                let ptr: Box<Value<T>> = box Value {
                    key: self,
                    value: UnsafeCell::new(None),
                };
                let ptr = Box::into_raw(ptr);
                self.os.set(ptr as *mut u8);
                Some(&(*ptr).value)
            }
        }
    }

    pub unsafe extern "C" fn destroy_value<T: 'static>(ptr: *mut u8) {
        // The OS TLS ensures that this key contains a NULL value when this
        // destructor starts to run. We set it back to a sentinel value of 1 to
        // ensure that any future calls to `get` for this thread will return
        // `None`.
        //
        // Note that to prevent an infinite loop we reset it back to null right
        // before we return from the destructor ourselves.
        let ptr = Box::from_raw(ptr as *mut Value<T>);
        let key = ptr.key;
        key.os.set(1 as *mut u8);
        drop(ptr);
        key.os.set(ptr::null_mut());
    }
}

#[cfg(all(test, not(target_os = "emscripten")))]
mod tests {
    use sync::mpsc::{channel, Sender};
    use cell::{Cell, UnsafeCell};
    use super::LocalKeyState;
    use thread;

    struct Foo(Sender<()>);

    impl Drop for Foo {
        fn drop(&mut self) {
            let Foo(ref s) = *self;
            s.send(()).unwrap();
        }
    }

    #[test]
    fn smoke_no_dtor() {
        thread_local!(static FOO: Cell<i32> = Cell::new(1));

        FOO.with(|f| {
            assert_eq!(f.get(), 1);
            f.set(2);
        });
        let (tx, rx) = channel();
        let _t = thread::spawn(move || {
            FOO.with(|f| {
                assert_eq!(f.get(), 1);
            });
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();

        FOO.with(|f| {
            assert_eq!(f.get(), 2);
        });
    }

    #[test]
    fn states() {
        struct Foo;
        impl Drop for Foo {
            fn drop(&mut self) {
                assert!(FOO.state() == LocalKeyState::Destroyed);
            }
        }
        fn foo() -> Foo {
            assert!(FOO.state() == LocalKeyState::Uninitialized);
            Foo
        }
        thread_local!(static FOO: Foo = foo());

        thread::spawn(|| {
                assert!(FOO.state() == LocalKeyState::Uninitialized);
                FOO.with(|_| {
                    assert!(FOO.state() == LocalKeyState::Valid);
                });
                assert!(FOO.state() == LocalKeyState::Valid);
            })
            .join()
            .ok()
            .unwrap();
    }

    #[test]
    fn smoke_dtor() {
        thread_local!(static FOO: UnsafeCell<Option<Foo>> = UnsafeCell::new(None));

        let (tx, rx) = channel();
        let _t = thread::spawn(move || unsafe {
            let mut tx = Some(tx);
            FOO.with(|f| {
                *f.get() = Some(Foo(tx.take().unwrap()));
            });
        });
        rx.recv().unwrap();
    }

    #[test]
    fn circular() {
        struct S1;
        struct S2;
        thread_local!(static K1: UnsafeCell<Option<S1>> = UnsafeCell::new(None));
        thread_local!(static K2: UnsafeCell<Option<S2>> = UnsafeCell::new(None));
        static mut HITS: u32 = 0;

        impl Drop for S1 {
            fn drop(&mut self) {
                unsafe {
                    HITS += 1;
                    if K2.state() == LocalKeyState::Destroyed {
                        assert_eq!(HITS, 3);
                    } else {
                        if HITS == 1 {
                            K2.with(|s| *s.get() = Some(S2));
                        } else {
                            assert_eq!(HITS, 3);
                        }
                    }
                }
            }
        }
        impl Drop for S2 {
            fn drop(&mut self) {
                unsafe {
                    HITS += 1;
                    assert!(K1.state() != LocalKeyState::Destroyed);
                    assert_eq!(HITS, 2);
                    K1.with(|s| *s.get() = Some(S1));
                }
            }
        }

        thread::spawn(move || {
                drop(S1);
            })
            .join()
            .ok()
            .unwrap();
    }

    #[test]
    fn self_referential() {
        struct S1;
        thread_local!(static K1: UnsafeCell<Option<S1>> = UnsafeCell::new(None));

        impl Drop for S1 {
            fn drop(&mut self) {
                assert!(K1.state() == LocalKeyState::Destroyed);
            }
        }

        thread::spawn(move || unsafe {
                K1.with(|s| *s.get() = Some(S1));
            })
            .join()
            .ok()
            .unwrap();
    }

    // Note that this test will deadlock if TLS destructors aren't run (this
    // requires the destructor to be run to pass the test). OSX has a known bug
    // where dtors-in-dtors may cancel other destructors, so we just ignore this
    // test on OSX.
    #[test]
    #[cfg_attr(target_os = "macos", ignore)]
    fn dtors_in_dtors_in_dtors() {
        struct S1(Sender<()>);
        thread_local!(static K1: UnsafeCell<Option<S1>> = UnsafeCell::new(None));
        thread_local!(static K2: UnsafeCell<Option<Foo>> = UnsafeCell::new(None));

        impl Drop for S1 {
            fn drop(&mut self) {
                let S1(ref tx) = *self;
                unsafe {
                    if K2.state() != LocalKeyState::Destroyed {
                        K2.with(|s| *s.get() = Some(Foo(tx.clone())));
                    }
                }
            }
        }

        let (tx, rx) = channel();
        let _t = thread::spawn(move || unsafe {
            let mut tx = Some(tx);
            K1.with(|s| *s.get() = Some(S1(tx.take().unwrap())));
        });
        rx.recv().unwrap();
    }
}

#[cfg(test)]
mod dynamic_tests {
    use cell::RefCell;
    use collections::HashMap;

    #[test]
    fn smoke() {
        fn square(i: i32) -> i32 {
            i * i
        }
        thread_local!(static FOO: i32 = square(3));

        FOO.with(|f| {
            assert_eq!(*f, 9);
        });
    }

    #[test]
    fn hashmap() {
        fn map() -> RefCell<HashMap<i32, i32>> {
            let mut m = HashMap::new();
            m.insert(1, 2);
            RefCell::new(m)
        }
        thread_local!(static FOO: RefCell<HashMap<i32, i32>> = map());

        FOO.with(|map| {
            assert_eq!(map.borrow()[&1], 2);
        });
    }

    #[test]
    fn refcell_vec() {
        thread_local!(static FOO: RefCell<Vec<u32>> = RefCell::new(vec![1, 2, 3]));

        FOO.with(|vec| {
            assert_eq!(vec.borrow().len(), 3);
            vec.borrow_mut().push(4);
            assert_eq!(vec.borrow()[3], 4);
        });
    }
}