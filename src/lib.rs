/*
 * Spare Buffer
 * This is free and unencumbered software released into the public domain.
 */

//! A wrapper around [**`Vec<T>`**](std::vec::Vec) that gives access to the
//! "spare" capacity of the vector as a `&mut[T]` slice. 
//! 
//! Useful to allocate ["spare" capacity](std::vec::Vec::spare_capacity_mut) at
//! the end of the underlying vector and fill it *directly*, e.g. by
//! [`read()`](std::io::Read::read)ing from a file or stream, **without**
//! initialize the memory first. Once filled, the vector can be "extended" into
//! the previously allocated spare capacity.
//! 
//! The following two steps are always required, usually performed in a loop:
//! 1. [Allocate](crate::SpareBuffer::allocate_spare) a new "spare" buffer of
//!    appropriate length.
//! 2. [Commit](crate::SpareBuffer::commit) the "spare" buffer, once it has
//!    been filled with some valid data.
//! 
//! Note that, after step&nbsp;#1, the "spare" buffer is **not** considered a
//! valid part of the underlying vector yet. Committing the data, in
//! step&nbsp;#2, effectively *appends* the contents of the "spare" buffer to
//! the underlying vector, but **without** copying the data.
//! 
//! It is **not** necessary to fill *all* of the "spare" buffer; only the first
//! `n` elements may be committed. However, *all* elements to be committed
//! **must** have been initialized, or the contents of the underlying vector
//! are ***unspecified*** after the commit!
//! 
//! # Example #1
//! 
//! For starters, fill a pre-allocated [**`SpareBuffer`**](crate::SpareBuffer)
//! with some numbers:
//! ```
//! fn main() {
//!     let mut vec: Vec<u8> = Vec::with_capacity(128);
//!     let mut buffer = SpareBuffer::from(&mut vec, None);
//!
//!     let spare = buffer.allocate_spare(NonZeroUsize::new(100).unwrap());
//!     for i in 0..50 {
//!         spare[i] = i as u8;
//!     }
//! 
//!     // Whoops: only &spare[0..50] was initialized, but 100 elements are committed!
//!     buffer.commit(100).expect("Failed to commit!");
//! 
//!     println!("Expect valid numbers:");
//!     println!("{:?}\n", &vec[..50]);
//! 
//!     println!("Expect \"unspecified\" garbage:");
//!     println!("{:?}\n", &vec[50..]);
//! }
//! ```
//! 
//! # Example #2
//!
//! Read a file into a vector, chunk by chunk, using a
//! [**`SpareBuffer`**](crate::SpareBuffer) to accumulate all data:
//! ```
//! fn main() {
//!     let mut vec: Vec<u8> = Vec::with_capacity(1048576);
//!     let mut buffer = SpareBuffer::from(&mut vec, NonZeroUsize::new(10485760));
//!     
//!     let chunk_size = NonZeroUsize::new(4096).unwrap();
//!     let mut file = File::open("input.dat").expect("Failed to open input file!");
//! 
//!     loop {
//!         let spare = buffer.allocate_spare(chunk_size);
//!         let count = file.read(spare).expect("File read error encountered!");
//!         if count > 0 {
//!             buffer.commit(count).expect("Failed to commit!");
//!         } else {
//!             break; /* EOF*/
//!         }
//!     }
//! 
//!     println!("Length: {:?}", vec.len());
//! }
//! ```
mod buffer;
mod primitive;

pub use buffer::SpareBuffer;
pub use primitive::Primitive;
