fn main() {
    let mut number = 5;
    number = 3;
    println!("{}", number);
    println!("Hello, world!");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

//returns the sum of all the numbers from 1 to that number
fn sum_to(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else {
        return n + sum_to(n - 1);
    }
}