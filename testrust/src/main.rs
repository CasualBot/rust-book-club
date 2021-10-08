
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let my_list = ["One", "Two", "Three"];

    for i in &my_list {
        println!("{}", i);
    }
}
