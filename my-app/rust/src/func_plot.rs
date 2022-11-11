use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern {
    fn alert(s: String);
}

/// Draw power function f(x) = x^power.
pub fn draw(canvas_id: &str, power: i32, shift_x: f32, shift_y:f32, zoom: i32, plotting_factor:i32) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {

    let left_border = if zoom == 0 {-1f32 + shift_x} else {-1f32+(zoom as f32/10f32) as f32 +shift_x};
    let right_border = if zoom == 0 {1f32 + shift_x} else {1f32-(zoom as f32/10f32) as f32 +shift_x};

    //Get series of data to plot
    let plot_data = LineSeries::new(
        (left_border.floor() as i32 * plotting_factor..=right_border.ceil() as i32 * plotting_factor)
                    .map(|x| x as f32 / plotting_factor as f32)
                    .map(|x| (x, x.powf(power as f32))),
        &RED,
    );

    let bottom_border:f32 = 
        (left_border.floor() as i32 * plotting_factor..=right_border.ceil() as i32 * plotting_factor)
        .map(|x| (x as f32 / plotting_factor as f32).powf(power as f32))
        .min_by(|a, b| a.partial_cmp(b)
        .expect("Tried to compare a NaN")).unwrap() + shift_y;
    
    let top_border:f32 = 
        (left_border.floor() as i32 * plotting_factor..=right_border.ceil() as i32 * plotting_factor)
        .map(|x| (x as f32 / plotting_factor as f32).powf(power as f32))
        .max_by(|a, b| a.partial_cmp(b)
        .expect("Tried to compare a NaN")).unwrap() + shift_y;
   
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(left_border..right_border, bottom_border..top_border)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(plot_data)?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}