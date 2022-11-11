extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::console; //Get console log

// Polars
use polars_core::prelude::*;
use polars_lazy::prelude::*; // import polars

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
Enums
*/
#[wasm_bindgen]
pub enum DataType {
    String = 0,
    Integer = 1,
    Boolean = 2
}

/*
Structs
*/

#[wasm_bindgen]
pub struct Coordinate {
    x : i32
    y : i32
}

#[wasm_bindgen]
struct Cell {
    dataType: DataType
    data: str //Should I make this a byte array?
}

/*
Functions
*/

impl Cell {
    fn set_data(&self, dataType: DataType, data: &vec<u8>){
        println!("hi")
    }
}


#[wasm_bindgen]
//pub fn updateCell(location: Coordinate, dataType: DataType, data: &vec<u8>]){
pub fn updateCell(){
    console::log_1(&"here".into());
}