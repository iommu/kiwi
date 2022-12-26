use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Pos {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

type Points = Vec<Point>;

pub struct Stroke {
    pub width: f64,
    pub s_type: u8,              // todo , reserved word?
    pub color: (u8, u8, u8, u8), // todo real color obj
}

type UUID = String; // todo : real uuid obj

pub struct Wire {
    pub points: Points,
    pub stroke: Stroke,
    pub uuid: UUID,
}

impl Wire {
    fn new(points: Points) -> Wire {
        Wire {
            points: points,
            stroke: Stroke {
                width: 1.0,
                s_type: 1,
                color: (0, 0, 0, 0),
            },
            uuid: "hello".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64)) {
        // draw point to point using stroke
        context.move_to(pos.0 + self.points[0].x, pos.1 + self.points[0].y);
        for point in &self.points {
            context.line_to(pos.0 + point.x, pos.1 + point.y);
        }
    }
}

pub struct Schematic {
    pub wires: Vec<Wire>,
}

impl Schematic {
    pub fn new() -> Schematic {
        let wires = vec![Wire::new(vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 100.0, y: 100.0 },
            Point { x: 100.0, y: 200.0 },
        ])];
        Schematic { wires: wires }
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64)) {
        for wire in &self.wires {
            wire.draw(context, pos);
        }
    }
}
