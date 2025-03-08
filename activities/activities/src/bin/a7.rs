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

enum Color{
    red,
    green,
    blue,
}

fn enum_print(list: Color){
    match list {
        Color::red => println!("red"),
        Color::green => println!("green"),
        Color::blue => println!("blue"),
    }
    
}
fn main() {
    enum_print(Color::blue);
    
}
