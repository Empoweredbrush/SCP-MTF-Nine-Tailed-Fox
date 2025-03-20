use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("What is your name?\n");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("{}", name);
}

fn start() {

}
