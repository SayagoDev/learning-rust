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
    fav_color: String,
}

fn print_data(name: &str, color: &str) {
    println!("Name: {:?}", name);
    println!("Favorite color: {:?}", color);
}

fn main() {
    let people = vec![
        Person {
            age: 9,
            name: "Angel".to_owned(),
            fav_color: "Green".to_owned(),
        },
        Person {
            age: 17,
            name: "Carlos".to_owned(),
            fav_color: "Red".to_owned(),
        },
        Person {
            age: 10,
            name: String::from("Luis"),
            fav_color: String::from("Blue"),
        },
    ];

    for person in people {
        if person.age <= 10 {
            println!("Age: {:?}", person.age);
            print_data(&person.name, &person.fav_color);
        }
    }
}
