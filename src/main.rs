// Program made by Xanths58
// Check out my other works at https://xanthus58.github.io/Xanthus58/
// or https://github.com/Xanthus58
// Feel fee to interact with the project here https://github.com/Xanthus58/robux_calculator

use std::io;
use colour::*;
//function that gets user input and returns a float
fn get_input() -> f64 {
    //create a variable to hold the user input
    let mut input = String::new();
    //get the user input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    //convert the input to a float
    input.trim().parse::<f64>().expect("Please type a number")
}

//function that clears the screen
fn cls() {
    print!("{esc}c", esc = 27 as char); 
}

//function that gets user input before shutting down the program
fn get_input_and_exit() {
    println!("Press enter key to exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn main() {
    cls();
    println!("Welcome to the Robux to Dollar Converter!");
    dark_grey_ln!("Based on the 400 Robux package, each Robux is worth $0.02 AUD or $0.0125 USD");
    println!("Please enter the amount of Robux you want to convert to Dollars");
    let robux = get_input();
    println!("----------------------------------------");
    let aud = robux * 0.02;
    let usd = robux * 0.0125;
    green_ln!("{} Robux is ${} AUD", robux, aud);
    green_ln!("{} Robux is ${} USD", robux, usd);
    println!("----------------------------------------");
    get_input_and_exit()
}