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

struct Item {
    quantity: i32,
    id: i32,
}

fn display_quantity(g_item: &Item) {
    println!("Quantity: {:?}", g_item.quantity);
}

fn display_id(g_item: &Item) {
    println!("Id: {:?}", g_item.id);
}

fn main() {
    let g_item = Item {
        quantity: 10,
        id: 777,
    };
    display_quantity(&g_item);
    display_id(&g_item);
}
