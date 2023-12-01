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
    cache: RefCell<Vec<Vec<(f64, Option<f64>)>>>
}

thread_local! {
    static APP_STATE : RefCell<AppState> = RefCell::new(AppState { calculators: RefCell::new(Vec::new()), context: None, canvas: None, cache: RefCell::new(Vec::new()) });
}

//2^-8
const DELTA : f64 = 0.00390625;
const MAX_CACHE_SIZE : usize = 100000;

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
        s.cache.borrow_mut().clear();
    });
}

//sets up Calculator structs for each function to graph, returns true if successful
#[wasm_bindgen]
pub fn initialize(expressions: JsValue) -> JsValue {
    APP_STATE.with(|state| {
        let mut result = Vec::new();
        let mut s = state.borrow_mut();
        let expressions: Vec<String> = serde_wasm_bindgen::from_value(expressions).unwrap();
        for expression in expressions.iter() {
            let tokens = scan(expression);
            if let Err(_e) = tokens { 
                result.push(false);
                continue;
            }
            let tokens = tokens.unwrap();
            let ast = parse(&tokens);
            if let Err(_e) = ast {
                result.push(false);
                continue;
            }
            let ast = ast.unwrap();
            let calculator = generate_calculator(ast, DELTA * 0.5);
            s.calculators.borrow_mut().push(calculator);
            s.cache.borrow_mut().push(Vec::new());
            result.push(true);
        }
        let (canvas, context) = initialize_canvas();
        s.canvas.replace(canvas);
        s.context.replace(context);
        serde_wasm_bindgen::to_value(&result).unwrap()
    })
}

#[wasm_bindgen]
pub fn expand_cache() {
    APP_STATE.with(|state| {
        let s = state.borrow();
        let mut cache = s.cache.borrow_mut();
        let mut calculators = s.calculators.borrow_mut();
        if cache.len() > 0 {
            for (i, calculator) in calculators.iter_mut().enumerate() {
                if cache[i].len() == 0 || cache[i].len() >= MAX_CACHE_SIZE { continue; }
                let cache_start = cache[i][0].0;
                let cache_end = cache[i][cache[i].len() - 1].0;
                let mut prepend = Vec::new();
                let mut append = Vec::new();
                for j in (1..=((1.0 / DELTA) as usize)).rev() {
                    let x = cache_start - DELTA * j as f64;
                    let y = calculator.calculate(x);
                    prepend.push((x,y));
                }
                for j in 1..=((1.0 / DELTA) as usize) {
                    let x = cache_end + DELTA * j as f64;
                    let y = calculator.calculate(x);
                    append.push((x,y));
                }
                prepend.append(&mut cache[i]);
                prepend.append(&mut append);
                cache[i] = prepend;
            }
        }
    })
}

fn graph_each_function(context: &CanvasRenderingContext2d, x_start: f64, x_end: f64, y_start: f64, y_end: f64)  {
    APP_STATE.with(|state| {
        let s = state.borrow();
        let mut cache = s.cache.borrow_mut();
        for (i, calculator) in s.calculators.borrow_mut().iter_mut().enumerate() {
            if cache.len() > i && cache[i].len() > 0 && cache[i][0].0 <= x_start && cache[i][cache[i].len() - 1].0 >= x_end {
                draw_function_graph_from_cache(context, &cache[i], x_start, x_end, y_start, y_end, DELTA, i)
            }
            else {
                if cache.len() > i { cache[i].clear(); }
                draw_function_graph(context, calculator, &mut cache[i], x_start, x_end, y_start, y_end, DELTA, i);
            }
        }
    })   
}