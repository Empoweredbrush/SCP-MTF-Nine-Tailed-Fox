use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::Duration;
use std::thread;


//structs and other items used to keep track of the player and their inventory

struct Player {
    health: i32,
    inventory: Vec<Item>,
    status_effects: Vec<StatusEffect>
}

impl Player {
    fn add_item(&mut self, item: Item){
        self.inventory.push(item);
    }

    fn remove_item(&mut self, item_name: &str) {
        self.inventory.retain(|item| item.name != item_name);
    }

    fn apply_status(&mut self, status: StatusEffect) {
        self.status_effects.push(status);
    }

    // Method to heal the player
    fn heal(&mut self, amount: i32) {
        self.health += amount;
        println!("You heal {} HP. Current HP: {}", amount, self.health);
    }
}

struct Item {
    name: String,
    item_type: ItemType,
}

enum ItemType {
    Weapon { damage: i32 },
    Medkit { heal_amount: i32 },
    KeyCard { clearance_level: i32 },
}

enum StatusEffect {
    Infected(i32), // Deals damage over time
    Bleeding(i32), // Gradually reduces HP
    Haste,         // Increases speed
}




fn main() {

    println!("What is your codename?");

    let mut codename = String::new();

    io::stdin()
    .read_line(&mut codename)
    .expect("Failed to read line");

    start(codename.trim());
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

    println!("ERROR\nERROR\nERROR\nERROR\nERROR\nAudio log corrupted............\n\n\n\n")
}

fn introduction() {

    thread::sleep(Duration::from_secs(5));

    println!("The room is dark outside of the yellow revolving safety light. Haven woken up, face once laying against a keyboard, the computer screen showing audio error, ");

}