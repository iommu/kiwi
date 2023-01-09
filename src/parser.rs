use crate::schematic::*;
use symbolic_expressions;
use symbolic_expressions::Sexp;


fn get_name(object: &Sexp) -> &str {
  match object.is_list() {
      true => object.list().unwrap()[0].string().unwrap().as_str(),
      false => object.string().unwrap().as_str(),
      _ => "",
  }
}

// generic parsers
impl Point {
    pub fn from_sexp(obj: &Sexp) -> Point {
        let mut point = Point::blank();
        let xya = obj.list().unwrap();
        point.x = xya[1].string().unwrap().parse::<f64>().unwrap();
        point.y = xya[2].string().unwrap().parse::<f64>().unwrap();
        point.a = if xya.len() >= 4 {
            xya[3].string().unwrap().parse::<f64>().unwrap()
        } else {
            0.0
        };
        point
    }
}

impl FillType {
    pub fn from_sexp(obj: &Sexp) -> FillType {
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
    }
}

impl Junction {
    pub fn from_sexp(obj: &Sexp) -> Junction {
        let mut junction = Junction::blank();
        //
        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "at") => {
                    junction.pos = Point::from_sexp(obj);
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
    }
}

impl Stroke {
    pub fn from_sexp(obj: &Sexp) -> Stroke {
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
    }
}

impl Polyline {
    pub fn from_sexp(obj: &Sexp) -> Polyline {
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
                        poly.poss.push(Point::from_sexp(obj));
                    }
                }
                (true, "uuid") => {
                    poly.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                }
                (true, "stroke") => {
                    poly.stroke = Stroke::from_sexp(obj);
                }
                (true, "fill") => {
                    poly.fill = FillType::from_sexp(obj);
                }
                _ => {
                    //println!("{:?}", name);
                }
            }
        }
        //
        poly
    }
}

impl Arc {
    pub fn from_sexp(obj: &Sexp) -> Arc {
        let mut arc = Arc::blank();

        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "start") => {
                    arc.poss.0 = Point::from_sexp(obj);
                }
                (true, "mid") => {
                    arc.poss.1 = Point::from_sexp(obj);
                }
                (true, "end") => {
                    arc.poss.2 = Point::from_sexp(obj);
                }
                (true, "uuid") => {
                    arc.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                }
                (true, "stroke") => {
                    arc.stroke = Stroke::from_sexp(obj);
                }
                (true, "fill") => {
                    arc.fill = FillType::from_sexp(obj);
                }
                _ => {
                    //println!("{:?}", name);
                }
            }
        }
        arc
    }
}

impl Pin {
    pub fn from_sexp(obj: &Sexp) -> Pin {
        let mut pin = Pin::blank();
        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "at") => {
                    pin.pos = Point::from_sexp(obj);
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
    }
}

impl Rect {
    pub fn from_sexp(obj: &Sexp) -> Rect {
        let mut rect = Rect::blank();

        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "start") => {
                    rect.poss.0 = Point::from_sexp(obj);
                }
                (true, "end") => {
                    rect.poss.1 = Point::from_sexp(obj);
                }
                (true, "fill") => {
                    rect.fill = FillType::from_sexp(obj);
                }
                (true, "uuid") => {
                    rect.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                }
                (true, "stroke") => {
                    rect.stroke = Stroke::from_sexp(obj);
                }
                _ => {
                    //println!("{:?}", name);
                }
            }
        }
        rect
    }
}

impl Circ {
    pub fn from_sexp(obj: &Sexp) -> Circ {
        let mut circ = Circ::blank();

        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "center") => {
                    circ.pos = Point::from_sexp(obj);
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
                    circ.stroke = Stroke::from_sexp(obj);
                }
                (true, "fill") => {
                    circ.fill = FillType::from_sexp(obj);
                }
                _ => {
                    //println!("{:?}", name);
                }
            }
        }
        circ
    }
}

impl Text {
    pub fn from_sexp(obj: &Sexp) -> Text {
        let mut text = Text::blank();
        //
        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "at") => {
                    text.pos = Point::from_sexp(obj);
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
    }
}

impl Wire {
    pub fn from_sexp(obj: &Sexp) -> Wire {
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
                        wire.poss.push(Point::from_sexp(obj));
                    }
                }
                (true, "uuid") => {
                    wire.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                }
                (true, "stroke") => {
                    wire.stroke = Stroke::from_sexp(obj);
                }
                _ => {
                    //println!("{:?}", name);
                }
            }
        }
        //
        wire
    }
}

impl Noconnect {
    pub fn from_sexp(obj: &Sexp) -> Noconnect {
        let mut nconn = Noconnect::blank();
        //
        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "at") => {
                    nconn.pos = Point::from_sexp(obj);
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
    }
}

impl Label {
    pub fn from_sexp(obj: &Sexp) -> Label {
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
                    label.pos = Point::from_sexp(obj);
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
    }
}

impl Property {
    pub fn from_sexp(obj: &Sexp) -> Property {
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
            prop.pos = Point::from_sexp(&props[4]);
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
    }
}
impl Symbol {
    pub fn from_sexp(obj: &Sexp) -> Symbol {
        let mut symb = Symbol::blank();
        //
        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (false, _) => {
                    symb.id = obj.string().unwrap().clone();
                }
                (true, "polyline") => {
                    symb.lines.push(Polyline::from_sexp(obj));
                }
                (true, "arc") => {
                    symb.arcs.push(Arc::from_sexp(obj));
                }
                (true, "pin") => {
                    symb.pins.push(Pin::from_sexp(obj));
                }
                (true, "rectangle") => {
                    symb.rects.push(Rect::from_sexp(obj));
                }
                (true, "circle") => {
                    symb.circs.push(Circ::from_sexp(obj));
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
    }
}

impl SymbolInst {
    pub fn from_sexp(obj: &Sexp) -> SymbolInst {
        let mut symb = SymbolInst::blank();
        //
        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (true, "lib_id") => {
                    symb.id = obj.list().unwrap()[1].string().unwrap().clone();
                }
                (true, "property") => {
                    symb.props.push(Property::from_sexp(obj));
                }
                (true, "uuid") => {
                    symb.uuid = obj.list().unwrap()[1].string().unwrap().to_string();
                }
                (true, "at") => {
                    symb.pos = Point::from_sexp(obj);
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
    }
}

impl SymbolTemp {
    pub fn from_sexp(obj: &Sexp) -> SymbolTemp {
        let mut symb = SymbolTemp::blank();
        //
        for obj in obj.list().unwrap() {
            let name = get_name(obj);
            match (obj.is_list(), name) {
                (false, _) => {
                    symb.id = obj.string().unwrap().clone();
                }
                (true, "property") => {
                    symb.props.push(Property::from_sexp(obj));
                }
                (true, "symbol") => {
                    symb.symbs.push(Symbol::from_sexp(obj));
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
    }
}

impl Schematic {
  pub fn from_sexp(obj: &Sexp) -> Schematic {
    let mut schem = Schematic::blank();



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
                            let symb = SymbolTemp::from_sexp(obj);
                            schem.lib.insert(symb.id.clone(), symb);
                        }
                        _ => {}
                    }
                }
            }
            (true, "wire") => schem.wires.push(Wire::from_sexp(obj)),
            (true, "junction") => schem.juncs.push(Junction::from_sexp(obj)),
            (true, "text") => schem.texts.push(Text::from_sexp(obj)),
            (true, "polyline") => schem.polys.push(Polyline::from_sexp(obj)),
            (true, "no_connect") => schem.nocons.push(Noconnect::from_sexp(obj)), // todo : fix naming
            (true, "hierarchical_label") => schem.labels.push(Label::from_sexp(obj)),
            (true, "symbol") => {
                let mut symb = SymbolInst::from_sexp(obj);
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