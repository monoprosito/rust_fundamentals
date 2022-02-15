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


// * Use a struct for the grocery item
struct GroceryItem {
    id: i32, // Use two i32 fields for the id number
    quantity: i32 // Use two i32 fields for the quantity
}


// * Create a function to display the id number, with the struct as a parameter
fn display_item_id(grocery_item: &GroceryItem) {
    println!("Grocery Item ID: {:?}", grocery_item.id);
}


// * Create a function to display the quantity, with the struct as a parameter
fn display_item_quantity(grocery_item: &GroceryItem) {
    println!("Grocery Item Quantity: {:?}", grocery_item.quantity);
}


fn main() {
    let grocery_item = GroceryItem {
        id: 1,
        quantity: 50
    };

    display_item_id(&grocery_item);
    display_item_quantity(&grocery_item);
}
