// Week 1: UI Design
use rand::Rng;
use std::{
    io::{self, stdout, Write},
    thread,
    time::Duration,
    fs::{self, File},
};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Serialize, Deserialize};

// -------------------- SAVE / LOAD --------------------
fn save_game(player: &Trainer) -> std::io::Result<()> {
    let file = File::create("savefile.json")?;
    serde_json::to_writer_pretty(file, player)?;
    Ok(())
}

fn load_game() -> Option<Trainer> {
    let data = fs::read_to_string("savefile.json").ok()?;
    let player: Trainer = serde_json::from_str(&data).ok()?;
    Some(player)
}

// -------------------- CONSTANTS --------------------
const Screen_Width: usize = 80;
const Screen_Height: usize = 30;

const Electripuff: [&str; 4] = [
    r#" (\__/) "#,
    r#"(o ⚡ o) "#,
    r#"(  -  )~"#,
    r#" UU  UU "#,
];

const Chlorowl: [&str; 4] = [
    r#"(\_._/)  "#,
    r#"( -w- ) 🌱"#,
    r#"(  V  )  "#,
    r#" ^^ ^^   "#,
];

const Norn: [&str; 4] = [
    r#"( o.o )"#,
    r#"<(   )>"#,
    r#"  U U  "#,
    r#"        "#,
];

const Magifish: [&str; 4] = [
    r#"        "#,
    r#"><(((°> "#,
    r#"        "#,
    r#"        "#,
];

const Flamalink: [&str; 4] = [
    r#"(  )     "#,
    r#"(o_o )  ~🔥"#,
    r#"<)   )╯   "#,
    r#" /   \    "#,
];

// -------------------- STRUCTS --------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Trainer {
    name: String,
    money: i32,
    m_owned: Vec<Monster>,
    battles: i32,
    potion: i32,
    mega: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Monster {
    m_name: String,
    art: Vec<String>,
    m_type: String,
    max_hp: i32,
    c_hp: i32,
    stamina: i32,
    skills: Vec<String>,
}

// -------------------- UI FUNCTIONS --------------------
fn screen(
    scr_width: usize,
    p_name: &str,
    p_art: &[String],
    p_max_hp: u32,
    p_hp: u32,
    opp_name: &str,
    opp_art: &[String],
    opp_max_hp: u32,
    opp_hp: u32,
) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0)).unwrap();

    println!("+{}+", "-".repeat(scr_width));

    execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
    println!("| {:>width$} |", opp_name, width = scr_width - 2);
    execute!(stdout, ResetColor).unwrap();

    let hp_bar_width = 20;
    let hp_filled = ((opp_hp as f32 / opp_max_hp as f32) * hp_bar_width as f32).round() as usize;
    let hp_empty = hp_bar_width - hp_filled;

    println!(
        "| HP: [{}{}] {}/{} {:width$} |",
        "\u{2588}".repeat(hp_filled),
        "\u{2591}".repeat(hp_empty),
        opp_hp,
        opp_max_hp,
        "",
        width = scr_width - (10 + hp_bar_width + 1)
    );

    println!("| {:width$} |", "", width = scr_width - 2);

    for line in opp_art {
        println!("| {:>width$} |", line, width = scr_width - 2);
    }

    // Padding
    for _ in 0..(Screen_Height/2 - opp_art.len()) {
        println!("| {:width$} |", "", width = scr_width - 2);
    }

    for line in p_art {
        println!("| {:<width$} |", line, width = scr_width - 2);
    }

    println!("| {:width$} |", "", width = scr_width - 2);

    execute!(stdout, SetForegroundColor(Color::Cyan)).unwrap();
    println!(
        "| {:<left_width$}{:>right_width$} |",
        p_name,
        "[A] Attack",
        left_width = scr_width - 2 - 22,
        right_width = 22
    );
    execute!(stdout, ResetColor).unwrap();

    let hp_filled = ((p_hp as f32 / p_max_hp as f32) * hp_bar_width as f32).round() as usize;
    let hp_empty = hp_bar_width - hp_filled;
    println!(
        "| HP: [{}{}] {}/{}{:>buttons_width$} |",
        "\u{2588}".repeat(hp_filled),
        "\u{2591}".repeat(hp_empty),
        p_hp,
        p_max_hp,
        "[B] Bag | [M] Exit",
        buttons_width = scr_width - (10 + hp_bar_width + 4)
    );

    println!("+{}+", "-".repeat(scr_width));
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn intro_print(message: &str, row: u16) {
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", message, width = Screen_Width - 2);
}

// -------------------- BATTLE FUNCTIONS --------------------
fn attack(attacker: &mut Monster, defender: &mut Monster, skill_index: usize) {
    let skill = &attacker.skills[skill_index];
    let base_damage = 5 + (skill_index as i32 * 5);
    let damage = base_damage.min(defender.c_hp);
    defender.c_hp -= damage;

    execute!(stdout(), MoveTo(0, 8)).unwrap();
    println!(
        "| {:^width$} |",
        format!("{} used {}! (-{} HP)", attacker.m_name, skill, damage),
        width = Screen_Width - 2
    );
    thread::sleep(Duration::from_secs(1));
}

fn bag(player: &mut Trainer, monster: &mut Monster) {
    loop {
        execute!(stdout(), MoveTo(0, 15), SetForegroundColor(Color::Cyan)).unwrap();
        println!("| {:<left_width$} |", monster.m_name, left_width = Screen_Width - 2);
        execute!(stdout(), ResetColor).unwrap();

        execute!(stdout(), MoveTo((78 - 6 - 6 - 18).try_into().unwrap(), 16)).unwrap();
        println!("[1] Potion | [2] Mega | [3] Back |");
        thread::sleep(Duration::from_millis(300));

        let choice = get_input("");
        match choice.as_str() {
            "1" => {
                if player.potion > 0 {
                    player.potion -= 1;
                    let heal_amount = 20;
                    let healed_hp = (monster.c_hp + heal_amount).min(monster.max_hp);
                    monster.c_hp = healed_hp;
                    intro_print(
                        &format!("You used a Potion! {} healed +{} HP", monster.m_name, heal_amount),
                        8,
                    );
                    thread::sleep(Duration::from_secs(1));
                    break;
                } else {
                    intro_print("No Potions left!", 8);
                }
            }
            "2" => {
                if player.mega > 0 {
                    player.mega -= 1;
                    let heal_amount = 50;
                    let healed_hp = (monster.c_hp + heal_amount).min(monster.max_hp);
                    monster.c_hp = healed_hp;
                    intro_print(
                        &format!("You used a Mega! {} healed +{} HP", monster.m_name, heal_amount),
                        8,
                    );
                    thread::sleep(Duration::from_secs(1));
                    break;
                } else {
                    intro_print("No Megas left!", 8);
                }
            }
            "3" => break, // Back button
            _ => {
                intro_print("Invalid choice.", 8);
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
}

// -------------------- OPPONENT --------------------
fn opp_random(battles: u32) -> Trainer {
    let mut rng = rand::thread_rng();
    let chance = rng.gen_range(0..100);
    let monsters: Vec<Monster> = match battles {
        0 => match chance {
            0..=59 => vec![Monster {
                m_name: "Electripuff".to_string(),
                art: Electripuff.iter().map(|s| s.to_string()).collect(),
                m_type: "Electric".to_string(),
                max_hp: 50,
                c_hp: 50,
                stamina: 30,
                skills: vec!["Spark".to_string(), "Thunderbolt".to_string(), "Charge Beam".to_string(), "Quick Attack".to_string()],
            }],
            _ => vec![Monster {
                m_name: "Chlorowl".to_string(),
                art: Chlorowl.iter().map(|s| s.to_string()).collect(),
                m_type: "Grass".to_string(),
                max_hp: 60,
                c_hp: 60,
                stamina: 25,
                skills: vec!["Leaf Blade".to_string(), "Vine Whip".to_string(), "Nature Heal".to_string(), "Peck".to_string()],
            }],
        },
        _ => vec![Monster {
            m_name: "Bossmon".to_string(),
            art: vec![r#" (•_•) "#.to_string(), r#"<)   )╯"#.to_string(), r#" /   \ "#.to_string(), r#"        "#.to_string()],
            m_type: "Legendary".to_string(),
            max_hp: 120,
            c_hp: 120,
            stamina: 50,
            skills: vec!["Hyper Beam".to_string(), "Dragon Claw".to_string(), "Recover".to_string(), "Shadow Ball".to_string()],
        }],
    };

    Trainer {
        name: "Opponent".to_string(),
        money: 100,
        m_owned: monsters,
        battles: battles as i32,
        potion: 1,
        mega: 0,
    }
}

// -------------------- SHOP --------------------
fn shop(player: &mut Trainer) {
    loop {
        execute!(stdout(), MoveTo(0, 3)).unwrap();
        println!("| {:^width$} |", "Welcome to the Shop!", width = Screen_Width - 2);
        println!("| {:^width$} |", format!("Your money: ${}", player.money), width = Screen_Width - 2);
        println!("| {:^width$} |", "", width = Screen_Width - 2);

        println!("| {:<width$} |", "[1] Potion - $50", width = Screen_Width - 2);
        println!("| {:<width$} |", "[2] Mega - $100", width = Screen_Width - 2);
        println!("| {:<width$} |", "[3] Exit Shop", width = Screen_Width - 2);

        io::stdout().flush().unwrap();

        let choice = get_input("");
        match choice.trim() {
            "1" => {
                if player.money >= 50 {
                    player.money -= 50;
                    player.potion += 1;
                    intro_print(&format!("You bought a Potion! You now have {} Potions.", player.potion), 8);
                    thread::sleep(Duration::from_secs(1));
                } else {
                    intro_print("Not enough money!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "2" => {
                if player.money >= 100 {
                    player.money -= 100;
                    player.mega += 1;
                    intro_print(&format!("You bought a Mega! You now have {} Mega.", player.mega), 8);
                    thread::sleep(Duration::from_secs(1));
                } else {
                    intro_print("Not enough money!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "3" => {
                break;
            }
            _ => intro_print("Invalid choice!", 8),
        }
    }
}

// -------------------- MAIN --------------------
fn main() {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    thread::sleep(Duration::from_secs(1));

    // Load or new player
    let mut player = if let Some(saved_player) = load_game() {
        saved_player
    } else {
        let player_mon = Monster {
            m_name: "Electripuff".to_string(),
            art: Electripuff.iter().map(|s| s.to_string()).collect(),
            m_type: "Electric".to_string(),
            max_hp: 50,
            c_hp: 50,
            stamina: 30,
            skills: vec!["Spark".to_string(), "Thunderbolt".to_string(), "Charge Beam".to_string(), "Quick Attack".to_string()],
        };
        Trainer {
            name: "You".to_string(),
            money: 100,
            m_owned: vec![player_mon],
            battles: 0,
            potion: 1,
            mega: 0,
        }
    };

    let mut lost = false;

    while !lost {
        let mut opponent = opp_random(player.battles as u32).m_owned[0].clone();
        let player_mon = &mut player.m_owned[0];

        // Battle loop
        loop {
            screen(
                Screen_Width,
                &player_mon.m_name,
                &player_mon.art,
                player_mon.max_hp as u32,
                player_mon.c_hp as u32,
                &opponent.m_name,
                &opponent.art,
                opponent.max_hp as u32,
                opponent.c_hp as u32,
            );

            if player_mon.c_hp <= 0 {
                intro_print("You lost the battle!", 8);
                lost = true;
                break;
            }
            if opponent.c_hp <= 0 {
                intro_print("You won the battle!", 8);
                player.battles += 1;
                player.money += 100;
                player_mon.max_hp += 5;
                player_mon.stamina += 2;
                player_mon.c_hp = player_mon.max_hp;
                let _ = save_game(&player);
                thread::sleep(Duration::from_secs(2));
                break;
            }

            let action = get_input("").to_uppercase();
            match action.as_str() {
                "A" => {
                    // Use first skill by default for simplicity
                    attack(player_mon, &mut opponent, 0);
                }
                "B" => {
                    bag(&mut player, player_mon);
                }
                "M" => {
                    intro_print("You ran away...", 8);
                    lost = true;
                    break;
                }
                _ => {}
            }

            // Opponent attacks
            if opponent.c_hp > 0 {
                let mut rng = rand::thread_rng();
                let opp_skill = rng.gen_range(0..opponent.skills.len());
                attack(&mut opponent, player_mon, opp_skill);
            }
        }

        if lost { break; }

        // Show shop
        shop(&mut player);
    }

    let _ = save_game(&player);
    thread::sleep(Duration::from_secs(2));
    execute!(stdout, LeaveAlternateScreen).unwrap();
}
