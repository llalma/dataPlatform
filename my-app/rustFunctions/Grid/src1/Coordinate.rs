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

    fn to_string(&self) -> String{
        //Get coordinate values in a simple string
        return format!("x: {0}, y: {1}", &self.x, &self.y)
    }
}

mod tests{
    use wasm_bindgen_test::*;

    use super::Coordinate;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn test_x() {
        let v1 : usize = 756;
        let v2 : usize  = 2;

        let test_coordinate = Coordinate::new(v1.clone(), v2.clone());

        assert_eq!(test_coordinate.x(), v1);
    }

    #[wasm_bindgen_test]
    pub fn test_y() {
        let v1 : usize = 756;
        let v2 : usize  = 2;

        let test_coordinate = Coordinate::new(v1.clone(), v2.clone());

        assert_eq!(test_coordinate.y(), v2);
    }

    #[wasm_bindgen_test]
    pub fn test_to_string() {
        let v1 : usize = 0;
        let v2 : usize  = 69;

        let test_coordinate = Coordinate::new(v1.clone(), v2.clone());

        let expected = format!("x: {0}, y: {1}", v1, v2);

        assert_eq!(test_coordinate.to_string(), expected);
    }
}