use std::fmt;
use std::sync::{Once, ONCE_INIT, Mutex, MutexGuard};
use std::ptr;
use std::io::{self, Write};
use system::OS;

struct PrintBuffer(Vec<u8>);

static mut BUFFER: *mut Mutex<PrintBuffer> = ptr::null_mut();
static START: Once = ONCE_INIT;

impl Write for PrintBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let PrintBuffer(ref mut buffer) = *self;

        for &byte in buf {
            buffer.push(byte);
            if byte == b'%' {
                buffer.push(b'%');
            }
        }

        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn get() -> MutexGuard<'static, PrintBuffer> {
    START.call_once(|| {
        let buffer = Mutex::new(PrintBuffer(Vec::new()));
        let buffer = Box::new(buffer);
        unsafe {
            BUFFER = Box::into_raw(buffer);
        }
    });

    let buffer = unsafe { &*BUFFER };
    buffer.lock()
}

pub fn print(args: fmt::Arguments) {
    write!(get(), "{}", args).unwrap();
}

pub fn flush() {
    let PrintBuffer(ref mut buffer) = *get();
    buffer.push(0);
    OS::report(buffer.as_ptr());
    buffer.clear();
}

/// Macro for printing to the standard output.
///
/// Equivalent to the `println!` macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stdout is frequently line-buffered by default so it may be
/// necessary to use `io::stdout().flush()` to ensure the output is emitted
/// immediately.
///
/// # Panics
///
/// Panics if writing to `io::stdout()` fails.
///
/// # Examples
///
/// ```
/// use std::io::{self, Write};
///
/// print!("this ");
/// print!("will ");
/// print!("be ");
/// print!("on ");
/// print!("the ");
/// print!("same ");
/// print!("line ");
///
/// io::stdout().flush().unwrap();
///
/// print!("this string has a newline, why not choose println! instead?\n");
///
/// io::stdout().flush().unwrap();
/// ```
#[macro_export]
#[allow_internal_unstable]
macro_rules! print {
    ($($arg:tt)*) => ($crate::std::io::print::print(format_args!($($arg)*)));
}

/// Macro for printing to the standard output, with a newline. On all
/// platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`).
///
/// Use the `format!` syntax to write data to the standard output.
/// See `std::fmt` for more information.
///
/// # Panics
///
/// Panics if writing to `io::stdout()` fails.
///
/// # Examples
///
/// ```
/// println!("hello there!");
/// println!("format {} arguments", "some");
/// ```
#[macro_export]
macro_rules! println {
    ($fmt:expr) => ({ print!($fmt); $crate::std::io::print::flush(); });
    ($fmt:expr, $($arg:tt)*) => ({ print!($fmt, $($arg)*); $crate::std::io::print::flush(); });
}