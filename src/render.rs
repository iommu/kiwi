use crate::theme::Theme;
use std::f64;

use crate::schematic::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

impl Wire {
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
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
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
        );

        self.fill.end(context);
    }
}

impl Junction {
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

impl Text {
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

impl Polyline {
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
            context.line_to(point.x * cmod.scale, point.y * cmod.scale);
        }
        self.fill.end(context);
    }
}

impl Arc {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
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
        
        //
        let cent = Point {
            x: Ax1 + uA * (Ax2 - Ax1),
            y: Ay1 + uA * (Ay2 - Ay1),
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
        );

        self.fill.end(context);
    }
}

impl Noconnect {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        // draws an "x"
        let size = 1.0;
        context.move_to(
            (self.pos.x - size) * cmod.scale,
            (self.pos.y - size) * cmod.scale,
        );
        context.line_to(
            (self.pos.x + size) * cmod.scale,
            (self.pos.y + size) * cmod.scale,
        );
        context.move_to(
            (self.pos.x - size) * cmod.scale,
            (self.pos.y + size) * cmod.scale,
        );
        context.line_to(
            (self.pos.x + size) * cmod.scale,
            (self.pos.y - size) * cmod.scale,
        );
    }
}

impl Property {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod, angle: f64) {
        if !self.show {
            return;
        } // don't continue if hiden
          // todo : inherit from Text rendering
        let angle = (self.pos.a + angle) / 180.0 * f64::consts::PI;
        context.translate(self.pos.x * cmod.scale, self.pos.y * cmod.scale);
        context.set_font(format!("{}px monospace", (1.8 * cmod.scale) as i32).as_str()); // todo : cache

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
    }
}

impl Pin {
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

impl Symbol {
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

impl SymbolTemp {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d, cmod: &CanvasMod) {
        for symb in &self.symbs {
            symb.draw(context, cmod)
        }
    }
}

impl SymbolInst {
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
            // todo collate template props
            for prop in &self.props {
                prop.draw(context, &cmod, self.pos.a);
            }
        }
    }
}

impl Label {
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

impl Schematic {
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
            theme: Theme::new(),
        }; //todo fix scaling
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
