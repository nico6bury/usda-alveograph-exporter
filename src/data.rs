
/// Represents a single row with a single value and header.
#[derive(Clone,PartialEq,PartialOrd,Debug,Default)]
pub struct Row {
    pub header: String,
    pub value: f64,
}//end struct Row

impl Row {
    /// Creates a new Row with given header and value
    pub fn new(header: String, value: f64) -> Row {Row{header,value}}
}//end impl for Row

/// Represents all the data from a file.
#[derive(Clone,PartialEq,PartialOrd,Debug,Default)]
pub struct Data {
    pub test_name: String,
    pub row_data: Vec<Row>,
}//end struct Data

impl Data {
    /// Creates a new Data struct with given test_name, empty row_data.
    pub fn new(test_name: String) -> Data {Data{test_name,row_data:Vec::new()}}
    /// Creates a new Data struct with given test_name and row_data.
    pub fn new1(test_name: String, row_data: Vec<Row>) -> Data {Data{test_name,row_data}}
}//end impl Data
