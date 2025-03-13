// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShirtColor(Color);

#[derive(Debug)]
struct ShoesColor(Color);

#[derive(Debug)]
struct PantsColor(Color);
 
impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}   

impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirt_color(shirt_color: &ShirtColor) {
    println!("The shirt color is {:?}", shirt_color.0);
}

fn print_shoes_color(shoes_color: &ShoesColor) {
    println!("The shoes color is {:?}", shoes_color.0);
}

fn print_pants_color(pants_color: &PantsColor) {
    println!("The pants color is {:?}", pants_color.0);
}

fn main() {
    let shirt_color = ShirtColor::new(Color::Red);
    let shoes_color = ShoesColor::new(Color::Blue);
    let pants_color = PantsColor::new(Color::Green);

    print_shirt_color(&shirt_color);
    print_shoes_color(&shoes_color);
    print_pants_color(&pants_color);
}
