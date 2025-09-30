fn main() {
    hello_world();
    variables();
    if_statement();
    if_else_statement();
    inline_formatting();
    num_var();
}

// Rust Standard function format
fn hello_world() {
    println!("Hello, World!");
}

// variable declarations
fn variables() {
    // Rust does allow variable declaration without immediate assignment,
    // but only if the variable is used later like in line 20 & 25.

    let x: i32;             // Declaration without assignment
    x = 5;                  // Assignment
    let x = x + 1;     // Shadowing
    let x = x * 2;

    let mut y: i32;         // Declaration without assignment, but mutable
    y = 5;                  // Assignment
    y += 1;                 // 'mut' in line 24 allows us to change 'y' here

    println!("The end value of x and y : {}, {}", x, y);
}

// Rust if statement
fn if_statement() {
    let age = 16;

    if age < 18 {
        println!("Not for persons under 18 years");
    }
}

// Rust if-else statement
fn if_else_statement() {
    let age = 22;

    if age < 21 {
        println!("Not for persons under 21 years");
    } else {
        println!("Drink responsibly.");
    }
}

// Rust inline formatting syntax
fn inline_formatting() {
    let age = 22;

    if age < 21 {
        println!("You are {} years old. Access denied!", age);
    } else {
        println!("You are {} years old. Access granted!", age);
    }
}

// variable shadowing
fn num_var() {
    let number = "T-H-R-E-E";
    println!("Spell a number: {}", number);

    let number: i32 = 3;
    println!("Number plus two is: {}", number + 2);
}