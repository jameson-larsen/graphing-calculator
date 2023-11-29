use calculator::{generate_calculator, Calculator};
use parser::parse;
use scanner::scan;
use graph::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::cell::RefCell;

mod scanner;
mod parser;
mod calculator;
mod graph;
struct AppState {
    calculators: RefCell<Vec<Calculator>>,
    context: Option<CanvasRenderingContext2d>,
    canvas: Option<HtmlCanvasElement>,
}

thread_local! {
    static APP_STATE : RefCell<AppState> = RefCell::new(AppState { calculators: RefCell::new(Vec::new()), context: None, canvas: None });
}

const DELTA : f64 = 0.002;

#[wasm_bindgen]
pub fn run(x_start: f64, x_end: f64, y_start: f64, y_end: f64) {
    APP_STATE.with(|state| {
        let s = state.borrow();
        let canvas = s.canvas.as_ref().unwrap();
        let context = s.context.as_ref().unwrap();
        reset_canvas(&canvas, &context);
        transform_canvas(&canvas, &context, x_start, x_end, y_start, y_end);
        draw_initial_grid(&context, x_start, x_end, y_start, y_end, 1);
        graph_each_function(&context, x_start.floor(), x_end.ceil(), y_start, y_end);
    })
}

#[wasm_bindgen]
pub fn reset() {
    APP_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.canvas.take();
        s.context.take();
        s.calculators.borrow_mut().clear();
    });
}

//sets up Calculator structs for each function to graph, returns true if successful
#[wasm_bindgen]
pub fn initialize(expressions: JsValue) -> bool {
    APP_STATE.with(|state| {
        let mut s = state.borrow_mut();
        let expressions: Vec<String> = serde_wasm_bindgen::from_value(expressions).unwrap();
        for expression in expressions.iter() {
            let tokens = scan(expression);
            if let Err(e) = tokens { 
                web_sys::console::log_1(&e.into());
                return false;
            }
            let tokens = tokens.unwrap();
            let ast = parse(&tokens);
            if let Err(e) = ast {
                web_sys::console::log_1(&e.into());
                return false;
            }
            let ast = ast.unwrap();
            let calculator = generate_calculator(ast, DELTA * 0.5);
            s.calculators.borrow_mut().push(calculator);
        }
        let (canvas, context) = initialize_canvas();
        s.canvas.replace(canvas);
        s.context.replace(context);
        true
    })
}

fn graph_each_function(context: &CanvasRenderingContext2d, x_start: f64, x_end: f64, y_start: f64, y_end: f64)  {
    APP_STATE.with(|state| {
        let s = state.borrow();
        for (i, calculator) in s.calculators.borrow_mut().iter_mut().enumerate() {
            draw_function_graph(context, calculator, x_start, x_end, y_start, y_end, DELTA, i);
        }
    })   
}