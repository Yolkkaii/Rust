//1.1
// fn main() {
//     let test_scores = vec![85, 92, 78, 96, 88, 73, 91, 84];
//     println!("Test scores: {:?}",test_scores);
//     let mut count = test_scores.len();
//     println!("There are {} scores.", count);
// }

//1.2
// fn main() {
//     let test_scores = vec![85, 92, 78, 96, 88, 73, 91, 84];
//     let mut highest = test_scores.iter().max().unwrap();
//     let mut lowest = test_scores.iter().min().unwrap();
//     let mut is_90 = false;
   
//     println!("Highest score: {}", highest);
//     println!("Lowest score: {}", lowest);

//     println!("First score = {}", test_scores[0]);
//     println!("Last score = {}", test_scores[test_scores.len() - 1]);
// }

//1.3
// fn main() {
//     let mut test_scores = vec![85, 92, 78, 96, 88, 73, 91, 84];
//     test_scores.push(87);
//     test_scores.pop();
//     test_scores.sort();
//     println!("Test scores: {:?}", test_scores);
// }

//1.4
// fn main() {
//     let mut test_scores = vec![85, 92, 78, 96, 88, 73, 91, 84];
//     let mut count = 0;
//     let mut over_80 = vec![];
//     test_scores.retain(|x| *x > 75);
//     println!("Test scores: {:?}", test_scores);
//     for i in test_scores {
//         if i >= 85 {
//             count += 1;
//         }
//         if i > 80 {
//             over_80.push(i);
//         }
//     }
    
//     println!("There are {} scores over 85.", count);
//     println!("Scores over 80: {:?}", over_80);
// }

//2 Warehouse Inventory Management
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
    let mut run = true;
    let mut warehouse:Vec<(u32, String, u32)> = Vec::new();
    while run == true {
        //Get input for Menu
        let input = loop {
            let input = get_input("Warehouse Inventory Management:\n1.Add New Product\n2.Update Stock Quantity\n3.Remove Product\n4.List All Products\n5.Exit\nEnter your choice:  ");

            match input.parse::<i32>() {
                Ok(num) => break num,
                Err(_) => println!("Please enter a valid id")
            }
        };

        //Add New Product
        if input == 1 {
            let mut id = loop {
                let input = get_input("Product ID: ");

                match input.parse::<u32>() {
                    Ok(num) => break num,
                    Err(_) => println!("Please enter a valid id")
                }
            };

            let mut id_exists = false;

            for (existing_id, _, _) in &warehouse {
                if *existing_id == id {
                    id_exists = true;
                    while id_exists {
                        id = loop {
                            let input = get_input("ID already exists, please enter a new ID: ");
            
                            match input.parse::<u32>() {
                                Ok(num) => break num,
                                Err(_) => println!("Please enter a valid id")
                            }
                        };
                        if *existing_id != id{
                            id_exists = false;
                            break;
                        }
                    }
                }
            }    

            let name = get_input("Product Name: ");

            let quantity = loop {
                let input = get_input("Product Quantity: ");

                match input.parse::<u32>() {
                    Ok(num) => break num,
                    Err(_) => println!("Please enter a valid id")
                }
            };

            let mut id_exists = false;
            for (existing_id, _, _) in &warehouse {
                if *existing_id == id {
                    id_exists = true;
                    break;
                }
            }

            warehouse.push((id, name, quantity));
            println!("Product added successfully!\n");
        }

        //Update Stock Quantity
        if input == 2 {
            let mut id = loop {
                let input = get_input("Product ID to update: ");

                match input.parse::<u32>() {
                    Ok(num) => break num,
                    Err(_) => println!("Please enter a valid id")
                }
            };

            let mut id_exists = false;
            let mut index = 0;
            let mut count = 0;
            
            for (existing_id, _, _) in &warehouse {
                if count >= 1 {
                    index += 1;
                    count += 1;
                }
                if count <= 0 {
                    count += 1;
                }
                if *existing_id == id {
                    id_exists = true;
                    break;
                }
                while *existing_id != id {
                    id = loop {
                        let input = get_input("ID does not exist, please enter a valid ID : ");
        
                        match input.parse::<u32>() {
                            Ok(num) => break num,
                            Err(_) => println!("Please enter a valid id\n")
                        }
                    };
                }
            }      

            let name: &String = &warehouse[index].1;
            let mut quantity: u32 = 0;
        
            quantity = loop {
                let input = get_input("Input updated stock quantity: ");
    
                match input.parse::<u32>() {
                    Ok(num) => break num,
                    Err(_) => println!("Please enter a valid quantity")
                }
            };

            warehouse[index].2 = quantity;
            println!("Quantity updated successfully!\n");
        }

        //Remove Product
        if input == 3 {
            loop {
                let mut id = loop {
                    let input = get_input("Product ID to remove: ");
    
                    match input.parse::<u32>() {
                        Ok(num) => break num,
                        Err(_) => println!("Please enter a valid id\n")
                    }
                };
    
                let mut id_exists = false;
                let mut index = 0;
                let mut count = 0;
                
                for (existing_id, _, _) in &warehouse {
                    if *existing_id == id {
                        id_exists = true;
                        break;
                    }
                    index += 1;
                }
    
                if id_exists == true {
                    warehouse.remove(index);
                    println!("Product removed successfully!\n");
                    break;
                }else {
                    println!("ID Does not exist");
                    continue;
                }
            }
        }

        //List All Products
        if input == 4 {
            println!("{:?}\n", warehouse);
        }

        //Exit
        if input == 5 {
            run = false;
        }
    }
    
}
