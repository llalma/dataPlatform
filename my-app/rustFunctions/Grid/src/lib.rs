#![feature(cursor_remaining)]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;


// use std::io::Cursor;
use js_sys::JsString;
// Polars
use polars::prelude::*;
use polars::prelude::*;
// use polars::prelude::{CsvEncoding, CsvReader, SerReader};
// use polars::frame::DataFrame;
// use polars_io::prelude::*;
use std::{io::Cursor, panic};
use std::any::type_name;
//Get console log
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::console;
// For getting the filereader from js
// use web_sys::*;
use web_sys::HtmlInputElement;

// use polars_lazy::prelude::col;
use crate::wasm_bindgen::JsCast;

// use polars_io::prelude::*;
// use polars_lazy::prelude::*;

// For saving to CSV
// use polars_core::csv::CsvWriter;
// use crate::StructType;

// use stdweb::web::FileReader;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
Structs
*/
#[wasm_bindgen]
pub struct Grid {
    height: usize,
    width: usize,
    data: DataFrame,
}

#[wasm_bindgen]
impl Grid {

    #[wasm_bindgen(constructor)]

    pub fn new(size_x: usize, size_y: usize) -> Self {
        console_error_panic_hook::set_once();   //For better error messages in the console
        Self {
            height: size_x,
            width: size_y,
            data : df!("Index" => &["0", "1", "2", "4"],
                       "Fruit" => &["Apple", "Apple", "Pear", "dog"],
                       "Color" => &["Red", "Yellow", "Green", "Blue"]).unwrap()

        }
    }

    pub fn get_cell(&self, row_index: usize, column_index: usize) -> String{
        let row = self.data.get(row_index).unwrap();
        let cell = row.get(column_index).unwrap().to_string();

        return cell.clone().replace('"', "").to_string()
    }

    pub fn set_cell(&mut self, row_index: usize, column_index: usize, data: String){

        let mut s1 = self.data.clone().select_at_idx(column_index.clone()).unwrap().utf8().unwrap().clone();

        s1 = s1.clone().into_iter()
            .enumerate()
            .map(|(i,v)| match i {
                idx if idx == row_index => data.to_string(),
                _ => v.unwrap().to_string()
            }).collect();

        self.data.replace_at_idx(column_index, s1).unwrap();
    }

    pub fn to_csv(&self) -> String{

        let mut output = Vec::with_capacity(self.width.clone());
        let mut row_str = Vec::with_capacity(self.height.clone());

        for i in 0..self.height.clone(){

            for j in 0..self.width.clone(){
                row_str.push(self.get_cell(i,j));
            }
            output.push(row_str.join(","));
            row_str = Vec::with_capacity(self.height.clone());
        }

        return output.join("\n")
    }

    fn print_type_of<T>(_: &T) {
        console::log_1(&std::any::type_name::<T>().to_string().into());
    }

    pub fn load_csv(&mut self, buff: &[u8]) -> String {
        let (prefix, shorts, suffix) = unsafe {buff.align_to::<u8>()};

        let cursor = Cursor::new(shorts);



        let lf = CsvReader::new(cursor).with_ignore_parser_errors(true).finish().unwrap().lazy();

        return lf.describe_plan();
    }



    // pub fn load_csv(&mut self, file_input: web_sys::HtmlInputElement) -> Result<JsValue, JsValue>{
    //     let filelist = file_input.files().expect("Failed to get filelist from File Input!");
    //     // if filelist.length() < 1 {
    //     //     return;
    //     // }
    //     // if filelist.get(0) == None {
    //     //     return ;
    //     // }
    //     let file = filelist.get(0).expect("Failed to get File from filelist!");
    //     let file_reader : web_sys::FileReader = match web_sys::FileReader::new() {
    //         Ok(f) => f,
    //         Err(e) => {
    //             web_sys::FileReader::new().expect("")
    //         }
    //     };
    //
    //     let fr_c = file_reader.clone();
    //
    //
    //     return fr_c.result()
    //     //
    //     // return match fr_c.result() {
    //     //     Some(value) => match value {
    //     //         web_sys::FileReaderResult::String(value) => value,
    //     //         _ => String::from("not a text"),
    //     //     }
    //     //     None => String::from("empty")
    //     // }
    //
    //
    //     // let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
    //     //     let data = fr_c.result().unwrap();
    //     //     return data;
    //     //     // console::log_1(&data.as_string().into());
    //     //     let file_string: JsString = data.as_string().unwrap().into();
    //     //     let file_vec: Vec<u8> = file_string.iter().map(|x| x as u8).collect();
    //     //     let cursor = Cursor::new(file_vec);
    //     //     let df = CsvReader::new(cursor)
    //     //         .has_header(true)
    //     //         .finish().unwrap();
    //     // }) as Box<dyn Fn(web_sys::ProgressEvent)>);
    //     //
    //     // file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    //     // file_reader.read_as_array_buffer(&file).expect("blob not readable");
    //     // onloadend_cb.forget();
    //
    // }

}