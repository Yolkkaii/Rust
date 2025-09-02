//Week 1: UI Design
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
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

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

    println!("| {:width$} |", "", width = scr_width - 2);

    for line in opp_art {
        println!("| {:>width$} |", line, width = scr_width - 2);
    }

    for _ in 0..2 {
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

//Week 2: Game introduction
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    input.trim().to_string()
}

fn empty_screen() {
    execute!(stdout(), MoveTo(0, 0)).unwrap();
    println!("+{}+", "-".repeat(Screen_Width));
    for _ in 0..15 {
        println!("|{}|", " ".repeat(Screen_Width));
    }
    println!("+{}+", "-".repeat(Screen_Width));
}

fn intro_print(message: &str, row: u16) {
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", message, width = Screen_Width - 2);
}

fn intro_clear() {
    execute!(stdout(), MoveTo(0, 6)).unwrap();
    for _ in 0..5 {
        println!("| {:^width$} |", " ", width = Screen_Width - 2);
    }
}

fn dot_waiting(row: u16) {
    execute!(stdout(), MoveTo(0, row)).unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("| {:^width$} |", ".", width = Screen_Width - 2);
    thread::sleep(Duration::from_secs(1));
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", "..", width = Screen_Width - 2);
    thread::sleep(Duration::from_secs(1));
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", "...", width = Screen_Width - 2);
    thread::sleep(Duration::from_secs(1));
}

fn battle_anim() {
    for i in 0..(Screen_Width as u16) / 2 {
        for j in 0..Screen_Height as u16 - 2 {
            execute!(stdout(), MoveTo(i, j)).unwrap();
            print!("{}", "\u{2588}");
            execute!(stdout(), MoveTo(Screen_Width as u16 - 1 - i, j)).unwrap();
            print!("{}", "\u{2588}");
        }
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(15));
    }
    thread::sleep(Duration::from_millis(500));
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
}

fn intro(battle: u32) {
    empty_screen();

    if battle == 0 {
        intro_print("In the year 2033, humans came upon a new creature.", 6);
        dot_waiting(8);

        intro_print("These creatures are called (due to legal purpose) \"Pocket Mon\"", 10);
        thread::sleep(Duration::from_secs(5));
        intro_clear();

        intro_print("The creatures have a nature of battling each other.", 6);
        thread::sleep(Duration::from_secs(1));

        dot_waiting(8);
        intro_print("But they are very friendly with humans.", 10);
        thread::sleep(Duration::from_secs(5));
        intro_clear();

        intro_print("You are a new trainer for these creatures.", 6);
        thread::sleep(Duration::from_secs(2));

        intro_print("Training, battling and spending time with them.", 10);
        thread::sleep(Duration::from_secs(5));
        intro_clear();

        intro_print("Your first battle starts here.", 6);
        thread::sleep(Duration::from_secs(1));

        dot_waiting(8);
        intro_print("Against little Timmy.", 10);
        thread::sleep(Duration::from_secs(2));
        intro_clear();

        intro_print("NN   NN   OOOOO   W     W   !!!", 6);
        intro_print("NNN  NN  O     O  W     W   !!!", 7);
        intro_print("NN N NN  O     O  W  W  W   !!!", 8);
        intro_print("NN  NNN  O     O   W W W       ", 9);
        intro_print("NN   NN   OOOOO     W W     !!! ", 10);
        thread::sleep(Duration::from_secs(1));

        battle_anim();
    }
}


//Week 3: Battle!
#[derive(Debug, Clone)]
struct Trainer {
    name: String, money: i32, m_owned: Vec<Monster>, battles: i32, potion: i32, mega: i32,
}

#[derive(Debug, Clone)]
struct Monster {
    m_name: String, art: Vec<&'static str>, m_type: String, max_hp: i32, c_hp: i32, stamina: i32, skills: Vec<&'static str>,
}

enum Actions{
    A,
    S,
    B,
    E,
}

fn opp_random(battles: u32) -> Trainer {
    let mut rng = rand::thread_rng();
    let chance = rng.gen_range(0..100);

    let monsters: Vec<Monster> = match battles {
        0 => match chance {
            0..=59 => vec![Monster {
                m_name: "Electripuff".to_string(),
                art: Electripuff.to_vec(),
                m_type: "Electric".to_string(),
                max_hp: 50,
                c_hp: 50,
                stamina: 30,
                skills: vec!["Spark", "Thunderbolt", "Charge Beam", "Quick Attack"],
            }],
            _ => vec![Monster {
                m_name: "Chlorowl".to_string(),
                art: Chlorowl.to_vec(),
                m_type: "Grass".to_string(),
                max_hp: 60,
                c_hp: 60,
                stamina: 25,
                skills: vec!["Leaf Blade", "Vine Whip", "Nature Heal", "Peck"],
            }],
        },
        1 => match chance {
            0..=19 => vec![Monster {
                m_name: "Magifish".to_string(),
                art: Magifish.to_vec(),
                m_type: "Water".to_string(),
                max_hp: 40,
                c_hp: 40,
                stamina: 20,
                skills: vec!["Splash", "Water Gun", "Bubble Beam", "Aqua Tail"],
            }],
            20..=74 => vec![Monster {
                m_name: "Flamalink".to_string(),
                art: Flamalink.to_vec(),
                m_type: "Fire".to_string(),
                max_hp: 55,
                c_hp: 55,
                stamina: 28,
                skills: vec!["Ember", "Flame Wheel", "Fire Fang", "Heat Wave"],
            }],
            _ => vec![Monster {
                m_name: "Norn".to_string(),
                art: Norn.to_vec(),
                m_type: "Normal".to_string(),
                max_hp: 70,
                c_hp: 70,
                stamina: 35,
                skills: vec!["Tackle", "Bite", "Headbutt", "Growl"],
            }],
        },
        _ => vec![Monster {
            m_name: "Bossmon".to_string(),
            art: vec![r#" (•_•) "#, r#"<)   )╯"#, r#" /   \ "#],
            m_type: "Legendary".to_string(),
            max_hp: 120,
            c_hp: 120,
            stamina: 50,
            skills: vec!["Hyper Beam", "Dragon Claw", "Recover", "Shadow Ball"],
        }],
    };

    Trainer {
        name: "Little Timmy".to_string(),
        money: 100,
        m_owned: monsters,
        battles: battles as i32,
        potion: 1,
        mega: 0,
    }
}

fn attack(attacker: &mut Monster, defender: &mut Monster, skill_index: usize) {
    let skill = attacker.skills[skill_index];
    let base_damage = 5 + (skill_index as i32 * 5);
    let damage = base_damage.min(defender.c_hp);
    defender.c_hp -= damage;

    execute!(stdout(), MoveTo(0, 8)).unwrap();
    println!("| {:^width$} |", 
        format!("{} used {}! (-{} HP)", attacker.m_name, skill, damage),
        width = Screen_Width - 2
    );
    thread::sleep(Duration::from_secs(1));
}

fn bag(player: &mut Trainer, monster: &mut Monster) {
    loop {
        execute!(stdout(), MoveTo(0, 15),SetForegroundColor(Color::Cyan)).unwrap();
        println!(
            "| {:<left_width$} |",
            monster.m_name,
            left_width = Screen_Width - 2,
        );
        execute!(stdout(), ResetColor).unwrap();

        execute!(stdout(), MoveTo((78 - 6 - 6 - 18).try_into().unwrap(), 16)).unwrap();
        println!("[1] Potion | [2] Mega | [3] Back |");
        thread::sleep(Duration::from_secs(1));

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
                    thread::sleep(Duration::from_secs(1));
                    break;
                } else {
                    intro_print("No Potions left!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "2" => {
                if player.mega > 0 {
                    player.mega -= 1;
                    let heal_amount = 50;
                    let healed_hp = (monster.c_hp + heal_amount).min(monster.max_hp);
                    let actual_heal = healed_hp - monster.c_hp;
                    monster.c_hp = healed_hp;
                    intro_print(
                        &format!("You used a Mega! {} healed +{} HP", monster.m_name, actual_heal),
                        8,
                    );
                    thread::sleep(Duration::from_secs(1));
                    break;
                } else {
                    intro_print("No Megas left!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "3" => break, // Back button
            _ => {
                intro_print("Invalid choice.", 8);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}


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

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                if player.money >= 50 {
                    player.money -= 50;
                    player.potion += 1;
                    intro_print(&format!("You bought a Potion! You now have {} Potions.", player.potion), 8);
                    thread::sleep(Duration::from_secs(1));
                } else {
                    intro_print("Not enough money for a Potion!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "2" => {
                if player.money >= 100 {
                    player.money -= 100;
                    player.mega += 1;
                    intro_print(&format!("You bought a Mega! You now have {} Mega.", player.mega), 8);
                    thread::sleep(Duration::from_secs(2));
                } else {
                    intro_print("Not enough money for a Mega!", 8);
                    thread::sleep(Duration::from_secs(2));
                }
            }
            "3" => break{
                empty_screen();
            },
            _ => intro_print("Invalid choice!", 8),
        }
    }
}

fn main() {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    thread::sleep(Duration::from_secs(1));

    let mut battle = 0;
    let mut lost = false;

    // Player's starting monster
    let mut player_mon = Monster {
        m_name: "Electripuff".to_string(),
        art: Electripuff.to_vec(),
        m_type: "Electric".to_string(),
        max_hp: 50,
        c_hp: 50,
        stamina: 30,
        skills: vec!["Spark", "Thunderbolt", "Charge Beam", "Quick Attack"],
    };

    let mut player = Trainer {
        name: "You".to_string(),
        money: 100,
        m_owned: vec![player_mon.clone()],
        battles: battle,
        potion: 1,
        mega: 0,
    };

    while !lost {
        // Generate opponent for current battle
        let mut opponent = opp_random(battle.try_into().unwrap()).m_owned[0].clone();

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

            // Check for lose
            if player_mon.c_hp <= 0 {
                intro_print("You lost the battle!", 8);
                lost = true;
                break;
            }

            // Check for victory
            if opponent.c_hp <= 0 {
                intro_print("You won the battle!", 8);
                battle += 1;
                player.money += 100;

                // Level up player monster
                player_mon.max_hp += 5;
                player_mon.stamina += 2;
                player_mon.c_hp = player_mon.max_hp; // Restore HP
                intro_print(
                    &format!(
                        "{} leveled up! Max HP: {}, Stamina: {}",
                        player_mon.m_name, player_mon.max_hp, player_mon.stamina
                    ),
                    9,
                );
                thread::sleep(Duration::from_secs(2));
                break;
            }

            // Player action
            let action = get_input("").to_uppercase();
            match action.as_str() {
                "A" => {
                    intro_print("You chose to attack!", 8);
                    let skill1 = player_mon.skills[0];
                    let skill2 = player_mon.skills[1];
                    let skill3 = player_mon.skills[2];
                    let skill4 = player_mon.skills[3];

                    execute!(
                        stdout,
                        MoveTo((78 - skill1.len() - skill2.len() - 9).try_into().unwrap(), 15)
                    )
                    .unwrap();
                    println!("[1] {} | [2] {} |", skill1, skill2);

                    execute!(
                        stdout,
                        MoveTo((78 - skill3.len() - skill4.len() - 9).try_into().unwrap(), 16)
                    )
                    .unwrap();
                    println!("[3] {} | [4] {} |", skill3, skill4);
                    thread::sleep(Duration::from_secs(1));
                    intro_print("", 8);

                    let skill_choice = get_input("");
                    if let Ok(num) = skill_choice.parse::<usize>() {
                        if num >= 1 && num <= player_mon.skills.len() {
                            attack(&mut player_mon, &mut opponent, num - 1);
                        }
                    }
                }
                "B" => {
                    bag(&mut player, &mut player_mon);
                }
                "M" => {
                    intro_print("You ran away...", 8);
                    lost = true;
                    break;
                }
                _ => println!("Invalid action."),
            }

            // Opponent attacks if alive
            if opponent.c_hp > 0 {
                let mut rng = rand::thread_rng();
                let opp_skill = rng.gen_range(0..opponent.skills.len());
                attack(&mut opponent, &mut player_mon, opp_skill);
            }
        }

        // If lost or ran away, exit loop
        if lost {
            break;
        }

        // Show shop between battles
        empty_screen();
        shop(&mut player);
    }

    thread::sleep(Duration::from_secs(2));
    execute!(stdout, LeaveAlternateScreen).unwrap();
}