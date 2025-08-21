//Week 1: UI Design
use std::io::{stdout, Write};
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetForegroundColor, ResetColor},
    execute,
};

const WIDTH: usize = 100;
const HEIGHT: usize = 30;

const PIKACHU_ART: [&str; 3] = [
    r#" (\__/)"#,
    r#" (o^.^)"#,
    r#" z(_(")(""#,
];

const CHARIZARD_ART: [&str; 4] = [
    r#"  /\_/\"#,
    r#"  ( o.o )"#,
    r#"   > ^ < "#,
    r#"  /     \"#,
];

fn draw_screen(
    score: usize,
    life: i32,
    player_name: &str,
    player_hp: u32,
    player_max_hp: u32,
    enemy_name: &str,
    enemy_hp: u32,
    enemy_max_hp: u32,
    turn_info: &str,
    moves: &[&str],
) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0)).unwrap();

    println!("+{}+", "-".repeat(WIDTH));

    execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
    println!("| {:^width$} |", format!("{} Lv. 50", enemy_name), width = WIDTH - 2);
    execute!(stdout, ResetColor).unwrap();

    let bar_width = 30;
    let filled = ((enemy_hp as f32 / enemy_max_hp as f32) * bar_width as f32).round() as usize;
    let empty = bar_width - filled;
    println!("| HP: [{}{}] {}/{}{:width$} |",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(empty),
        enemy_hp,
        enemy_max_hp,
        "",
        width = WIDTH - 15 - bar_width
    );

    for line in CHARIZARD_ART.iter() {
        println!("|{:^width$}|", line, width = WIDTH - 2);
    }

    for _ in 0..2 {
        println!("|{:width$}|", "", width = WIDTH - 2);
    }

    for line in PIKACHU_ART.iter() {
        println!("|{:^width$}|", line, width = WIDTH - 2);
    }

    execute!(stdout, SetForegroundColor(Color::Cyan)).unwrap();
    println!("| {:^width$} |", format!("{} Lv. 50", player_name), width = WIDTH - 2);
    execute!(stdout, ResetColor).unwrap();

    let filled = ((player_hp as f32 / player_max_hp as f32) * bar_width as f32).round() as usize;
    let empty = bar_width - filled;
    println!("| HP: [{}{}] {}/{}{:width$} |",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(empty),
        player_hp,
        player_max_hp,
        "",
        width = WIDTH - 15 - bar_width
    );

    println!("|{:width$}|", "", width = WIDTH - 2);

    println!("| {:<width$}|", format!("What will {} do?", player_name), width = WIDTH - 2);
    for (i, m) in moves.iter().enumerate() {
        println!("| {:<width$}|", format!("{}. {}", i + 1, m), width = WIDTH - 2);
    }

    println!("+{}+", "-".repeat(WIDTH));

    println!("Score: {}, Life: {}", score, life);
    println!("{}", turn_info);

    stdout.flush().unwrap();
}

fn main() {
    draw_screen(
        123, 
        3,   
        "Pikachu", 35, 100,   
        "Charizard", 90, 150, 
        "It's super effective!",
        &["Thunderbolt", "Quick Attack", "Iron Tail", "Volt Tackle"],
    );
}

fn random_M()