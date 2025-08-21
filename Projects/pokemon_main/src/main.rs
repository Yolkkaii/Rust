//Week 1: UI Design
use std::io;
use std::io::{stdout, Write};

use std::thread;
use std::time::Duration;

use rand;

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetForegroundColor, ResetColor},
    execute,
    terminal::{self, ClearType}
};

const Screen_Width: usize = 80;
const Screen_Height: usize = 30;

const Electripuff: [&str; 4] = [
    r#" (\__/) "#, r#"(o ⚡ o) "#, r#"(  -  )~"#, r#" UU  UU "#,
];

const Chlorowl: [&str; 4] = [
    r#"(\_._/)  "#, r#"( -w- ) 🌱"#, r#"(  V  )  "#, r#" ^^ ^^   "#,
];

const Norn: [&str; 3] = [
    r#"( o.o )"#, r#"<(   )>"#, r#"  U U  "#,
];

const Magifish: [&str; 1] = [
    r#"><(((°>"#,
];

const Flamalink: [&str; 4] = [
    r#"(  )     "#, r#"(o_o )  ~🔥"#, r#"<)   )╯   "#, r#" /   \    "#,
];

fn screen(
    scr_width: usize, scr_height: usize, p_name: &str, p_art: &[&str], p_max_hp: u32, p_hp: u32, opp_name: &str, opp_art: &[&str], opp_max_hp: u32, opp_hp: u32,
) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0)).unwrap();

    println!("+{}+", "-".repeat(scr_width));

    execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
    println!("| {:>width$} |", format!("{}", opp_name), width = scr_width - 2);
    execute!(stdout, ResetColor).unwrap();

    let hp_bar_width = 20;
    let hp_filled = ((opp_hp as f32 / opp_max_hp as f32) * hp_bar_width as f32).round() as usize;
    let hp_empty = hp_bar_width - hp_filled;

    let opp_hp_string = format!(
    "HP: [{}{}] {}/{}",
    "\u{2588}".repeat(hp_filled),
    "\u{2591}".repeat(hp_empty),
    opp_hp,
    opp_max_hp
    );

    println!("| {:>width$} |", opp_hp_string, width = scr_width - 2);

    println!("| {:width$} |", "", width = scr_width - 2);

    for line in opp_art.iter() {
        println!("| {:>width$} |", line, width = scr_width - 2);
    }

    for _ in 0..2{
        println!("| {:width$} |", "", width = scr_width - 2);
    }

    for line in p_art.iter() {
        println!("| {:<width$} |", line, width = scr_width -2);
    }

    println!("| {:width$} |", "", width = scr_width - 2);

    execute!(stdout, SetForegroundColor(Color::Cyan)).unwrap();
    println!("| {:<width$} |", format!("{}", p_name), width = scr_width - 2);
    execute!(stdout, ResetColor).unwrap();

    let hp_filled = ((p_hp as f32 / p_max_hp as f32) * hp_bar_width as f32).round() as usize;
    let hp_empty = hp_bar_width - hp_filled;
    println!("| HP: [{}{}] {}/{}{:<width$} |",
        "\u{2588}".repeat(hp_filled),
        "\u{2591}".repeat(hp_empty),
        p_hp,
        p_max_hp,
        "",
        width = scr_width - 14 - hp_bar_width
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

fn dot_waiting(row: u16){
    execute!(stdout(), MoveTo(0, row)).unwrap();

    thread::sleep(Duration::from_secs(1));
    println!("| {:^width$} |", format!("."), width = Screen_Width - 2);

    thread::sleep(Duration::from_secs(1));
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", format!(".."), width = Screen_Width - 2);

    thread::sleep(Duration::from_secs(1));
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", format!("..."), width = Screen_Width - 2);

    thread::sleep(Duration::from_secs(1));
}

fn empty_screen(){
    execute!(stdout(), MoveTo(0, 0)).unwrap();

    println!("+{}+", "-".repeat(Screen_Width));
    for i in 0..15 {
        println!("|{}|", " ".repeat(Screen_Width));
    }

    println!("+{}+", "-".repeat(Screen_Width));
}

fn intro_print(message: &str, row: u16) {
    execute!(stdout(), MoveTo(0, row)).unwrap();
    println!("| {:^width$} |", message, width = Screen_Width - 2);
}

fn intro_clear(){
    execute!(stdout(), MoveTo(0, 6)).unwrap();
        for i in 0..5{
            println!("| {:^width$} |", format!(" "), width = Screen_Width - 2);
        }
}

fn battle_anim() {
    for i in 0..(Screen_Width as u16) / 2{
        for j in 0..Screen_Height as u16 - 2{
            execute!(stdout(), MoveTo(i, j)).unwrap();
            print!("{}", "\u{2588}");

            execute!(stdout(), MoveTo(Screen_Width as u16 - 1 - i, j)).unwrap();
            print!("{}", "\u{2588}");
        }
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(15));
    }
    thread::sleep(Duration::from_millis(500));

    execute!(stdout(),terminal::Clear(ClearType::All)).unwrap();
}

fn intro(battle: u32, scr_width: usize, scr_height: usize,){
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0)).unwrap();

    empty_screen();
    intro_print("Please set terminal size to fit the screen size for best experience.", 8);

    intro_clear();

    //Change the intro to a battle based on battles fought

    //No Battles, intro to monster and their purpose
    if battle == 0 {
        intro_print("In the year 2033, humans came apon a new creature.", 6);
        
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
        intro_print("NNN  NN  O     O  W     W   !!!", 7) ;
        intro_print("NN N NN  O     O  W  W  W   !!!", 8);
        intro_print("NN  NNN  O     O   W W W       ", 9);        
        intro_print("NN   NN   OOOOO     W W     !!! ", 10);
        thread::sleep(Duration::from_secs(1));

        battle_anim();
    }
}

//Week 3: Battle!
struct Trainer {
    name: String, money: i32, m_owned: i32, battles: i32, potion: i32, revive: i32,
}

struct Monster {
    m_name: String, art: Vec<&'static str>, m_type: String, max_hp: i32, c_hp: i32, stamina: i32, skill_set: i32,
}

fn opp_random(battles: u32) -> Trainer {
    let mut rng = rand::thread_rng();
    let chance = rng.gen_range(0, 100);
    if battles == 0 {
        match chance {
            0..=59 => Trainer {m_owned: 1},
            _ => Trainer {m_owned: 2},
        }
    }if battle == 1{
        match chance {
            0..=19 => Trainer {m_owned: 1},
            20..74 => Trainer {m_owned: 2},
            _ => Trainer {m_owned: 3},
        }
    }if battle == 2{
        match chance {
            0..=9 => Trainer {m_owned: 3},
            10..29 => Trainer {m_owned: 4},
            _ => Trainer {m_owned: 5},
        }
    }
}



fn main(){
    let mut battle = 0;

    intro(battle, Screen_Width, Screen_Height);

    screen(Screen_Width, Screen_Height, "Electripuff", &Electripuff, 50, 35, "Electripuff", &Electripuff, 65, 24);

    //--For testing and bug fixing purposes--\\
    thread::sleep(Duration::from_secs(30));
}
