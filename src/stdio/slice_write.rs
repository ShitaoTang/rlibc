use crate::include::ctype::*;
use core::fmt;

#[repr(C)]
pub struct SliceWriter<'a, T> {
    pub buffer: &'a mut [T],
    pub pos: size_t,
}

impl<'a, T> SliceWriter<'a, T>
where
    T: Copy + Into<u8>,
{
    pub fn new(buffer: &'a mut [T]) -> Self {
        Self { buffer, pos: 0 }
    }

    pub fn written_len(&self) -> size_t {
        self.pos
    }

    pub fn as_slice(&self) -> &[T] {
        &self.buffer[..self.pos as size_t]
    }
}

impl<'a, T> fmt::Write for SliceWriter<'a, T>
where
    T: Copy + Into<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        if self.pos + bytes.len() <= self.buffer.len() {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    self.buffer.as_mut_ptr().add(self.pos) as *mut u8,
                    bytes.len()
                );
            }
            self.pos += bytes.len();
            Ok(())
        } else {
            Err(fmt::Error)
        }
    } 
}