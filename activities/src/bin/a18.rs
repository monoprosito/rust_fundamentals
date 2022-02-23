// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase


struct Customer {
    name: Option<String>,
    age: i8,
}

fn can_purchase_restricted_things(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        return Ok(())
    }
    return Err(String::from("You must be over 21 years old to enter the store."));
}


fn main() {
    let john = Customer {
        name: Some(String::from("John")),
        age: 19
    };

    let unknown = Customer {
        name: None,
        age: 22
    };

    let customers: Vec<Customer> = vec![john, unknown];

    for customer in customers {
        println!("----------");

        match &customer.name {
            Some(name) => print!("Security Guard: Hello {}, ", name),
            None => print!("Security Guard: Hello, ")
        }

        match can_purchase_restricted_things(&customer) {
            Ok(_) => println!("Welcome to the Gun Shop."),
            Err(message) => println!("{}", message)
        }
    }
}
