extern crate wasm_bindgen;

use js_sys::Array;
//Get console log
use std::cmp;
use wasm_bindgen::prelude::*;

// use web_sys::console;

// Polars
// use polars_core::prelude::*;
// use polars_lazy::prelude::*;

//Created Packages
#[allow(non_snake_case)]
#[path = "Cell.rs"] mod Cell;
#[allow(non_snake_case)]
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
    data: Vec<Vec<Cell::Cell>>,
    visible_rows: Vec<bool>
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
            visible_rows: vec![true; size_x]
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

    pub fn set_headers(&mut self, data: String){
        data.split(",").enumerate().for_each(|(i, h)| self.set_header(i, h.to_string()));
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

    pub fn get_visible(&self, row_index: usize) -> bool{
        self.visible_rows[row_index].clone()
    }

    pub fn set_visible(&mut self, row_index: usize, value: bool){
        self.visible_rows[row_index] = value;
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
            .filter(|&(row_index, _)| row_index >= min_x && row_index <= max_x && self.get_visible(row_index))  //Check row index is inside the extract and the row is visible
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

    pub fn filter(&mut self, filter_column: String, filter_condition: Array){
        let filtered_index_rows: Vec<usize> = self.data
            .clone()
            .into_iter()
            .enumerate()
            .map(|(row_index, row)|
                 row.into_iter()
                     .enumerate()
                     .filter(|(column_index, cell)| self.headers[column_index.clone()].clone() == filter_column && !filter_condition.includes(&JsValue::from_str(&cell.get_data()), 0) )  //Filter to the specific column that is being filtered o
                     .map(|(_, _)| row_index)
                     .collect::<Vec<usize>>()
            )
            .flatten()
            .collect();

        //Hide the rows which should not be displayed
        filtered_index_rows
            .into_iter()
            .for_each(|r| self.set_visible(r, false));

    }
}

mod tests {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    use crate::Coordinate;

    use super::Grid;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn test_get_header_empty() {
        let expected: String = "Header_0".to_string();
        let test_grid = Grid::new(1, 1);

        assert_eq!(test_grid.get_header(0), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_set_header() {
        let expected: String = "test Value".to_string();
        let mut test_grid = Grid::new(1, 6);

        test_grid.set_header(5, expected.clone());

        assert_eq!(test_grid.get_header(5), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_set_headers() {
        let expected: String = "test,Value,Header".to_string();
        let mut test_grid = Grid::new(1, 3);

        test_grid.set_headers(expected.clone());

        assert_eq!((0..test_grid.width()).into_iter().map(|h| test_grid.get_header(h)).collect::<Vec<String>>(), expected.split(",").map(|s| s.to_string()).collect::<Vec<String>>());
    }

    #[wasm_bindgen_test]
    pub fn test_height() {
        let expected: usize = 8;
        let test_grid = Grid::new(expected.clone(), 4);

        assert_eq!(test_grid.height(), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_width() {
        let expected: usize = 420;
        let test_grid = Grid::new(54, expected.clone());

        assert_eq!(test_grid.width(), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_resize() {
        let expected: Vec<usize> = vec![32, 64];
        let mut test_grid = Grid::new(1, 1);

        test_grid.resize(expected[0], expected[1]);

        assert_eq!(vec![test_grid.height(), test_grid.width()], expected);
    }

    #[wasm_bindgen_test]
    pub fn test_get_cell_empty() {
        let expected: String = "".to_string();
        let test_grid = Grid::new(1, 1);

        assert_eq!(test_grid.get_cell(&Coordinate::Coordinate::new(0, 0)), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_set_cell() {
        let expected: String = "test Value".to_string();
        let mut test_grid = Grid::new(1, 1);

        test_grid.set_cell(Coordinate::Coordinate::new(0, 0), expected.clone());

        assert_eq!(test_grid.get_cell(&Coordinate::Coordinate::new(0, 0)), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_visible_rows() {
        let mut test_grid = Grid::new(2, 2);

        //Test initial value is true
        assert_eq!(test_grid.get_visible(1), true);

        //Set value to false
        test_grid.set_visible(1, false);

        //Test updated value is false
        assert_eq!(test_grid.get_visible(1), false);
    }

    #[wasm_bindgen_test]
    pub fn test_empty_to_string() {
        let test_grid = Grid::new(2, 3);

        let expected: String = ["\"\",\"\",\"\"",
            "\"\",\"\",\"\""].join("\n");

        assert_eq!(test_grid.get_csv_string(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(2, 3)), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_non_empty_to_string() {
        let mut test_grid = Grid::new(1, 6);
        test_grid.set_cell(Coordinate::Coordinate::new(0, 2), "test".to_string());
        test_grid.set_cell(Coordinate::Coordinate::new(0, 5), "hi".to_string());


        let expected: String = ["\"\",\"\",\"test\",\"\",\"\",\"hi\""].join("\n");

        assert_eq!(test_grid.get_csv_string(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(0, 6)), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_paste() {
        let mut test_grid = Grid::new(4, 4);

        let paste_insert = "x,y".to_string();

        test_grid.paste(&Coordinate::Coordinate::new(1, 1), paste_insert);

        let expected: String = ["\"\",\"\",\"\",\"\"",
            "\"\",\"x\",\"y\",\"\"",
            "\"\",\"\",\"\",\"\"",
            "\"\",\"\",\"\",\"\""].join("\n");

        assert_eq!(test_grid.get_csv_string(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(4, 4)), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_get_csv_export() {
        let mut test_grid = Grid::new(2, 3);
        test_grid.set_cell(Coordinate::Coordinate::new(0, 2), "test".to_string());
        test_grid.set_cell(Coordinate::Coordinate::new(1, 2), "hi".to_string());
        test_grid.set_header(2, "test_header".to_string());

        let expected: String = ["Header_0,Header_1,test_header",
            "\"\",\"\",\"test\"",
            "\"\",\"\",\"hi\""].join("\n");

        assert_eq!(test_grid.get_csv_export(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(2, 3)), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_delete_area() {
        let mut test_grid = Grid::new(4, 4);

        let paste_insert = "x,y".to_string();

        test_grid.paste(&Coordinate::Coordinate::new(1, 1), paste_insert);

        let expected: String = ["\"\",\"\",\"\",\"\"",
            "\"\",\"\",\"\",\"\"",
            "\"\",\"\",\"\",\"\"",
            "\"\",\"\",\"\",\"\""].join("\n");

        test_grid.delete_area(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(3, 3));

        assert_eq!(test_grid.get_csv_string(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(3, 3)), expected);
    }

    #[wasm_bindgen_test]
    pub fn test_filter_single() {
        let mut test_grid = Grid::new(4, 2);

        //Set headers
        let column_headers = "x,y";
        test_grid.set_headers(column_headers.to_string());

        //Set data
        let paste_insert = ["2,2",
            "1,4",
            "1,6",
            "7,8"].join("\n").to_string();
        test_grid.paste(&Coordinate::Coordinate::new(0, 0), paste_insert);

        //Filter where "x" == 1
        let filter_value = js_sys::Array::new();
        filter_value.set(0, JsValue::from_str("1"));
        test_grid.filter("x".to_string(), filter_value);

        //Expected value
        let expected = ["\"1\",\"4\"",
            "\"1\",\"6\""].join("\n").to_string();

        //Export filtered results and compare
        assert_eq!(test_grid.get_csv_string(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(3, 1)), expected)
    }

    #[wasm_bindgen_test]
    pub fn test_filter_multiple() {
        let mut test_grid = Grid::new(4, 2);

        //Set headers}
        let column_headers = "x,y";
        test_grid.set_headers(column_headers.to_string());

        //Set data
        let paste_insert = ["2,2",
            "1,4",
            "1,6",
            "7,8"].join("\n").to_string();
        test_grid.paste(&Coordinate::Coordinate::new(0, 0), paste_insert);

        //Filter where "x" == 1
        let filter_value = js_sys::Array::new();
        filter_value.set(0, JsValue::from_str("1"));
        test_grid.filter("x".to_string(), filter_value);

        //Filter where "y" = 4
        let filter_value = js_sys::Array::new();
        filter_value.set(0, JsValue::from_str("4"));        //Expected value
        test_grid.filter("y".to_string(), filter_value);

        let expected = ["\"1\",\"4\""].join("\n").to_string();

        //Export filtered results and compare
        assert_eq!(test_grid.get_csv_string(&Coordinate::Coordinate::new(0, 0), &Coordinate::Coordinate::new(3, 1)), expected)
    }
}