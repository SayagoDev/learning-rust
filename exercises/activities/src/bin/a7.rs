// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    White,
    Black,
    Red,
    Green
}

fn print_color(my_color: Colors) {
    match my_color {
        Colors::White => println!("white"),
        Colors::Black => println!("black"),
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
    }
}

fn main() {
    print_color(Colors::Black);
}
