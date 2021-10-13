#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String, 
    action: VisitorAction,
    age: i8
}
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
} 

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("We lcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", &self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in.", self.name),
        }
        println!("{}", self.greeting);
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Shawn", "Hello Shawn, enjoy your treehouse!", VisitorAction::Accept, 32),
        Visitor::new("Bert", "Hey Bernt Krishna, I like your stand up.", VisitorAction::AcceptWithNote{note: String::from("Severe alcoholic")}, 40),
        Visitor::new("Tom", "Tom, get out.", VisitorAction::Refuse, 40),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();
    
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    
        if let Some(visitor) = known_visitor {
             visitor.greet_visitor();
        } else {
            if name.is_empty() {
                break;
            } 

            println!("{} is not on the visitor list.", name);
            visitor_list.push(Visitor::new(&name, "New friend"));
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line.");

    your_name.trim().to_lowercase() // This is the same as return your_name
}
