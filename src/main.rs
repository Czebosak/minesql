#![allow(unused)]

use std::fs;
use std::io::{stdout, stdin, Write};
use std::path::Path;
use serialization::*;
use data::new_table;
//use fileio::*;

mod fileio;
mod serialization;
mod data;
mod ui;

fn main() {
    ui::main_loop();
    if !(Path::new("./test.metadata").exists()) {
    print!("\nEnter first column name: ");
    stdout().flush().unwrap();
    let mut string = String::new();
    let _ = stdin().read_line(&mut string).unwrap();

    print!("\nEnter first column data type: ");
    stdout().flush().unwrap();
    let mut string2 = String::new();
    let _ = stdin().read_line(&mut string2).unwrap();

    print!("\nEnter first column data length: ");
    stdout().flush().unwrap();
    let mut string3 = String::new();
    let _ = stdin().read_line(&mut string3).unwrap();

    print!("\nEnter second column name: ");
    stdout().flush().unwrap();
    let mut string4 = String::new();
    let _ = stdin().read_line(&mut string4).unwrap();

    print!("\nEnter second column data type: ");
    stdout().flush().unwrap();
    let mut string5 = String::new();
    let _ = stdin().read_line(&mut string5).unwrap();

    print!("\nEnter second column data length: ");
    stdout().flush().unwrap();
    let mut string6 = String::new();
    let _ = stdin().read_line(&mut string6).unwrap();

    let ascii_slice = string.trim().as_bytes();
    let mut ascii_array = [0u8; 32];

    let ascii_slice2 = string4.trim().as_bytes();
    let mut ascii_array2 = [0u8; 32];

    ascii_array[..ascii_slice.len()].copy_from_slice(ascii_slice);
    ascii_array2[..ascii_slice2.len()].copy_from_slice(ascii_slice2);

    new_table("test", vec!(
        Column {
            name: ascii_array,
            data_type: string2.trim().parse::<u8>().unwrap(),
            length: string3.trim().parse::<u32>().unwrap(),
        },
        Column {
            name: ascii_array2,
            data_type: string5.trim().parse::<u8>().unwrap(),
            length: string6.trim().parse::<u32>().unwrap(),
        },
    ));

/*     let table = Table {
        line_size: 260,
        columns: vec!(
            Column {
                name: ascii_array,
                data_type: string2.trim().parse::<u8>().unwrap(),
                length: string3.trim().parse::<u32>().unwrap(),
            },
            Column {
                name: ascii_array2,
                data_type: string5.trim().parse::<u8>().unwrap(),
                length: string6.trim().parse::<u32>().unwrap(),
            },
        )
    };

    let serialized = serialize_table(table);
    
    write_chunks("metadata", &serialized);
 */
    return;
    }

    let data = fs::read("./test.metadata").unwrap();

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