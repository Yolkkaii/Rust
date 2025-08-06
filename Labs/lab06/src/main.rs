use std::io;
use std::io::Write;
use rand::Rng;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    input.trim().to_string()
}

struct Player{
    hp: i32,
    stamina: i32,
    power: i32,
    gold: i32,
}

struct Enemy {
    name: String,
    hp: i32,
    power: i32,
    reward: i32,
}

enum Encounter{
    Nothing,
    Meat,
    Water,
    Herb,
    Enemy,
}

enum Direction{
    N,
    W,
    S,
    E,
}

fn encounter(player:&mut Player, encounter:&Encounter){
    match encounter{
        Encounter::Nothing => {
            println!("You found...Nothing!");
        },
        Encounter::Meat => {
            player.hp += 5;
            println!("You found meat!\nHP +5");
        },
        Encounter::Water => {
            player.stamina += 2;
            println!("You found water!\nStamina +2");
        },
        Encounter::Herb => {
            player.power += 1;
            println!("You found herbs!\nPower +1");
        },
        Encounter::Enemy => {
            println!("You encountered an enemy!");
            let mut enemy = enemy();
            battle(player, &mut enemy);
        },
    }
    if player.hp > 100{
        player.hp = 100;
    }
}

fn enemy() -> Enemy {
    let mut rng = rand::thread_rng();
    let enemy_type = rng.gen_range(0..100);
    match enemy_type {
        0..=59 => Enemy {name: "Rat".to_string(), hp: 10, power: 2, reward: 10},
        60..=89 => Enemy {name: "Wolf".to_string(), hp: 20, power: 5, reward: 20},
        _ => Enemy {name: "Boar".to_string(), hp: 30, power: 8, reward: 30},
    }
}

fn battle(player: &mut Player, enemy: &mut Enemy) {
    println!("You encounterd a {}. HP: {}, Power: {}", enemy.name, enemy.hp, enemy.power);
    while player.hp > 0 && enemy.hp > 0 {
        enemy.hp -= player.power;
        println!("You hit the {} for {} damage!", enemy.name, player.power);

        if enemy.hp <= 0 {
            println!("You defeated the {}!", enemy.name);
            player.gold += enemy.reward;
            return;
        }

        player.hp -= enemy.power;
        println!("The {} hits you for {} damage!", enemy.name, enemy.power);
    }

    if player.hp <= 0 {
        println!("You were defeated.\nGame Over.");
    }
}

fn main() {
    let mut player = Player {hp: 100, stamina: 50, power: 10, gold: 0};
    let mut rng = rand::thread_rng();

    println!("You started on an adventure in a dungeon.");
    
    loop {
        let chance: u32 = rng.gen_range(0..100);
        let roll = match chance {
            0..=24 => Encounter::Nothing,
            25..=44 => Encounter::Meat,
            45..=64 => Encounter::Water,
            65..=79 => Encounter::Herb,
            _ => Encounter::Enemy,
        };

        let direction = get_input("Which direction will you go (N/W/S/E)?\n").to_uppercase();

        if direction.is_empty() {
            println!("Invalid direction. Please enter N, W, S, E");
            continue;
        }

        match direction.as_str() {
            "N" | "W" | "S" | "E" => {
                println!("You move towards the {}.", direction);
                player.stamina -= 1;
            },
            _ => {
                println!("Invalid direction. Please enter N, W, S, or E");
                continue;
            }
        }

        encounter(&mut player, &roll);

        println!("Your current stats: HP: {}, Stamina: {}, Power: {}, Gold: {}", player.hp, player.stamina, player.power, player.gold);
        
        if player.hp <= 0 {
            println!("You bleed out to death...\nGame over.");
            break;
        }
        if player.gold >= 100{
            println!("You win!");
            break;
        }
        if player.stamina <= 0 {
            println!("Your body has ran out of energy...\nGame over.")
        }
    }
}