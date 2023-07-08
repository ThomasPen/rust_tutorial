#![allow(unused)]

use std::io;
use rand::Rng;

// to work with files
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

/* MUTABLE - INPUT - EXPECT
fn main() {
    println!("What is your name?");
    // [...] mut [...] => mutable variable; by default, they're immutable.
    let mut name = String::new();
    let greeting = "Nice to meet you";

    // to receive user input
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);
    
}
*/

/* CONSTANT
fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    // shadowing: same name, different value. perfectly legal
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a numer");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
*/

/* Data Types
fn main() {
    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16? i32, i64, i128, isize
    println!("Max u32 : {}", u32::MAX);
    println!("Max usize : {}", usize::MAX);    
    println!("Max isize : {}", isize::MAX);
    let is_true = true;
    let my_grade = 'A';
}
*/

/* MATH
fn main() {
    let num_1: f32 = 1.1111111111111111;
    println!("f32 : {}", num_1 + 0.1111111111111111);
    let num_2: f64 = 1.1111111111111111;
    println!("f64 : {}", num_2 + 0.1111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    let mut num_5 = 1;
    num_5 += 1;
    println!("num_5: {}", num_5);

}
*/

/* RANDOM
fn main() {
    // end number of range is not included
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}
*/

/* If statements and ternary operator
fn main() {
    let age = 8;
    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");       
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    // simulate a ternary operator
    let mut my_age = 47;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can Vote : {}", can_vote);
}
*/

/* Match case
fn main() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println("Important Birthday"),
        _ => println!("Not an important Birthday"),
    };
}
*/

fn main() {
    
}