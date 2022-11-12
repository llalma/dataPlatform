extern crate wasm_bindgen;

//Get console log
use std::cmp;
use wasm_bindgen::prelude::*;
use web_sys::console;

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

    /// Resize the grid to the new specified size
    pub fn resize(&mut self, size_x: usize, size_y: usize){

        // Increase Y vectors
        for x in 0..self.height.clone() {
            self.data[x].resize(size_y.clone(), Cell::Cell::new());
        }

        //Resize headers and set newly added columns to default header
        self.headers.resize(size_y, "".to_string());
        self.headers = self.headers
                        .clone()
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| {
                            if i >= self.width {
                                format!("Header_{0}", i.to_string())
                            }else{
                                v
                            }
                        }).collect();

        // Increase X vectors
        let temp_data = vec![Cell::Cell::new(); size_y.clone()];
        self.data.resize(size_x.clone(), temp_data);

        //Set recorded sizes
        self.width =  size_y.clone();
        self.height = size_x.clone();
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

    pub fn get_cell(&self, coord: &Coordinate::Coordinate) -> String{
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

    pub fn get_csv_string(&self, start: &Coordinate::Coordinate, end: &Coordinate::Coordinate) -> String{

        let start_coord = start.clone();
        let end_coord = end.clone();

        // TODO Handling for non full size exports

        //Get corners of highlighted box
        let min_x = cmp::min(&start_coord.x(), &end_coord.x()).clone();
        let max_x = cmp::max(&start_coord.x(), &end_coord.x()).clone();
        let min_y = cmp::min(&start_coord.y(), &end_coord.y()).clone();
        let max_y = cmp::max(&start_coord.y(), &end_coord.y()).clone();

        return self.data
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(row_index, _)| row_index >= min_x && row_index <= max_x )
            .map(|(_, r)| r
                .clone()
                .into_iter()
                .enumerate()
                .filter(|&(column_index, _)| column_index >= min_y && column_index <= max_y )
                .map(|(_, c)| format!("\"{0}\"", c.get_data()))
                .collect::<Vec<String>>().join(","))
            .collect::<Vec<String>>().join("\n");
    }

    pub fn get_csv_export(&self, start: &Coordinate::Coordinate, end: &Coordinate::Coordinate) -> String{

        let start_coord = start.clone();
        let end_coord = end.clone();

        // TODO Handling for non full size exports

        let min_y = cmp::min(&start_coord.y(), &end_coord.y()).clone();
        let max_y = cmp::max(&start_coord.y(), &end_coord.y()).clone();

        let header_string: String = self.headers
                                    .clone()
                                    .into_iter()
                                    .enumerate()
                                    .filter(|&(column_index, _)| column_index >= min_y && column_index <= max_y )
                                    .map(|(_, r)| r)
                                    .collect::<Vec<String>>().join(",");

        return format!("{0}\n{1}", header_string, self.get_csv_string(start, end))

    }
    pub fn paste(&mut self, start: &Coordinate::Coordinate, data: String){
        // Pastes from top left corner if highlighted area it is ignored and just pastest anyway even outside of highlighted arewa

        for (x, row_data) in data.split("\n").enumerate(){
            for (y, column_data) in row_data.split(",").enumerate() {
                self.data[&start.x() + x][&start.y() + y].set_data(Cell::DataType::String, column_data.to_string().replace("\"", ""));
            }
        }
    }
}

