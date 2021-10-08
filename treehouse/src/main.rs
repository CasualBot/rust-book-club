#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    println!("Hello, what's your name?\n");
    let name = what_is_your_name();
    let mut allow_them_in = false;

    let visitor_list = [
        Visitor::new("Shawn", "Hello Shawn, enjoy your treehouse!"),
        Visitor::new("Bert", "Hey Bernt Krishna, I like your stand up."),
        Visitor::new("Tom", "Tom, get out.")
    ];

    for visitor in &visitor_list {
        if visitor.name == name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Welcome, {}", &name);
    } else {
        println!("You are not on the guest list.");
    }
}


fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line.");

    your_name.trim().to_lowercase() // This is the same as return your_name
}
