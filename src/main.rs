use rustyfingers;

fn main() {
    match rustyfingers::run() {
        Ok(_) => println!("Bye bye"),
        Err(error) => println!("Something went wrong: {}", error),
    }
}
