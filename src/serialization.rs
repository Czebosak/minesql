use std::fs::File;

pub struct Column {
    pub name: [u8; 32],
    pub data_type: u8,
    pub length: u16,
}

pub struct Table {
    pub line_size: u32,
    pub columns: Vec<Column>,
}

pub fn serialize_table(table: Table) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    let line_size_serialized = [
        ((table.line_size >> 24) & 0xFF) as u8,
        ((table.line_size >> 16) & 0xFF) as u8,
        ((table.line_size >> 8) & 0xFF) as u8,
        (table.line_size & 0xFF) as u8,
    ];

    data.extend_from_slice(&line_size_serialized);

    for column in table.columns {
        data.extend_from_slice(&column.name);
        data.push(column.data_type);
        
        let split_length = [(column.length >> 8) as u8, (column.length & 0xFF) as u8];
        data.extend_from_slice(&split_length);
    }

    return data;
}

pub fn deserialize_table(mut data: Vec<u8>) -> Table {
    let mut deserialized_table = Table {
        line_size: 0_u32,
        columns: Vec::new(),
    };

    let mut column_data = data.split_off(4);

    deserialized_table.line_size = (data[0] as u32) << 24
                | (data[1] as u32) << 16
                | (data[2] as u32) << 8
                | (data[3] as u32);

    while column_data.len() > 0 {
        let mut name = [0_u8; 32];
        let remaining_data = column_data.split_off(32);

        name.copy_from_slice(&column_data);

        let mut data_iter = remaining_data.into_iter();

        let data_type = data_iter.next().unwrap();
        let length = ((data_iter.next().unwrap() as u16) << 8) | (data_iter.next().unwrap() as u16);

        let column = Column {
            name,
            data_type,
            length,
        };

        deserialized_table.columns.push(column);

        column_data = data_iter.collect();
    }

    deserialized_table
}
