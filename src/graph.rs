use web_sys::CanvasRenderingContext2d;
use crate::calculator::Calculator;
use wasm_bindgen::JsCast;

//colors used to draw function graphs
const COLORS : &[& str] = &["red", "green", "blue", "purple", "orange"];

//function to get the canvas element from HTML document and set up a rendering context for it
pub fn initialize_canvas() -> (web_sys::HtmlCanvasElement, CanvasRenderingContext2d) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
   (canvas, context)
}

//function to clear canvas
pub fn reset_canvas(canvas: &web_sys::HtmlCanvasElement, rendering_context: &CanvasRenderingContext2d) {
    rendering_context.reset_transform().unwrap();
    rendering_context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}

//function to transform context coordinates to align with current graph viewport
pub fn transform_canvas(canvas: &web_sys::HtmlCanvasElement, rendering_context: &CanvasRenderingContext2d, x_start: f64, x_end: f64, y_start: f64, y_end:f64) {
    let x_range = x_end - x_start;
    let y_range = y_end - y_start;
    let x_scale = (canvas.width() as f64) / x_range;
    let y_scale = (canvas.height() as f64) / y_range;
    rendering_context.transform(1.0, 0.0, 0.0, -1.0, 0.0, canvas.height() as f64).unwrap();
    rendering_context.scale(x_scale, y_scale).unwrap();
    rendering_context.translate(-x_start, -y_start).unwrap();
}

//function to draw grid lines on canvas
pub fn draw_initial_grid(rendering_context: &CanvasRenderingContext2d, x_start: f64, x_end: f64, y_start: f64, y_end: f64, step_size: usize) {
    rendering_context.set_stroke_style(&"gray".into());
    let start = (x_start + 1.0).floor() as i32;
    let end = x_end.ceil() as i32;
    //use 1px and 2px lines
    let line_size = 1.0 / (rendering_context.canvas().unwrap().width() as f64 / (x_end - x_start));
    let main_axis_size = 2.0 / (rendering_context.canvas().unwrap().width() as f64 / (x_end - x_start));
    rendering_context.begin_path();
    rendering_context.set_line_width(line_size);
    //draw all vertical minor grid lines
    for i in (start..end).step_by(step_size) {
        if i == 0 { continue; }
        rendering_context.move_to(i as f64, y_start);
        rendering_context.line_to(i as f64, y_end);
    }
    let start = (y_start + 1.0).floor() as i32;
    let end = y_end.ceil() as i32;
    //draw all horizontal minor grid lines
    for i in (start..end).step_by(step_size) {
        if i == 0 { continue; } 
        rendering_context.move_to(x_start, i as f64);
        rendering_context.line_to(x_end, i as f64);
    }    
    rendering_context.stroke();
    rendering_context.begin_path();
    rendering_context.set_line_width(main_axis_size);
    //draw main axes
    rendering_context.move_to(x_start, 0.0);
    rendering_context.line_to(x_end, 0.0);
    rendering_context.move_to(0.0, y_start);
    rendering_context.line_to(0.0, y_end);
    rendering_context.stroke();
}

//function to draw a given function on canvas based on the values provided by the Calculator struct representing that function - caches values for future drawing
pub fn draw_function_graph(rendering_context: &CanvasRenderingContext2d, calculator: &mut Calculator, cache: &mut Vec<(f64, Option<f64>)>, x_start: f64, x_end: f64, y_start: f64, y_end: f64, step_size: f64, idx: usize) {
    rendering_context.set_stroke_style(&COLORS[idx % COLORS.len()].into());
    //0.9px
    let line_size = 0.9 / (rendering_context.canvas().unwrap().width() as f64 / (x_end - x_start));
    rendering_context.set_line_width(line_size);
    let mut x = x_start;
    let mut y;
    //find first point that is within our graph area and set x to it
    while x <= x_end {
        let next_y = calculator.calculate(x + step_size);
        if next_y == None {
            x += step_size;
            continue;
        }
        if let Some(num) = next_y {
            if num < y_start || num > y_end {
                x += step_size;
                continue;
            }
            else {
                break; 
            }
        }
    }
    //represents whether the last point we calculated was inside our current graph viewport or not
    let mut in_graph_area = false;
    rendering_context.begin_path();
    while x <= x_end {
        y = calculator.calculate(x);
        //cache point
        cache.push((x, y));
        match y {
            Some(val) => {
                if val < y_start || val > y_end {
                    if in_graph_area {
                        rendering_context.line_to(x, val);
                        in_graph_area = false;
                    }
                    else {
                        let next_y = calculator.calculate(x + step_size);
                        if let Some(next_val) = next_y {
                            if next_val > y_start && next_val < y_end {
                                in_graph_area = true;
                                rendering_context.move_to(x, val);
                            }
                        }
                    }
                }
                else {
                    if !in_graph_area {
                        in_graph_area = true;
                        rendering_context.move_to(x, val);
                    }
                    else { rendering_context.line_to(x, val); }
                }
            },
            None => {
                in_graph_area = false;
            }
        }
        x += step_size;
    }
    rendering_context.stroke();
}

//function to draw a given function on canvas based on the values provided in that function's cache
pub fn draw_function_graph_from_cache(rendering_context: &CanvasRenderingContext2d, cache: &Vec<(f64, Option<f64>)>, x_start: f64, x_end: f64, y_start: f64, y_end: f64, step_size: f64, idx: usize) {
    rendering_context.set_stroke_style(&COLORS[idx % COLORS.len()].into());
    //0.9px
    let line_size = 0.9 / (rendering_context.canvas().unwrap().width() as f64 / (x_end - x_start));
    rendering_context.set_line_width(line_size);
    //index into cache
    let mut i = ((x_start - cache[0].0) / step_size).floor() as usize;
    let mut x = cache[i].0;
    let mut y;
    //find first point that is within our graph area and set x to it, set i to that cache index
    while x <= x_end && i + 1 < cache.len() {
        let next_y = cache[i + 1].1;
        if next_y == None {
            i += 1;
            x = cache[i].0;
            continue;
        }
        if let Some(num) = next_y {
            if num < y_start || num > y_end {
                i += 1;
                x = cache[i].0;
                continue;
            }
            else {
                break; 
            }
        }
    }
    //represents whether the last point we calculated was inside our current graph viewport or not
    let mut in_graph_area = false;
    rendering_context.begin_path();
    while x <= x_end && i + 1 < cache.len() {
        y = cache[i].1;
        match y {
            Some(val) => {
                if val < y_start || val > y_end {
                    if in_graph_area {
                        rendering_context.line_to(x, val);
                        in_graph_area = false;
                    }
                    else {
                        let next_y = cache[i + 1].1;
                        if let Some(next_val) = next_y {
                            if next_val > y_start && next_val < y_end {
                                in_graph_area = true;
                                rendering_context.move_to(x, val);
                            }
                        }
                    }
                }
                else {
                    if !in_graph_area {
                        in_graph_area = true;
                        rendering_context.move_to(x, val);
                    }
                    else { rendering_context.line_to(x, val); }
                }
            },
            None => {
                in_graph_area = false;
            }
        }
        i += 1;
        x = cache[i].0;
    }
    rendering_context.stroke();
}
