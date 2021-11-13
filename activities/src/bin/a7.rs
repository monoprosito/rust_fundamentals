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


// * Use an enum with color names as variants
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Magenta
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match color {
        Color::Black   => println!("black!"),
        Color::Red     => println!("red!"),
        Color::Green   => println!("green!"),
        Color::Yellow  => println!("yellow!"),
        Color::Magenta => println!("magenta!"),
    }
}

fn main() {
    let color = Color::Black;
    print_color(color);
}
