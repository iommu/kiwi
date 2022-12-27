use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use schematic::Schematic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod schematic;

fn draw(context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), schematic: &Schematic) {
    context.clear_rect(0.0, 0.0, 640.0, 480.0);
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(pos.0 + 75.0, pos.1 + 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(pos.0 + 110.0, pos.1 + 75.0);
    context
        .arc(pos.0 + 75.0, pos.1 + 75.0, 35.0, 0.0, f64::consts::PI)
        .unwrap();

    // Draw the left eye.
    context.move_to(pos.0 + 65.0, pos.1 + 65.0);
    context
        .arc(pos.0 + 60.0, pos.1 + 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(pos.0 + 95.0, pos.1 + 65.0);
    context
        .arc(pos.0 + 90.0, pos.1 + 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();
    schematic.draw(context, pos);

    context.stroke();
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
    let delta = Rc::new(Cell::new((0.0f64, 0.0f64)));
    let schematic = Schematic::new(file);
    draw(&context, (0.0, 0.0), &schematic);
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let coords = coords.clone();
        // let schematic = schematic.clone();
        let delta = delta.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                coords.set((
                    coords.get().0 + event.offset_x() as f64 - delta.get().0,
                    coords.get().1 + event.offset_y() as f64 - delta.get().1,
                ));
                delta.set((event.offset_x() as f64, event.offset_y() as f64));
                draw(&context, coords.get(), &schematic);
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

    Ok(())
}
