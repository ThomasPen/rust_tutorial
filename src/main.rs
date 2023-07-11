#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;

// to work with files
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

/* MUTABLE - INPUT - EXPECT
fn main() {
    println!("What is your name?");
    // [...] mut [...] => mutable variable; by default, they're immutable.
    // you can declare one immutable variable and only one mutable with the same name
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

/* String manipulation
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
    let st6_7 = st6 + &st7;
    for char in st6_7.bytes() {
        println!("{}", char);
    }
}
*/

/* Casting
fn main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

}
*/

/* Enumerators
fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}", today.is_weekend());
}
*/

/* Vectors
fn main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}
 */

/* Rundown of functions
fn say_hello() {
    println!("Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x+y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn main() {
    say_hello();   
    get_sum(5, 4);
    println!("{}", get_sum_2(5, 4));
    println!("{}", get_sum_3(5, 4));
    let (val_1, val_2) = get_2(3);
    println!("Nums: {} {}", val_1, val_2);

    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));

}
*/

/* Basics of Generics
use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}


fn main() {
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));
}
*/

/* IMPORTANT : Ownership


// Stack : Stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Heap : When putting data on the heap you request a certain amount of space.
// The OS finds space available and returns for that space called a pointer.

// RULES
    // 1. Each value has a variable that's called its owner
    // 2. There is only one owner at a time
    // 3. When the owner goes out of scope the value dissapears

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name);
}

fn main() {
    let str1 = String::from("World");
    let str2 = str1; //str1 stopped existing
    let str2_clone = str2.clone();
    // println!("Hello {}", str1); =>>>> Should have an error
    println!("Hellp {}", str2); // Should be ok

    print_str(str2);

    // let str3 = print_return_str(str2); // value has moved when it entered print_str scope, so it triggers an error

    let mut str4 = String::from("Derek");
    change_string(&mut str4); // passing a reference is not changing the scope of a variable
    println!("{}", str4);
}
*/

fn main() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clart Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }
    println!("Length : {}", heroes.len());

    // to check a specific key in a hashmap
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
    
}