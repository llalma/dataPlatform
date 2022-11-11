extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::console; //Get console log
use std::str;

use polars_core::prelude::*;
use polars_lazy::prelude::*; // import polars
// use polars_core::prelude::*;
// use polars::prelude::LazyCsvReader;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: String);
}

use wasm_bindgen::JsCast;
use std::io::Cursor;
use js_sys::JsString;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn upload_excel(file_input : web_sys::HtmlInputElement){
    let filelist = file_input.files().expect("Failed to get filelist from File Input!");
    let file = filelist.get(0);

    match file {
        Some(ref file) => {

            // let df = LazyCsvReader::new(file)
            //     .has_header(true)
            //     .finish();
            //
            // console::log_1(df.lazy().select([count()]).collect().unwrap())

            // Convert js promise to rust future then get the response.
            let result: Result<JsValue, JsValue> = JsFuture::from(file.text()).await;

            // Use the result
            if let Ok(content) = result {
                console::log_1(&content)
            }
        },
        None => {
            console::log_1(&"file is Missing".into());
        }
    }
}

#[wasm_bindgen]
pub fn test(){

    let df = df! {
         "column_a" => &[1, 2, 3, 4, 5],
         "column_b" => &["a", "b", "c", "dh", "e"]
    }.unwrap();

    console::log_1(&df.column("column_b").unwrap().to_string().into())

}