use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::Duration;
use std::thread;


fn main() {
    println!("What is your codename?");

    let mut codename = String::new();

    io::stdin()
        .read_line(&mut codename)
        .expect("Failed to read line");

    start(&codename);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top-left
    std::io::Write::flush(&mut std::io::stdout()).unwrap(); // Ensure it updates immediately
}

fn start(codename: &str) {
    clear_screen();

    let recruit_num = rand::rng().random_range(1..=10000);
    let audio_num = rand::rng().random_range(1..=10000);
    
    let computer_startup_message = format!("One moment........\n\nReplaying audio log #{}......\n\nplaying log.......", audio_num);

    for line in computer_startup_message.split('\n') {
        println!("{}", line);
        thread::sleep(Duration::from_secs(2));
    }
    println!("Welcome recruit #{}, codename: {}", recruit_num, codename);

    println!("\nAfter going through our records, it seems you have been cleared to join Mobile Task Force Epsilon-11, code named 'Nine Tailed Fox.'\nYou will be stationed at Site [READACTED].\nYou are expected to arrive at 0900 hours and meet with Director [REDACTED]...................\n");

    println!("ERROR\nERROR\nERROR\nERROR\nERROR\n Audio log corrupted............")
}