// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let saul = Student {
        name: "Saul Sayago".to_owned(),
        locker: Some(232),
    };
    println!("name: {:?}", saul.name);
    match saul.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assigned"),
    }
}
