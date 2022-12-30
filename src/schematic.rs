use std::f64;
use std::fs;
use symbolic_expressions;
use symbolic_expressions::Sexp;
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
pub struct Pos {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

type Points = Vec<Point>;

#[derive(Debug, Clone)]
pub struct Stroke {
    pub width: f64,
    pub s_type: u8,              // todo , reserved word?
    pub color: (u8, u8, u8, u8), // todo real color obj
}

type UUID = String; // todo : real uuid obj

#[derive(Debug, Clone)]
pub struct Wire {
    pub points: Points,
    pub stroke: Stroke,
    pub uuid: UUID,
}

impl Wire {
    fn blank() -> Wire {
        Wire {
            points: Vec::<Point>::new(),
            stroke: Stroke {
                width: 1.0,
                s_type: 1,
                color: (0, 0, 0, 0),
            },
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // draw point to point using stroke
        context.move_to(
            (self.points[0].x) * scale + pos.0,
            (self.points[0].y) * scale + pos.1,
        );
        for point in &self.points {
            context.line_to((point.x) * scale + pos.0, (point.y) * scale + pos.1);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Junction {
    pub point: Point,
    pub diameter: f64,
    pub color: (u8, u8, u8, u8),
    pub uuid: UUID,
}

impl Junction {
    fn blank() -> Junction {
        Junction {
            point: Point { x: 0.0, y: 0.0 },
            diameter: 1.0,
            color: (0, 0, 0, 0),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // todo : move point based on diam
        context.move_to(
            (self.point.x) * scale + pos.0,
            (self.point.y) * scale + pos.1,
        );
        context
            .arc(
                (self.point.x) * scale + pos.0,
                (self.point.y) * scale + pos.1,
                ((self.diameter + 1.0) * 1.0) * scale,
                0.0,
                f64::consts::PI * 2.0,
            )
            .unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub point: Point,
    // todo : effect
    pub uuid: UUID,
}

impl Text {
    fn blank() -> Text {
        Text {
            text: "".to_string(),
            point: Point { x: 0.0, y: 0.0 },
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // todo : move point based on diam
        context.move_to(
            (self.point.x) * scale + pos.0,
            (self.point.y) * scale + pos.1,
        );
        context.set_font(format!("{}px monospace", (2.0 * scale) as i32).as_str());
        context.fill_text(
            self.text.as_str(),
            (self.point.x) * scale + pos.0,
            (self.point.y) * scale + pos.1,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Polyline {
    pub points: Points,
    pub stroke: Stroke,
    pub uuid: UUID,
}

impl Polyline {
    fn blank() -> Polyline {
        Polyline {
            points: Vec::<Point>::new(),
            stroke: Stroke {
                width: 1.0,
                s_type: 1,
                color: (0, 0, 0, 0),
            },
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // draw point to point using stroke
        context.move_to(
            (self.points[0].x) * scale + pos.0,
            (self.points[0].y) * scale + pos.1,
        );
        for point in &self.points {
            context.line_to((point.x) * scale + pos.0, (point.y) * scale + pos.1);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub points: (Point, Point, Point), /* start, mid, end*/
    pub stroke: Stroke,
    pub fill: bool,
    pub uuid: UUID,
} // todo

impl Arc {
    fn blank() -> Arc {
        Arc {
            points: (
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 0.0 },
            ),
            stroke: Stroke {
                width: 1.0,
                s_type: 1,
                color: (0, 0, 0, 0),
            },
            fill: false,
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // draw point to point using stroke
        context.move_to(
            (self.points.0.x) * scale + pos.0,
            (self.points.0.y) * scale + pos.1,
        );
        // triiggg
        let radius = f64::sqrt(
            f64::powf(self.points.0.x - self.points.1.x, 2.0)
                + f64::powf(self.points.0.y - self.points.1.y, 2.0),
        );
        let angle_start = f64::atan2(
            self.points.0.y - self.points.1.y,
            self.points.0.x - self.points.1.x,
        );
        let angle_stop = f64::atan2(
            self.points.2.y - self.points.1.y,
            self.points.2.x - self.points.1.x,
        );
        context
            .arc(
                (self.points.0.x) * scale + pos.0,
                (self.points.0.y) * scale + pos.1,
                radius * scale,
                angle_start,
                angle_stop,
            )
            .unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Noconnect {
    pub point: Point,
    pub uuid: UUID,
}

impl Noconnect {
    fn blank() -> Noconnect {
        Noconnect {
            point: Point { x: 0.0, y: 0.0 },
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // draws an "x"
        let size = 1.0;
        context.move_to(
            (self.point.x - size) * scale + pos.0,
            (self.point.y - size) * scale + pos.1,
        );
        context.line_to(
            (self.point.x + size) * scale + pos.0,
            (self.point.y + size) * scale + pos.1,
        );
        context.move_to(
            (self.point.x - size) * scale + pos.0,
            (self.point.y + size) * scale + pos.1,
        );
        context.line_to(
            (self.point.x + size) * scale + pos.0,
            (self.point.y - size) * scale + pos.1,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: String,
    pub id: i32,
    pub pos: Pos,
    // todo : effect
}

impl Property {
    fn blank() -> Property {
        Property {
            key: "".to_string(),
            value: "".to_string(),
            id: 0,
            pos: Pos {
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pin {
    pub pos: Pos,
    pub len: f64,
    // pub name: (String, Effect),
    // pub numb: (i32, Effect),
}

impl Pin {
    fn blank() -> Pin {
        Pin {
            pos: Pos { x: 0.0, y: 0.0, angle: 0.0 },
            len: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: String,
    pub lines: Vec<Polyline>,
    pub arcs: Vec<Arc>,
    pub pins: Vec<Pin>,
}

impl Symbol {
    fn blank() -> Symbol {
        Symbol {
            id: "".to_string(),
            lines: Vec::<Polyline>::new(),
            arcs: Vec::<Arc>::new(),
            pins: Vec::<Pin>::new(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        console_log!("drawing symbol {}", self.id);
        for line in &self.lines {
            console_log!("drawing_line");
            line.draw(context, pos, scale);
        }
        for arc in &self.arcs {
            console_log!("drawing_arc");
            arc.draw(context, pos, scale);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolInst {
    pub id: String,
    pub props: Vec<Property>,
    pub point: Point,
    pub symb: Symbol,
    pub uuid: UUID,
}

impl SymbolInst {
    fn blank() -> SymbolInst {
        SymbolInst {
            id: "".to_string(),
            props: Vec::<Property>::new(),
            point: Point { x: 0.0, y: 0.0 },
            symb: Symbol::blank(),
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        self.symb.draw(context, pos, scale)
        // // draws an "x"
        // let size = 1.0;
        // context.move_to(
        //     ( self.point.x - size) * scale + pos.0,
        //     (self.point.y - size) * scale + pos.1,
        // );
        // context.line_to(( self.point.x + size) * scale + pos.0,
        // (self.point.y + size) * scale + pos.1);
        // context.move_to(
        //     ( self.point.x - size) * scale + pos.0,
        //     (self.point.y + size) * scale + pos.1,
        // );
        // context.line_to(( self.point.x + size) * scale + pos.0,
        // (self.point.y - size) * scale + pos.1);
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub id: String,
    pub shape: u8, // todo : enum
    pub pos: Pos,
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
            pos: Pos {
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
            uuid: "".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // draws an label based on label type
        // todo : type based rendering
        if (self.shape != 0xf0) {
            return;
        }
        let size = 1.0;

        // let points = [
            
        // ];

        // draw text
        context.move_to(
            (self.pos.x + size * 2.5) * scale + pos.0,
            (self.pos.y) * scale + pos.1,
        );
        context.set_font(format!("{}px monospace", (2.0 * scale) as i32).as_str());
        context.fill_text(
            self.id.as_str(),
            (self.pos.x + size * 2.5) * scale + pos.0,
            (self.pos.y + 1.0) * scale + pos.1,
        );

        // draw frame
        context.move_to((self.pos.x) * scale + pos.0, (self.pos.y) * scale + pos.1);
        context.line_to(
            (self.pos.x + size) * scale + pos.0,
            (self.pos.y + size) * scale + pos.1,
        );
        context.line_to(
            (self.pos.x + size * 2.0) * scale + pos.0,
            (self.pos.y + size) * scale + pos.1,
        );
        context.line_to(
            (self.pos.x + size * 2.0) * scale + pos.0,
            (self.pos.y - size) * scale + pos.1,
        );
        context.line_to(
            (self.pos.x + size) * scale + pos.0,
            (self.pos.y - size) * scale + pos.1,
        );
        context.line_to((self.pos.x) * scale + pos.0, (self.pos.y) * scale + pos.1);
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
        let p_junc = |obj: &Sexp| -> Junction {
            let mut junction = Junction::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "at") => {
                        if !obj.is_list() {
                            continue;
                        }
                        let xy = obj.list().unwrap();
                        junction.point.x = xy[1].string().unwrap().parse::<f64>().unwrap();
                        junction.point.y = xy[2].string().unwrap().parse::<f64>().unwrap();
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
        let p_poly = |obj: &Sexp| -> Polyline {
            let mut poly = Polyline::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "pts") => {
                        for xy in obj.list().unwrap() {
                            if !xy.is_list() {
                                continue;
                            }
                            let xy = xy.list().unwrap();
                            poly.points.push(Point {
                                x: xy[1].string().unwrap().parse::<f64>().unwrap(),
                                y: xy[2].string().unwrap().parse::<f64>().unwrap(),
                            });
                        }
                    }
                    (true, "uuid") => {
                        poly.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
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
                match name {
                    "start" | "mid" | "end" => {
                        if obj.is_list() {
                            let xy = obj.list().unwrap();
                            let point = Point {
                                x: xy[1].string().unwrap().parse::<f64>().unwrap(),
                                y: xy[2].string().unwrap().parse::<f64>().unwrap(),
                            };
                            match name {
                                "start" => {
                                    arc.points.0 = point;
                                }
                                "mid" => {
                                    arc.points.1 = point;
                                }
                                "end" => {
                                    arc.points.2 = point;
                                }
                                _ => {}
                            }
                        }
                    }
                    "uuid" => {
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
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match name {
                    "start" | "mid" | "end" => {
                        if obj.is_list() {
                            let xy = obj.list().unwrap();
                            let point = Point {
                                x: xy[1].string().unwrap().parse::<f64>().unwrap(),
                                y: xy[2].string().unwrap().parse::<f64>().unwrap(),
                            };
                        }
                    }
                    // todo : stroke
                    _ => {
                        //println!("{:?}", name);
                    }
                }
            }
            //
            pin
        };
        let p_text = |obj: &Sexp| -> Text {
            let mut text = Text::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (true, "at") => {
                        if !obj.is_list() {
                            continue;
                        }
                        let xy = obj.list().unwrap();
                        text.point.x = xy[1].string().unwrap().parse::<f64>().unwrap();
                        text.point.y = xy[2].string().unwrap().parse::<f64>().unwrap();
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
                        for xy in obj.list().unwrap() {
                            if !xy.is_list() {
                                continue;
                            }
                            let xy = xy.list().unwrap();
                            wire.points.push(Point {
                                x: xy[1].string().unwrap().parse::<f64>().unwrap(),
                                y: xy[2].string().unwrap().parse::<f64>().unwrap(),
                            });
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
                        if !obj.is_list() {
                            continue;
                        }
                        let xy = obj.list().unwrap();
                        nconn.point.x = xy[1].string().unwrap().parse::<f64>().unwrap();
                        nconn.point.y = xy[2].string().unwrap().parse::<f64>().unwrap();
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
                        if !obj.is_list() {
                            continue;
                        }
                        let xya = obj.list().unwrap();
                        label.pos.x = xya[1].string().unwrap().parse::<f64>().unwrap();
                        label.pos.y = xya[2].string().unwrap().parse::<f64>().unwrap();
                        label.pos.angle = xya[3].string().unwrap().parse::<f64>().unwrap();
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
                        symb.arcs.push(p_arc(obj));
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
        let p_symb_inst = | obj : &Sexp | -> SymbolInst {
            let mut symb = SymbolInst::blank();
            //
            for obj in obj.list().unwrap() {
                let name = get_name(obj);
                match (obj.is_list(), name) {
                    (false, _) => {
                        symb.id = obj.string().unwrap().clone();
                    }
                    (true, "property") => {
                        let props =  obj.list().unwrap();
                        let mut prop = Property::blank();
                        //
                        prop.key = props[1].string().unwrap().clone();
                        prop.value = props[2].string().unwrap().clone();
                        prop.id = props[3].list().unwrap()[1].string().unwrap().parse::<i32>().unwrap();
                        if props[4].is_list(){
                            let at = props[4].list().unwrap();
                            if at[0].string().unwrap().as_str() != "at" {break;}
                            prop.pos.x = at[1].string().unwrap().parse::<f64>().unwrap();
                            prop.pos.y = at[2].string().unwrap().parse::<f64>().unwrap();
                            prop.pos.angle = at[3].string().unwrap().parse::<f64>().unwrap();
                        }
                        // todo : effect
                        // todo : "at" parser
                        // todo : point => pos
                        symb.props.push(prop);
                    }
                    (true, "symbol") => {
                       symb.symb = p_symb(obj);
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
                        (true, "symbol") => {schem.lib.push(p_symb_inst(obj))}
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
    pub lib: Vec<SymbolInst>,
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
            lib: Vec::<SymbolInst>::new(),
            symbs: Vec::<SymbolInst>::new(),
            version: 0i32,
        }
    }

    pub fn new(file: &str) -> Schematic {
        let schem_obj = symbolic_expressions::parser::parse_str(file).unwrap();

        Parser::schematic(&schem_obj)
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        context.clear_rect(0.0, 0.0, 640.0, 480.0);
        context.begin_path();
        for wire in &self.wires {
            wire.draw(context, pos, scale);
        }
        for junc in &self.juncs {
            junc.draw(context, pos, scale);
        }
        for text in &self.texts {
            text.draw(context, pos, scale);
        }
        for poly in &self.polys {
            poly.draw(context, pos, scale);
        }
        for nocon in &self.nocons {
            nocon.draw(context, pos, scale);
        }
        for label in &self.labels {
            label.draw(context, pos, scale);
        }
        self.lib[1].draw(context, pos, scale);
        context.stroke();
    }
}
