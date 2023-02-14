/*
 * Spare Buffer
 * This is free and unencumbered software released into the public domain.
 */
use std::io::{Result as IoResult, Error as IoError, ErrorKind};
use std::num::NonZeroUsize;
use std::slice::from_raw_parts_mut;

use crate::Primitive;

/// A wrapper around [**`Vec<T>`**](std::vec::Vec) that provides access to the
/// "spare" capacity of the vector as a `&mut[T]` slice.
/// 
/// See [module level documentation](crate) for more information.
pub struct SpareBuffer<'a, T>
where
    T: Primitive
{
    buffer: &'a mut Vec<T>,
    limit: Option<NonZeroUsize>,
    allocated: bool,
}

impl<'a, T> SpareBuffer<'a, T>
where
    T: Primitive
{
    /// Creates a new **`SpareBuffer`** from an existing vector.
    /// 
    /// An *optional* `limit` for the length of the vector can be specified.
    /// The [`commit()`](Self::commit) fails, if it would exceed this limit.
    pub fn from(buffer: &'a mut Vec<T>, limit: Option<NonZeroUsize>) -> Self {
        Self {
            buffer,
            limit,
            allocated: false,
        }
    }

    /// Returns the number of "committed" elements in the underlying vector.
    /// This is equivalent to [`Vec::len()`](std::vec::Vec::len).
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    ///  Returns `true` if the underlying vector contains no "committed"
    ///  elements. This is equivalent to
    ///  [`Vec::is_empty()`](std::vec::Vec::is_empty).
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns the length limit, if a limit has been specified. Otherwise
    /// `None` is returned.
    pub fn limit(&self) -> Option<NonZeroUsize> {
        self.limit
    }

    /// Returns a `&[T]` slice of all "committed" elements in the underlying
    /// vector. This is equivalent to
    /// [`Vec::as_slice()`](std::vec::Vec::as_slice).
    pub fn data(&self) -> &[T] {
        &self.buffer[..]
    }

    /// Allocates a "spare" buffer of the specified `length`.
    /// 
    /// Reserves capacity for *at least* `length` additional elements in the
    /// underlying vector. May reserve more space to speculatively avoid
    /// frequent reallocations. Does nothing, if the unused "spare" capacity of
    /// the underlying vector is already sufficient.
    /// 
    /// Returns a `&mut[T]` slice which allows the caller to access the
    /// allocated "spare" buffer. No guarantees are provided about the
    /// *initial* contents of the buffer! It is recommended that the caller
    /// only *writes* data to the slice instead of reading its contents.
    /// 
    /// The returned `&mut[T]` slice can be passed to
    /// [`Read::read()`](std::io::Read::read) or similar I/O routines.
    /// 
    /// The "spare" buffer is **not** considered to be a valid part of the
    /// underlying vector, until the [`commit()`](Self::commit) function is
    /// called eventually.
    pub fn allocate_spare(&mut self, length: NonZeroUsize) -> &mut[T] {
        self.buffer.reserve(length.get());
        self.allocated = true;
        let spare = self.buffer.spare_capacity_mut();
        unsafe {
            from_raw_parts_mut(spare.as_mut_ptr() as *mut T, spare.len())
        }
    }

    /// Commits the first `additional` elements of the "spare" buffer.
    /// 
    /// The underlying vector is *extended* into the previously
    /// [allocated](Self::allocate_spare) "spare" buffer, by increasing its
    /// length, so that the first `additional` elements in the "spare" buffer
    /// effectively are *appended* to the vector **without** copying the data.
    /// 
    /// All elements to be *committed* fom the "spare" buffer **must** have
    /// been initialized, i.e. the whole of `&spare[0..additional]` **must**
    /// have been *filled* with valid data. Otherwise, the contents of the
    /// underlying vector are ***unspecified*** after the commit 😨
    /// 
    /// This function always invalidates the current "spare" buffer. A new
    /// "spare" buffer must be [allocated](Self::allocate_spare) in order to
    /// append more data!
    ///
    /// # Errors
    /// 
    /// If a length limit has been specified, then this function will fail, if
    /// adding `additional` more elements to the underlying vector would cause
    /// its total length to exceed the specified limit. Otherwise, the function
    /// always returns `Ok(())`.
    ///
    /// # Panics
    /// 
    /// Panics if `additional` is greater than the available "spare" capacity,
    /// or if **no** "spare" buffer was allocated before!
    /// 
    /// A panic may also occur, if the new length would overflow `usize::MAX`.
    pub fn commit(&mut self, additional: usize) -> IoResult<()> {
        assert!(std::mem::replace(&mut self.allocated, false), "No spare buffer allocated!");
        if additional > 0 {
            let new_length = self.buffer.len().checked_add(additional).expect("Numerical overflow! (new_length)");
            assert!(new_length <= self.buffer.capacity(), "Commit size exceeds available capacity!");
            if new_length <= self.limit.map_or(usize::MAX, NonZeroUsize::get) {
                unsafe {
                    self.buffer.set_len(new_length)
                }
            } else {
                return Err(IoError::new(ErrorKind::OutOfMemory, "The new length exceeds the specified limit!"))
            }
        }
        Ok(())
    }

    /// The same as [`commit()`](Self::commit) but **without** any checks.
    /// 
    /// This function is **`unsafe`**, for obvious reasons, and therefore
    /// should be used with great care!
    pub unsafe fn commit_unchecked(&mut self, additional: usize) {
        self.allocated = false;
        if additional > 0 {
            self.buffer.set_len(self.buffer.len() + additional)
        }
    }
}
