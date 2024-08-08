use crate::serialization::*;
use crate::fileio::{write_chunks, create};

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

/* pub fn write<T: DatabaseValue>(table: Table, column_index: u16, data: T) {
    write(table, column_index, data)
} */
