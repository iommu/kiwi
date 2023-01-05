use std::collections::HashMap;
use std::f64;
use std::fs;
use symbolic_expressions;
use symbolic_expressions::Sexp;
use wasm_bindgen::describe::F64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub a: f64,
}

impl Point {
    pub fn blank() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            a: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StrokeFormat {
    dash,
    dash_dot,
    dash_dot_dot,
    dot,
    default,
    solid,
}

#[derive(Debug, Clone)]
pub struct Stroke {
    pub width: f64,
    pub format: StrokeFormat,
    pub color: (u8, u8, u8, u8), // todo real color obj
}

impl Stroke {
    fn blank() -> Stroke {
        Stroke {
            width: 0.0,
            format: StrokeFormat::default,
            color: (0, 0, 0, 0),
        }
    }
}

type UUID = String; // todo : real uuid obj

#[derive(Debug, Clone)]
pub struct Wire {
    pub poss: Vec<Point>,
    pub stroke: Stroke,
    pub uuid: UUID,
}

impl Wire {
    fn blank() -> Wire {
        Wire {
            poss: Vec::<Point>::new(),
            stroke: Stroke::blank(),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        // draw pos to pos using stroke
        // todo : ensure that vector exists
        context.move_to(
            (self.poss[0].x) * scale,
            (self.poss[0].y) * scale,
        );
        for point in &self.poss {
            context.line_to((point.x) * scale, (point.y) * scale);
        }
    }
}

#[derive(Debug, Clone)]
pub enum FillType {
    none,
    outline,
    background,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub poss: (Point, Point),
    pub stroke: Stroke,
    pub fill: FillType,
    pub uuid: UUID,
}

impl Rect {
    fn blank() -> Rect {
        Rect {
            poss: (Point::blank(), Point::blank()),
            stroke: Stroke::blank(),
            fill: FillType::none,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        // draw pos to pos using stroke
        context.move_to(
            (self.poss.0.x) * scale + pos.x,
            (self.poss.0.y) * scale + pos.y,
        );

        match self.fill {
            FillType::background => {
                context.set_fill_style(&JsValue::from("orange"));
                context.fill_rect(
                    (self.poss.0.x) * scale + pos.x,
                    (self.poss.0.y) * scale + pos.y,
                    (self.poss.1.x - self.poss.0.x) * scale,
                    (self.poss.1.y - self.poss.0.y) * scale, //todo : why?
                );
            }
            _ => {
                context.rect(
                    (self.poss.0.x) * scale + pos.x,
                    (self.poss.0.y) * scale + pos.y,
                    (self.poss.1.x - self.poss.0.x) * scale,
                    (self.poss.1.y - self.poss.0.y) * scale, //todo : why?
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Circ {
    pub pos: Point,
    pub radius: f64,
    pub stroke: Stroke,
    pub fill: u8,
    pub uuid: UUID,
}

impl Circ {
    fn blank() -> Circ {
        Circ {
            pos: Point::blank(),
            radius: 0.0,
            stroke: Stroke::blank(),
            fill: 0,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        console_log!("DRAWING CIRC");
        // draw pos to pos using stroke
        context.move_to(
            (self.pos.x) * scale + pos.x + (self.radius * scale),
            (self.pos.y) * scale + pos.y + (self.radius * scale),
        );
        context.arc(
            (self.pos.x) * scale + pos.x,
            (self.pos.y) * scale + pos.y,
            (self.radius) * scale,
            0.0,
            f64::consts::PI * 2.0,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Junction {
    pub pos: Point,
    pub diameter: f64,
    pub color: (u8, u8, u8, u8),
    pub uuid: UUID,
}

impl Junction {
    fn blank() -> Junction {
        Junction {
            pos: Point {
                x: 0.0,
                y: 0.0,
                a: 0.0,
            },
            diameter: 1.0,
            color: (0, 0, 0, 0),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        // todo : move pos based on diam
        context.move_to((self.pos.x) * scale + pos.x, (self.pos.y) * scale + pos.y);
        context.arc(
            (self.pos.x) * scale + pos.x,
            (self.pos.y) * scale + pos.y,
            ((self.diameter + 0.2) * 1.0) * scale,
            0.0,
            f64::consts::PI * 2.0,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub pos: Point,
    // todo : effect
    pub uuid: UUID,
}

impl Text {
    fn blank() -> Text {
        Text {
            text: "".to_string(),
            pos: Point {
                x: 0.0,
                y: 0.0,
                a: 0.0,
            },
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        // todo : move pos based on diam
        let angle = pos.a + self.pos.a;
        context.move_to((self.pos.x) * scale + pos.x, (self.pos.y) * scale + pos.y);
        // if angle != 0.0 {
        //     context.save();
        //     context.rotate(angle);
        // }
        context.set_font(format!("{}px monospace", (2.0 * scale) as i32).as_str());
        context.fill_text(
            self.text.as_str(),
            (self.pos.x) * scale + pos.x,
            (self.pos.y) * scale + pos.y,
        );
        // if angle != 0.0 {
        //     context.restore();
        // }
    }
}

#[derive(Debug, Clone)]
pub struct Effect {}

impl Effect {
    fn blank() -> Effect {
        Effect {}
    }
}

#[derive(Debug, Clone)]
pub struct Polyline {
    pub poss: Vec<Point>,
    pub stroke: Stroke,
    pub uuid: UUID,
}

impl Polyline {
    fn blank() -> Polyline {
        Polyline {
            poss: Vec::<Point>::new(),
            stroke: Stroke::blank(),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        // draw pos to pos using stroke
        context.move_to(
            (self.poss[0].x) * scale + pos.x,
            (self.poss[0].y) * scale + pos.y,
        );
        for point in &self.poss {
            console_log!(
                "drawing line : {}:{}",
                (point.x) * scale + pos.x,
                (point.y) * scale + pos.y
            );
            context.set_line_dash(&JsValue::from(""));
            context.set_stroke_style(&JsValue::from(format!(
                "rgba({}, {}, {}, {})",
                self.stroke.color.0, self.stroke.color.0, self.stroke.color.2, 255
            )));
            context.line_to((point.x) * scale + pos.x, (point.y) * scale + pos.y);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub poss: (Point, Point, Point), /* start, mid, end*/
    pub stroke: Stroke,
    pub fill: bool,
    pub uuid: UUID,
} // todo

impl Arc {
    fn blank() -> Arc {
        Arc {
            poss: (Point::blank(), Point::blank(), Point::blank()),
            stroke: Stroke::blank(),
            fill: false,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        // draw pos to pos using stroke

        // triiggg
        let radius = f64::sqrt(
            f64::powf(self.poss.0.x - self.poss.1.x, 2.0)
                + f64::powf(self.poss.0.y - self.poss.1.y, 2.0),
        );
        let angle_start = f64::atan2(self.poss.0.y - self.poss.1.y, self.poss.0.x - self.poss.1.x);
        let angle_stop = f64::atan2(self.poss.2.y - self.poss.1.y, self.poss.2.x - self.poss.1.x);
        context.move_to(
            (self.poss.0.x) * scale + pos.x,
            (self.poss.0.y) * scale + pos.y,
        );
        context.arc(
            (self.poss.1.x) * scale + pos.x,
            (self.poss.1.y) * scale + pos.y,
            radius * scale,
            angle_start,
            angle_stop,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Noconnect {
    pub pos: Point,
    pub uuid: UUID,
}

impl Noconnect {
    fn blank() -> Noconnect {
        Noconnect {
            pos: Point {
                x: 0.0,
                y: 0.0,
                a: 0.0,
            },
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        // draws an "x"
        let size = 1.0;
        context.move_to(
            (self.pos.x - size) * scale + pos.x,
            (self.pos.y - size) * scale + pos.y,
        );
        context.line_to(
            (self.pos.x + size) * scale + pos.x,
            (self.pos.y + size) * scale + pos.y,
        );
        context.move_to(
            (self.pos.x - size) * scale + pos.x,
            (self.pos.y + size) * scale + pos.y,
        );
        context.line_to(
            (self.pos.x + size) * scale + pos.x,
            (self.pos.y - size) * scale + pos.y,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: String,
    pub id: i32,
    pub pos: Point,
    // todo : effect
}

impl Property {
    fn blank() -> Property {
        // todo : rotate
        Property {
            key: "".to_string(),
            value: "".to_string(),
            id: 0,
            pos: Point::blank(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pin {
    // type "passive"
    // type2? "line"
    pub pos: Point,
    pub len: f64,
    pub name: (String, Effect),
    pub numb: (i32, Effect),
}

impl Pin {
    fn blank() -> Pin {
        Pin {
            pos: Point::blank(),
            len: 0.0,
            name: ("".to_string(), Effect::blank()),
            numb: (0i32, Effect::blank()),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        let angle = (self.pos.a + pos.a) / 180.0 * f64::consts::PI;
        context.translate((self.pos.x) * scale + pos.x, (self.pos.y) * scale + pos.y);
        context.rotate(angle);
        context.move_to(0.0, 0.0);
        context.line_to((self.len) * scale, 0.0);
        context.rotate(-angle);
        context.translate(-((self.pos.x) * scale + pos.x), -((self.pos.y) * scale + pos.y));
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: String,
    pub lines: Vec<Polyline>,
    pub arcs: Vec<Arc>,
    pub pins: Vec<Pin>,
    pub rects: Vec<Rect>,
    pub circs: Vec<Circ>,
}

impl Symbol {
    fn blank() -> Symbol {
        Symbol {
            id: "".to_string(),
            lines: Vec::<Polyline>::new(),
            arcs: Vec::<Arc>::new(),
            pins: Vec::<Pin>::new(),
            rects: Vec::<Rect>::new(),
            circs: Vec::<Circ>::new(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        console_log!("drawing {}", self.id);
        for line in &self.lines {
            line.draw(context, pos.clone(), scale);
        }
        for arc in &self.arcs {
            arc.draw(context, pos.clone(), scale);
        }
        for pin in &self.pins {
            pin.draw(context, pos.clone(), scale);
        }
        for rect in &self.rects {
            rect.draw(context, pos.clone(), scale);
        }
        for circ in &self.circs {
            circ.draw(context, pos.clone(), scale);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTemp {
    pub id: String,
    pub props: Vec<Property>,
    pub pos: Point, // todo : no pos on template
    pub symbs: Vec<Symbol>,
    pub uuid: UUID,
}

impl SymbolTemp {
    fn blank() -> SymbolTemp {
        SymbolTemp {
            id: "".to_string(),
            props: Vec::<Property>::new(),
            pos: Point::blank(), // todo : not a thing
            symbs: Vec::<Symbol>::new(),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        for symb in &self.symbs {
            symb.draw(context, pos.clone(), scale)
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolInst {
    pub id: String,
    pub parent: Option<SymbolTemp>,
    pub props: Vec<Property>,
    pub pos: Point,
    pub mirror: (bool, bool),
    pub uuid: UUID,
}

impl SymbolInst {
    fn blank() -> SymbolInst {
        SymbolInst {
            id: "".to_string(),
            parent: None,
            props: Vec::<Property>::new(),
            pos: Point::blank(),
            mirror: (false, false),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: Point, scale: f64) {
        if self.parent.is_some() {
            let angle = (self.pos.a + pos.a) / 180.0 * f64::consts::PI;
            context.translate((self.pos.x) * scale + pos.x, (self.pos.y) * scale + pos.y);
            context.scale(1.0, (self.mirror.1 as i32 as f64 * 2.0 - 1.0));
            context.rotate(angle);
            // console_log!("id {} mirror {}:{}", self.id, (self.mirror.0 as i32 as f64 * 2.0 - 1.0), (self.mirror.1 as i32 as f64 * 2.0 - 1.0));
            self.parent.as_ref().unwrap().draw(context, Point::blank(), scale);
            context.rotate(-angle);
            context.scale(1.0, (self.mirror.1 as i32 as f64 * 2.0 - 1.0));
            context.translate(-((self.pos.x) * scale + pos.x), -((self.pos.y) * scale + pos.y));
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub id: String,
    pub shape: u8, // todo : enum
    pub pos: Point,
    // todo : effects
    pub uuid: UUID,
}

// types : 0x00 = heir:
// types : 0x10 = glob:
// types : 0x20 = local:
// types : 0x30 = noconn:

impl Label {
    fn blank() -> Label {
        Label {
            id: "".to_string(),
            shape: 0,
            pos: Point::blank(),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        // draws an label based on label type
        // todo : type based rendering
        if (self.shape != 0xf0) {
            return;
        }

        let size = 1.0; // todo global size?


        let angle = (self.pos.a) / 180.0 * f64::consts::PI;
        context.translate((self.pos.x) * scale, (self.pos.y) * scale);
        context.rotate(-angle); // why inverse?

        // draw text
        context.move_to(
            0.0,
            0.0,
        );
        // todo flip text at certain rotation
        context.set_font(format!("{}px monospace", (2.0 * scale) as i32).as_str());
        context.fill_text(
            self.id.as_str(),
            (size * 2.5) * scale,
            (1.0) * scale,
        );

        // draw frame
        context.move_to(0.0, 0.0);
        context.line_to(
            (size) * scale,
            (size) * scale,
        );
        context.line_to(
            (size * 2.0) * scale,
            (size) * scale,
        );
        context.line_to(
            (size * 2.0) * scale,
            -(size) * scale,
        );
        context.line_to(
            (size) * scale,
            -(size) * scale,
        );
        context.line_to(0.0, 0.0);
        context.rotate(angle);
        context.translate(-((self.pos.x) * scale), -((self.pos.y) * scale));
    }
}

fn get_name(object: &Sexp) -> &str {
    match object.is_list() {
        true => object.list().unwrap()[0].string().unwrap().as_str(),
        false => object.string().unwrap().as_str(),
        _ => "",
    }
}

pub struct Parser {
    pub sexp: Sexp,
}

impl Parser {
    pub fn new(file: &str) -> Parser {
        Parser {
            sexp: symbolic_expressions::parser::parse_str(file).unwrap(),
        }
    }

    pub fn schematic(obj: &Sexp) -> Schematic {
        let mut schem = Schematic::blank();

        // generic parsers
        let p_pos = |obj: &Sexp| -> Point {
            let xya = obj.list().unwrap();
            let x = xya[1].string().unwrap().parse::<f64>().unwrap();
            let y = xya[2].string().unwrap().parse::<f64>().unwrap();
            let a = if xya.len() >= 4 {
                xya[3].string().unwrap().parse::<f64>().unwrap()
            } else {
                0.0
            };

            return Point { x: x, y: y, a: a };
        };

        let p_junc = |obj: &Sexp| -> Junction {
            let mut junction = Junction::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "at") => {
                        junction.pos = p_pos(obj);
                    }
                    (true, "diameter") => {
                        junction.diameter = obj.list().unwrap()[1]
                            .string()
                            .unwrap()
                            .parse::<f64>()
                            .unwrap();
                    }
                    // todo color
                    (true, "uuid") => {
                        junction.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            //
            junction
        };
        let p_stroke = |obj: &Sexp| -> Stroke {
            let mut stroke = Stroke::blank();
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "width") => {
                        stroke.width = obj.list().unwrap()[1]
                            .string()
                            .unwrap()
                            .parse::<f64>()
                            .unwrap()
                    }
                    (true, "type") => {
                        stroke.format = match obj.list().unwrap()[1].string().unwrap().as_str() {
                            "dash" => StrokeFormat::dash,
                            _ => StrokeFormat::default,
                        };
                    }
                    (true, "color") => {
                        let color = obj.list().unwrap();
                        stroke.color = (
                            color[1].string().unwrap().parse::<u8>().unwrap(),
                            color[2].string().unwrap().parse::<u8>().unwrap(),
                            color[3].string().unwrap().parse::<u8>().unwrap(),
                            color[4].string().unwrap().parse::<u8>().unwrap(),
                        )
                    }
                    _ => {}
                }
            }
            console_log!(
                "stroke : w {} t {} c {},{},{},{}",
                stroke.width,
                0,
                stroke.color.0,
                stroke.color.1,
                stroke.color.2,
                stroke.color.3
            );
            stroke
        };
        let p_poly = |obj: &Sexp| -> Polyline {
            let mut poly = Polyline::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "pts") => {
                        for obj in obj.list().unwrap() {
                            if !obj.is_list() {
                                continue;
                            }
                            poly.poss.push(p_pos(obj));
                        }
                    }
                    (true, "uuid") => {
                        poly.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    (true, "stroke") => {
                        poly.stroke = p_stroke(obj);
                    }
                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            //
            poly
        };
        let p_arc = |obj: &Sexp| -> Arc {
            let mut arc = Arc::blank();

            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "start") => {
                        arc.poss.0 = p_pos(obj);
                    }
                    (true, "mid") => {
                        arc.poss.1 = p_pos(obj);
                    }
                    (true, "end") => {
                        arc.poss.2 = p_pos(obj);
                    }
                    (true, "uuid") => {
                        arc.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            arc
        };
        let p_pin = |obj: &Sexp| -> Pin {
            let mut pin = Pin::blank();
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "at") => {
                        pin.pos = p_pos(obj);
                    }
                    (true, "length") => {
                        if !obj.is_list() {
                            continue;
                        }
                        let xy = obj.list().unwrap();
                        pin.len = xy[1].string().unwrap().parse::<f64>().unwrap();
                    }
                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            pin
        };
        let p_rect = |obj: &Sexp| -> Rect {
            let mut rect = Rect::blank();

            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "start") => {
                        rect.poss.0 = p_pos(obj);
                    }
                    (true, "end") => {
                        rect.poss.1 = p_pos(obj);
                    }
                    (true, "fill") => {
                        rect.fill = match obj.list().unwrap()[1].list().unwrap()[1]
                            .string()
                            .unwrap()
                            .as_str()
                        {
                            "background" => FillType::background,
                            _ => FillType::none,
                        };
                    }
                    (true, "uuid") => {
                        rect.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }

                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            rect
        };
        let p_circ = |obj: &Sexp| -> Circ {
            let mut circ = Circ::blank();

            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "center") => {
                        circ.pos = p_pos(obj);
                    }
                    (true, "radius") => {
                        if obj.is_list() {
                            circ.radius = obj.list().unwrap()[1]
                                .string()
                                .unwrap()
                                .parse::<f64>()
                                .unwrap();
                        }
                    }
                    (true, "uuid") => {
                        circ.uuid = obj.list().unwrap()[1].string().unwrap().clone();
                    }
                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            circ
        };
        let p_text = |obj: &Sexp| -> Text {
            let mut text = Text::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "at") => {
                        text.pos = p_pos(obj);
                    }
                    // todo color
                    (true, "uuid") => {
                        text.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : stroke
                    (false, _) => {
                        text.text = obj.string().unwrap().clone();
                    }
                    _ => {}
                }
            }
            //
            text
        };
        let p_wire = |obj: &Sexp| -> Wire {
            let mut wire = Wire::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "pts") => {
                        for obj in obj.list().unwrap() {
                            if !obj.is_list() {
                                continue;
                            }
                            wire.poss.push(p_pos(obj));
                        }
                    }
                    (true, "uuid") => {
                        wire.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            //
            wire
        };
        let p_nconn = |obj: &Sexp| -> Noconnect {
            let mut nconn = Noconnect::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "at") => {
                        nconn.pos = p_pos(obj);
                    }
                    // todo color
                    (true, "uuid") => {
                        nconn.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : stroke
                    _ => { // should be string
                         //println!("{:?}", name);
                    }
                }
            }
            //
            nconn
        };
        let p_label = |obj: &Sexp| -> Label {
            let mut label = Label::blank();
            //
            let label_name = get_name(obj);
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (false, _) => {
                        label.id = obj.string().unwrap().clone();
                    }
                    (true, "shape") => {
                        match label_name {
                            "hierarchical_label" => label.shape = 0xf0,
                            _ => {}
                        }
                        // todo shape = input...
                    }
                    (true, "at") => {
                        label.pos = p_pos(obj);
                    }
                    (true, "uuid") => {
                        label.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : stroke
                    _ => { // should be string
                         //println!("{:?}", name);
                    }
                }
            }
            //
            label
        };
        let p_symb = |obj: &Sexp| -> Symbol {
            let mut symb = Symbol::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (false, _) => {
                        symb.id = obj.string().unwrap().clone();
                    }
                    (true, "polyline") => {
                        symb.lines.push(p_poly(obj));
                    }
                    (true, "arc") => {
                        symb.arcs.push(p_arc(obj));
                    }
                    (true, "pin") => {
                        symb.pins.push(p_pin(obj));
                    }
                    (true, "rectangle") => {
                        symb.rects.push(p_rect(obj));
                    }
                    (true, "circle") => {
                        symb.circs.push(p_circ(obj));
                    }

                    // todo : (power)
                    // todo : pin_names
                    // todo : offset
                    // todo : in_bom
                    // todo : on_board
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            console_log!("new symbol : {}", symb.id);
            //
            symb
        };
        let p_symb_inst = |obj: &Sexp| -> SymbolInst {
            let mut symb = SymbolInst::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "lib_id") => {
                        symb.id = obj.list().unwrap()[1].string().unwrap().clone();
                    }
                    (true, "property") => {
                        let props = obj.list().unwrap();
                        let mut prop = Property::blank();
                        //
                        prop.key = props[1].string().unwrap().clone();
                        prop.value = props[2].string().unwrap().clone();
                        prop.id = props[3].list().unwrap()[1]
                            .string()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
                        if props[4].is_list() {
                            prop.pos = p_pos(&props[4]);
                        }
                        // todo : effect
                        // todo : "at" parser
                        // todo : pos => pos
                        symb.props.push(prop);
                    }
                    (true, "uuid") => {
                        symb.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    (true, "at") => {
                        symb.pos = p_pos(obj);
                    }
                    (true, "mirror") => {
                        symb.mirror = match obj.list().unwrap()[1].string().unwrap().as_str() {
                            "x" => (true, false),
                            "y" => (false, true),
                            "xy" | "yx" => (true, true),
                            _ => (false, false)
                        }
                    }
                    // todo : (power)
                    // todo : pin_names
                    // todo : offset
                    // todo : in_bom
                    // todo : on_board
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            //
            symb
        };
        let p_symb_temp = |obj: &Sexp| -> SymbolTemp {
            let mut symb = SymbolTemp::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (false, _) => {
                        symb.id = obj.string().unwrap().clone();
                    }
                    (true, "property") => {
                        let props = obj.list().unwrap();
                        let mut prop = Property::blank();
                        //
                        prop.key = props[1].string().unwrap().clone();
                        prop.value = props[2].string().unwrap().clone();
                        prop.id = props[3].list().unwrap()[1]
                            .string()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
                        if props[4].is_list() {
                            prop.pos = p_pos(&props[4]);
                        }
                        // todo : effect
                        // todo : "at" parser
                        // todo : pos => pos
                        symb.props.push(prop);
                    }
                    (true, "symbol") => {
                        symb.symbs.push(p_symb(obj));
                    }
                    (true, "uuid") => {
                        symb.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : (power)
                    // todo : pin_names
                    // todo : offset
                    // todo : in_bom
                    // todo : on_board
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            //
            symb
        };

        // Lets Parse!
        for obj in obj.list().unwrap() {
            //
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (false, "version") => schem.version = obj.string().unwrap().parse::<i32>().unwrap(),
                (true, "lib_symbols") => {
                    for obj in obj.list().unwrap() {
                        let name = get_name(obj);
                        match (obj.is_list(), name) {
                            (true, "symbol") => {
                                let symb = p_symb_temp(obj);
                                schem.lib.insert(symb.id.clone(), symb);
                            }
                            _ => {}
                        }
                    }
                }
                (true, "wire") => schem.wires.push(p_wire(obj)),
                (true, "junction") => schem.juncs.push(p_junc(obj)),
                (true, "text") => schem.texts.push(p_text(obj)),
                (true, "polyline") => schem.polys.push(p_poly(obj)),
                (true, "no_connect") => schem.nocons.push(p_nconn(obj)), // todo : fix naming
                (true, "hierarchical_label") => schem.labels.push(p_label(obj)),
                (true, "symbol") => {
                    let mut symb = p_symb_inst(obj);
                    console_log!("{}", symb.id);
                    symb.parent = Some(schem.lib.get(&symb.id).unwrap().clone());
                    schem.symbs.push(symb);
                }
                _ => {}
            }
        }
        //
        schem
    }
}

#[derive(Debug, Clone)]
pub struct Schematic {
    pub wires: Vec<Wire>,
    pub juncs: Vec<Junction>,
    pub texts: Vec<Text>,
    pub polys: Vec<Polyline>,
    pub nocons: Vec<Noconnect>,
    pub labels: Vec<Label>,
    pub lib: HashMap<String, SymbolTemp>,
    pub symbs: Vec<SymbolInst>,

    //
    pub version: i32,
}

impl Schematic {
    pub fn blank() -> Schematic {
        Schematic {
            wires: Vec::<Wire>::new(),
            juncs: Vec::<Junction>::new(),
            texts: Vec::<Text>::new(),
            polys: Vec::<Polyline>::new(),
            nocons: Vec::<Noconnect>::new(),
            labels: Vec::<Label>::new(),
            lib: HashMap::<String, SymbolTemp>::new(),
            symbs: Vec::<SymbolInst>::new(),
            version: 0i32,
        }
    }

    pub fn new(file: &str) -> Schematic {
        let schem_obj = symbolic_expressions::parser::parse_str(file).unwrap();
        Parser::schematic(&schem_obj)
    }

    pub fn draw(&self, canvas: &web_sys::HtmlCanvasElement, scale: f64) {
        canvas.set_height((1080.0 * scale) as u32);
        canvas.set_width((1080.0 * 1.414 * scale) as u32);
        let scale = scale * 4.0; //todo fix scaling
        let context = &canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.clear_rect(0.0, 0.0, 640.0, 480.0);
        context.begin_path();
        let pos = Point::blank();
        for wire in &self.wires {
            wire.draw(context, scale);
        }
        for junc in &self.juncs {
            junc.draw(context, pos.clone(), scale);
        }
        for text in &self.texts {
            text.draw(context, pos.clone(), scale);
        }
        for poly in &self.polys {
            poly.draw(context, pos.clone(), scale);
        }
        for nocon in &self.nocons {
            nocon.draw(context, pos.clone(), scale);
        }
        for label in &self.labels {
            label.draw(context, scale);
        }
        for symb in &self.symbs {
            symb.draw(context, pos.clone(), scale);
        }
        context.stroke();
    }
}
