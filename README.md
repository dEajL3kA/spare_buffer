# Spare Buffer

[![Crates.io][crates-badge]][crates-url]
[![Unlicense][unlicense-badge]][unlicense-url]

[crates-badge]: https://img.shields.io/crates/v/spare_buffer.svg
[crates-url]: https://crates.io/crates/spare_buffer
[unlicense-badge]: https://img.shields.io/badge/license-Unlicense-blue.svg
[unlicense-url]: LICENSE

A wrapper around [**`Vec<T>`**](https://doc.rust-lang.org/std/vec/struct.Vec.html) that provides access to the "spare" capacity of the vector as a `&mut[T]` slice.

Useful to allocate "spare" capacity at the end of the underlying vector and
fill it *directly*, e.g. by `read()`ing from a file or stream, **without**
initialize the memory first. Once filled, the vector can be "extended" into the
previously allocated spare capacity.

**Crates.io:**  
https://crates.io/crates/spare_buffer

**API Documentation:**  
https://docs.rs/spare_buffer/latest/index.html

**Examples:**  
https://github.com/dEajL3kA/spare_buffer/tree/master/examples
