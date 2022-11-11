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
            data:"a".to_string()
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