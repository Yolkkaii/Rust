// Task 1: Bank Account
mod BankAccount {
    pub struct Account{
        owner: String,
        balance: u64,
    }

    impl Account{
        pub fn new(owner: String) -> Account{Account{owner:owner ,balance:0}}

        pub fn deposit(&mut self, amount:u64){
            self.balance += amount
        }

        pub fn withdraw(&mut self, amount:u64) -> Result<(), String>{
            if self.balance < amount{
                return Err("You broke af boy".to_string())
            }
            else{
                self.balance -= amount;
            }
            Ok(())
        }

        pub fn balance(&self) -> u64{
            return self.balance
        }

        pub fn owner(&self) -> &str{
            return &self.owner
        }
    }
}

fn main() {
    let mut alice = BankAccount::Account::new("Alice".to_string());

    alice.deposit(100);
    BankAccount::Account::deposit(&mut alice, 150);

    println!("{:?}", alice.withdraw(500));
    println!("{:?}", alice.withdraw(50));

    println!("Owner: {:?}\nBalance: {:?}", alice.owner(), alice.balance());
}

// //Task 2: Command Handler
// // fn char_match(char: String) -> String
// fn match_char(char: char){
//     match char {
//         'q' => println!("quit"),
//         'a' | 's' | 'w' | 'd' => println!("move"),
//         '0'..='9'  => println!("digit"),
//         key if key.is_lowercase() => println!("lowercase"),
//         _ => println!("_other"),
//     };
// }
// fn main() {
//     let array = ['q', 'a', '7', 'x', '%', '9', 'A', 'd'];
//     for i in array{
//         match_char(i);
//     }
// }

// //Task 3: Destructing & @ Bindings
// struct Coords {
//     x: i32,
//     y: i32
// }

// fn check_quad(coor: (i32, i32)) {
//     match coor {
//         x if coor.0 > 0 && coor.1 > 0 => println!("I"),
//         x if coor.0 < 0 && coor.1 > 0 => println!("II"),
//         x if coor.0 < 0 && coor.1 < 0 => println!("III"),
//         x if coor.0 > 0 && coor.1 < 0 => println!("IIII"),
//         (0, _) | (0, 0)  | (_, 0) | (_, _) => println!("Axis"), 
//     };
// }

// fn main() {
//     let mut coor = vec![Coords{x: 0, y:0},
//     Coords{x:1, y:0}, Coords{x:0, y:2},
//     Coords{x:1, y:3}, Coords{x:-1, y:2},
//     Coords{x:-1, y:-2}, Coords{x:1, y:-2},
//     Coords{x:-10, y:-25},];
//     for i in coor{check_quad((i.x, i.y))}
// }

// //Task 4: let else, while let, if let
// pub fn first_hex_digit(maybe: Option<String>) -> Result<u32, String>{
//     let Some(s) = maybe else {return Err("none".to_string());};
//     let Some(first) = s.chars().next() else {return Err("empty".to_string());};
//     let Some(digit) = first.to_digit(16) else {return Err("not-hex".to_string());};

//     Ok(digit)
// }

// pub fn pop_all(s: &mut String) -> Vec<char> {
//     let mut removed: Vec<char> = vec![];
//     while let Some(c) = s.pop() {
//         removed.push(c);
//     }
//     return removed
// }

// pub fn print_parse_u8(s: &str){
//     if let Some(n) = s.parse::<u8>().ok(){
//         println!("n = {}", n);
//     }
// }

// fn main() {
//     //first hex digit function
//     println!("{:?}", first_hex_digit(Some("BEEF".to_string())));
//     println!("{:?}", first_hex_digit(Some("".to_string())));
//     println!("{:?}", first_hex_digit(None));
//     println!("{:?}", first_hex_digit(Some("P".to_string())));

//     //pop all function
//     let mut string = "abc123".to_string();
//     println!("Removed: {:?}\nOriginal: {}", pop_all(&mut string), string);

//     //parse u8 function
//     print_parse_u8("42");
//     print_parse_u8("x");
// }