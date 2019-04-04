extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{Zero, One};

use std::io;
use std::io::Write;

// Get fibonacci sequence.
fn fibonacci(x: usize) -> std::vec::Vec<BigUint> {
    let f0: BigUint = Zero::zero();
    let f1: BigUint = One::one();

    // first element only
    if x == 1 {
        return vec![f0];
    }

    // two first elements
    if x == 2 {
        return vec![f0, f1];
    }

    let mut vec = vec![f0, f1];
    for v in 2..x {
        let sum = &vec[v-2] + &vec[v-1];
        vec.push( sum );
    }

    vec
}

fn user_input(text: &str) -> String {
    print!("\n{} ", text);

    io::stdout()
        .flush()
        .expect("Failed to write to console");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)   // return Result Ok or Err
        .expect("Failed to read line");

    guess
}

fn main() {
    println!(" * * * Fibonacci sequence * * *");

    loop {
        let guess = user_input("Type number:");
        let guess: usize = match guess.trim().parse() {
            Ok(num) => {
                // filter out zero and negative inputs.
                if num <= 0 {
                    println!("Invalid option");
                    continue;
                };

                num
            },
            Err(_) => {
                println!("Invalid option");
                continue;
            }
        };

        // get fibonacci
        let vec = fibonacci(guess);

        // print
        print!("\n >> ");
        for element in &vec {
            print!("{} ", element);
        }

        // reverse print
        print!("\n\n << ");
        for element in vec.iter().rev() {
            print!("{} ", element);
        }

        let guess = user_input("\nSelect one of the numbers:");
        match guess.trim().parse() {
            Ok(num) => if vec.contains(&num) {
                println!("\nWell Done!");
                return;
            }
            _ => (),
        };

        // no match
        println!("\nError!");
        return;
    }
}
