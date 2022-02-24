// simple container struct to store information
// about a variable.

pub struct Variable {
    pub variable_data: String,
    pub variable_type: u8,
    pub variable_name: String
}

impl Variable {
    pub fn new(name: String, data: String, type_: u8) -> Self {
        return Variable {
            variable_data: data,
            variable_name: name,
            variable_type: type_
        }
    }
}
