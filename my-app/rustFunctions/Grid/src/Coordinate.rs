extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
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

    pub fn is_blank(&self) -> bool{
        // Assume unset is -1
        return self.x.clone() as i32 == -1;
    }

    fn to_string(&self) -> String{
        //Get coordinate values in a simple string
        return format!("x: {0}, y: {1}", &self.x, &self.y)
    }
}