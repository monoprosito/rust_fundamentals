// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print


// * Use a function to print the messages
fn print_big_or_small(value: i32, limit: i32) {
    // * Use a boolean variable set to the result of
    //   an if..else expression to store whether the value
    //   is > 100 or <= 100
    let is_big: bool = value > limit;

    // * Use a match expression to determine which message
    //   to print
    match is_big {
        true  => println!("The value is big!"),
        false => println!("The value is small!"),
    };
}


fn main() {
    print_big_or_small(25, 100);
}
