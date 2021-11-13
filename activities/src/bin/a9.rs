// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print


// * Use a function that returns a tuple
fn set_coordinates(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let limit = 5;

    // * Destructure the return value into two variables
    let (_, y) = set_coordinates(4, 7);

    // * Use an if..else if..else block to determine what to print
    if y > limit {
        println!("y-value ({:?}) is greater than the limit ({:?}).", y, limit);
    } else if y < limit {
        println!("y-value ({:?}) is less than the limit ({:?}).", y, limit);
    } else {
        println!("y-value ({:?}) is equal to the limit ({:?}).", y, limit);
    }
}
