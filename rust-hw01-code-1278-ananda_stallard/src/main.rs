use std::io; //Import io library

use std::thread;
use std::time::Duration;

fn main() {
    //Variables for devices
        let mut heater = false;
        let mut air_con = false;

    loop {
        println!("Enter the temperature");

        let mut input = String::new();

        //Read and store into Input variable
        match io::stdin().read_line(&mut input){
            Ok(_)=>{
                print!("");
            },
            Err(error)=>{
                println!("Error reading from stdin: {}", error);
            }
        }

        //check if the user manually stopped the program
        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("stop") {
            println!("Program manually stopped.");
            break;
        }

        //check if invalid input
        let temp: i32 = match trimmed_input.parse() {
            Ok(num) => {num},
            Err(_) => {
                println!("Invalid input. Please enter a number or type 'Stop' to exit.");
                continue;
            }
        };

        check_temp(temp, &mut heater, &mut air_con);

        println!("Temperature will be measured again in 5 minutes");
        thread::sleep(Duration::from_secs(5 * 60));
    }
    
}

fn check_temp(value: i32, heat: &mut bool, air_c: &mut bool) {
    //* is used to refer to a variable
    //if used in an if statement it works as
    //if *heat --> if heat is on
    //if !*heat --> if heat is off

    //! is like a NOT statement

    if value < 18 {
        if *air_c {
            *air_c = false;
            println!("Turned off air conditioner!");
        }
        if !*heat {
            *heat = true;
            println!("Turned on heater!");
        }

    } else if value > 24 {
        if *heat {
            *heat = false;
            println!("Turned off heater!");
        }
        if !*air_c {
            *air_c = true;
            println!("Turned on air conditioner!");
        }

    } else {
        if *heat {
            *heat = false;
            println!("Turned off heater!");
        }
        if *air_c {
            *air_c = false;
            println!("Turned off air conditioner!");
        }
    }
}