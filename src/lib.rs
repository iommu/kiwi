use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use schematic::Schematic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod schematic;
mod theme;
mod parser;
mod render;


#[wasm_bindgen]
pub fn start(file: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.style().set_property("border", "solid")?;

    let canvas = Rc::new(canvas);
    let scale = Rc::new(Cell::new(2.0f64));
    let schematic = Schematic::from_str(file);
    schematic.draw(&canvas, scale.get());

    {
        let _canvas = canvas.clone();
        let scale = scale.clone();
        let schematic = schematic.clone();
        // let schematic = schematic.clone();
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
