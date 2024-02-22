fn main() {
    // The number of the term in fibonacci sequence
    let term: i32 = 10;
    for i in 1..term {
        println!("{} : {}", i, fibonacci(i));
    }
    // Print out the number
    println!("The number at {} position in fibonacci series is {}", term, fibonacci(term));
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 { 0 }
    else if n == 2 { 1 }
    else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}
