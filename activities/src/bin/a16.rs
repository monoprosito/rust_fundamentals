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
    locker: Option<i32>
}


fn main() {
    let jules = Student {
        name: String::from("Jules"),
        locker: None
    };

    let rue = Student {
        name: String::from("Rue"),
        locker: Some(40)
    };

    let sydney = Student {
        name: String::from("Sydney"),
        locker: Some(45)
    };

    let students: Vec<Student> = vec![
        jules,
        rue,
        sydney
    ];

    for student in students {
        match student.locker {
            Some(locker) => println!("Welcome {}, your assigned school locker is A{:?}.", student.name, locker),
            None => println!("Welcome {}, you don\'t have a school locker assigned, yet.", student.name)
        }
    }
}

