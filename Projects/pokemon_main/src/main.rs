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

//const for screen size
const Screen_Width: usize = 80;
const Screen_Height: usize = 30;

//const for pokemon ascii art
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

//Index 0: Electric, 1: Grass, 2: Fire, 3: Water, 5: Normal
const TYPE_CHART: [[f32; 5]; 5] = [
    //Electric
    [0.5, 0.5, 1.0, 2.0, 1.0], 
    //Grass
    [1.0, 0.5, 0.5, 2.0, 1.0], 
    //Fire
    [1.0, 2.0, 0.5, 0.5, 1.0], 
    //Water
    [2.0, 0.5, 2.0, 0.5, 1.0], 
    //Normal
    [1.0, 1.0, 1.0, 1.0, 1.0], 
];

//Make battle screen ui
fn screen(
    scr_width: usize,
    p_name: &str,
    p_art: &[&str],
    p_max_hp: u32,
    p_hp: u32,
    opp_name: &str,
    opp_art: &[&str],
    opp_max_hp: u32,
    opp_hp: u32,) {
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
//Get input from player
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    input.trim().to_string()
}

//Creat new empty screen
fn empty_screen() {
    execute!(stdout(), MoveTo(0, 0)).unwrap();
    println!("+{}+", "-".repeat(Screen_Width));
    for _ in 0..15 {
        println!("|{}|", " ".repeat(Screen_Width));
    }
    println!("+{}+", "-".repeat(Screen_Width));
}

//Prints intro texts in the middle at a certain row
fn intro_print(message: &str, row: u16) {
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", message, width = Screen_Width - 2);
}

//Clears intro texts
fn intro_clear() {
    execute!(stdout(), MoveTo(0, 6)).unwrap();
    for _ in 0..5 {
        println!("| {:^width$} |", " ", width = Screen_Width - 2);
    }
}

//Does a dot dot dot in the intro
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

//Animation for closing curtain animation
fn battle_anim() {
    //It works by moving to a position and making it a white block (\u{2588}) it starts at the far ends then moves to middle of screen
    //Moves to next colum after finishing coloring current colum
    for i in 0..(Screen_Width as u16) / 2 {
        for j in 0..Screen_Height as u16 - 2 {
            execute!(stdout(), MoveTo(i, j)).unwrap();
            print!("\x07{}", "\u{2588}");
            execute!(stdout(), MoveTo(Screen_Width as u16 - 1 - i, j)).unwrap();
            print!("\x07{}", "\u{2588}");
        }
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(15));
    }
    thread::sleep(Duration::from_millis(500));
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
}

//For showing the intro to the game
fn intro(battle: u32) {
    empty_screen();

    if battle == 0 {
        intro_print("\x07In the year 2033, humans came upon a new creature.", 6);
        dot_waiting(8);

        intro_print("\x07These creatures are called (due to legal purpose) \"Pocket Mon\"", 10);
        thread::sleep(Duration::from_secs(5));
        intro_clear();

        intro_print("\x07The creatures have a nature of battling each other.", 6);
        thread::sleep(Duration::from_secs(1));

        dot_waiting(8);
        intro_print("\x07But they are very friendly with humans.", 10);
        thread::sleep(Duration::from_secs(5));
        intro_clear();

        intro_print("\x07You are a new trainer for these creatures.", 6);
        thread::sleep(Duration::from_secs(2));

        intro_print("\x07Training, battling and spending time with them.", 10);
        thread::sleep(Duration::from_secs(5));
        intro_clear();

        intro_print("\x07Your first battle starts here.", 6);
        thread::sleep(Duration::from_secs(1));

        dot_waiting(8);
        intro_print("\x07Against little Timmy.", 10);
        thread::sleep(Duration::from_secs(2));
        intro_clear();

        intro_print("\x07NN   NN   OOOOO   W     W   !!!", 6);
        intro_print("NNN  NN  O     O  W     W   !!!", 7);
        intro_print("NN N NN  O     O  W  W  W   !!!", 8);
        intro_print("NN  NNN  O     O   W W W       ", 9);
        intro_print("NN   NN   OOOOO     W W     !!! ", 10);
        thread::sleep(Duration::from_secs(1));

        battle_anim();
    }
}


//Week 3: Battle
//Struct for player and opponent stats
#[derive(Debug, Clone)]
struct Trainer {
    name: String, money: i32, m_owned: Vec<Monster>, battles: i32, potion: i32, mega: i32,
}

//Struct for monster stats
#[derive(Debug, Clone)]
struct Monster {
    m_name: String, art: Vec<&'static str>, m_type: i32, max_hp: i32, c_hp: i32, stamina: i32, skills: Vec<&'static str>,
}

//Enum for choosing actions in battle
enum Actions{
    A,
    S,
    B,
    E,
}

//Randomize the enemy pokemon based on the battles you've won (up to 3rd battle)
fn opp_random(battles: u32) -> Trainer {
    let mut rng = rand::thread_rng();
    let chance = rng.gen_range(0..100);

    let monsters: Vec<Monster> = match battles {
        0 => match chance {
            0..=59 => vec![Monster {
                m_name: "Electripuff".to_string(),
                art: Electripuff.to_vec(),
                m_type: 0,
                max_hp: 50,
                c_hp: 50,
                stamina: 30,
                skills: vec!["Spark", "Thunderbolt", "Charge Beam", "Quick Attack"],
            }],
            _ => vec![Monster {
                m_name: "Chlorowl".to_string(),
                art: Chlorowl.to_vec(),
                m_type: 1,
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
                m_type: 4,
                max_hp: 40,
                c_hp: 40,
                stamina: 20,
                skills: vec!["Splash", "Water Gun", "Bubble Beam", "Aqua Tail"],
            }],
            20..=74 => vec![Monster {
                m_name: "Flamalink".to_string(),
                art: Flamalink.to_vec(),
                m_type: 3,
                max_hp: 55,
                c_hp: 55,
                stamina: 28,
                skills: vec!["Ember", "Flame Wheel", "Fire Fang", "Heat Wave"],
            }],
            _ => vec![Monster {
                m_name: "Norn".to_string(),
                art: Norn.to_vec(),
                m_type: 4,
                max_hp: 70,
                c_hp: 70,
                stamina: 35,
                skills: vec!["Tackle", "Bite", "Headbutt", "Growl"],
            }],
        },
        _ => vec![Monster {
            m_name: "Bossmon".to_string(),
            art: vec![r#" (•_•) "#, r#"<)   )╯"#, r#" /   \ "#, r#"       "#],
            m_type: 4,
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

//Enum for turn actions
enum TurnResult {
    Continue,
    Ran,
    Back,
    Exit,
}

fn attack(attacker: &mut Monster, defender: &mut Monster, skill_index: usize) {
    //Calculate damage based on pokemon types
    let multiplier = TYPE_CHART[attacker.m_type as usize][defender.m_type as usize];
    let skill = attacker.skills[skill_index];
    let base_damage = (5 + (skill_index as i32 * 5));
    let damage = ((base_damage as f32) * multiplier).round() as i32;
    let real_dmg = damage.min(defender.c_hp);
    defender.c_hp -= real_dmg;

    execute!(stdout(), MoveTo(0, 8)).unwrap();
    println!("| {:^width$} |", 
        format!("\x07{} used {}! (-{} HP)", attacker.m_name, skill, damage),
        width = Screen_Width - 2
    );

    //Attac effectiveness message
    if (multiplier - 2.0).abs() < f32::EPSILON {
        println!("| {:^width$} |", "It's super effective!", width = Screen_Width - 2);
    } else if (multiplier - 0.5).abs() < f32::EPSILON {
        println!("| {:^width$} |", "It's not very effective...", width = Screen_Width - 2);
    }

    thread::sleep(Duration::from_secs(1));
}


fn bag(player: &mut Trainer, monster: &mut Monster) -> Option<String> {
    //Create bag ui
    loop {
        execute!(stdout(), MoveTo(0, 15), SetForegroundColor(Color::Cyan)).unwrap();
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
            //Use potion to heal pokemon
            "1" => {
                if player.potion > 0 {
                    player.potion -= 1;
                    let heal_amount = 20;
                    let healed_hp = (monster.c_hp + heal_amount).min(monster.max_hp);
                    let actual_heal = healed_hp - monster.c_hp;
                    monster.c_hp = healed_hp;
                    intro_print(
                        &format!("\x07You used a Potion! {} healed +{} HP", monster.m_name, actual_heal),
                        8,
                    );
                    thread::sleep(Duration::from_secs(1));
                    return Some("used_item".to_string());
                } else {
                    intro_print("\x07No Potions left!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            //Use mega potion to heal pokemon
            "2" => {
                if player.mega > 0 {
                    player.mega -= 1;
                    let heal_amount = 50;
                    let healed_hp = (monster.c_hp + heal_amount).min(monster.max_hp);
                    let actual_heal = healed_hp - monster.c_hp;
                    monster.c_hp = healed_hp;
                    intro_print(
                        &format!("\x07You used a Mega! {} healed +{} HP", monster.m_name, actual_heal),
                        8,
                    );
                    thread::sleep(Duration::from_secs(1));
                    return Some("used_item".to_string());
                } else {
                    intro_print("\x07No Megas left!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            //Go back to choose other actions
            "3" => {
                print!("\x07");
                return Some("back".to_string())
            },
            //Print invalid if press wrong button
            _ => {
                intro_print("\x07Invalid choice.", 8);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn shop(player: &mut Trainer) {
    //Creat shop screen
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
            //Buy a potion if player has enough money
            "1" => {
                if player.money >= 50 {
                    player.money -= 50;
                    player.potion += 1;
                    intro_print(&format!("\x07You bought a Potion! You now have {} Potions.", player.potion), 8);
                    thread::sleep(Duration::from_secs(1));
                } else {
                    intro_print("\x07Not enough money for a Potion!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            //Buy a mega potion if player has enough money
            "2" => {
                if player.money >= 100 {
                    player.money -= 100;
                    player.mega += 1;
                    intro_print(&format!("\x07You bought a Mega! You now have {} Mega.", player.mega), 8);
                    thread::sleep(Duration::from_secs(2));
                } else {
                    intro_print("\x07Not enough money for a Mega!", 8);
                    thread::sleep(Duration::from_secs(2));
                }
            }
            //Go to the next battle
            "3" => break{
                print!("\x07");
                battle_anim();
                empty_screen();
            },
            _ => intro_print("\x07Invalid choice!", 8),
        }
    }
}

//Function to take player action choice in battle
fn action(
    player: &mut Trainer,
    player_mon: &mut Monster,
    opponent: &mut Monster,
    lost: &mut bool,
    stdout: &mut std::io::Stdout,
) -> TurnResult {
    let action = get_input("").to_uppercase();
    match action.as_str() {
        //Show pokemon's skills if player press A
        "A" => {
            intro_print("\x07You chose to attack!", 8);
            let skill1 = player_mon.skills[0];
            let skill2 = player_mon.skills[1];
            let skill3 = player_mon.skills[2];
            let skill4 = player_mon.skills[3];

            execute!(*stdout, MoveTo((78 - skill1.len() - skill2.len() - 9).try_into().unwrap(), 15))
            .unwrap();
            println!("[1] {} | [2] {} |", skill1, skill2);

            execute!(*stdout, MoveTo((78 - skill3.len() - skill4.len() - 9).try_into().unwrap(), 16))
            .unwrap();
            println!("[3] {} | [4] {} |", skill3, skill4);

            let skill_choice = get_input("");
            if let Ok(num) = skill_choice.parse::<usize>() {
                if num >= 1 && num <= player_mon.skills.len() {
                    attack(player_mon, opponent, num - 1);
                } else {
                    intro_print("Invalid skill choice!", 8);
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                intro_print("Please enter a valid number!", 8);
                thread::sleep(Duration::from_secs(1));
            }

            TurnResult::Continue
        }
        //Show items in bag if player press B
        "B" => {
            print!("\x07");
            bag(player, player_mon);
            TurnResult::Back
        }
        //Exit the game if player press M
        "M" => {
            intro_print("\x07You ran away...", 8);
            *lost = true;
            TurnResult::Exit
        }
        _ => {
            println!("\x07Invalid action.");
            TurnResult::Continue
        }
    }
}

fn main() {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    thread::sleep(Duration::from_secs(1));

    let mut battle = 0;
    let mut lost = false;

    intro(battle.try_into().unwrap());

    //Make player pokemon
    let mut player_mon = Monster {
        m_name: "Electripuff".to_string(),
        art: Electripuff.to_vec(),
        m_type: 1,
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
        //Random enemy pokemon
        let mut opponent = opp_random(battle.try_into().unwrap()).m_owned[0].clone();

        //Loop for battling
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

            //Check if player pokemon died
            if player_mon.c_hp <= 0 {
                intro_print("You lost the battle!", 8);
                lost = true;
                break;
            }

            //Check if enemy pokemon died
            if opponent.c_hp <= 0 {
                intro_print("You won the battle!", 8);
                battle += 1;
                player.money += 100;

                player_mon.max_hp += 5;
                player_mon.stamina += 2;
                player_mon.c_hp = player_mon.max_hp;
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

            //Player turn
            let result = action(&mut player, &mut player_mon, &mut opponent, &mut lost, &mut stdout);

            match result {
                TurnResult::Ran | TurnResult::Exit => {
                    lost = true;
                    break;
                }
                TurnResult::Back => continue,
                TurnResult::Continue => {}
            }

            //Opponent turn if not died yet
            if opponent.c_hp > 0 {
                let mut rng = rand::thread_rng();
                let opp_skill = rng.gen_range(0..opponent.skills.len());
                attack(&mut opponent, &mut player_mon, opp_skill);
            }

            if lost {
                break;
            }
        } 

        //Check if lost before going to shop, exit game if lost
        if lost {
            break;
        }

        //Enter shop if won battle
        empty_screen();
        shop(&mut player);
    }

    thread::sleep(Duration::from_secs(2));
    execute!(stdout, LeaveAlternateScreen).unwrap();
}
