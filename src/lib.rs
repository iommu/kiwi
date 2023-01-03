use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use schematic::Schematic;
use schematic::Point;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
mod schematic;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
pub fn start(file: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.style().set_property("border", "solid")?;

    let canvas = Rc::new(canvas);
    let pressed = Rc::new(Cell::new(false));
    let scale = Rc::new(Cell::new(2.0f64));
    let delta = Rc::new(Cell::new((0.0f64, 0.0f64)));
    let schematic = Schematic::new(file);
    schematic.draw(&canvas, scale.get());

    {
        let _canvas = canvas.clone();
        let pressed = pressed.clone();
        let scale = scale.clone();
        let schematic = schematic.clone();
        // let schematic = schematic.clone();
        let delta = delta.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            event.prevent_default();
            scale.set(scale.get()+ (event.delta_y() as f64 / 500.0));
            schematic.draw(&_canvas, scale.get());
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
