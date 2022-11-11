// #[macro_use]
// extern crate fstrings;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::console; //Get console log
use std::cmp;

// Polars
// use polars_core::prelude::*;
// use polars_lazy::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
Enums
*/
#[wasm_bindgen]
#[derive(Clone)]
pub enum DataType {
    String,
    Integer,
    Boolean
}

/*
Structs
*/
#[wasm_bindgen]
pub struct Grid {
    height: usize,
    width: usize,
    default_value: Cell,
    headers: Vec<String>,
    data: Vec<Vec<Cell>>
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            height: size_x,
            width: size_y,
            default_value: Cell{data_type: DataType::String, data:"".to_string()},
            headers: vec!["".to_string(); size_y],
            data : vec![vec![Cell{data_type: DataType::String, data:"".to_string()}; size_y]; size_x],
        }
    }

    pub fn update_height_width( &mut self, size_x: usize, size_y: usize){

        // Increase Y vectors
        if (&size_y - &self.width) > 0{
            self.width =  size_y.clone();
            for x in 0..&self.width+1 {
                self.data[x].resize(size_y.clone(), self.default_value.clone());
            }
        }

        // Increase X vectors
        if ( &size_x - &self.height) > 0{
            self.height = size_x.clone();
            let x = vec![self.default_value.clone(); size_y];
            self.data.resize(size_x, x);
        }

    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        return self.height.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        return self.width.clone()
    }

    pub fn set_header(&mut self, index: usize, data: String){
        self.headers[index] = data
    }

    pub fn get_header(self, index: usize) -> String {
        return format!("header: {0}", self.headers[index].to_string())
    }

    pub fn set_cell(&mut self, coord:Coordinate, data:String){
        self.data[coord.x][coord.y].set_data(DataType::String, data)
    }

    pub fn get_cell(&self, coord:Coordinate) -> String{
        return self.data[coord.x][coord.y].get_data()
    }

    pub fn get_dragged_cells(&self, start: &Coordinate, end: &Coordinate) -> String{
        //Get corners of highlighted box
        let min_x = cmp::min(&start.x, &end.x).clone();
        let max_x = cmp::max(&start.x, &end.x).clone()+1;
        let min_y = cmp::min(&start.y, &end.y).clone();
        let max_y = cmp::max(&start.y, &end.y).clone()+1;

        let mut csv_data: String = "".to_owned();

        // For each cell in the area reset the cell
        for x in min_x..max_x{
            for y in min_y..max_y{
                csv_data.push_str(&format!("\"{0}\",", self.data[x][y].get_data()));
            }
            //Remove last , from line
            //If statement to prevent if first row has not been highlighted resulting in a crash as csv_data has a length of 0
            if csv_data.chars().count() > 0{
                csv_data = csv_data[0..csv_data.len() - 1].to_string();
            }

            //Add new line to string
            csv_data.push_str(&"\n");
        }

        return csv_data.to_string()
    }

    pub fn delete_area(&mut self, start: &Coordinate, end: &Coordinate){

        //Get corners of highlighted box
        let min_x = cmp::min(&start.x, &end.x).clone();
        let max_x = cmp::max(&start.x, &end.x).clone()+1;
        let min_y = cmp::min(&start.y, &end.y).clone();
        let max_y = cmp::max(&start.y, &end.y).clone()+1;

        // For each cell in the area reset the cell
        for x in min_x..max_x{
            for y in min_y..max_y{
                self.data[x][y].reset_cell();
            }
        }
    }

    pub fn to_csv(&self) -> String{
        // 2D vector array of cells to csv string
        return self.data
            .clone()
            .into_iter()
            .map(|r| r
                .clone()
                .into_iter()
                .map(|c| format!("\"{0}\"", c.data))
                .collect::<Vec<String>>().join(","))
            .collect::<Vec<String>>().join("\n");
    }
}

#[wasm_bindgen]
pub struct Coordinate {
    x : usize,
    y : usize
}

#[wasm_bindgen]
impl Coordinate {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize ) -> Self {
        Self {
            x,
            y
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> usize {
        return self.x.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> usize {
        return self.y.clone()
    }

    fn to_string(&self) -> String{
        //Get coordinate values in a simple string
        return format!("x: {0}, y: {1}", &self.x, &self.y)
    }
}

#[wasm_bindgen]
#[derive(Clone)]
struct Cell {
    data_type: DataType,
    data: String //Should I make this a byte array?
}

#[wasm_bindgen]
impl Cell {

   fn is_empty(&self) -> bool {
       return &self.data == ""
   }

    fn get_data(&self) -> String {
        return format!("{0}", &self.data)
    }

    fn set_data(&mut self, data_type: DataType, data: String){
        self.data_type = data_type;
        self.data = data;
    }

    fn reset_cell(&mut self){
        self.data_type = DataType::String;
        self.data = "".to_string();
    }
}


/*
Functions
*/

#[wasm_bindgen]
//pub fn update_cell(location: Coordinate, dataType: DataType, data: &vec<u8>]){
pub fn update_cell(coords: Coordinate){
    console::log_1(&coords.to_string().into());
}