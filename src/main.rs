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

fn generate_pascals_triangle(rows: usize) -> Vec<Vec<u32>> {
    let mut triangle: Vec<Vec<u32>> = Vec::new();

    for i in 0..rows {
        let mut row: Vec<u32> = vec![1; i + 1];

        for j in 1..i {
            row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
        }

        triangle.push(row);
    }

    triangle
}

fn print_triangle(triangle: &Vec<Vec<u32>>) {
    let total_rows = triangle.len();

    for (i, row) in triangle.iter().enumerate() {
        // Center align rows
        let padding = " ".repeat(total_rows - i);
        print!("{}", padding);

        for value in row {
            print!("{} ", value);
        }
        println!();
    }
}

fn main() {
    println!("Enter number of rows for Pascal's Triangle:");

    let input = get_input("Input rows: ");

    let rows: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    let triangle = generate_pascals_triangle(rows);
    print_triangle(&triangle);
}