fn main() {
    //let mut number = 5;
    let number = 3;
    println!("{}", number);
    println!("Hello, world!");
    let result = fibonacci(5);
    println!("{}", result);
    let sum = sum_to(5);
    println!("{}", sum);
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