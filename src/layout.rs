struct Box {
    dimensions: Dimensions,
    color: Color,
    margin: i16,
    padding: i16,
    children: Vec<Box>,
}

struct Dimensions {
    height: i16,
    width: i16,
    x: i16,
    y: i16,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Box {
    // pub fn new()
}
