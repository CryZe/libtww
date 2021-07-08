// use system::{libc, memory};
use system::memory;
use libc;
use std::fmt::{Error, Write};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
pub struct Line {
    pub visible: bool,
    pub text: [u8; 61],
}

#[repr(C)]
pub struct Console {
    pub x: u32,          // a60
    pub y: u32,          // a64
    pub line_count: u32, // a68
    _p0: [u8; 4],
    pub font_scale_x: f32, // a70
    pub font_scale_y: f32, // a74
    _p1: [u8; 8],
    pub background_color: Color, // a80
    pub visible: bool,           // a84
    _p2: [u8; 3],
    pub lines: [Line; 32], // a88
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl Console {
    #[cfg(feature = "ntsc_j")]
    pub fn get() -> &'static mut Console {
        memory::reference(0x80491A60)
    }
    #[cfg(feature = "ntsc_u")]
    pub fn get() -> &'static mut Console {
        memory::reference(0x804BE580)
    }

    pub fn setup(&mut self) {
        self.x = 33;
        self.y = 112;
        self.background_color = Color::rgba(0, 0, 0, 0);
        self.visible = true;
        self.clear();
    }

    pub fn clear(&mut self) {
        for line in &mut self.lines {
            line.visible = true;
            line.clear();
        }
    }
}

impl Line {
    pub fn begin(&mut self) -> LineWriter {
        LineWriter {
            line: self,
            position: 0,
        }
    }

    pub fn append(&mut self) -> LineWriter {
        let len = self.len();
        LineWriter {
            line: self,
            position: len,
        }
    }

    pub fn len(&self) -> usize {
        unsafe { libc::strlen(self.text.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        unsafe {
            *self.text.as_mut_ptr() = 0;
        }
    }
}

pub struct LineWriter<'a> {
    line: &'a mut Line,
    position: usize,
}

impl<'a> Write for LineWriter<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        memory::write_str(self.line.text[self.position..].as_mut_ptr(), s);
        self.position += s.len();
        Ok(())
    }
}
