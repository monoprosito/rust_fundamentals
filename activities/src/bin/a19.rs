// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock


use std::collections::HashMap;


fn main() {
    // Use a HashMap for the furniture store stock
    let mut furniture_store_stock: HashMap<String, i8> = HashMap::new();

    // The store has:
    //   * 5 Chairs
    //   * 3 Beds
    //   * 2 Tables
    //   * 0 Couches
    furniture_store_stock.insert(String::from("chairs"), 5);
    furniture_store_stock.insert(String::from("beds"), 3);
    furniture_store_stock.insert(String::from("tables"), 2);
    furniture_store_stock.insert(String::from("couches"), 0);

    // Print the name and number of items in stock for a furniture store
    for (item_name, stock) in furniture_store_stock.iter() {
        match stock {
            // If the number of items is 0, print "out of stock" instead of 0
            0 => println!("Item: {} - Out of stock", item_name),
            _ => println!("Item: {} - Stock: {:?}", item_name, stock)
        }
    }

    // Print the total number of items in stock
    println!("Total stock: {:?}", furniture_store_stock.values().sum::<i8>());
}
