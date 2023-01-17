pub struct Box {
    pub rect: Rect,
    pub color: Color,
    pub margin: Margin,
    pub padding: Padding,
    pub children: Vec<Box>,
}

pub struct Rect {
    pub height: i16,
    pub width: i16,
    pub x: i16,
    pub y: i16,
}

pub struct Margin {
   pub top: i16,
   pub right: i16,
   pub bottom: i16,
   pub left: i16,
}

pub struct Padding {
   pub top: i16,
   pub right: i16,
   pub bottom: i16,
   pub left: i16,
}

pub struct Color {
   pub r: u8,
   pub g: u8,
   pub b: u8,
   pub a: u8,
}

impl Color {
    pub fn to_array(&self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }
}


