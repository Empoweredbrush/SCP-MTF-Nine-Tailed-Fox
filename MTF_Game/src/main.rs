use std::collections::HashMap;
use std::io;
use rand::Rng;

type Inventory = HashMap<String, HashMap<String, String>>; // Key: Item name, Value: Item properties (like type, used, etc.)

fn create_player(name: &str) -> HashMap<String, String> {
    let mut player = HashMap::new();
    player.insert("name".to_string(), name.to_string());
    player.insert("health".to_string(), "100".to_string()); // Default health
    player.insert("status_effects".to_string(), "None".to_string()); // Empty status effects
    player
}

fn add_item(player_inventory: &mut Inventory, item_name: &str, item_type: &str) {
    let mut item = HashMap::new();
    item.insert("type".to_string(), item_type.to_string());
    item.insert("used".to_string(), "false".to_string());
    player_inventory.insert(item_name.to_string(), item);
}

fn remove_item(player_inventory: &mut Inventory, item_name: &str) {
    player_inventory.remove(item_name);
}

fn heal(player: &mut HashMap<String, String>, amount: i32) {
    let current_health: i32 = player.get("health").unwrap().parse().unwrap();
    let new_health = current_health + amount;
    player.insert("health".to_string(), new_health.to_string());
    println!("You heal {} HP. Current HP: {}", amount, new_health);
}

fn attack(player: &HashMap<String, String>, weapon: &str) -> i32 {
    if let Some(weapon_info) = player.get(weapon) {
        if weapon_info == "Weapon" {
            // Assuming weapon damage is constant for simplicity
            return 20;
        }
    }
    0
}

fn use_item(player_inventory: &mut Inventory, player: &mut HashMap<String, String>, item_name: &str) {
    if let Some(item) = player_inventory.get_mut(item_name) {
        if item.get("used").unwrap() == "false" {
            if item.get("type").unwrap() == "Medkit" {
                heal(player, 50); // Heal the player
                item.insert("used".to_string(), "true".to_string()); // Mark as used
                println!("You used a Medkit and healed 50 HP.");
            } else {
                println!("This item cannot be used right now.");
            }
        } else {
            println!("This item has already been used.");
        }
    } else {
        println!("Item not found in inventory.");
    }
}

fn create_enemy(name: &str, health: i32, attack_power: i32) -> HashMap<String, String> {
    let mut enemy = HashMap::new();
    enemy.insert("name".to_string(), name.to_string());
    enemy.insert("health".to_string(), health.to_string());
    enemy.insert("attack_power".to_string(), attack_power.to_string());
    enemy
}

fn enemy_attack(player: &mut HashMap<String, String>, enemy: &HashMap<String, String>) {
    let enemy_attack_power: i32 = enemy.get("attack_power").unwrap().parse().unwrap();
    let current_health: i32 = player.get("health").unwrap().parse().unwrap();
    let new_health = current_health - enemy_attack_power;
    player.insert("health".to_string(), new_health.to_string());

    println!(
        "The {} attacks you for {} damage. Your current HP: {}",
        enemy.get("name").unwrap(),
        enemy_attack_power,
        new_health
    );

    if new_health <= 0 {
        println!("You have been defeated by the {}!", enemy.get("name").unwrap());
    }
}

fn handle_attack(player: &mut HashMap<String, String>, enemy: &mut HashMap<String, String>, player_inventory: &Inventory) {
    let damage = attack(player, "Weapon");
    if damage > 0 {
        let enemy_health: i32 = enemy.get("health").unwrap().parse().unwrap();
        let new_health = enemy_health - damage;
        enemy.insert("health".to_string(), new_health.to_string());
        println!(
            "You attack the {} for {} damage. Enemy HP: {}",
            enemy.get("name").unwrap(),
            damage,
            new_health
        );
    } else {
        println!("You have no weapon to attack with!");
    }
}

fn battle(player: &mut HashMap<String, String>, player_inventory: &mut Inventory) {
    let mut rng = rand::thread_rng();
    let enemy_choice = rng.gen_range(0..2);

    let mut enemy = match enemy_choice {
        0 => create_enemy("SCP-008 Zombie", 50, 10),
        1 => create_enemy("Class-D Personnel", 30, 5),
        _ => panic!("Unexpected random enemy choice"),
    };

    loop {
        println!("\nYour health: {} | {}'s health: {}", player.get("health").unwrap(), enemy.get("name").unwrap(), enemy.get("health").unwrap());
        println!("Choose your action:");
        println!("1. Attack | 2. Use Item | 3. Flee");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                handle_attack(player, &mut enemy, player_inventory);
            }
            "2" => {
                println!("Enter item name to use:");
                let mut item_name = String::new();
                io::stdin().read_line(&mut item_name).expect("Failed to read input");
                let item_name = item_name.trim();
                use_item(player_inventory, player, item_name);
            }
            "3" => {
                println!("You attempt to flee...");
                break;
            }
            _ => println!("Invalid choice, please select again."),
        }

        if player.get("health").unwrap().parse::<i32>().unwrap() <= 0 {
            break;
        }

        enemy_attack(player, &enemy);
    }
}

fn main() {
    println!("What is your codename?");
    let mut codename = String::new();
    io::stdin().read_line(&mut codename).expect("Failed to read input");

    let mut player = create_player(codename.trim());
    let mut player_inventory: Inventory = HashMap::new();

    add_item(&mut player_inventory, "Pistol", "Weapon");
    add_item(&mut player_inventory, "Medkit", "Medkit");

    battle(&mut player, &mut player_inventory);
}
