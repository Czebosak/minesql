use std::fs;
use std::io::Write;

pub fn write_chunks(filename: &str, data: &Vec<u8>) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    
    file.set_len(0);
    file.write(data);
}

pub fn create(filename: &str) {
    let _ = fs::File::create(filename).unwrap();
}
