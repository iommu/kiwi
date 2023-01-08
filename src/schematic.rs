use std::collections::HashMap;
use std::f64;
use symbolic_expressions;
use symbolic_expressions::Sexp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::theme::Theme;

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
pub struct CanvasMod {
    pub scale: f64,
    pub flip: (bool, bool),
}

impl CanvasMod {
    pub fn new() -> CanvasMod {
        CanvasMod {
            scale: 1.0,
            flip: (false, false),
        }
    }
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
    Dash,
    DashDot,
    DashDotDot,
    Dot,
    Default,
    Solid,
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
            format: StrokeFormat::Default,
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        // draw pos to pos using stroke
        if self.poss.is_empty() {
            return;
        } // ensure vec exists
        context.move_to((self.poss[0].x) * cmod.scale, (self.poss[0].y) * cmod.scale);
        for point in &self.poss {
            context.line_to((point.x) * cmod.scale, (point.y) * cmod.scale);
        }
    }
}

#[derive(Debug, Clone)]
pub enum FillType {
    None,
    Outline,
    Background,
}

impl FillType {
    fn begin(&self, context: &web_sys::CanvasRenderingContext2d, color: &JsValue) {
        // todo uses theme instead of color
        context.stroke();
        context.set_fill_style(&color);
        context.begin_path();
    }

    fn end(&self, context: &web_sys::CanvasRenderingContext2d) {
        match self {
            FillType::Background => {
                context.fill();
            }
            FillType::Outline => {
                context.fill();
                context.stroke();
            }
            FillType::None => {
                context.stroke();
            }
            _ => {}
        }
        context.begin_path();
    }
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
            fill: FillType::None,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        // draw pos to pos using stroke
        self.fill.begin(context, &JsValue::from("orange"));

        context.move_to(self.poss.0.x * cmod.scale, self.poss.0.y * cmod.scale);
        context.rect(
            (self.poss.0.x) * cmod.scale,
            (self.poss.0.y) * cmod.scale,
            (self.poss.1.x - self.poss.0.x) * cmod.scale,
            (self.poss.1.y - self.poss.0.y) * cmod.scale, //todo : why?
        );

        self.fill.end(context);
    }
}

#[derive(Debug, Clone)]
pub struct Circ {
    pub pos: Point,
    pub radius: f64,
    pub stroke: Stroke,
    pub fill: FillType,
    pub uuid: UUID,
}

impl Circ {
    fn blank() -> Circ {
        Circ {
            pos: Point::blank(),
            radius: 0.0,
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        self.fill.begin(context, &JsValue::from("black"));

        // draw pos to pos using stroke
        context.move_to((self.pos.x + self.radius) * cmod.scale, self.pos.y * cmod.scale);
        context.arc(
            self.pos.x * cmod.scale,
            self.pos.y * cmod.scale,
            self.radius * cmod.scale,
            0.0,
            f64::consts::PI * 2.0,
        );

        self.fill.end(context);
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        // todo : move pos based on diam
        context.move_to(self.pos.x * cmod.scale, self.pos.y * cmod.scale);
        context.arc(
            self.pos.x * cmod.scale,
            self.pos.y * cmod.scale,
            ((self.diameter + 0.2) * 1.0) * cmod.scale,
            0.0,
            f64::consts::PI * 2.0,
        );
        context.fill();
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        let angle = self.pos.a / 180.0 * f64::consts::PI;
        context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale);
        context.set_font(format!("{}px monospace", (1.8 * cmod.scale) as i32).as_str());
        if angle > f64::consts::PI * 0.5 && angle <= f64::consts::PI * 1.5 {
            context.rotate(-angle - f64::consts::PI); // half rotate to flip text
            context.set_text_align("right");
            for (index, newline) in self.text.split("\\n").enumerate() {
                context.fill_text(newline, 0.0, (1.8 * index as f64) * cmod.scale);
            }
            context.rotate(f64::consts::PI); // finish rotation
        } else {
            context.rotate(-angle); // why inverse?
            context.set_text_align("left");
            for (index, newline) in self.text.split("\\n").enumerate() {
                context.fill_text(newline, 0.0, (1.8 * index as f64) * cmod.scale);
            }
        }
        context.rotate(angle);
        context.translate(-((self.pos.x) * cmod.scale), -((self.pos.y) * cmod.scale));
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
    pub fill: FillType,
    pub uuid: UUID,
}

impl Polyline {
    fn blank() -> Polyline {
        Polyline {
            poss: Vec::<Point>::new(),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        self.fill.begin(context, &JsValue::from("black"));
        // draw pos to pos using stroke
        if self.poss.is_empty() {
            return;
        } // break if none
        context.move_to(self.poss[0].x * cmod.scale, self.poss[0].y * cmod.scale);
        for point in &self.poss {
            match self.stroke.format {
                StrokeFormat::Default => {
                    context.set_line_dash(&js_sys::Array::new());
                }
                _ => {
                    let x = js_sys::Array::new();
                    x.push(&JsValue::from_f64(2.0 * cmod.scale));
                    x.push(&JsValue::from_f64(2.0 * cmod.scale));
                    context.set_line_dash(&x);
                }
            }

            // context.set_stroke_style(&JsValue::from(format!(
            //     "rgba({}, {}, {}, {})",
            //     self.stroke.color.0, self.stroke.color.0, self.stroke.color.2, 255
            // )));
            context.line_to(point.x * cmod.scale, point.y * cmod.scale);
        }
        self.fill.end(context);
    }
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub poss: (Point, Point, Point), /* start, mid, end*/
    pub stroke: Stroke,
    pub fill: FillType,
    pub uuid: UUID,
} // todo

impl Arc {
    fn blank() -> Arc {
        Arc {
            poss: (Point::blank(), Point::blank(), Point::blank()),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        // draw pos to pos using stroke
        self.fill.begin(context, &JsValue::from("black"));

        {
            let line1_angle =
                f64::atan2(self.poss.1.y - self.poss.0.y, self.poss.1.x - self.poss.0.x)
                    + f64::consts::PI / 2.0;
            let line2_angle =
                f64::atan2(self.poss.2.y - self.poss.1.y, self.poss.2.x - self.poss.1.x)
                    + f64::consts::PI / 2.0;
            let line1_mid = Point {
                x: (self.poss.1.x + self.poss.0.x) / 2.0,
                y: (self.poss.1.y + self.poss.0.y) / 2.0,
                a: 0.0,
            };
            let line2_mid = Point {
                x: (self.poss.2.x + self.poss.1.x) / 2.0,
                y: (self.poss.2.y + self.poss.1.y) / 2.0,
                a: 0.0,
            };

            let Ax1 = line1_mid.x;
            let Ay1 = line1_mid.y;
            let Ax2 = line1_mid.x + 10.0;
            let Ay2 = f64::tan(line1_angle) * 10.0 + line1_mid.y;

            let Bx1 = line2_mid.x;
            let By1 = line2_mid.y;
            let Bx2 = line2_mid.x + 10.0;
            let By2 = f64::tan(line2_angle) * 10.0 + line2_mid.y;

            let d = (By2 - By1) * (Ax2 - Ax1) - (Bx2 - Bx1) * (Ay2 - Ay1);
            let uA = ((Bx2 - Bx1) * (Ay1 - By1) - (By2 - By1) * (Ax1 - Bx1)) / d;
            let uB = ((Ax2 - Ax1) * (Ay1 - By1) - (Ay2 - Ay1) * (Ax1 - Bx1)) / d;

            let d = (By2 - By1) * (Ax2 - Ax1) - (Bx2 - Bx1) * (Ay2 - Ay1);
            let uA = ((Bx2 - Bx1) * (Ay1 - By1) - (By2 - By1) * (Ax1 - Bx1)) / d;
            let uB = ((Ax2 - Ax1) * (Ay1 - By1) - (Ay2 - Ay1) * (Ax1 - Bx1)) / d;
            //
            let cent = Point {
                x: Ax1 + uA * (Ax2 - Ax1),
                y: Ay1 + uA * (Ay2 - Ay1),
                a: 0.0,
            };

            let radius =
                f64::sqrt((self.poss.1.x - cent.x).powi(2) + (self.poss.1.y - cent.y).powi(2));

            let angle_start = f64::atan2(self.poss.0.y - cent.y, self.poss.0.x - cent.x);
            let angle_stop = f64::atan2(self.poss.2.y - cent.y, self.poss.2.x - cent.x);

            context.move_to(self.poss.0.x * cmod.scale, self.poss.0.y * cmod.scale);
            context.arc(
                cent.x * cmod.scale,
                cent.y * cmod.scale,
                radius * cmod.scale,
                angle_start,
                angle_stop,
            );

            // console_log!("angle angle : {},{} : r {}", pos2.x, pos2.y, radius);
            self.fill.end(context);
        }

        // triiggg
        let radius = f64::sqrt(
            f64::powf(self.poss.0.x - self.poss.1.x, 2.0)
                + f64::powf(self.poss.0.y - self.poss.1.y, 2.0),
        );
        let angle_start = f64::atan2(self.poss.0.y - self.poss.1.y, self.poss.0.x - self.poss.1.x);
        let angle_stop = f64::atan2(self.poss.2.y - self.poss.1.y, self.poss.2.x - self.poss.1.x);
        // context.move_to(
        //     self.poss.0.x * cmod.scale,
        //     self.poss.0.y * cmod.scale,
        // );
        // context.arc(
        //     self.poss.1.x * cmod.scale,
        //     self.poss.1.y* cmod.scale,
        //     radius * cmod.scale,
        //     angle_start,
        //     angle_stop,
        // );
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        // draws an "x"
        let size = 1.0;
        context.move_to((self.pos.x - size) * cmod.scale, (self.pos.y - size) * cmod.scale);
        context.line_to((self.pos.x + size) * cmod.scale, (self.pos.y + size) * cmod.scale);
        context.move_to((self.pos.x - size) * cmod.scale, (self.pos.y + size) * cmod.scale);
        context.line_to((self.pos.x + size) * cmod.scale, (self.pos.y - size) * cmod.scale);
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: String,
    pub id: i32,
    pub show: bool,
    pub effect: Effect,
    pub pos: Point,
}

impl Property {
    fn blank() -> Property {
        // todo : rotate
        Property {
            key: "".to_string(),
            value: "".to_string(),
            id: 0,
            show: true,
            effect: Effect::blank(),
            pos: Point::blank(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod, angle: f64) {
        if !self.show {
            return;
        } // don't continue if hiden
          // todo : inherit from Text rendering
        let angle = (self.pos.a + angle) / 180.0 * f64::consts::PI;
        // // if context is flipped then flip back to draw text
        // context.scale(
        //     -(cmod.flip.0 as i32 as f64 * 2.0 - 1.0),
        //     (cmod.flip.1 as i32 as f64 * 2.0 - 1.0),
        // );
        context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale);
        console_log!("prop {}, {}", self.pos.x, self.pos.y);
        context.set_font(format!("{}px monospace", (1.8 * cmod.scale) as i32).as_str());



        if angle > f64::consts::PI * 0.5 && angle <= f64::consts::PI * 1.5 {
            context.rotate(-angle - f64::consts::PI); // half rotate to flip text
            context.set_text_align("right");
            context.fill_text(self.value.as_str(), 0.0, (1.8) * cmod.scale);
            context.rotate(f64::consts::PI); // finish rotation
        } else {
            context.rotate(-angle); // why inverse?
            context.set_text_align("left");
            context.fill_text(self.value.as_str(), 0.0, (1.8) * cmod.scale);
        }
        context.rotate(angle);

        context.translate(-(self.pos.x * cmod.scale), -(self.pos.y * cmod.scale));
                        // if context is flipped then flip back to draw text
        // context.scale(
        //     -(cmod.flip.0 as i32 as f64 * 2.0 - 1.0),
        //     (cmod.flip.1 as i32 as f64 * 2.0 - 1.0),
        // );
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        let angle = (self.pos.a) / 180.0 * f64::consts::PI;
        context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale);
        context.rotate(angle);
        context.move_to(0.0, 0.0);
        context.line_to(self.len * cmod.scale, 0.0);
        context.rotate(-angle);
        context.translate(-(self.pos.x * cmod.scale), -(self.pos.y * cmod.scale));
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        for rect in &self.rects {
            rect.draw(context, cmod);
        }
        for circ in &self.circs {
            circ.draw(context, cmod);
        }
        for line in &self.lines {
            line.draw(context, cmod);
        }
        for arc in &self.arcs {
            arc.draw(context, cmod);
        }
        for pin in &self.pins {
            pin.draw(context, cmod);
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        for symb in &self.symbs {
            symb.draw(context, cmod)
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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        if self.parent.is_some() {
            let angle = (self.pos.a) / 180.0 * f64::consts::PI;
            context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale);
            context.scale(
                -(self.mirror.0 as i32 as f64 * 2.0 - 1.0),
                (self.mirror.1 as i32 as f64 * 2.0 - 1.0),
            );
            context.rotate(angle);
            // let mut cmod = cmod.clone();
            // cmod.flip = (cmod.flip.0 ^ self.mirror.0, cmod.flip.1 ^ self.mirror.1);
            self.parent.as_ref().unwrap().draw(context, &cmod);


            context.rotate(-angle);
            context.scale(
                -(self.mirror.0 as i32 as f64 * 2.0 - 1.0),
                (self.mirror.1 as i32 as f64 * 2.0 - 1.0),
            );
            context.translate(-(self.pos.x * cmod.scale), -(self.pos.y * cmod.scale));

            // apparently properties are absolute compared to their parent symbol?
            for prop in &self.props {
                prop.draw(context, &cmod, self.pos.a);
            }

        }
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Heir,
    Global,
    Local,
    Noconn,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub id: String,
    pub shape: Shape,
    pub pos: Point,
    // todo : effects
    pub uuid: UUID,
}

impl Label {
    fn blank() -> Label {
        Label {
            id: "".to_string(),
            shape: Shape::Heir,
            pos: Point::blank(),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        // draws an label based on label type
        // todo : type based rendering
        let size = 1.0; // todo global size?
        let angle = (self.pos.a) / 180.0 * f64::consts::PI;
        context.translate((self.pos.x) * cmod.scale, (self.pos.y) * cmod.scale);
        context.rotate(-angle); // why inverse?
        match self.shape {
            Shape::Heir => {
                context.set_font(format!("{}px monospace", (1.8 * cmod.scale) as i32).as_str());
                context.set_text_baseline("middle");
                if angle > f64::consts::PI * 0.5 && angle <= f64::consts::PI * 1.5 {
                    context.rotate(-f64::consts::PI); // half rotate to flip text
                    context.set_text_align("right");
                    context.fill_text(self.id.as_str(), -(size * 2.5) * cmod.scale, 0.0);
                    context.rotate(f64::consts::PI); // finish rotation
                } else {
                    context.set_text_align("left");
                    context.fill_text(self.id.as_str(), (size * 2.5) * cmod.scale, 0.0);
                }
                context.set_text_baseline("bottom");
                // draw frame
                context.move_to(0.0, 0.0);
                context.line_to((size) * cmod.scale, (size) * cmod.scale);
                context.line_to((size * 2.0) * cmod.scale, (size) * cmod.scale);
                context.line_to((size * 2.0) * cmod.scale, -(size) * cmod.scale);
                context.line_to((size) * cmod.scale, -(size) * cmod.scale);
                context.line_to(0.0, 0.0);
            }
            _ => {}
        }
        context.rotate(angle);
        context.translate(-((self.pos.x) * cmod.scale), -((self.pos.y) * cmod.scale));
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
        let p_fill = |obj: &Sexp| -> FillType {
            match obj.list().unwrap()[1].list().unwrap()[1]
                .string()
                .unwrap()
                .as_str()
            {
                "none" => FillType::None,
                "outline" => FillType::Outline,
                "background" => FillType::Background,
                _ => FillType::None,
            }
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
                            "dash" => StrokeFormat::Dash,
                            _ => StrokeFormat::Default,
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
                    (true, "fill") => {
                        poly.fill = p_fill(obj);
                    }
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
                    (true, "stroke") => {
                        arc.stroke = p_stroke(obj);
                    }
                    (true, "fill") => {
                        arc.fill = p_fill(obj);
                    }
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
                        rect.fill = p_fill(obj);
                    }
                    (true, "uuid") => {
                        rect.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    (true, "stroke") => {
                        rect.stroke = p_stroke(obj);
                    }
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
                    (true, "stroke") => {
                        circ.stroke = p_stroke(obj);
                    }
                    (true, "fill") => {
                        circ.fill = p_fill(obj);
                    }
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
                    // todo : effects
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
                    (true, "stroke") => {
                        wire.stroke = p_stroke(obj);
                    }
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
                        label.shape = match label_name {
                            "hierarchical_label" => Shape::Heir,
                            _ => Shape::Local,
                        }
                        // todo shape = input...
                    }
                    (true, "at") => {
                        label.pos = p_pos(obj);
                    }
                    (true, "uuid") => {
                        label.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    // todo : effects
                    _ => { // should be string
                         //println!("{:?}", name);
                    }
                }
            }
            //
            label
        };
        let p_prop = |obj: &Sexp| -> Property {
            let mut prop = Property::blank();
            //
            let props = obj.list().unwrap();
            prop.key = props[1].string().unwrap().clone();
            prop.value = props[2].string().unwrap().clone();
            prop.id = props[3].list().unwrap()[1]
                .string()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            if props[4].is_list() {
                prop.pos = p_pos(&props[4]);
                console_log!("PROP : {}, {}", prop.pos.x, prop.pos.y);
            }
            if props.len() >= 6 && props[5].is_list() {
                // todo : effect
                for obj in props[5].list().unwrap() {
                    if obj.is_string() && obj.string().unwrap().as_str() == "hide" {
                        prop.show = false;
                    }
                }
            }
            //
            prop
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
                    // todo : on_board
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
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
                        symb.props.push(p_prop(obj));
                    }
                    (true, "uuid") => {
                        symb.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                    }
                    (true, "at") => {
                        symb.pos = p_pos(obj);
                    }
                    (true, "mirror") => {
                        symb.mirror = match obj.list().unwrap()[1].string().unwrap().as_str() {
                            "x" => (false, true),
                            "y" => (true, false), // todo why x/y swapped?
                            "xy" | "yx" => (true, true),
                            _ => (false, false),
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
                        symb.props.push(p_prop(obj));
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
        let context = &canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.begin_path();
        let cmod = &CanvasMod {
            scale: scale * 4.0,
            flip: (false, false),
        }; //todo fix scaling
           // todo context.scale
        let pos = Point::blank();
        for symb in &self.symbs {
            symb.draw(context, cmod);
        }
        for wire in &self.wires {
            wire.draw(context, cmod);
        }
        for junc in &self.juncs {
            junc.draw(context, cmod);
        }
        for text in &self.texts {
            text.draw(context, cmod);
        }
        for poly in &self.polys {
            poly.draw(context, cmod);
        }
        for nocon in &self.nocons {
            nocon.draw(context, cmod);
        }
        for label in &self.labels {
            label.draw(context, cmod);
        }

        context.stroke();
    }
}
