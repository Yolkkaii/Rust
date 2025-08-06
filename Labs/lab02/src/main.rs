//Lab 01
use std::io;

fn main(){
    println!("Player 1:");
    let mut player_1 = String::new();
    
    match io::stdin().read_line(&mut player_1){
        Ok(_)=>{
            println!("");
        },
        Err(error)=>{
            println!("Error reading from stdin: {}", error);
        }
    }
    let player_1 = player_1.trim();
    
    println! ("Player 2:");
    let mut player_2 = String::new();
    
    match io::stdin().read_line(&mut player_2){
        Ok(_)=>{
            println!("");
        },
        Err(error)=>{
            println!("Error reading from stdin: {}", error);
        }
    }
    let player_2 = player_2.trim();

    let p1_len = player_1.len();
    let p2_len = player_2.len();
    let mut longest = p1_len;
    let mut v_length = 14;
    
    if p1_len > p2_len{
        longest = p1_len;
        v_length += p1_len;
    }if p1_len < p2_len{
        longest = p2_len;
        v_length += p2_len;
    }if p1_len == p2_len{
        longest = p1_len;
        v_length += p1_len;
    }
    
    println!("Vertical Pattern:");
    
    //p1 vertical
    for i in 0..v_length{
        print!("*");
    }
    println!("");
    
    print!("*");
    for i in 0..(v_length - 2){
        print!(" ");
    }
    print!("*");
    println!("");
    
    print!("* Player 1: {}", player_1);
    for i in 0..(v_length - p1_len - 13){
        print!(" ");
    }
    print!("*");
    println!("");
    
    print!("*");
    for i in 0..(v_length - 2){
        print!(" ");
    }
    print!("*");
    println!("");
    
    for i in 0..v_length{
        print!("*");
    }
    println!("");
    
    //p2 vertical
    for i in 0..v_length{
        print!("*");
    }
    println!("");
    
    print!("*");
    for i in 0..(v_length - 2){
        print!(" ");
    }
    print!("*");
    println!("");
    
    print!("* Player 2: {}", player_2);
    for i in 0..(v_length - p2_len - 13){
        print!(" ");
    }
    print!("*");
    println!("");
    
    print!("*");
    for i in 0..(v_length - 2){
        print!(" ");
    }
    print!("*");
    println!("");
    
    for i in 0..v_length{
        print!("*");
    }
    println!("");
    
    //p1 & p2 horizontal
    println!("Horizontal Pattern:");
    let h_length = p1_len + p2_len + 28;
    for i in 0..h_length{
        print!("*");
    }
    println!("");
    
    print!("*");
    for i in 0..(p1_len + 13){
        print!(" ");
    }
    print!("*");
    for i in 0..(p2_len + 13){
        print!(" ");
    }
    print!("*");
    println!("");
    
    println!("* Player 1: {} * Player 2:  {} *", player_1, player_2);
    
    print!("*");
    for i in 0..(p1_len + 13){
        print!(" ");
    }
    print!("*");
    for i in 0..(p2_len + 12){
        print!(" ");
    }
    print!("*");
    println!("");
    
    for i in 0..h_length{
        print!("*");
    }
    println!("");
}

//02
use std::io::Write;
use std::io;

fn get_input(prompt: &str) -> String {

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    input.trim().to_string()
}

fn main() {

    const EMPLOYEES: usize = 5;
    let mut data: [(String, i32, i32); EMPLOYEES] = Default::default();
    let mut total_salary = 0;
    let mut highest_name = String::new();
    let mut highest_salary = 0;
    let mut oldest_name = String::new();
    let mut oldest_age = 0;


    // prompt user for 5 employees

    for i in 0..EMPLOYEES {

        println!("\nEmployee {}", i + 1);

        // name
        let name = get_input("Name: ");

        // age, oldest
        let age = loop {
            let input = get_input("Age: ");

            match input.parse::<i32>() {

            Ok(num) => break num,
            Err(_) => println!("Please enter a valid age")

            }

        };

        if age > oldest_age {

            oldest_name = name.clone();
            oldest_age = age;

        }

        // salary, total_salary, highest_salary
        let salary = loop {

            let input = get_input("Salary: ");

            match input.parse::<i32>() {
                
            Ok(num) => break num,
            Err(_) => println!("Please enter a valid salary")

            }

        };

        total_salary += salary;

        if salary > highest_salary {

            highest_name = name.clone();
            highest_salary = salary;
        }

        data[i] = (name, age, salary);
    }

    println!();

    for  (i, (name, age, salary)) in data.iter().enumerate() {

        println!("Employee {}: Name = {}, Age = {}, Salary = {}", i + 1, name, age, salary);

    }

    println!("Total salary = {total_salary}");
    println!("Average salary = {}", total_salary / EMPLOYEES as i32);
    println!("The employee with the highest salary is: {highest_name} with a salary of {highest_salary}");
    println!("The oldest employee is: {oldest_name}, {oldest_age} years old")
}

//03

use std::io;

fn main() {
    let num = 42;
    
    let mut correct = false;
    let mut tries = 0;
    
    while correct == false{
        println!("Guess a number (1-100)");
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input){
            Ok(_)=>{
                println!("");
            },
            Err(error)=>{
                println!("Error reading from stdin");
            }
        }
        let guess: i32 = input.trim().parse().expect("Input is not an integer");
        
        if guess > num {
            let correct = false;
            tries += 1;
            println!("Too high");
            
        }if guess < num {
            let correct = false;
            tries += 1;
            println!("Too low");
            
        }if guess == num {
            let correct = true;
            tries += 1;
            println!("Correct! You guessed it in {} tries", tries);
            break
        }
    }
}