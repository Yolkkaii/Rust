fn main() {
    let arr = [5, 10, 15, 20, 25];
    let slice = &arr[1..4];
    let tuple = ("Rust", 2024, true);

    for x in slice {
        println!("{}", x);
    };

    println!("{} {} {}", tuple.0, tuple.1, tuple.2);

    let mut _count = 1;
    let mut _sum = 0;

    while _count <= 10 {
        _sum += _count;
        _count += 1;
    }

    println!("{}", _sum)
}
