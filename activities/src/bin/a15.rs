// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


enum Ticket {
    Standard(String, f64),
    Backstage(String, String, f64),
    Vip(String, String, f64)
}


fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Standard(String::from("La Solar"), 50.0),
        Ticket::Backstage(String::from("Bad Bunny"), String::from("Santiago"), 500.0),
        Ticket::Standard(String::from("La Solar"), 50.0),
        Ticket::Standard(String::from("Bad Bunny"), 110.0),
        Ticket::Standard(String::from("Harry Styles"), 70.0),
        Ticket::Vip(String::from("Harry Styles"), String::from("Manuela"), 133.33)
    ];
    let mut i = 1;

    for ticket in tickets {
        println!("----------");

        match ticket {
            Ticket::Backstage(event, holder, price) => {
                println!("{:?}: Backstage ticket #{} ({}), price for ${} USD", event, i, holder, price)
            },
            Ticket::Vip(event, holder, price) => {
                println!("{:?}: VIP ticket #{} ({}), price for ${} USD", event, i, holder, price)
            },
            Ticket::Standard(event, price) => {
                println!("{:?} ticket #{}, price for ${} USD", event, i, price)
            }
        }

        i += 1;
    }
}
