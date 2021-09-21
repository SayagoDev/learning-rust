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

enum  Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Ticket::Standard(9.99),
        Ticket::Backstage(19.99, "Angel".to_owned()),
        Ticket::Vip(25.99, String::from("Brian")),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard Ticket Price: {:?}", price),
            Ticket::Backstage(price, holder) => {
                println!("Backstage Ticket Holder: {:?}, Price: {:?}", holder, price);
            }
            Ticket::Vip(price, holder) => {
                println!("VIP Ticket Holder: {:?}, Price: {:?}", holder, price);
            }
        }
    }
}
