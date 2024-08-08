use crate::serialization::*;
use crate::fileio::{write_chunks, create};
use std::fs::read;
use std::io;

pub trait DatabaseValue {
    fn to_db_value(&self) -> Vec<u8>;
}

impl DatabaseValue for i32 {
    fn to_db_value(&self) -> Vec<u8> {
        return vec![
            ((self >> 24) & 0xFF) as u8,
            ((self >> 16) & 0xFF) as u8,
            ((self >> 8) & 0xFF) as u8,
            (self & 0xFF) as u8,
        ];
    }
}

pub fn new_table(name: &str, columns: Vec<Column>) {
    let mut line_size = 0_u32;

    for column in &columns {
        line_size += column.length as u32;
    }

    let table = Table {
        line_size,
        columns,
    };

    let data = serialize_table(table);

    let metadata_filename = format!("{}.metadata", name);
    let data_filename = format!("{}.table", name);

    write_chunks(&metadata_filename, &data);
    create(&data_filename);
}

pub fn get_table_metadata(name: &str) -> Result<Table, io::Error> {
    let filename = format!("{}.metadata", name);

    let data: Vec<u8>;
    match read(filename) {
        Ok(r) => {data = r;}
        Err(e) => {return Err(e)}
    }

    return Ok(deserialize_table(data));
}

pub fn write_table_metadata(name: &str, table: Table) {
    let filename = format!("{}.metadata", name);

    let data = serialize_table(table);

    write_chunks(&filename, &data);
}

pub fn get_highest_column_index(table: &Table) -> u8 {
    return table.columns.len().try_into().unwrap();
}

//pub fn add_column(name: &str, column: Column) -> Result<Vec<u8>, io::Error> {
//    let mut table: Table;
//    match get_table_metadata(name) {
//       Ok(r) => {table = r}
//       Err(e) => {return Err(e)}
//    }
//
//    table.columns.push(column);
//
//    Ok(serialize_table(table))
//}

/* pub fn write<T: DatabaseValue>(table: Table, column_index: u16, data: T) {
    write(table, column_index, data)
} */
