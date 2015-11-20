use std::io;

fn main() {
    println!("What's your name?");
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name),
        Err(e) => println!("Sorry, {}", e)
    }
}
