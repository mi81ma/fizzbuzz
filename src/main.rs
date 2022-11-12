fn fizzbuzz(end: i32) {
    // let mut x: i32 = 1;
    let r = 1..=end;

    for x in r {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

fn main() {
    fizzbuzz(30);
}
