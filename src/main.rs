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

fn _main_slices() {
    let my_string = "okok fo";
    let word = first_world_with_slice(&my_string);
    println!("Ok {}", word);

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn _main_struct() {
    let mut user1 = User {
        email: String::from("someone@email.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@email.com");
    let user2 = build_user(String::from("thenew@email.com"), String::from("the-username"));

    let _user3 = User{
        email: String::from("anew@email.com"),
        ..user2
    };
    //println!("Nope, it's moved {}", user2.username)

    let _origin = Point(0, 0, 0);
    let _black = Color(0, 0, 0);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main_rectangle() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50
    };
    println!(
        "The area of the retangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the retangle is {} square pixels...",
        rect1.area()
    );
    println!("rect1 is {:?}", rect1);
    dbg!(&rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("The square {}", square.area())
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    main_rectangle()
}