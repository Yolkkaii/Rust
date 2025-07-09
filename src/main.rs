fn main() {
    let arr = [5, 10, 15, 20, 25];
    let slice = &arr[1..4];
    let tuple = ("Rust", 2024, true);

    for x in slice {
        println!("{}", x);
    };

    println!("{} {} {}", tuple.0, tuple.1, tuple.2);
}
