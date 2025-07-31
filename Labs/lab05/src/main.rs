// //Part 1
// use std::io::Write;
// use std::io;

// fn get_input(prompt: &str) -> String {
//     print!("{}", prompt);

//     io::stdout().flush().unwrap();

//     let mut input = String::new();

//     io::stdin()
//     .read_line(&mut input)
//     .expect("Failed to read line");

//     input.trim().to_string()
// }

// fn celsius_to_fahrenheit(c: f64) -> f64 {
//     let converted = (c * 9.0 / 5.0) + 32.0;
//     converted
// }

// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     let converted = (f - 32.0) * 5.0 / 9.0;
//     converted
// }

// fn show_temperature(label: &str, value: &f64){
//     println!("{} {:.2}",label ,value)
// }

// fn adjust_temperature(value: &mut f64, delta: f64){
//     *value += delta;
// }

// fn main(){
//     let mut running = true;

//     while running == true {    
//         let input = get_input("Enter the convertion mode:\n1.Celcius -> Fahrenheit\n2.Fahrenheit -> Celcius\n3.Exit the program\nChoice: ");
//         let mode: u32 = match input.parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Invalid input.");
//                 return;
//             }
//         };

//         if mode == 1 {
//             let input = get_input("Enter temperature in Celsius: ");
//             let mut celsius: f64 = match input.parse() {
//                 Ok(num) => num,
//                 Err(_) => {
//                     println!("Invalid input.");
//                     return;
//                 }
//             };
//             let mut convert = celsius_to_fahrenheit(celsius);
//             adjust_temperature(&mut convert, 0.5);
//             show_temperature("That equals to", &convert);
//             break;
//         }else if mode == 2 {
//             let input = get_input("Enter temperature in Fahrenheit: ");
//             let fahrenheit: f64 = match input.parse() {
//                 Ok(num) => num,
//                 Err(_) => {
//                     println!("Invalid input.");
//                     return;
//                 }
//             };
//             let mut convert = fahrenheit_to_celsius(fahrenheit);
//             adjust_temperature(&mut convert, 0.5);
//             show_temperature("That equals to", &convert);
//             break;
//         }else if mode == 3 {
//             running = false;
//             return;
//         }else {
//             println!("Invalid mode selected.\n");
//         }
//     }
// }

// //Part 2
// use std::io::Write;
// use std::io;

// fn get_input(prompt: &str) -> String {
//     print!("{}", prompt);

//     io::stdout().flush().unwrap();

//     let mut input = String::new();

//     io::stdin()
//     .read_line(&mut input)
//     .expect("Failed to read line");

//     input.trim().to_string()
// }

// fn sum_of_digits(n: u32) -> u32 {
//     if n == 0 {
//         0
//     }else{
//         // n % 10 essentially takes the last digit while n/10 cuts out the last digit after use (floating point get cut out cuz integer type)
//         n % 10 + sum_of_digits(n/10)
//     }
// }

// fn digits_check(n: i32) -> Result<u32, String> {
//     if n < 0 {
//         Err(String::from("There is a negative number"))
//     }else{
//         Ok(sum_of_digits(n as u32))
//     }
// }yu

// fn main(){
//     let input = get_input("Enter a number: ");

//     let number: i32 = match input.parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("Invalid input.");
//             return;
//         }
//     };
    
//     print!("{:?}",digits_check(number));    
// }

//Part 3
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

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn knr(row: u32, position: u32) -> u32 {
    factorial(row) / (factorial(position) * factorial(row - position))
}

fn make_pascal(rows: u32){
    for i in 0..rows {
        for _ in 0..rows - i - 1{
            print!("  ");
        }

        for p in 0..=i {
            print!("{:4}", knr(i, p));
        }
        println!();
    }
}

fn main(){
    let mut prompt = true;

    let input = get_input("Enter rows for Pascal's Triangle (Between 1-9): ");
    
    let mut rows: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    while prompt == true {
        if rows < 1 {
            println!("Invalid input");

            let input = get_input("Enter rows for Pascal's Triangle (Between 1-9): ");

            rows = match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input.");
                    return;
                }
            };
            
        }else if rows > 9 {
            println!("Invalid input");

            let input = get_input("Enter rows for Pascal's Triangle (Between 1-9): ");
            
            rows = match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input.");
                    return;
                }
            };
        }else{
            prompt = false;
            break;
        }
    }

    make_pascal(rows);
}