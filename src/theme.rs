
pub struct Color {
  pub r : u8,
  pub g : u8,
  pub b : u8,
  pub a : u8
}

impl Color {
  pub fn blank() -> Color {
    Color {
      r: 0,
      g: 0,
      b: 0,
      a: 0
    }
  }
}

pub struct Theme {
  pub anchors : Color,
  pub axes : Color,
  pub background : Color,
  pub bus_junction : Color,
  pub busses : Color,
  pub cursor : Color,
  pub drawing_sheet : Color,
  pub global_label : Color,
  pub grid : Color,
  pub helper_items : Color,
  pub hidden_items : Color,
  pub hierarchical_label : Color,
  pub highlighted_item : Color,
  pub junction : Color,
  pub labels : Color,
  // todo : more  
}

impl Theme {
  pub fn new() -> Theme {
    Theme {
      anchors : Color::blank(),
      axes : Color::blank(),
      background : Color::blank(),
      bus_junction : Color::blank(),
      busses : Color::blank(),
      cursor : Color::blank(),
      drawing_sheet : Color::blank(),
      global_label : Color::blank(),
      grid : Color::blank(),
      helper_items : Color::blank(),
      hidden_items : Color::blank(),
      hierarchical_label : Color::blank(),
      highlighted_item : Color::blank(),
      junction : Color::blank(),
      labels : Color::blank(),
    }
  }
}