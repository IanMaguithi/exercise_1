pub fn fibonacci() {
    // user input: choice of fibonacci
    println!("Choose fibonacci:");
    println!("1. Recursive");
    println!("2. Iterative");
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number!");

    // user input: n
    println!("Enter n:");
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");

    // fibonacci
    if choice == 1 {
        println!("Fibonacci({}) = {}", n, fibonacci_recursive(n));
    } else if choice == 2 {
        println!("Fibonacci({}) = {}", n, fibonacci_iterative(n));
    } else {
        println!("Invalid choice!");
    }
}

//? Fibonacci recursive is inefficient for large values of n
fn fibonacci_recursive(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
    }
}

//? Fibonacci iterative is more efficient for large values of n
fn fibonacci_iterative(n: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    let mut c: i32;
    for _i in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    return a;
}
