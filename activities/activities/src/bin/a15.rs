// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Tickets::Backstage(30.5, String::from("Mike")),
        Tickets::Vip(50.2, "John".to_owned()),
        Tickets::Standard(20.2),
    ];
    for ticket in tickets{
        match ticket{
            Tickets::Backstage(price,name) => println!("price : {:?}, name : {:?}",price,name),
            Tickets::Vip(price,name) => println!("price : {:?}, name : {:?}",price,name),
            Tickets::Standard(price) => println!("price : {:?}",price),
        }
    }
    }

