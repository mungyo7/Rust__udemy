// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item{
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &Item){
    println!("{}",item.quantity);
}

fn display_id(item: &Item){
    println!("{}",item.id);
}

fn main() {
    let new_item = Item{
        quantity: 3,
        id: 5,
    };

    display_quantity(&new_item);
    display_id(&new_item)
}
