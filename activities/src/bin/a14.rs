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


// Use a struct for a persons age, name, and favorite color
// The color and name should be stored as a String
struct Person {
    name: String,
    age: i32,
    favorite_color: String
}


impl Person {
    fn new(name: String, age: i32, favorite_color: String) -> Self {
        Self {
            name: name,
            age: age,
            favorite_color: favorite_color
        }
    }

    // The name and colors should be printed using a function
    fn traits(&self) {
        println!(
            "{} has {:?} years old, and his/her favorite color is {}.",
            self.name, self.age, self.favorite_color);
    }
}


fn main() {
    let john = Person::new(
        String::from("John"), 4,
        String::from("Blue"));
    let manuela = Person::new(
        String::from("Manuela"), 7,
        String::from("Black"));
    let hugo = Person::new(
        String::from("Hugo"), 45,
        String::from("Red"));

    // Create and store at least 3 people in a vector
    let persons = vec![john, manuela, hugo];

    // Iterate through the vector using a for..in loop
    for person in persons {
        // Use an if expression to determine which person's
        // info should be printed
        if person.age <= 10 {
            person.traits();
        }
    }
}
