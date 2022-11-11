extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::console; //Get console log
use std::cmp;

// Polars
// use polars_core::prelude::*;
// use polars_lazy::prelude::*;

//Created Packages
#[path = "Cell.rs"] mod Cell;
#[path = "Coordinate.rs"] mod Coordinate;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/*
Structs
*/
#[wasm_bindgen]
pub struct Grid {
    height: usize,
    width: usize,
    headers: Vec<String>,
    data: Vec<Vec<Cell::Cell>>
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            height: size_x,
            width: size_y,
            headers: (0..size_y).map(|v| format!("Header_{0}", v.to_string())).collect(),
            data : vec![vec![Cell::Cell::new(); size_y]; size_x],
        }
    }

    pub fn update_height_width( &mut self, size_x: usize, size_y: usize){

        // Increase Y vectors
        if (&size_y - &self.width) > 0{
            self.width =  size_y.clone();
            for x in 0..&self.width+1 {
                self.data[x].resize(size_y.clone(), Cell::Cell::new());
            }
        }

        // Increase X vectors
        if ( &size_x - &self.height) > 0{
            self.height = size_x.clone();
            let x = vec![Cell::Cell::new(); size_y];
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

    pub fn get_header(&self, index: usize) -> String {
        return format!("{0}", self.headers[index].to_string())
    }

    pub fn set_cell(&mut self, coord:Coordinate::Coordinate, data:String){
        self.data[coord.x()][coord.y()].set_data(Cell::DataType::String, data)
    }

    pub fn get_cell(&self, coord:Coordinate::Coordinate) -> String{
        return self.data[coord.x()][coord.y()].get_data()
    }

    pub fn get_dragged_cells(&self, start: &Coordinate::Coordinate, end: &Coordinate::Coordinate) -> String{
        //Get corners of highlighted box
        let min_x = cmp::min(&start.x(), &end.x()).clone();
        let max_x = cmp::max(&start.x(), &end.x()).clone()+1;
        let min_y = cmp::min(&start.y(), &end.y()).clone();
        let max_y = cmp::max(&start.y(), &end.y()).clone()+1;

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

    pub fn delete_area(&mut self, start: &Coordinate::Coordinate, end: &Coordinate::Coordinate){

        //Get corners of highlighted box
        let min_x = cmp::min(&start.x(), &end.x()).clone();
        let max_x = cmp::max(&start.x(), &end.x()).clone()+1;
        let min_y = cmp::min(&start.y(), &end.y()).clone();
        let max_y = cmp::max(&start.y(), &end.y()).clone()+1;

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
                .map(|c| format!("\"{0}\"", c.get_data()))
                .collect::<Vec<String>>().join(","))
            .collect::<Vec<String>>().join("\n");
    }
}

