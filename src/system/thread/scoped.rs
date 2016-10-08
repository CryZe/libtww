use std::marker::PhantomData;
use system::OS;
use std::mem;
use alloc::boxed::FnBox;

struct Handle {
    thread: Box<[u8]>,
    _stack: Box<[u8]>,
}

#[must_use = "thread will be immediately joined if `JoinGuard` is not used"]
pub struct JoinGuard<'a, T: Send + 'a> {
    handle: Option<Handle>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Send + 'a> JoinGuard<'a, T> {
    pub fn join(mut self) -> T {
        let handle = self.handle.take().unwrap();
        let mut ret: *mut u8 = unsafe { mem::uninitialized() };
        OS::join_thread(handle.thread.as_ptr(), &mut ret);
        let ret = ret as *mut T;
        let ret = unsafe { Box::from_raw(ret) };
        *ret
    }
}

impl<'a, T: Send + 'a> Drop for JoinGuard<'a, T> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            let mut ret: *mut u8 = unsafe { mem::uninitialized() };
            OS::join_thread(handle.thread.as_ptr(), &mut ret);
        }
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

pub fn spawn<'a, T, F>(f: F) -> JoinGuard<'a, T>
    where T: Send + 'a,
          F: FnOnce() -> T,
          F: Send + 'a
{
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

    OS::resume_thread(thread.as_ptr());

    JoinGuard {
        handle: Some(Handle {
            thread: thread,
            _stack: stack,
        }),
        _marker: PhantomData,
    }
}