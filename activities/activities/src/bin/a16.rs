// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Lockers {
    name: String,
    assignment: Option<i32>,
}

fn main() {
    let list = vec![
        Lockers{
            name: String::from("Jason"),
            assignment: Some(32),
        },
        Lockers{
            name: String::from("Mike"),
            assignment: Some(10),
        },
        Lockers{
            name: String::from("Minsoo"),
            assignment: None,
        }
    ];
    for a in &list{
        println!("name : {}",a.name);
    }
    for i in list{
        match i.assignment {
            Some(number) => println!("number : {:?}",number),
            None => println!("None"),
        }
    }
}
