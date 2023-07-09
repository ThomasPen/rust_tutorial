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

fn main() {
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}
*/

/* Loops and arrays
fn main() {
    let arr_1 = [1,2,3,4,5,6,7,8,9];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    // Ways to loop (first one is weird on purpose)
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}
*/

/* Tuple
fn main() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}
*/

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string";
    // heap allocated string
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6]; // 6 not included !
    println!("String length : {}", st6.len());
    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");    
    // st6 will dissapear, existing in st6_7.
    let st6_7 = st6 + & st7;
    for char in st6_7.bytes() {
        println!("{}", char);
    }
}