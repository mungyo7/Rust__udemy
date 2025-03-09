// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print_name(name: &str){
    println!("name : {:?}",name);
}

fn print_color(color: &str){
    println!("color : {:?}",color);
}

fn main() {
    let people = vec![
        Person{
            age: 22,
            name: String::from("jason"),
            color: String::from("red"),
        },
        Person{
            age: 8,
            name: String::from("mike"),
            color: String::from("green"),
        },
        Person{
            age: 21,
            name: String::from("sally"),
            color: String::from("blue"),
        }
    ];

    for i in people{
        if (i.age > 10){
            print_name(&i.name);
            print_color(&i.color);
        }
    }
}
