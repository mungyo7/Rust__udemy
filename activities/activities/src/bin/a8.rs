// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    sweet,
    salty,
    hot,
}

struct Drinks {
    flavor: Flavor,
    oz: f64,
}

fn printFlavor(what: Drinks){
    match what.flavor {
        Flavor::sweet => println!("sweet"),
        Flavor::salty => println!("salty"),
        Flavor::hot => println!("hot"),
    };

    println!("fluid oz : {:?}", what.oz);
}

fn main() {
    let mydrink = Drinks{
        flavor: Flavor::sweet,
        oz: 32.1,
    };
    printFlavor(mydrink);

}
