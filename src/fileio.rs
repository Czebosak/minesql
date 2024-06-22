/* use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

pub fn read_bytes(path, buffer_size) {
    let file = File::open(&path).unwrap();
    
    let mut reader = BufReader::with_capacity(buffer_size, file);

    loop {
        let buffer = reader.fill_buf()?;

        if buffer.len() == 0 {
            break;
        }

        // return buffer;
        // reader.consume(buffer_length);
    }
}
 */