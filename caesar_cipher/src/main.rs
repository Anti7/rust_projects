// I want to encrypt and decrypt
// Encrypt will take a number from 1 - 26 and shift a message that many letters
// Decrypt will do the same but shifting the opposite direction
use std::io;

fn encrypt(message: &str, shift: u8) -> String {
    shift_message(message, shift as i8)
}

fn decrypt(message: &str, shift: u8) -> String {
    shift_message(message, -(shift as i8))
}

fn shift_message(message: &str, shift: i8) -> String {
    message.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A'} else { b'a' };
            let shifted = (c as u8 - base) as i8;
            let wrapped = (shifted + shift).rem_euclid(26); // Wrap remainder around 26, so shifting backwards works
            (base + wrapped as u8) as char
        } else {
            c
        }
    })
    .collect()
}
fn main() {
    println!("Greetings Agent Rust. Do you want to (e)ncrypt or (d)ecrypt?");

    let mut input= String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().to_lowercase();
    input.clear();

    println!("Enter a number from 1 to 26:");

    io::stdin().read_line(&mut input).unwrap();
    let shift: u8 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter your code:");

    io::stdin().read_line(&mut input).unwrap();
    let message = input.trim_end();

    let result = if choice == "e" {
        encrypt(message, shift)
    } else if choice == "d" {
        decrypt(message, shift)
    } else {
        String::from("Error: Invalid choice")
    };

    println!("{}", result);

}
