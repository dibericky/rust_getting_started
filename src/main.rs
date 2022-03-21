use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn _guessing_number () {
    println!("Guess the number!");

    let secret_number =  rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Pleas input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess : u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
    
        println!("You guesses: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You got it!".green());
                break;
            }
        }
    }
}

fn _first_word (s: &String) -> usize {
    /*
    write a function that takes a string and returns the first word it finds in that string.
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned
    */
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_world_with_slice (s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // == [0..i]
            return &s[..i];
        }
    }
    // == [0..s.len()]
    &s[..]
}

fn main() {
    let my_string = "okok fo";
    let word = first_world_with_slice(&my_string);
    println!("Ok {}", word);

}
