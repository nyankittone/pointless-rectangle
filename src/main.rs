// Practicing structs in this program

use std::env::args;
use std::io::stdin;
use std::iter::Iterator;
use std::process::exit;

#[derive(Debug)]
struct Rectangle {
    length: u32,
    height: u32,
}

impl Rectangle {
    fn new(length: u32, height: u32) -> Self {
        Self {
            length,
            height,
        }
    }

    fn new_square(size: u32) -> Self {
        Self {
            length: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.length * self.height
    }

    fn can_hold(&self, other_thing: &Rectangle) -> bool {
        self.length >= other_thing.length && self.height >= other_thing.height
    }
}

fn print_help(program_name: &str) {
    println! (
        "usage: {program_name} <length> <width>\n\
         Prints the area of a rectangle given length and width parameters."
    );
}

fn try_parsing_thing(thing: &String) -> u32 {
    let returned = thing.trim().parse();
    if returned.is_err() {
        eprintln!("\"{thing}\" isn't an integer, moron.");
        exit(2);
    }

    returned.unwrap()
}

// Takes an iterator over strings, and tries to use its first two iterations to create a rectangle
// out of it. The iterator is destroyed afterwards, as this takes ownership of it, I think.
fn rectangle_from_iterator<'a, I> (mut iter: I) -> Result<Rectangle,&'static str>
where I: Iterator<Item = &'a str>, {
    // get two string slices
    let first_token = iter.next();
    let second_token = iter.next();
    if second_token.is_none() {
        return Err("not enough tokens provided!");
    }

    // convert string slices to u32's
    let first_token: Result<u32,_> = first_token.unwrap().parse();
    let second_token: Result<u32,_> = second_token.unwrap().parse();
    if first_token.is_err() || second_token.is_err() {
        return Err("Both tokens typed must be positive integers!");
    }

    // build the fuckin' rectangle
    Ok(Rectangle {
        length: first_token.unwrap(),
        height: second_token.unwrap(),
    })
}

fn run_eval_loop(compared_rectangle: Rectangle) {
    println!("Source rectangle has length {} and height {}.",compared_rectangle.length,compared_rectangle.height);
    println!("That's an area of {}.",compared_rectangle.area());

    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();
        eprint!(">");

        if stdin().read_line(&mut input_buffer).is_err() {
            eprintln!("Something REALLY BAD happened when trying to read a line... I die now :(");
            exit(3);
        }

        // Possible memory leak???
        input_buffer = input_buffer.trim().to_lowercase();

        if input_buffer.eq("quit") || input_buffer.eq("q") {
            break;
        }

        let new_rectangle = rectangle_from_iterator(input_buffer.split_whitespace());
        if new_rectangle.is_err() {
            eprintln!("\x1b[1;91m{}\x1b[m",new_rectangle.err().unwrap());
            continue;
        }

        let new_rectangle = new_rectangle.unwrap();

        println! (
            "New rectangle's area is {}.\n{}",
            new_rectangle.area(),
            if compared_rectangle.can_hold(&new_rectangle) {
                "New rectangle \x1b[1;32mcan\x1b[m fit within old one! :3"
            } else {
                "New rectangle \x1b[1;31mcannot\x1b[m fit within old one! :("
            }
        );
    }

    println!("Goodbye \x1b[91m<3\x1b[m");
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        print_help(args[0].as_str());
        exit(1);
    }

    let kid_named_rectangle = Rectangle::new(try_parsing_thing(&args[1]),try_parsing_thing(&args[2]));

    drop(args);
    run_eval_loop(kid_named_rectangle);
}

