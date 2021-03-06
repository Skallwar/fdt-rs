use core::mem::size_of;
use core::ptr::read_unaligned;

#[derive(Debug, Copy, Clone)]
pub enum SliceReadError {
    InvalidOffset(usize, usize),
    UnexpectedEndOfInput,
}

pub(crate) type SliceReadResult<T> = Result<T, SliceReadError>;

pub(crate) trait SliceRead<'a> {
    unsafe fn unsafe_read_be_u32(&self, pos: usize) -> SliceReadResult<u32>;
    unsafe fn unsafe_read_be_u64(&self, pos: usize) -> SliceReadResult<u64>;
    fn read_be_u32(&self, pos: usize) -> SliceReadResult<u32>;
    fn read_be_u64(&self, pos: usize) -> SliceReadResult<u64>;
    fn read_bstring0(&self, pos: usize) -> SliceReadResult<&'a [u8]>;
    fn nread_bstring0(&self, pos: usize, len: usize) -> SliceReadResult<&'a [u8]>;
}

macro_rules! unchecked_be_read {
    ( $buf:ident, $type:ident , $off:expr ) => {
        (if $off + size_of::<$type>() > $buf.len() {
            Err(SliceReadError::InvalidOffset($off, size_of::<$type>()))
        } else {
            Ok((*($buf.as_ptr().add($off) as *const $type)).to_be())
        })
    };
}

macro_rules! be_read {
    ( $buf:ident, $type:ident , $off:expr ) => {
        (if $off + size_of::<$type>() > $buf.len() {
            Err(SliceReadError::UnexpectedEndOfInput)
        } else {
            // Unsafe okay, we checked length above.
            // We call read_unaligned, so alignment isn't required.
            unsafe {
                // We explicitly read unaligned.
                #[allow(clippy::cast_ptr_alignment)]
                Ok((read_unaligned::<$type>($buf.as_ptr().add($off) as *const $type)).to_be())
            }
        })
    };
}

impl<'a> SliceRead<'a> for &'a [u8] {
    unsafe fn unsafe_read_be_u32(&self, pos: usize) -> SliceReadResult<u32> {
        unchecked_be_read!(self, u32, pos)
    }

    unsafe fn unsafe_read_be_u64(&self, pos: usize) -> SliceReadResult<u64> {
        unchecked_be_read!(self, u64, pos)
    }

    fn read_be_u32(&self, pos: usize) -> SliceReadResult<u32> {
        be_read!(self, u32, pos)
    }

    fn read_be_u64(&self, pos: usize) -> SliceReadResult<u64> {
        be_read!(self, u64, pos)
    }

    fn read_bstring0(&self, pos: usize) -> SliceReadResult<&'a [u8]> {
        for i in pos..self.len() {
            if self[i] == 0 {
                return Ok(&self[pos..i]);
            }
        }
        Err(SliceReadError::UnexpectedEndOfInput)
    }

    fn nread_bstring0(&self, pos: usize, len: usize) -> SliceReadResult<&'a [u8]> {
        let end = core::cmp::min(len + pos, self.len());
        for i in pos..end {
            // Unsafe okay, we just confirmed the length in the let above.
            unsafe {
                if *self.get_unchecked(i) == 0 {
                    return Ok(&self[pos..i]);
                }
            }
        }
        Err(SliceReadError::UnexpectedEndOfInput)
    }
}
