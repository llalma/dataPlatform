extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
#[derive(Copy)]
pub enum DataType {
    String,
    Integer,
    Boolean
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cell {
    data_type: DataType,
    data: String //Should I make this a byte array?
}

#[wasm_bindgen]
impl Cell {

    pub fn new() -> Self {
        Self {
            data_type: DataType::String,
            data:"".to_string()
        }
    }

    pub fn is_empty(&self) -> bool {
        return &self.data == ""
    }

    pub fn get_data(&self) -> String {
        return format!("{0}", &self.data)
    }

    pub fn set_data(&mut self, data_type: DataType, data: String){
        self.data_type = data_type;
        self.data = data;
    }

    pub fn reset_cell(&mut self){
        self.data_type = DataType::String;
        self.data = "".to_string();
    }
}

mod tests{
    use wasm_bindgen_test::*;

    use super::Cell;
    use super::DataType;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn test_empty() {
        let test_cell = Cell::new();

        assert_eq!(test_cell.is_empty(), true);
    }

    #[wasm_bindgen_test]
    pub fn test_get_data() {
        let test_cell = Cell::new();

        assert_eq!(test_cell.get_data(), "".to_string());
    }


    #[wasm_bindgen_test]
    pub fn test_set_data() {
        let testing_string = "test".to_string();

        let mut test_cell = Cell::new();

        test_cell.set_data(DataType::String, testing_string.clone());

        assert_eq!(test_cell.get_data(), testing_string);
    }

    #[wasm_bindgen_test]
    pub fn test_reset_cell() {
        let testing_string = "test".to_string();

        let mut test_cell = Cell::new();

        test_cell.set_data(DataType::String, testing_string);

        test_cell.reset_cell();

        assert_eq!(test_cell.get_data(), "");
    }
}