use rustyfingers;

fn main() {
    match rustyfingers::run() {
        Ok(_) => (),
        Err(error) => println!("Something went wrong: {}", error),
    }
}
