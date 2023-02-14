/*
 * Spare Buffer
 * This is free and unencumbered software released into the public domain.
 */
use std::fs::File;
use std::io::Read;
use std::num::NonZeroUsize;

use spare_buffer::SpareBuffer;

fn main() {
    let mut vec: Vec<u8> = Vec::with_capacity(1048576);
    let mut buffer = SpareBuffer::from(&mut vec, NonZeroUsize::new(10485760));
    
    let chunk_size = NonZeroUsize::new(4096).unwrap();
    let mut file = File::open("input.dat").expect("Failed to open input file!");

    loop {
        let spare = buffer.allocate_spare(chunk_size);
        let count = file.read(spare).expect("File read error encountered!");
        if count > 0 {
            buffer.commit(count).expect("Failed to commit!");
        } else {
            break; /* EOF*/
        }
    }

    println!("Length: {:?}", vec.len());
}
