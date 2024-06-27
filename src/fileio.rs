use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::FileExt;

pub fn write_chunks(filename: &str, data: &Vec<u8>) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    
    let _ = file.set_len(0);
    let _ = file.write(data);
}

pub fn read_buf_at(filename: &str, pos: u64, buf_size: usize) -> Box<[u8]> {
    let file = File::open(filename).unwrap();

    let mut buf = vec![0_u8; buf_size].into_boxed_slice();
    let _ = file.read_at(&mut buf, pos);

    return buf;
}

pub fn write_buf_at(filename: &str, pos: u64, buf: Box<[u8]>) {
    let file = File::open(filename).unwrap();

    let _ = file.write_at(&buf, pos);
}

pub fn create(filename: &str) {
    let _ = File::create(filename).unwrap();
}
