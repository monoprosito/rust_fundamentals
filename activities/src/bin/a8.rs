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


// * Use an enum to create different flavors of drinks
enum Flavor {
    CocaCola,
    Pepsi
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    oz: i32
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::CocaCola => println!("flavor: Coca Cola"),
        Flavor::Pepsi    => println!("flavor: Pepsi"),
    }

    println!("fluid oz: {:?}", drink.oz);
}

fn main() {
    let pepsi_drink = Drink {
        flavor: Flavor::Pepsi,
        oz: 15,
    };

    let coca_cola_drink = Drink {
        flavor: Flavor::CocaCola,
        oz: 17,
    };

    print_drink(pepsi_drink);
    print_drink(coca_cola_drink);
}
