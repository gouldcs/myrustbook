/**
 * Rust book chapter 3: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
 */
fn main() {
    println!("Rust book chapter 3.");
    /* 3.1: Variables and Mutability */
    mutability();
    constants();
    shadowing();

    /* 3.2 Data Types */
    scalar_types();
}

/**
 * Variables in Rust are immutable by default.
 */
fn mutability() {
    let y = 10; // immutable.
    println!("y is {y}");
    // y = 11; this line will not compile.

    let mut x = 5; // use `mut` keyword to allow mutation.
    println!("x is {x}");
    x = 6;
    println!("x is updated to {x}");
}

/**
 * Constants are not mutable and require type specification upon
 * initialization. See reference on constant evaluation for what
 * can be assigned to a constant: https://doc.rust-lang.org/reference/const_eval.html
 */
fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}

/**
 * You can declare a new variable with the same name as a previous variable.
 * The new variable will "overshadow" the old one (within the same scope) and
 * is thus called "shadowing."
 */
fn shadowing() {
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is {y}");

    /*
    Shadowing allows us to reuse variable names without mutation.
    The variable gets redeclared; we are not mutating the original.
    Shadowing allows us to also change the type of the variable, as
    seen below changing the type from `&str` to `usize`.
     */
    let spaces = "     ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces.")

    /* Mutable variables will not allow you to change the type:
        let mut spaces = "     ";
        spaces = spaces.len(); // Results in error: expected `&str`, found `usize`
     */
}

#[allow(dead_code)]
/**
 * Represents a single value. There are four primary scalar types:
 * integers, floating-point numbers, Booleans,and characters.
 */
fn scalar_types() {
    // Integer types
    let _sint8: i8 = 127;
    let _uint8: u8 = 255;
    let _sint16: i16 = 32_767; // can use underscores for better readability.
    let _uint16: u16 = 65_535;
    let _sint32: i32 = 2_147_483_647; // Rust `int` defaults to `i32`.
    let _uint32: u32 = 4294967295;
    let _sint64: i64 = 9223372036854775807;
    let _uint64: u64 = 18446744073709551615;
    let _sint128: i128 = 170141183460469231731687303715884105727;
    let _uint128: u128 = 340282366920938463463374607431768211455;
    // Depends on system architecture. 64 bits if on 64-bit architecture, 32 for 32-bit, etc.
    let _sarch: isize = 0;
    let _uarch: usize = 0;
    // other number literals
    let _hex: u8 = 0xff;
    let _octal: u8 = 0o77;
    let _binary: u8 = 0b1111_0000;
    let _byte: u8 = b'A'; // u8 only!
}
