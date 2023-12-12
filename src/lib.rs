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

//struct to represent global state
struct AppState {
    calculators: RefCell<Vec<Calculator>>,
    context: Option<CanvasRenderingContext2d>,
    canvas: Option<HtmlCanvasElement>,
    cache: RefCell<Vec<Vec<(f64, Option<f64>)>>>,
    delta: f64
}

//global app state
thread_local! {
    static APP_STATE : RefCell<AppState> = RefCell::new(AppState { 
        calculators: RefCell::new(Vec::new()), 
        context: None, 
        canvas: None, 
        cache: RefCell::new(Vec::new()),
        delta: 0.0009765625
    });
}

const MAX_CACHE_SIZE : usize = 100000;

//main function called from JS
#[wasm_bindgen]
pub fn run(x_start: f64, x_end: f64, y_start: f64, y_end: f64) {
    APP_STATE.with(|state| {
        set_delta(x_start, x_end);
        let s = state.borrow();
        let canvas = s.canvas.as_ref().unwrap();
        let context = s.context.as_ref().unwrap();
        reset_canvas(&canvas, &context);
        transform_canvas(&canvas, &context, x_start, x_end, y_start, y_end);
        draw_initial_grid(&context, x_start, x_end, y_start, y_end, 1);
        graph_each_function(&context, x_start.floor(), x_end.ceil(), y_start, y_end);
        //move 0.5px and redraw each graph to create thicker line without having to set line thickness > 0.9, which causes line joining algorithms to take effect and slows performance
        context.translate(0.5 / (canvas.width() as f64 / (x_end - x_start)), 0.5 / (canvas.height() as f64 / (y_end - y_start))).unwrap();
        graph_each_function(&context, x_start.floor(), x_end.ceil(), y_start, y_end);
    })
}

//function to reset global app state
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

/*sets up Calculator structs for each expression passed in from JS - returns array of booleans indicating whether a Calculator was 
successfully initialize for each struct*/
#[wasm_bindgen]
pub fn initialize(expressions: JsValue) -> JsValue {
    APP_STATE.with(|state| {
        let mut result = Vec::new();
        let mut s = state.borrow_mut();
        //convert array of expression strings from JS array to Rust vector
        let expressions: Vec<String> = serde_wasm_bindgen::from_value(expressions).unwrap();
        //scan, parse, and generate calculator for each expression and add to global state
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
            let calculator = generate_calculator(ast, s.delta * 0.5);
            s.calculators.borrow_mut().push(calculator);
            s.cache.borrow_mut().push(Vec::new());
            result.push(true);
        }
        let (canvas, context) = initialize_canvas();
        s.canvas.replace(canvas);
        s.context.replace(context);
        //convert back to JS value to pass to JS
        serde_wasm_bindgen::to_value(&result).unwrap()
    })
}

//function to precalculate points for the current graphed functions outside of the current visible graph viewport
#[wasm_bindgen]
pub fn expand_cache() {
    APP_STATE.with(|state| {
        let s = state.borrow();
        let mut cache = s.cache.borrow_mut();
        let mut calculators = s.calculators.borrow_mut();
        if cache.len() > 0 {
            for (i, calculator) in calculators.iter_mut().enumerate() {
                //expect each function's cache to already contiain the points in the current graph viewport
                if cache[i].len() == 0 || cache[i].len() >= MAX_CACHE_SIZE { continue; }
                let cache_start = cache[i][0].0;
                let cache_end = cache[i][cache[i].len() - 1].0;
                let mut prepend = Vec::new();
                let mut append = Vec::new();
                //expand cache to the left of current viewport
                for j in (1..=((1.0 / s.delta) as usize)).rev() {
                    let x = cache_start - s.delta * j as f64;
                    let y = calculator.calculate(x);
                    prepend.push((x,y));
                }
                //expand cache to the right of current viewport
                for j in 1..=((1.0 / s.delta) as usize) {
                    let x = cache_end + s.delta * j as f64;
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

//function to graph each function in global state - if that function's cache contains all needed points, use the cache, otherwise, calculate points as we go
fn graph_each_function(context: &CanvasRenderingContext2d, x_start: f64, x_end: f64, y_start: f64, y_end: f64)  {
    APP_STATE.with(|state| {
        let s = state.borrow();
        let mut cache = s.cache.borrow_mut();
        for (i, calculator) in s.calculators.borrow_mut().iter_mut().enumerate() {
            if cache.len() > i && cache[i].len() > 0 && cache[i][0].0 <= x_start && cache[i][cache[i].len() - 1].0 >= x_end {
                draw_function_graph_from_cache(context, &cache[i], x_start, x_end, y_start, y_end, s.delta, i)
            }
            else {
                if cache.len() > i { cache[i].clear(); }
                draw_function_graph(context, calculator, &mut cache[i], x_start, x_end, y_start, y_end, s.delta, i);
            }
        }
    })   
}

//function to set the current step size used for graphing, depending on viewport size
fn set_delta(x_start: f64, x_end: f64) {
    APP_STATE.with(|state| {
        let mut s = state.borrow_mut();
        let mut clear_cache = false;
        if x_end - x_start > 20.0 {
            if s.delta == 0.0009765625 {
                clear_cache = true;
            }
            s.delta = 0.001953125;
        }
        else {
            if s.delta == 0.001953125 {
                clear_cache = true;
            }
            s.delta = 0.0009765625;
        }
        //if we change step size, all cached points are invalidated
        if clear_cache {
            for c in s.cache.borrow_mut().iter_mut() {
                c.clear();
            }
        }
    })
}