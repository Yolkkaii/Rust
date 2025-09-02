// Week 1: UI Design
use rand::Rng;
use std::{
    io::{self, stdout, Write},
    thread,
    time::Duration,
};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

const Screen_Width: usize = 80;

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

const Norn: [&str; 3] = [
    r#"( o.o )"#,
    r#"<(   )>"#,
    r#"  U U  "#,
];

const Magifish: [&str; 1] = [r#"><(((°>"#];

const Flamalink: [&str; 4] = [
    r#"(  )     "#,
    r#"(o_o )  ~🔥"#,
    r#"<)   )╯   "#,
    r#" /   \    "#,
];

#[derive(Debug, Clone)]
struct Monster {
    m_name: String,
    art: Vec<&'static str>,
    m_type: String,
    max_hp: i32,
    c_hp: i32,
    stamina: i32,
    skills: Vec<&'static str>,
}

#[derive(Debug, Clone)]
struct Trainer {
    name: String,
    money: i32,
    m_owned: Vec<Monster>,
    battles: i32,
    potion: i32,
    mega: i32,
}

// ------------------------- UI Functions -------------------------
fn screen(
    scr_width: usize,
    p_name: &str,
    p_art: &[&str],
    p_max_hp: u32,
    p_hp: u32,
    opp_name: &str,
    opp_art: &[&str],
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

    for line in opp_art {
        println!("| {:>width$} |", line, width = scr_width - 2);
    }

    for line in p_art {
        println!("| {:<width$} |", line, width = scr_width - 2);
    }

    execute!(stdout, SetForegroundColor(Color::Cyan)).unwrap();
    println!(
        "| {:<left_width$}{:>right_width$} |",
        p_name,
        "[A] Attack | [S] Swap",
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
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn intro_print(message: &str, row: u16) {
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", message, width = Screen_Width - 2);
}

// ------------------------- Game Functions -------------------------
fn attack(attacker: &mut Monster, defender: &mut Monster, skill_index: usize) {
    let skill = attacker.skills[skill_index];
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
    intro_print("Opening your bag...", 8);
    thread::sleep(Duration::from_secs(1));
    intro_print("[1] Potion | [2] Mega", 9);

    let choice = get_input("");
    match choice.as_str() {
        "1" => {
            if player.potion > 0 {
                player.potion -= 1;
                let heal_amount = 20;
                let healed_hp = (monster.c_hp + heal_amount).min(monster.max_hp);
                let actual_heal = healed_hp - monster.c_hp;
                monster.c_hp = healed_hp;
                intro_print(
                    &format!("You used a Potion! {} healed +{} HP", monster.m_name, actual_heal),
                    8,
                );
            } else {
                intro_print("No Potions left!", 8);
            }
        }
        "2" => {
            if player.mega > 0 {
                player.mega -= 1;
                intro_print(
                    &format!("You used a Mega Stone on {}!", monster.m_name),
                    8,
                );
            } else {
                intro_print("No Mega Stones left!", 8);
            }
        }
        _ => intro_print("Invalid choice.", 8),
    }
    thread::sleep(Duration::from_secs(2));
}

fn opp_random_multi(battle_num: usize, max_opp: usize) -> Trainer {
    let mut rng = rand::thread_rng();
    let mut mons = Vec::new();

    let num_pokemon = rng.gen_range(1..=max_opp);

    for _ in 0..num_pokemon {
        let chance = rng.gen_range(0..100);
        let mon = match battle_num {
            0 => match chance {
                0..=59 => Monster {
                    m_name: "Electripuff".to_string(),
                    art: Electripuff.to_vec(),
                    m_type: "Electric".to_string(),
                    max_hp: 50,
                    c_hp: 50,
                    stamina: 30,
                    skills: vec!["Spark", "Thunderbolt", "Charge Beam", "Quick Attack"],
                },
                _ => Monster {
                    m_name: "Chlorowl".to_string(),
                    art: Chlorowl.to_vec(),
                    m_type: "Grass".to_string(),
                    max_hp: 60,
                    c_hp: 60,
                    stamina: 25,
                    skills: vec!["Leaf Blade", "Vine Whip", "Nature Heal", "Peck"],
                },
            },
            1 => match chance {
                0..=19 => Monster {
                    m_name: "Magifish".to_string(),
                    art: Magifish.to_vec(),
                    m_type: "Water".to_string(),
                    max_hp: 40,
                    c_hp: 40,
                    stamina: 20,
                    skills: vec!["Splash", "Water Gun", "Bubble Beam", "Aqua Tail"],
                },
                20..=74 => Monster {
                    m_name: "Flamalink".to_string(),
                    art: Flamalink.to_vec(),
                    m_type: "Fire".to_string(),
                    max_hp: 55,
                    c_hp: 55,
                    stamina: 28,
                    skills: vec!["Ember", "Flame Wheel", "Fire Fang", "Heat Wave"],
                },
                _ => Monster {
                    m_name: "Norn".to_string(),
                    art: Norn.to_vec(),
                    m_type: "Normal".to_string(),
                    max_hp: 70,
                    c_hp: 70,
                    stamina: 35,
                    skills: vec!["Tackle", "Bite", "Headbutt", "Growl"],
                },
            },
            _ => Monster {
                m_name: "Bossmon".to_string(),
                art: vec![r#" (•_•) "#, r#"<)   )╯"#, r#" /   \ "#],
                m_type: "Legendary".to_string(),
                max_hp: 120,
                c_hp: 120,
                stamina: 50,
                skills: vec!["Hyper Beam", "Dragon Claw", "Recover", "Shadow Ball"],
            },
        };
        mons.push(mon);
    }

    Trainer {
        name: format!("Opponent {}", battle_num + 1),
        money: 100,
        m_owned: mons,
        battles: battle_num as i32,
        potion: 1,
        mega: 0,
    }
}

fn main() {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let mut battle_num = 0;

    // Player Pokémon setup
    let mut player = Trainer {
        name: "You".to_string(),
        money: 100,
        m_owned: vec![Monster {
            m_name: "Electripuff".to_string(),
            art: Electripuff.to_vec(),
            m_type: "Electric".to_string(),
            max_hp: 50,
            c_hp: 50,
            stamina: 30,
            skills: vec!["Spark", "Thunderbolt", "Charge Beam", "Quick Attack"],
        }],
        battles: battle_num,
        potion: 2,
        mega: 0,
    };

    loop {
        let max_opp = match battle_num {
            0 => 2,
            1 => 3,
            _ => 5,
        };

        let mut opponent = opp_random_multi(battle_num, max_opp);
        let mut player_idx = 0;
        let mut opp_idx = 0;

        // Reset all Pokémon HP to full
        for m in player.m_owned.iter_mut() {
            m.c_hp = m.max_hp;
        }
        for m in opponent.m_owned.iter_mut() {
            m.c_hp = m.max_hp;
        }

        'battle: loop {
            // Reborrow player_mon and opp_mon temporarily
            {
                let player_mon = &mut player.m_owned[player_idx];
                let opp_mon = &mut opponent.m_owned[opp_idx];

                screen(
                    Screen_Width,
                    &player_mon.m_name,
                    &player_mon.art,
                    player_mon.max_hp as u32,
                    player_mon.c_hp as u32,
                    &opp_mon.m_name,
                    &opp_mon.art,
                    opp_mon.max_hp as u32,
                    opp_mon.c_hp as u32,
                );

                // Check if player fainted
                if player_mon.c_hp <= 0 {
                    // Release borrow first
                    drop(player_mon);

                    if let Some((i, _)) = player.m_owned.iter().enumerate().find(|(_, m)| m.c_hp > 0) {
                        player_idx = i;
                        println!("Go {}!", player.m_owned[player_idx].m_name);
                    } else {
                        println!("All your Pokémon fainted. Game over!");
                        execute!(stdout, LeaveAlternateScreen).unwrap();
                        return;
                    }
                    continue;
                }

                // Check if opponent fainted
                if opp_mon.c_hp <= 0 {
                    opp_idx += 1;
                    if opp_idx >= opponent.m_owned.len() {
                        println!("You won the battle!");
                        let captured = opponent.m_owned[rand::thread_rng()
                            .gen_range(0..opponent.m_owned.len())]
                            .clone();
                        println!("You captured {}!", captured.m_name);
                        player.m_owned.push(captured);
                        break 'battle;
                    }
                    continue;
                }
            } // End temporary borrow

            // Player turn
            let action = get_input("Choose action (A/S/B/M): ").to_uppercase();
            match action.as_str() {
                "A" => {
                    let player_mon = &mut player.m_owned[player_idx];
                    println!("[1] {} | [2] {}", player_mon.skills[0], player_mon.skills[1]);
                    println!("[3] {} | [4] {}", player_mon.skills[2], player_mon.skills[3]);
                    if let Ok(num) = get_input("Choose skill: ").parse::<usize>() {
                        if num >= 1 && num <= player_mon.skills.len() {
                            let opp_mon = &mut opponent.m_owned[opp_idx];
                            attack(player_mon, opp_mon, num - 1);
                        }
                    }
                }
                "S" => {
                    for (i, m) in player.m_owned.iter().enumerate() {
                        println!("[{}] {} (HP: {}/{})", i + 1, m.m_name, m.c_hp, m.max_hp);
                    }
                    if let Ok(num) = get_input("Swap to which? ").parse::<usize>() {
                        if num > 0 && num <= player.m_owned.len() && player.m_owned[num - 1].c_hp > 0 {
                            player_idx = num - 1;
                            println!("Go {}!", player.m_owned[player_idx].m_name);
                        }
                    }
                }
                "B" => {
                    let idx = player_idx;
                    bag(&mut player, idx); // pass index instead of mutable ref
                }
                "M" => {
                    println!("You ran away...");
                    execute!(stdout, LeaveAlternateScreen).unwrap();
                    return;
                }
                _ => println!("Invalid action."),
            }

            // Opponent turn
            {
                let player_mon = &mut player.m_owned[player_idx];
                let opp_mon = &mut opponent.m_owned[opp_idx];
                if opp_mon.c_hp > 0 {
                    let opp_skill = rand::thread_rng().gen_range(0..opp_mon.skills.len());
                    attack(opp_mon, player_mon, opp_skill);
                }
            }
        } // end battle loop

        // Shop
        loop {
            println!("--- Shop ---");
            println!("You have ${}", player.money);
            println!("[1] Potion (10$) - Heals 20 HP");
            println!("[2] Mega (50$) - Increases attack temporarily");
            println!("[3] Continue to next battle");

            match get_input("Choose option: ").as_str() {
                "1" => {
                    if player.money >= 10 {
                        player.money -= 10;
                        player.potion += 1;
                        println!("Purchased Potion!");
                    } else {
                        println!("Not enough money!");
                    }
                }
                "2" => {
                    if player.money >= 50 {
                        player.money -= 50;
                        player.mega += 1;
                        println!("Purchased Mega!");
                    } else {
                        println!("Not enough money!");
                    }
                }
                "3" => break,
                _ => println!("Invalid choice."),
            }
        }

        battle_num += 1;
    }
}
