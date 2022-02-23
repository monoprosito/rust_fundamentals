// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


// Company positions:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
enum Position {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Chef,
    AssemblyTechnician
}

// Use a struct to store the employee type and whether they are still employed
enum Status {
    Active,
    Terminated
}


struct Employee {
    name: String,
    position: Position,
    status: Status
}


impl Employee {
    fn new(name: String, position: Position, status: Status) -> Self {
        Self {
            name,
            position,
            status
        }
    }

    fn has_building_access(&self) -> Result<(), String> {
        // Ensure that terminated employees cannot access the building
        // regardless of their position
        if matches!(self.status, Status::Terminated) {
            return Err(format!("Sorry {}, you're no longer allowed in the building.", self.name));
        }

        // Employees that can access the building are:
        //   * Maintenance crews
        //   * Managers
        //   * Marketing department employees
        match self.position {
            Position::Maintenance => Ok(()),
            Position::Manager     => Ok(()),
            Position::Marketing   => Ok(()),
            _                     => Err(format!("{}, you don't have access, try again later.", self.name))
        }
    }
}


/// Print whether the employee may access the building
fn print_employee_access(employee: &Employee) -> Result<String, String> {
    let _has_access = employee.has_building_access()?;
    Ok(format!("Welcome {}, have a nice day!", employee.name))
}


/// Determine if an employee can access a building using a digital keycard
fn main() {
    let michael   = Employee::new(String::from("Michael"), Position::Chef, Status::Active);
    let amanda    = Employee::new(String::from("Amanda"), Position::AssemblyTechnician, Status::Terminated);
    let michelle  = Employee::new(String::from("Michelle"), Position::Maintenance, Status::Active);
    let barack    = Employee::new(String::from("Barack"), Position::Maintenance, Status::Active);
    let jules     = Employee::new(String::from("Jules"), Position::Marketing, Status::Active);
    let holmes    = Employee::new(String::from("Holmes"), Position::Supervisor, Status::Terminated);
    let sarah     = Employee::new(String::from("Sarah"), Position::Chef, Status::Terminated);
    let otis      = Employee::new(String::from("Otis"), Position::Marketing, Status::Active);
    let val       = Employee::new(String::from("Val"), Position::Manager, Status::Terminated);
    let fiona     = Employee::new(String::from("Fiona"), Position::Chef, Status::Active);
    let fer       = Employee::new(String::from("Fer"), Position::AssemblyTechnician, Status::Active);
    let rosie     = Employee::new(String::from("Rosie"), Position::AssemblyTechnician, Status::Terminated);
    let richie    = Employee::new(String::from("Richie"), Position::Maintenance, Status::Active);
    let gabrielle = Employee::new(String::from("Gabrielle"), Position::Supervisor, Status::Active);
    let usain     = Employee::new(String::from("Usain"), Position::Manager, Status::Terminated);

    let employees = vec![
        michael, amanda, michelle, barack, jules, holmes, sarah, otis, val,
        fiona, fer, rosie, richie, gabrielle, usain
    ];

    println!("Processing employees entering the building...");

    for employee in employees {
        match print_employee_access(&employee) {
            Ok(message)  => println!("Access allowed: {}", message),
            Err(message) => println!("Access denied: {}", message)
        }
    }
}
