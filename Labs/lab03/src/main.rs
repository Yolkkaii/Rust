fn main(){
   // //--Lab 3.1--//
   // //Part 1
   // let mut year = 2025;
   // let mut month = 7;
   // month = 12;

   // println!("{}, {}", year, month);

   // year = 200;  

   // //Part 2
   // println!("");
   // let mut price = 99.99;
   // price = 100.0;//this is expected to be a floating point number not an integer

   // let mut discount_applied: bool;
   // discount_applied = price < 100.0;
   // println!("{}", discount_applied);

   // //Part 3
   // println!("");
   // let mut available = true;
   // let mut in_stock = false;
   // let mut rating = 4.5;

   // let mut is_good = available && rating > 4.0 && in_stock;
   // println!("{}", is_good);

   // in_stock = true;
   // println!("{}", is_good);

   // println!("{} {}", !available, available);
   // println!("{} {}", in_stock, rating < 3.0);
   // println!("{}", rating > 4.0);

   // //Part 4
   // println!("");
   // let mut score = 80;
   // score = score + 10;

   // let score = score > 85;//turned into bool
   // let score = if score{"Passed"}else{"Failed"};;//if true then pass, else failed

   // println!("{}",score);

   // //Part 5
   // println!("");
   // let a = 10;
   // {let b = a + 5;(b)}
   // println!("{}", b);

   //Part 6


   //--Lab 3.2--//
   let base_price = 150.0;

   let is_student = true;
   let is_early_bird = false;
   let has_coupon = true;

   let mut discount = 0.0;
   if is_student{discount += 10.0};
   if is_early_bird{discount += 20.0};
   if has_coupon{discount += 5.0};

   let final_price = base_price - (base_price * discount / 100.0);

   let mut free_entry = false;

   if final_price < 50.0{
       free_entry = true;
   }

   println!("Base ticket price: {}\nStudent discount applied: {}\nEarly bird discount applied: {}\nCoupon used: {}\nFinal ticket price: {}\nFree entry: {}", base_price, is_student, is_early_bird, has_coupon, final_price, free_entry);
}