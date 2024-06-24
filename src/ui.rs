use std::io::{stdout, stdin, Write};
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
            "1" => create_new_table(),
            "4" => exit(),
            _ => {}
        }
    }
}

fn create_new_table() {
    println!("Enter table name\n");
    print!("> ");
    stdout().flush().unwrap();

    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
}

fn exit() {
    process::exit(0);
}