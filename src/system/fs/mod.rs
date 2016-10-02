use super::tww::JKRDvdFile;
use std::mem::transmute;
use std::io::prelude::*;
use std::io::{self, Cursor, SeekFrom};
use std::cmp;
// use system::memory;

pub struct File {
    // This is not the buffer, but the file object itself.
    // This is heap allocated because JKRDvdFile objects are immovable.
    data: Box<[u8]>,
    cursor: i32,
    file_size: i32,
    buf: Box<[u8]>,
    pos: usize,
    start_pos: usize,
    end_pos: usize,
}

const CAPACITY: usize = 1024;

impl File {
    fn data(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }

    pub fn open(path: &str) -> Self {
        let mut data = vec![0; 512].into_boxed_slice();
        let buf = vec![0; CAPACITY + 32].into_boxed_slice();
        let original_start_pos = unsafe { transmute::<_, usize>(buf.as_ptr()) };
        let aligned_start_pos = (original_start_pos | 0x1F) + 1;
        let start_pos = aligned_start_pos - original_start_pos;
        let end_pos = start_pos + CAPACITY;

        JKRDvdFile::constructor(data.as_mut_ptr());

        // TODO Use OsString
        let mut path = path.bytes().collect::<Vec<_>>();
        path.push(0);

        JKRDvdFile::open(data.as_mut_ptr(), path.as_ptr());

        let file_size = JKRDvdFile::get_file_size(data.as_mut_ptr());

        File {
            data: data,
            cursor: 0,
            file_size: file_size,
            buf: buf,
            pos: end_pos,
            start_pos: start_pos,
            end_pos: end_pos,
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buf = Cursor::new(buf);

        let mut total_read = 0;
        loop {
            let nread = {
                let rem = self.fill_buf()?;
                buf.write(rem)?
            };
            self.consume(nread);
            total_read += nread;
            if nread < CAPACITY {
                break;
            }
        }
        Ok(total_read)
    }
}

impl BufRead for File {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        if self.pos >= self.end_pos {
            let mut bytes_to_read = cmp::min(self.file_size - self.cursor, CAPACITY as i32);
            if bytes_to_read & 0x1F != 0 {
                bytes_to_read = (bytes_to_read | 0x1F) + 1;
            }
            if bytes_to_read > 0 {
                self.pos = self.start_pos;
                self.end_pos = self.start_pos + bytes_to_read as usize;

                JKRDvdFile::read(self.data(),
                                 self.buf[self.pos..self.end_pos].as_mut_ptr(),
                                 bytes_to_read,
                                 self.cursor);

                self.cursor += bytes_to_read;
            }
        }
        Ok(&self.buf[self.pos..self.end_pos])
    }

    fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.end_pos);
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let mut target_cursor = match pos {
            SeekFrom::Start(n) => n as i32,
            SeekFrom::End(n) => self.file_size + n as i32,
            SeekFrom::Current(n) => self.cursor - (self.end_pos - self.pos) as i32 + n as i32,
        };

        if target_cursor < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Seek out of file"));
        }
        if target_cursor > self.file_size {
            target_cursor = self.file_size;
        }

        let aligned_cursor = target_cursor & !0x1F;
        let buf_offset = target_cursor - aligned_cursor;

        self.pos = self.end_pos; // empty the buffer
        self.cursor = aligned_cursor;

        self.fill_buf()?;
        self.consume(buf_offset as usize); // skip by unalignment

        Ok((self.cursor + buf_offset) as u64)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        JKRDvdFile::close(self.data());
        JKRDvdFile::destructor(self.data());
    }
}