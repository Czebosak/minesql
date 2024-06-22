use std::fs::File;

pub struct Column {
    pub name: [u8; 32],
    pub data_type: u8,
    pub length: u16,
}

pub struct Table {
    pub columns: Vec<Column>
}

pub fn serialize_table(table: Table) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for column in table.columns {
        data.extend_from_slice(&column.name);
        data.push(column.data_type);
        
        let split_length = [(column.length >> 8) as u8, (column.length & 0xFF) as u8];
        data.extend_from_slice(&split_length);
    }

    return data;
}

/* pub fn deserialize_table(data: Vec<u8>) {
    let deserialized_table = Table {
        columns: Vec::new()
    };
    
    let remaining_data = &data;

    while remaining_data.len() > 0 {
        let new_data: &Vec<u8>;
        let column: Column;

        new_data = &remaining_data.split_off(31);
        column.name[..remaining_data.len()].copy_from_slice(remaining_data);

        let data_iter = new_data.into_iter();

        column.data_type = data_iter.next().unwrap().clone();
        column.length = (data_iter.next().unwrap().clone() as u16) << 8 | (data_iter.next().unwrap().clone() as u16);

        deserialized_table.columns.push(column);
        remaining_data = data_iter.collect();
    }

    return deserialized_table;
} */

/* pub fn deserialize_table(mut data: Vec<u8>) -> Table {
    let mut deserialized_table = Table {
        columns: Vec::new(),
    };

    while data.len() > 0 {
        let mut name = [0u8; 32];
        let remaining_data = data.split_off(31);

        // Copy the column name
        name.copy_from_slice(&data[..32]);

        // Prepare to read data type and length
        let mut data_iter = remaining_data.into_iter();

        let data_type = data_iter.next().unwrap();
        let length = ((data_iter.next().unwrap() as u16) << 8) | (data_iter.next().unwrap() as u16);

        let column = Column {
            name,
            data_type,
            length,
        };

        deserialized_table.columns.push(column);

        // Collect the remaining data back into the vector
        data = data_iter.collect();
    }

    deserialized_table
} */

pub fn deserialize_table(mut data: Vec<u8>) -> Table {
    let mut deserialized_table = Table {
        columns: Vec::new(),
    };

    while data.len() > 0 {
        let mut name = [0u8; 32];
        let remaining_data = data.split_off(32);

        name.copy_from_slice(&data[..32]);

        let mut data_iter = remaining_data.into_iter();

        let data_type = data_iter.next().unwrap();
        let length = ((data_iter.next().unwrap() as u16) << 8) | (data_iter.next().unwrap() as u16);

        let column = Column {
            name,
            data_type,
            length,
        };

        deserialized_table.columns.push(column);

        data = data_iter.collect();
    }

    deserialized_table
}