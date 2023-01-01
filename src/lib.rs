use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use schematic::Schematic;
use schematic::Point;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    let coords = Rc::new(Cell::new((0.0f64, 0.0f64)));
    let scale = Rc::new(Cell::new(2.0f64));
    let delta = Rc::new(Cell::new((0.0f64, 0.0f64)));
    let schematic = Schematic::new(file);
    schematic.draw(&context, Point { x: coords.get().0, y: coords.get().1, a: 0.0 }, scale.get());
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let coords = coords.clone();
        let scale = scale.clone();
        let schematic = schematic.clone();
        let delta = delta.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                coords.set((
                    coords.get().0 + event.offset_x() as f64 - delta.get().0,
                    coords.get().1 + event.offset_y() as f64 - delta.get().1,
                ));
                delta.set((event.offset_x() as f64, event.offset_y() as f64));
                schematic.draw(&context, Point { x: coords.get().0, y: coords.get().1, a: 0.0 }, scale.get());

            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let coords = coords.clone();
        
        let delta = delta.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(true);
            delta.set((event.offset_x() as f64, event.offset_y() as f64));
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let coords = coords.clone();
        let scale = scale.clone();
        let schematic = schematic.clone();
        // let schematic = schematic.clone();
        let delta = delta.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            event.prevent_default();
            scale.set(scale.get()+ (event.delta_y() as f64 / 500.0));
            schematic.draw(&context, Point { x: coords.get().0, y: coords.get().1, a: 0.0 }, scale.get());
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
