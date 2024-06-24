pub struct Column { // Each column is 37 bytes
    pub name: [u8; 32], // an array of 32 ASCII characters
    pub data_type: u8,
    pub length: u32, // The amount of bytes per line stored in column
}

pub struct Table {
    pub line_size: u32,
    pub columns: Vec<Column>,
}

pub fn serialize_table(table: Table) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    // Split a 4 byte integer into 4 1-byte elements
    let line_size_serialized = [
        ((table.line_size >> 24) & 0xFF) as u8,
        ((table.line_size >> 16) & 0xFF) as u8,
        ((table.line_size >> 8) & 0xFF) as u8,
        (table.line_size & 0xFF) as u8,
    ];

    data.extend_from_slice(&line_size_serialized);

    // Add data in bytes from all the fields of the columns to the data variable
    for column in table.columns {
        data.extend_from_slice(&column.name);
        data.push(column.data_type);
        
        // Split a 4 byte integer into 4 1-byte elements
        let split_length = [
            ((column.length >> 24) & 0xFF) as u8,
            ((column.length >> 16) & 0xFF) as u8,
            ((column.length >> 8) & 0xFF) as u8,
            (column.length & 0xFF) as u8,
        ];

        data.extend_from_slice(&split_length);
    }

    return data;
}

pub fn deserialize_table(mut data: Vec<u8>) -> Table {
    // Initialize table
    let mut deserialized_table = Table {
        line_size: 0_u32,
        columns: Vec::new(),
    };

    // This method returns the first part of the Vec and puts the second part back into the variable,
    // which is exactly the opposite of what I need and why this part of the code looks so cursed
    //
    // Get the size of a line in the table and pass the rest of the data to the column data variable
    let mut column_data = data.split_off(4);

    // Join 4 1-byte elements into a 4 byte integer
    deserialized_table.line_size = 
        (data[0] as u32) << 24
        | (data[1] as u32) << 16
        | (data[2] as u32) << 8
        | (data[3] as u32);

    // Check if there's any data for the remaining columns (each column is 37 bytes)
    while column_data.len() >= 37 {
        // Get the name of the column and pass the rest of the data to the remaining_data variable
        let mut name = [0_u8; 32];
        let remaining_data = column_data.split_off(32);

        name.copy_from_slice(&column_data);

        let mut data_iter = remaining_data.into_iter();

        let data_type = data_iter.next().unwrap();

        // Join 2 1-byte elements into a 2 byte integer
        let length = (
            (data_iter.next().unwrap() as u32) << 24) 
            | ((data_iter.next().unwrap() as u32) << 16)
            | ((data_iter.next().unwrap() as u32) << 8)
            | (data_iter.next().unwrap() as u32);

        let column = Column {
            name,
            data_type,
            length,
        };

        deserialized_table.columns.push(column);

        // Collect the data for the remaining columns back into a Vec and put it in the column_data variable
        column_data = data_iter.collect();
    }

    deserialized_table
}
