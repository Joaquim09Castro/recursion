use std::io;
use std::cmp::Ordering;

macro_rules! recursion_fun {
    ($repeat_el: expr) => {
        recursion_fun($repeat_el, 0);
    };
}

fn main() {
    let mut user_input: String = String::new();

    println!("Please enter a number:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the line");

    let user_input: u8 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(error_obj) => panic!("Invalid input: {:?}", error_obj),
    };

    println!("---");
    recursion_fun!(user_input);
}

fn recursion_fun(repeat_el: u8, counter: u8) {
    match repeat_el.cmp(&0) {
        Ordering::Less => {
            println!("Nothing happens");
        },
        Ordering::Greater => {
            println!("{repeat_el}");
            recursion_fun(repeat_el - 1, counter + 1);
        },
        Ordering::Equal => {
            println!("{repeat_el}");
            println!("Finished!");
        }
    }
}