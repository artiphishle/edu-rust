/**
 * RUST-LANG
 * The very basics
 */
use std::ops::{Range, RangeInclusive};

fn main() {
    run_1_declarations();
    run_2_loops();
    run_3_assertions();
}

// Variable declaration
fn run_1_declarations() {
     // Declare two variables at once
    let (u, v);
    (u,..) = (3, 4);
    [.., v] = [1, 2];
    
    // Declare signed 32-bit integer
    let x: i32 = -10;
    
    // Declare mutable signed 32-bit integer
    let mut z: i32 = 8;
    z = z + 4;
    
    // Declare unsigned (positive number)  32-bit integer
    let _a: u32 = 5;
    
    // Declare 'pi' as a 64-bit float
    let pi: f64 = 3.141;
    
    // Declare readonly string
    let y: &'static str = "readonly";
    
    // Output variables using placeholder '{}'
    println!("pi: {}, y: '{}': u: {}, v: {}, x: {}, z: {}", pi, y, u, v, x, z);
}

// Chapter 2 keeps you in the loop!
fn do_loops() {
    // Loop integer range
    for i in -3..2 {
        println!("{}", i);
    }
    
    // Loop string range
    for c in 'a'..='z' {
        // Output character & decimal ASCII
        println!("{}: {}", c, c as u8);
    }
}

// Chapter 3 assertions!
fn do_assertions() -> &'static str {
    // Assertion of ranges
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);
}
