// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Yellow,
    White,
    Green,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Yellow => println!("color: yellow"),
            Color::White => println!("color: white"),
            Color::Green => println!("color: green"),
            Color::Blue => println!("color: blue"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    length: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("length: {:?}", self.length);
    }
}

struct ShippingBox {
    weight: f64,
    color: Color,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}


fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 5.5,
        length: 2.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Yellow, small_dimensions);
    small_box.print();
}
