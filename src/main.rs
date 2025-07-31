// fn main() {
//     let arr = [5, 10, 15, 20, 25];
//     let slice = &arr[1..4];
//     let tuple = ("Rust", 2024, true);

//     for x in slice {
//         println!("{}", x);
//     };

//     println!("{} {} {}", tuple.0, tuple.1, tuple.2);

//     let mut _count = 1;
//     let mut _sum = 0;

//     while _count <= 10 {
//         _sum += _count;
//         _count += 1;
//     }

//     println!("{}", _sum)
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
    let input = get_input("Enter rows for Pascal's Triangle: ");
    
    let rows: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };
    make_pascal(rows);
}
