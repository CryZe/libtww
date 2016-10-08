use std::sync::{Arc, Weak, Mutex, MutexGuard, Once, ONCE_INIT};
use std::{ptr, mem};
use std::collections::HashMap;
use std::marker::PhantomData;
use system::OS;
use alloc::boxed::FnBox;

static mut MANAGER: *mut Mutex<Manager> = ptr::null_mut();
static START: Once = ONCE_INIT;

struct ThreadData {
    object: Box<[u8]>,
    _stack: Box<[u8]>,
}

struct Manager {
    threads: HashMap<*const u8, Arc<ThreadData>>,
}

impl Manager {
    fn collect(&mut self) {
        let dead_threads = self.threads
            .keys()
            .cloned()
            .filter(|&t| OS::is_thread_terminated(t))
            .collect::<Vec<_>>();

        for thread in dead_threads {
            self.threads.remove(&thread);
        }
    }
}

fn get() -> MutexGuard<'static, Manager> {
    START.call_once(|| {
        let manager = Manager { threads: HashMap::new() };
        let manager = Mutex::new(manager);
        let manager = Box::new(manager);
        unsafe {
            MANAGER = Box::into_raw(manager);
        }
    });

    let manager = unsafe { &*MANAGER };
    let mut manager = manager.lock();

    manager.collect();

    manager
}

#[derive(Clone)]
pub struct Thread {
    thread: Weak<ThreadData>,
}

impl Thread {
    pub fn unpark(&self) {
        if let Some(thread) = self.thread.upgrade() {
            OS::resume_thread(thread.object.as_ptr());
        }
    }
}

pub struct JoinHandle<T> {
    thread: Thread,
    handle: Arc<ThreadData>,
    _phantom_data: PhantomData<T>,
}

impl<T> JoinHandle<T> {
    pub fn thread(&self) -> &Thread {
        &self.thread
    }

    pub fn join(self) -> T {
        let handle = self.handle;
        let ptr = handle.object.as_ptr();

        let mut ret: *mut u8 = unsafe { mem::uninitialized() };
        OS::join_thread(ptr, &mut ret);
        let ret = ret as *mut T;
        let ret = unsafe { Box::from_raw(ret) };

        drop(handle);
        collect();

        *ret
    }
}

struct WrappedFn<'a, T: 'a>(Box<FnBox() -> T + 'a>);

extern "C" fn run_thread<T>(function: *mut u8) -> *mut u8 {
    // The parameter is the FnOnce representing the thread.
    // It's behind a thin pointer pointing to a fat pointer.
    let function = function as *mut WrappedFn<T>;
    let function = unsafe { Box::from_raw(function) };

    // Let's start the thread.
    let ret = function.0();

    // The return value needs to be boxed, cause we need
    // to return a pointer sized type.
    let ret = Box::new(ret);
    let ret = Box::into_raw(ret);
    ret as *mut u8
}

pub fn collect() {
    // Just get access to the manager,
    // it'll automatically clean up everything.
    let _manager = get();
}

pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where F: FnOnce() -> T,
          F: Send + 'static,
          T: Send + 'static
{
    let mut manager = get();

    let mut thread = OS::allocate_thread();
    let mut stack = vec![0xCE; 128 * 1024].into_boxed_slice();

    let f = WrappedFn(Box::new(f));
    let f = Box::into_raw(Box::new(f));

    OS::create_thread(thread.as_mut_ptr(),
                      run_thread::<T>,
                      f as *mut u8,
                      unsafe { stack.as_mut_ptr().offset(stack.len() as isize) },
                      stack.len(),
                      0,
                      0);

    let ptr = thread.as_ptr();

    manager.threads.insert(ptr,
                           Arc::new(ThreadData {
                               object: thread,
                               _stack: stack,
                           }));

    let handle = manager.threads[&ptr].clone();
    let thread = Thread { thread: Arc::downgrade(&handle) };

    thread.unpark();

    JoinHandle {
        thread: thread,
        handle: handle,
        _phantom_data: PhantomData,
    }
}

pub fn current() -> Thread {
    // TODO Thread might not be handled by our manager
    // Solution is probably to turn ThreadData into an enum
    // that it can either be an unowned ptr or an Arc'ed ThreadData.
    Thread {
        thread: Arc::downgrade(&get()
            .threads
            .get(&OS::get_current_thread())
            .expect("Thread is not handled by the Thread Manager")),
    }
}