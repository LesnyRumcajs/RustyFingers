use rustyfingers;

fn main() {
    match rustyfingers::run() {
        Ok(_) => println!("Everything went well"),
        Err(error) => println!("Something went wrong: {}", error),
    }
}
