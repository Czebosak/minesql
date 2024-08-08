use crate::{data, serialization::*};
use std::io::{Write, stdout, stdin};
use std::process;

pub fn main_loop() {
    loop {
        println!("1. Create new table");
        println!("2. Read table metadata");
        println!("3. Add column to table");
        println!("4. Exit\n");
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        let _ = stdin().read_line(&mut input).unwrap();

        println!();

        match input.trim() {
            "1" => create_table(),
            "2" => read_metadata(),
            "4" => exit(),
            _ => {}
        }
    }
}

fn get_name() -> String{
    println!("Enter table name");
    print!("> ");
    stdout().flush().unwrap();

    let mut name = String::new();
    let _ = stdin().read_line(&mut name).unwrap();
    return name;
}

fn column_loop(starting_column_index: u8) -> Vec<Column> {
    let mut columns: Vec<Column> = Vec::new();
    let mut i = starting_column_index;

    loop {
        println!("\n1. Create column {}", i);
        println!("2. Exit\n");
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        let _ = stdin().read_line(&mut input).unwrap();

        println!();

        match input.trim() {
            "2" => break,
            _ => {}
        }


        println!("\nEnter column name");
        print!("> ");
        stdout().flush().unwrap();

        let mut column_name = String::new();
        let _ = stdin().read_line(&mut column_name).unwrap();


        println!("\nEnter column data type");
        print!("> ");
        stdout().flush().unwrap();
        
        let mut data_type = String::new();
        let _ = stdin().read_line(&mut data_type);


        println!("\nEnter column length (bytes)");
        print!("> ");
        stdout().flush().unwrap();

        let mut length = String::new();
        let _ = stdin().read_line(&mut length);

        let ascii_slice = column_name.trim().as_bytes();
        let mut ascii_array = [0u8; 32];

        ascii_array[..ascii_slice.len()].copy_from_slice(ascii_slice);

        columns.push(Column {
            name: ascii_array,
            data_type: data_type.trim().parse::<u8>().unwrap(),
            length: length.trim().parse::<u32>().unwrap(),
            index: i,
        });
        i += 1;
    }

    return columns;
}

fn create_table() {
    let name = get_name();

    let columns: Vec<Column> = column_loop(0);

    data::new_table(&name.trim(), columns);
}

fn read_metadata() {
    println!("\nEnter table name");
    print!("> ");
    stdout().flush().unwrap();

    let mut table_name = String::new();
    let _ = stdin().read_line(&mut table_name);

    let table: Table;
    match data::get_table_metadata(table_name.trim()) {
        Ok(r) => {table = r;}
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("Table:");
    println!("  Line size: {}B", table.line_size);
    println!("  Columns:");
    for (i, column) in table.columns.into_iter().enumerate() {
        println!("    Column {}", i+1);
        println!("      Name: {} ", String::from_utf8(column.name.to_vec()).unwrap());
        println!("      Data type: {} ", column.data_type);
        println!("      Length {}", column.length);
        println!("      Index {}", column.index);
    }

    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
}

fn add_columns() {
    let name = get_name();
    let mut table: Table;
    match data::get_table_metadata(&name) {
        Ok(r) => {table = r}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let columns = column_loop(table.columns.len().try_into().unwrap());
    for column in columns {
        table.columns.push(column);
    }
}

fn exit() {
    process::exit(0);
}
