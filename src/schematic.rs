use std::collections::HashMap;
use std::f64;
use crate::theme::Theme;

use wasm_bindgen::prelude::*;

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
    pub theme: Theme,
}

impl CanvasMod {
    pub fn new() -> CanvasMod {
        CanvasMod {
            scale: 1.0,
            flip: (false, false),
            theme: Theme::new()
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
    pub fn blank() -> Stroke {
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
    pub fn blank() -> Wire {
        Wire {
            poss: Vec::<Point>::new(),
            stroke: Stroke::blank(),
            uuid: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FillType {
    None,
    Outline,
    Background,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub poss: (Point, Point),
    pub stroke: Stroke,
    pub fill: FillType,
    pub uuid: UUID,
}

impl Rect {
    pub fn blank() -> Rect {
        Rect {
            poss: (Point::blank(), Point::blank()),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
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
    pub fn blank() -> Circ {
        Circ {
            pos: Point::blank(),
            radius: 0.0,
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
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
    pub fn blank() -> Junction {
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
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub pos: Point,
    // todo : effect
    pub uuid: UUID,
}

impl Text {
    pub fn blank() -> Text {
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
}

#[derive(Debug, Clone)]
pub struct Effect {
    pub font_name: String,
    pub size: (f64, f64),
    pub thickness: f64,
    pub bold: bool,
    pub italic: bool,
    pub line_spacing: f64,
    pub justify: (bool, bool, bool),
    pub hide: bool,
}

impl Effect {
    pub fn blank() -> Effect {
        Effect {
            font_name: "".to_string(),
            size: (0.0, 0.0),
            thickness: 0.0,
            bold: false,
            italic: false,
            line_spacing: 0.0,
            justify: (false, false, false),
            hide: false,
        }
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
    pub fn blank() -> Polyline {
        Polyline {
            poss: Vec::<Point>::new(),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
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
    pub fn blank() -> Arc {
        Arc {
            poss: (Point::blank(), Point::blank(), Point::blank()),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Noconnect {
    pub pos: Point,
    pub uuid: UUID,
}

impl Noconnect {
    pub fn blank() -> Noconnect {
        Noconnect {
            pos: Point {
                x: 0.0,
                y: 0.0,
                a: 0.0,
            },
            uuid: "".to_string(),
        }
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
    pub fn blank() -> Property {
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
    pub fn blank() -> Pin {
        Pin {
            pos: Point::blank(),
            len: 0.0,
            name: ("".to_string(), Effect::blank()),
            numb: (0i32, Effect::blank()),
        }
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
    pub fn blank() -> Symbol {
        Symbol {
            id: "".to_string(),
            lines: Vec::<Polyline>::new(),
            arcs: Vec::<Arc>::new(),
            pins: Vec::<Pin>::new(),
            rects: Vec::<Rect>::new(),
            circs: Vec::<Circ>::new(),
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
    pub fn blank() -> SymbolTemp {
        SymbolTemp {
            id: "".to_string(),
            props: Vec::<Property>::new(),
            pos: Point::blank(), // todo : not a thing
            symbs: Vec::<Symbol>::new(),
            uuid: "".to_string(),
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
    pub fn blank() -> SymbolInst {
        SymbolInst {
            id: "".to_string(),
            parent: None,
            props: Vec::<Property>::new(),
            pos: Point::blank(),
            mirror: (false, false),
            uuid: "".to_string(),
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
    pub effect: Effect,
    pub uuid: UUID,
}

impl Label {
    pub fn blank() -> Label {
        Label {
            id: "".to_string(),
            shape: Shape::Heir,
            pos: Point::blank(),
            effect: Effect::blank(),
            uuid: "".to_string(),
        }
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

    pub fn from_str(file: &str) -> Schematic {
        let sexp = &symbolic_expressions::parser::parse_str(file).unwrap();
        Schematic::from_sexp(sexp)
    }    
}
