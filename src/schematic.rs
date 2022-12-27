use std::f64;
use std::fs;
use symbolic_expressions;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // draw point to point using stroke
        context.move_to(
            ( self.points[0].x) * scale + pos.0,
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
    fn new(point: Point) -> Junction {
        Junction {
            point: point,
            diameter: 1.0,
            color: (0, 0, 0, 0),
            uuid: "hello".to_string(),
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, pos: (f64, f64), scale: f64) {
        // todo : move point based on diam
        context.move_to(
            ( self.point.x) * scale + pos.0,
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
            ( self.point.x) * scale + pos.0,
            (self.point.y) * scale + pos.1,
        );
        context.set_font(format!("{}px monospace", (1.0*scale) as i32).as_str());
        context.fill_text(self.text.as_str(), ( self.point.x) * scale + pos.0, (self.point.y) * scale + pos.1);
    }
}

#[derive(Debug, Clone)]
pub struct Schematic {
    pub wires: Vec<Wire>,
    pub juncs: Vec<Junction>,
    pub texts: Vec<Text>,
    //
    pub version: i32,
}

impl Schematic {
    pub fn new(file: &str) -> Schematic {
        let mut wires = Vec::<Wire>::new();
        let mut juncs = Vec::<Junction>::new();
        let mut texts = Vec::<Text>::new();

        let mut version = 0i32;
        let schems_sexp = symbolic_expressions::parser::parse_str(file).unwrap();

        // Parse
        if !schems_sexp.is_list() { /* todo error handle */ }
        // Parse
        if !schems_sexp.is_list() { /* todo error handle */ }
        for schem_sexp in schems_sexp.list() {
            for object in schem_sexp {
                let name = match object.is_list() {
                    true => object.list().unwrap()[0].string().unwrap().as_str(),
                    false => object.string().unwrap().as_str(),
                    _ => "",
                };
                match name {
                    "version" => {
                        println!("{}", object);
                    },
                    "wire" => {
                        let mut points = Vec::<Point>::new();
                        let stroke = Stroke {
                            width: 1.0,
                            s_type: 1,
                            color: (0, 0, 0, 0),
                        };
                        let mut uuid = String::new();

                        let object = object.list().unwrap();
                        for property in object {
                            let name = match property.is_list() {
                                true => property.list().unwrap()[0].string().unwrap().as_str(),
                                false => property.string().unwrap().as_str(),
                                _ => "",
                            };
                            match name {
                                "pts" => {
                                    for xy in property.list().unwrap() {
                                        if xy.is_list() {
                                            let xy = xy.list().unwrap();
                                            points.push(Point {
                                                x: xy[1].string().unwrap().parse::<f64>().unwrap(),
                                                y: xy[2].string().unwrap().parse::<f64>().unwrap(),
                                            });
                                        }
                                    }
                                },
                                "uuid" => {
                                    uuid =
                                        property.list().unwrap()[1].string().unwrap().to_string();
                                },
                                // todo : stroke
                                _ => {
                                    //println!("{:?}", name);
                                }
                            }
                        }
                        wires.push(Wire {
                            points: points,
                            stroke: stroke,
                            uuid: uuid,
                        });
                        //println!("{:?}", object);
                    },
                    "junction" => {
                        let mut junction = Junction::new(Point { x: 0.0, y: 0.0 });

                        let object = object.list().unwrap();
                        for property in object {
                            let name = match property.is_list() {
                                true => property.list().unwrap()[0].string().unwrap().as_str(),
                                false => property.string().unwrap().as_str(),
                                _ => "",
                            };
                            match name {
                                "at" => {
                                        let xy = property.list().unwrap();
                                        junction.point.x = xy[1].string().unwrap().parse::<f64>().unwrap();
                                        junction.point.y = xy[2].string().unwrap().parse::<f64>().unwrap();
                                },
                                "diameter" => {
                                    junction.diameter = property.list().unwrap()[1].string().unwrap().parse::<f64>().unwrap();
                                },
                                // todo color
                                "uuid" => {
                                    junction.uuid =
                                        property.list().unwrap()[1].string().unwrap().to_string();
                                },
                                // todo : stroke
                                _ => {
                                    //println!("{:?}", name);
                                }
                            }
                        }
                        juncs.push(junction);
                        //println!("{:?}", object);
                    },
                    "text" => {
                        let mut text = Text::blank();

                        let object = object.list().unwrap();
                        for property in object {
                            let name = match property.is_list() {
                                true => property.list().unwrap()[0].string().unwrap().as_str(),
                                false => property.string().unwrap().as_str(),
                                _ => "",
                            };
                            match name {
                                "at" => {
                                        let xy = property.list().unwrap();
                                        text.point.x = xy[1].string().unwrap().parse::<f64>().unwrap();
                                        text.point.y = xy[2].string().unwrap().parse::<f64>().unwrap();
                                },
                                // todo color
                                "uuid" => {
                                    text.uuid =
                                        property.list().unwrap()[1].string().unwrap().to_string();
                                },
                                // todo : stroke
                                _ => { // should be string
                                    //println!("{:?}", name);
                                    if property.is_string() {
                                        text.text = property.string().unwrap().clone();
                                    }
                                }
                            }
                        }
                        texts.push(text);
                        //println!("{:?}", object);
                    },
                    _ => {
                        //println!("{}", name);
                    }
                }
            }
        }

        Schematic {
            wires: wires,
            juncs: juncs,
            texts: texts,
            version: version,
        }
    }

    // pub fn new_from_file() -> Schematic {
    //     let wires = vec![Wire::new(vec![
    //         Point { x: 0.0, y: 0.0 },
    //         Point { x: 100.0, y: 100.0 },
    //         Point { x: 100.0, y: 200.0 },
    //     ])];
    //     let juncs = vec![
    //         Junction::new(Point { x: 0.0, y: 100.0 }),
    //         Junction::new(Point {
    //             x: 100.0,
    //             y: -100.0,
    //         }),
    //     ];
    //     Schematic {
    //         wires: wires,
    //         juncs: juncs,

    //         version: 0,
    //     }
    // }

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
        context.stroke();
    }
}
