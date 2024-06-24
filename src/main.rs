#![allow(unused)]

use std::fs;
use std::io::{stdout, stdin, Write};
use serialization::*;
use std::path::Path;

mod serialization;
mod fileio;

fn main() {
    if (!(Path::new("./metadata").exists())) {
    print!("\nEnter first column name: ");
    stdout().flush().unwrap();
    let mut string = String::new();
    let input = stdin().read_line(&mut string).unwrap();

    print!("\nEnter first column data type: ");
    stdout().flush().unwrap();
    let mut string2 = String::new();
    let input = stdin().read_line(&mut string2).unwrap();

    print!("\nEnter first column data length: ");
    stdout().flush().unwrap();
    let mut string3 = String::new();
    let input = stdin().read_line(&mut string3).unwrap();

    print!("\nEnter second column name: ");
    stdout().flush().unwrap();
    let mut string4 = String::new();
    let input = stdin().read_line(&mut string4).unwrap();

    print!("\nEnter second column data type: ");
    stdout().flush().unwrap();
    let mut string5 = String::new();
    let input = stdin().read_line(&mut string5).unwrap();

    print!("\nEnter second column data length: ");
    stdout().flush().unwrap();
    let mut string6 = String::new();
    let input = stdin().read_line(&mut string6).unwrap();

    let ascii_slice = string.trim().as_bytes();
    let mut ascii_array = [0u8; 32];

    let ascii_slice2 = string4.trim().as_bytes();
    let mut ascii_array2 = [0u8; 32];

    ascii_array[..ascii_slice.len()].copy_from_slice(ascii_slice);
    ascii_array2[..ascii_slice2.len()].copy_from_slice(ascii_slice2);

    let table = Table {
        line_size: 260,
        columns: vec!(
            Column {
                name: ascii_array,
                data_type: string2.trim().parse::<u8>().unwrap(),
                length: string3.trim().parse::<u16>().unwrap(),
            },
            Column {
                name: ascii_array2,
                data_type: string5.trim().parse::<u8>().unwrap(),
                length: string6.trim().parse::<u16>().unwrap(),
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
    return;
    }

    let data = fs::read("./metadata").unwrap();

    let table = deserialize_table(data);

    println!("Table:");
    println!("  Line size: {}B", table.line_size);
    println!("  Columns:");
    for (i, column) in table.columns.into_iter().enumerate() {
        println!("    Column {}", i+1);
        println!("      Name: {} ", String::from_utf8(column.name.to_vec()).unwrap());
        println!("      Data type: {} ", column.data_type);
        println!("      Length {}", column.length);
    }
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
