use field::FlowField;

#[derive(Debug)]
pub enum Record {
    Data(DataRecord),
    OptionData(OptionRecord),
}

impl Record {
    pub fn make_data(fields: Vec<FlowField>) -> Record {
        Record::Data(DataRecord::new(fields))
    }

    pub fn make_option(scopes: Vec<FlowField>, options: Vec<FlowField>) -> Record {
        Record::OptionData(OptionRecord::new(scopes, options))
    }

    // TODO: need test
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Record::Data(data) => data.to_bytes(),
            Record::OptionData(option) => option.to_bytes(),
        }
    }
}

// TODO: accessing method
#[derive(Debug)]
pub struct DataRecord {
    fields: Vec<FlowField>,
}

impl DataRecord {
    fn new(fields: Vec<FlowField>) -> DataRecord {
        DataRecord { fields: fields }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for field in &self.fields {
            bytes.append(&mut field.to_bytes());
        }

        bytes
    }
}

#[derive(Debug)]
pub struct OptionRecord {
    scope_fields: Vec<FlowField>,
    option_fields: Vec<FlowField>,
}

impl OptionRecord {
    fn new(scopes: Vec<FlowField>, options: Vec<FlowField>) -> OptionRecord {
        OptionRecord {
            scope_fields: scopes,
            option_fields: options,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for field in &self.scope_fields {
            bytes.append(&mut field.to_bytes());
        }

        for field in &self.option_fields {
            bytes.append(&mut field.to_bytes());
        }

        bytes
    }
}
