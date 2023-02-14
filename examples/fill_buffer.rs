/*
 * Spare Buffer
 * This is free and unencumbered software released into the public domain.
 */
use std::num::NonZeroUsize;

use spare_buffer::SpareBuffer;

fn main() {
    let mut vec: Vec<u8> = Vec::with_capacity(128);
    let mut buffer = SpareBuffer::from(&mut vec, None);
    
    let spare = buffer.allocate_spare(NonZeroUsize::new(100).unwrap());
    for i in 0..50 {
        spare[i] = i as u8;
    }

    // Whoops: only &spare[0..50] was initialized, but 100 elements are committed!
    buffer.commit(100).expect("Failed to commit!");

    println!("Expect valid numbers:");
    println!("{:?}\n", &vec[..50]);

    println!("Expect \"unspecified\" garbage:");
    println!("{:?}\n", &vec[50..]);
}
