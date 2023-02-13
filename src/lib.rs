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
//! # Example
//! 
//! Read a file into a vector, chunk by chunk, using a `SpareBuffer`:
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
