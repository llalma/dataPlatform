extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;



#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: String);
}

#[wasm_bindgen]
pub fn upload_excel(data: &vec<u8>]){
   
}