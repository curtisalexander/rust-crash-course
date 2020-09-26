fn main() {
    for i in 1..101 {
        match (i % 5 == 0, i % 3 == 0) {
            (true, true) => println!("fizzbuzz"),
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (false, false) => {}
        }
    }
}
