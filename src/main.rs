#![allow(unused)]

use std::fs;
use std::io::Write;
use serialization::*;

mod serialization;
mod fileio;

fn main() {
    let ascii_slice = "Haha funny column".as_bytes();
    let mut ascii_array = [0u8; 32];

    let ascii_slice2 = "Haha crazy column".as_bytes();
    let mut ascii_array2 = [0u8; 32];

    ascii_array[..ascii_slice.len()].copy_from_slice(ascii_slice);
    ascii_array2[..ascii_slice2.len()].copy_from_slice(ascii_slice2);

    let table = Table {
        columns: vec!(
            Column {
                name: ascii_array,
                data_type: 1,
                length: 2,
            },
            Column {
                name: ascii_array2,
                data_type: 2,
                length: 4096,
            },
        )
    };

    let serialized = serialize_table(table);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("metadata")
        .unwrap();
    
    file.set_len(0);
    file.write(&serialized);

    let table = deserialize_table(serialized);

    for (i, column) in table.columns.into_iter().enumerate() {
        println!("\nColumn {}", i+1);
        println!("  Name: {} ", String::from_utf8(column.name.to_vec()).unwrap());
        println!("  Data type: {} ", column.data_type);
        println!("  Length {}", column.length);
    }

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
