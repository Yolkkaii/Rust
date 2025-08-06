// use std::io;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let mut heater = false;
//     let mut air_con = false;

//     loop {
//         println!("Enter the temperature");

//         let mut input = String::new();
//         match io::stdin().read_line(&mut input){
//             Ok(_) => println!("Valid input!"),
//             Err(error) => {
//                 println!("Error reading from stdin: {}", error);
//                 continue;
//             }
//         }

//         let temp: i32 = match input.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Input not an integer!");
//                 continue;
//             }
//         };

//         check_temp(temp, &mut heater, &mut air_con);
//         thread::sleep(Duration::from_secs(5));
//     }
// }

// fn check_temp(value: i32, heat: &mut bool, air_c: &mut bool) {
//     if value < 18 {
//         if !*heat {
//             *heat = true;
//             println!("Turned on heater!");
//         }
//         if *air_c {
//             *air_c = false;
//             println!("Turned off air conditioner!");
//         }

//     } else if value > 24 {
//         if !*air_c {
//             *air_c = true;
//             println!("Turned on air conditioner!");
//         }
//         if *heat {
//             *heat = false;
//             println!("Turned off heater!");
//         }

//     } else {
//         if *heat {
//             *heat = false;
//             println!("Turned off heater!");
//         }
//         if *air_c {
//             *air_c = false;
//             println!("Turned off air conditioner!");
//         }
//     }
// }

//Practice: Trait, Implementation

trait HasArea {
    fn area(&self) -> f64;
}

struct Rectangle{
    width: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("Area: {}", shape.area())
}

fn main(){
    let r = Rectangle { width: 4.0, height: 5.0};
    print_area(r);
}
