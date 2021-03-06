use core::mem::size_of;
use core::str::from_utf8;

use crate::prelude::*;

use crate::base::DevTree;
use crate::error::DevTreeError;
use crate::spec::Phandle;

use crate::error::Result;

#[cfg(doc)]
use crate::base::DevTreeProp;

pub trait PropReader<'dt> {
    type NodeType;

    /// Returns the buffer associtated with the property's data.
    #[doc(hidden)]
    fn propbuf(&self) -> &'dt [u8];

    /// Returns offset of this property's name in the device tree buffer.
    #[doc(hidden)]
    fn nameoff(&self) -> usize;

    #[doc(hidden)]
    fn fdt(&self) -> &DevTree<'dt>;

    /// Returns the name of the property within the device tree.
    #[inline]
    fn name(&self) -> Result<&'dt str> {
        let str_offset = self.fdt().off_dt_strings() + self.nameoff();
        let name = self.fdt().buf().read_bstring0(str_offset)?;
        Ok(from_utf8(name)?)
    }

    /// Returns the length of the property value within the device tree
    #[inline]
    #[must_use]
    fn length(&self) -> usize {
        self.propbuf().len()
    }

    /// Returns the node which this property is contained within.
    fn node(&self) -> Self::NodeType;

    /// Read a big-endian [`u32`] from the provided offset in this device tree property's value.
    /// Convert the read value into the machines' native [`u32`] format and return it.
    ///
    /// If an offset which would cause this read to access memory outside of this property's value
    /// an [`Err`] containing [`DevTreeError::InvalidOffset`] will be returned.
    #[inline]
    fn u32(&self, index: usize) -> Result<u32> {
        // Safety: propbuf is guaranteed aligned to u32
        // We'll read without re-checking alignment
        unsafe {
            self.propbuf()
                .unsafe_read_be_u32(index * size_of::<u32>())
                .or(Err(DevTreeError::InvalidOffset))
        }
    }

    /// Read a big-endian [`u64`] from the provided offset in this device tree property's value.
    /// Convert the read value into the machines' native [`u64`] format and return it.
    ///
    /// If an offset which would cause this read to access memory outside of this property's value
    /// an [`Err`] containing [`DevTreeError::InvalidOffset`] will be returned.
    #[inline]
    fn u64(&self, index: usize) -> Result<u64> {
        self.propbuf()
            .read_be_u64(index * size_of::<u64>())
            .or(Err(DevTreeError::InvalidOffset))
    }

    /// A Phandle is simply defined as a u32 value, as such this method performs the same action as
    /// [`self.u32`]
    #[inline]
    fn phandle(&self, index: usize) -> Result<Phandle> {
        // Safety: propbuf is guaranteed aligned to u32
        // We'll read without re-checking alignment
        unsafe {
            self.propbuf()
                .unsafe_read_be_u32(index * size_of::<Phandle>())
                .or(Err(DevTreeError::InvalidOffset))
        }
    }

    /// Returns the string property as a string if it can be parsed as one.
    /// # Safety
    ///
    /// See the safety note of [`PropReader::u32`]
    #[inline]
    fn str(&self) -> Result<&'dt str> {
        self.iter_str().next()?.ok_or(DevTreeError::ParseError)
    }

    /// Returns the property as a string fallible_iterator.
    /// # Safety
    ///
    /// See the safety note of [`PropReader::u32`]
    #[inline]
    fn iter_str(&self) -> StringPropIter<'dt> {
        StringPropIter::new(self.propbuf())
    }
    /// Returns this property's data as a raw slice
    ///
    /// # Safety
    ///
    /// See the safety note of [`PropReader::get_u32`]
    #[inline]
    fn raw(&self) -> &'dt [u8] {
        self.propbuf()
    }
}

use fallible_iterator::FallibleIterator;

#[derive(Debug, Clone)]
pub struct StringPropIter<'dt> {
    offset: usize,
    propbuf: &'dt [u8],
}

impl<'dt> StringPropIter<'dt> {
    fn new(propbuf: &'dt [u8]) -> Self {
        Self { propbuf, offset: 0 }
    }
}

impl<'dt> FallibleIterator for StringPropIter<'dt> {
    type Error = DevTreeError;
    type Item = &'dt str;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        if self.offset == self.propbuf.len() {
            return Ok(None);
        }
        if self.offset > self.propbuf.len() {
            return Err(DevTreeError::InvalidOffset);
        }

        let u8_slice = self.propbuf.read_bstring0(self.offset)?;
        // Include null byte
        self.offset += u8_slice.len() + 1;
        Ok(Some(from_utf8(u8_slice)?))
    }
}
