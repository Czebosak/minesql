use std::fmt::format;
use crate::serialization::*;
use crate::fileio::{write_chunks, create};

fn new_table(mut name: &str, columns: Vec<Column>) {
    let mut line_size = 0_u32;

    for column in &columns {
        line_size += column.length as u32;
    }

    let table = Table {
        line_size: line_size,
        columns: columns,
    };

    let data = serialize_table(table);

    let metadata_filename = format!("{}.metadata", name);
    let data_filename = format!("{}.metadata", name);

    write_chunks(&metadata_filename, &data);
    create(&data_filename);
}
