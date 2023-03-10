use crate::theme::Theme;
use std::f64;

use crate::schematic::*;
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

impl Wire {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        // draw pos to pos using stroke
        if !self.poss.is_empty() {
            // ensure vec exists
            context.move_to((self.poss[0].x) * cmod.scale, (self.poss[0].y) * cmod.scale);
            for point in &self.poss {
                context.line_to((point.x) * cmod.scale, (point.y) * cmod.scale);
            }
        }

        Ok(())
    }
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
                // todo : does not work? dual rendering?
            }
            FillType::None => {
                context.stroke();
            }
        }
        context.begin_path();
    }
}

impl Rect {
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

impl Circ {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        self.fill.begin(context, &JsValue::from("black"));

        // draw pos to pos using stroke
        context.move_to(
            (self.pos.x + self.radius) * cmod.scale,
            self.pos.y * cmod.scale,
        );
        context.arc(
            self.pos.x * cmod.scale,
            self.pos.y * cmod.scale,
            self.radius * cmod.scale,
            0.0,
            f64::consts::PI * 2.0,
        )?;

        self.fill.end(context);
        Ok(())
    }
}

impl Junction {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        // todo : move pos based on diam
        context.move_to(self.pos.x * cmod.scale, self.pos.y * cmod.scale);
        context.arc(
            self.pos.x * cmod.scale,
            self.pos.y * cmod.scale,
            ((self.diameter + 0.2) * 1.0) * cmod.scale,
            0.0,
            f64::consts::PI * 2.0,
        )?;
        context.fill();
        Ok(())
    }
}

impl Text {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        let angle = self.pos.a / 180.0 * f64::consts::PI;
        context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale)?;
        context.set_font(format!("{}px monospace", (1.8 * cmod.scale) as i32).as_str());
        if angle > f64::consts::PI * 0.5 && angle <= f64::consts::PI * 1.5 {
            context.rotate(-angle - f64::consts::PI)?; // half rotate to flip text
            context.set_text_align("right");
            for (index, newline) in self.text.split("\\n").enumerate() {
                context.fill_text(newline, 0.0, (1.8 * index as f64) * cmod.scale)?;
            }
            context.rotate(f64::consts::PI)?; // finish rotation
        } else {
            context.rotate(-angle)?; // why inverse?
            context.set_text_align("left");
            for (index, newline) in self.text.split("\\n").enumerate() {
                context.fill_text(newline, 0.0, (1.8 * index as f64) * cmod.scale)?;
            }
        }
        context.rotate(angle)?;
        context.translate(-((self.pos.x) * cmod.scale), -((self.pos.y) * cmod.scale))?;
        Ok(())
    }
}

impl Polyline {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        self.fill.begin(context, &JsValue::from("black"));
        // draw pos to pos using stroke
        if !self.poss.is_empty() {
            context.move_to(self.poss[0].x * cmod.scale, self.poss[0].y * cmod.scale);
            for point in &self.poss {
                match self.stroke.format {
                    StrokeFormat::Default => {
                        context.set_line_dash(&js_sys::Array::new())?;
                    }
                    _ => {
                        let x = js_sys::Array::new();
                        x.push(&JsValue::from_f64(2.0 * cmod.scale));
                        x.push(&JsValue::from_f64(2.0 * cmod.scale));
                        context.set_line_dash(&x)?;
                    }
                }
                context.line_to(point.x * cmod.scale, point.y * cmod.scale);
            }
            self.fill.end(context);
        }
        Ok(())
    }
}

impl Arc {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        // draw pos to pos using stroke
        self.fill.begin(context, &JsValue::from("black"));

        let line1_angle = f64::atan2(self.poss.1.y - self.poss.0.y, self.poss.1.x - self.poss.0.x)
            + f64::consts::PI / 2.0;
        let line2_angle = f64::atan2(self.poss.2.y - self.poss.1.y, self.poss.2.x - self.poss.1.x)
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

        let ax1 = line1_mid.x;
        let ay1 = line1_mid.y;
        let ax2 = line1_mid.x + 10.0;
        let ay2 = f64::tan(line1_angle) * 10.0 + line1_mid.y;

        let bx1 = line2_mid.x;
        let by1 = line2_mid.y;
        let bx2 = line2_mid.x + 10.0;
        let by2 = f64::tan(line2_angle) * 10.0 + line2_mid.y;

        let d = (by2 - by1) * (ax2 - ax1) - (bx2 - bx1) * (ay2 - ay1);
        let ua = ((bx2 - bx1) * (ay1 - by1) - (by2 - by1) * (ax1 - bx1)) / d;

        //
        let cent = Point {
            x: ax1 + ua * (ax2 - ax1),
            y: ay1 + ua * (ay2 - ay1),
            a: 0.0,
        };

        let radius = f64::sqrt((self.poss.1.x - cent.x).powi(2) + (self.poss.1.y - cent.y).powi(2));

        let angle_start = f64::atan2(self.poss.0.y - cent.y, self.poss.0.x - cent.x);
        let angle_stop = f64::atan2(self.poss.2.y - cent.y, self.poss.2.x - cent.x);

        context.move_to(self.poss.0.x * cmod.scale, self.poss.0.y * cmod.scale);
        context.arc(
            cent.x * cmod.scale,
            cent.y * cmod.scale,
            radius * cmod.scale,
            angle_start,
            angle_stop,
        )?;

        self.fill.end(context);
        Ok(())
    }
}

impl Property {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
        angle: f64,
    ) -> Result<(), JsValue> {
        if self.show {
            // don't continue if hiden
            // todo : inherit from Text rendering
            let angle = (self.pos.a + angle) / 180.0 * f64::consts::PI;
            context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale)?;
            context.set_font(format!("{}px monospace", (1.8 * cmod.scale) as i32).as_str()); // todo : cache

            if angle > f64::consts::PI * 0.5 && angle <= f64::consts::PI * 1.5 {
                context.rotate(-angle - f64::consts::PI)?; // half rotate to flip text
                context.set_text_align("right");
                context.fill_text(self.value.as_str(), 0.0, (1.8) * cmod.scale)?;
                context.rotate(f64::consts::PI)?; // finish rotation
            } else {
                context.rotate(-angle)?; // why inverse?
                context.set_text_align("left");
                context.fill_text(self.value.as_str(), 0.0, (1.8) * cmod.scale)?;
            }
            context.rotate(angle)?;

            context.translate(-(self.pos.x * cmod.scale), -(self.pos.y * cmod.scale))?;
        }

        Ok(())
    }
}

impl Pin {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        let angle = (self.pos.a) / 180.0 * f64::consts::PI;
        context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale)?;
        context.rotate(angle)?;
        context.move_to(0.0, 0.0);
        context.line_to(self.len * cmod.scale, 0.0);
        context.rotate(-angle)?;
        context.translate(-(self.pos.x * cmod.scale), -(self.pos.y * cmod.scale))?;
        Ok(())
    }
}

impl Symbol {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        for rect in &self.rects {
            rect.draw(context, cmod);
        }
        for circ in &self.circs {
            circ.draw(context, cmod)?;
        }
        for line in &self.lines {
            line.draw(context, cmod)?;
        }
        for arc in &self.arcs {
            arc.draw(context, cmod)?;
        }
        for pin in &self.pins {
            pin.draw(context, cmod)?;
        }
        Ok(())
    }
}

impl SymbolTemp {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        for symb in &self.symbs {
            symb.draw(context, cmod)?;
        }
        Ok(())
    }
}

impl SymbolInst {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        if self.parent.is_some() {
            let angle = (self.pos.a) / 180.0 * f64::consts::PI;
            context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale)?;
            context.scale(
                -(self.mirror.0 as i32 as f64 * 2.0 - 1.0),
                (self.mirror.1 as i32 as f64 * 2.0 - 1.0),
            )?;
            context.rotate(angle)?;
            // let mut cmod = cmod.clone();
            // cmod.flip = (cmod.flip.0 ^ self.mirror.0, cmod.flip.1 ^ self.mirror.1);
            self.parent.as_ref().unwrap().draw(context, &cmod)?;

            context.rotate(-angle)?;
            context.scale(
                -(self.mirror.0 as i32 as f64 * 2.0 - 1.0),
                (self.mirror.1 as i32 as f64 * 2.0 - 1.0),
            )?;
            context.translate(-(self.pos.x * cmod.scale), -(self.pos.y * cmod.scale))?;

            // apparently properties are absolute compared to their parent symbol?
            // todo collate template props
            for prop in &self.props {
                prop.draw(context, &cmod, self.pos.a)?;
            }
        }
        Ok(())
    }
}

impl Label {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        // draws an label based on label type
        // todo : type based rendering
        let size = 1.0; // todo global size?
        let angle = (self.pos.a) / 180.0 * f64::consts::PI;
        context.translate((self.pos.x) * cmod.scale, (self.pos.y) * cmod.scale)?;
        context.rotate(-angle)?; // why inverse?
        match self.shape {
            Style::Heir => {
                context.set_font(format!("{}px monospace", (1.8 * cmod.scale) as i32).as_str());
                context.set_text_baseline("middle");
                if angle > f64::consts::PI * 0.5 && angle <= f64::consts::PI * 1.5 {
                    context.rotate(-f64::consts::PI)?; // half rotate to flip text
                    context.set_text_align("right");
                    context.fill_text(self.id.as_str(), -(size * 2.5) * cmod.scale, 0.0)?;
                    context.rotate(f64::consts::PI)?; // finish rotation
                } else {
                    context.set_text_align("left");
                    context.fill_text(self.id.as_str(), (size * 2.5) * cmod.scale, 0.0)?;
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
            Style::Noconn => {
                // draws an "x"
                let size = 1.0;
                context.move_to(-size * cmod.scale, -size * cmod.scale);
                context.line_to(size * cmod.scale, size * cmod.scale);
                context.move_to(-size * cmod.scale, size * cmod.scale);
                context.line_to(size * cmod.scale, -size * cmod.scale);
            }
            _ => {}
        }
        context.rotate(angle)?;
        context.translate(-((self.pos.x) * cmod.scale), -((self.pos.y) * cmod.scale))?;
        Ok(())
    }
}

impl Page {
    fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        cmod: &CanvasMod,
    ) -> Result<(), JsValue> {
        let (size, margin) = match self {
            Page::A4 => {
                // size // margin (tlbr)
                ((297.0, 210.0), (10.0, 10.0, 10.0, 10.0))
            }
        };
        // draw full page size
        Rect {
            poss: (
                Point {
                    x: 0.0,
                    y: 0.0,
                    a: 0.0,
                },
                Point {
                    x: size.0,
                    y: size.1,
                    a: 0.0,
                },
            ),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
        .draw(context, cmod);
        // draw page margin
        Rect {
            poss: (
                Point {
                    x: margin.1,
                    y: margin.0,
                    a: 0.0,
                },
                Point {
                    x: size.0 - margin.3,
                    y: size.1 - margin.2,
                    a: 0.0,
                },
            ),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
        .draw(context, cmod);
        Rect {
            poss: (
                Point {
                    x: margin.1 + 2.0,
                    y: margin.0 + 2.0,
                    a: 0.0,
                },
                Point {
                    x: size.0 - margin.3 - 2.0,
                    y: size.1 - margin.2 - 2.0,
                    a: 0.0,
                },
            ),
            stroke: Stroke::blank(),
            fill: FillType::None,
            uuid: "".to_string(),
        }
        .draw(context, cmod);
        // draw "chess" grid
        let inc = 50;
        for index in 0..(size.0 / inc as f64) as u32 + 1 {
            for y in [margin.0, size.1 - margin.2 - 2.0] {
                let x = (index * 50 + 25) as f64 + margin.1;
                if x > size.0 + margin.1 - margin.3 {
                    continue;
                }
                Text {
                    text: (index + 1).to_string(),
                    pos: Point {
                        x: x,
                        y: y + 1.7,
                        a: 0.0,
                    },
                    uuid: "".to_string(),
                }
                .draw(context, cmod);
                let x = x + 25.0;
                if x > size.0 + margin.1 - margin.3 {
                    continue;
                }
                Rect {
                    poss: (
                        Point { x: x, y: y, a: 0.0 },
                        Point {
                            x: x,
                            y: y + 2.0,
                            a: 0.0,
                        },
                    ),
                    stroke: Stroke::blank(),
                    fill: FillType::None,
                    uuid: "".to_string(),
                }
                .draw(context, cmod);
            }
        }
        for index in 0..(size.1 / inc as f64) as u32 + 1 {
            for x in [margin.1, size.0 - margin.3 - 2.0] {
                let y = (index * 50 + 25) as f64 + margin.0;
                if y > size.1 + margin.0 - margin.2 {
                    continue;
                }
                Text {
                    text: ((index + 97) as u8 as char).to_string(),
                    pos: Point {
                        x: x + 0.3,
                        y: y,
                        a: 0.0,
                    },
                    uuid: "".to_string(),
                }
                .draw(context, cmod);
                let y = y + 25.0;
                if y > size.1 + margin.0 - margin.2 {
                    continue;
                }
                Rect {
                    poss: (
                        Point { x: x, y: y, a: 0.0 },
                        Point {
                            x: x + 2.0,
                            y: y,
                            a: 0.0,
                        },
                    ),
                    stroke: Stroke::blank(),
                    fill: FillType::None,
                    uuid: "".to_string(),
                }
                .draw(context, cmod);
            }
        }
        Ok(())
    }
}

impl Schematic {
    pub fn draw(&self, canvas: &web_sys::HtmlCanvasElement, scale: f64) -> Result<(), JsValue> {
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
            theme: Theme::new(),
        }; //todo fix scaling
        self.page.draw(context, cmod)?;

        for symb in &self.symbs {
            symb.draw(context, cmod)?;
        }
        for wire in &self.wires {
            wire.draw(context, cmod)?;
        }
        for junc in &self.juncs {
            junc.draw(context, cmod)?;
        }
        for text in &self.texts {
            text.draw(context, cmod)?;
        }
        for poly in &self.polys {
            poly.draw(context, cmod)?;
        }
        for label in &self.labels {
            label.draw(context, cmod)?;
        }

        context.stroke();
        Ok(())
    }
}
