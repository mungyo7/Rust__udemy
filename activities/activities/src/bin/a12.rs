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
struct Dimensions{
    width: i32,
    height: i32,
    depth: i32,
}
impl Dimensions{
    fn print(&self){
        println!("width : {:?}",self.width);
        println!("height : {:?}",self.height);
        println!("depth : {:?}",self.depth);
    }
}
enum Color {
    red,
    green,
    blue,
}

impl Color{
    fn print(&self){
        match self{
            Color::red => println!("red"),
            Color::green => println!("green"),
            Color::blue => println!("blue"),
        }
    }
}
struct Box{
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}
impl Box {
    fn new_box(dimensions: Dimensions, weight: i32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_box(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight : {:?}",self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions{
        width: 2,
        height: 4,
        depth: 7,
    };
    let newnewbox = Box::new_box(small_dimensions, 3, Color::red);
    newnewbox.print_box();
}
