use std::io::{self, Write};
use rand::Rng;
use chrono::NaiveDate;
use chrono::Datelike;
use std::collections::{HashSet, HashMap};

fn ordinal(n: u32) -> String {
    let suffix = match n % 10 {
        1 if n % 100 != 11 => "st",
        2 if n % 100 != 12 => "nd",
        3 if n % 100 != 13 => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}

fn main() {
    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];

    let num: usize = loop {
        println! ("How many birthdays shall I generate? (Maximum 100)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input
            .trim()
            .parse() {
             Ok(n) if (1..=100).contains(&n) => break n,
            _ => {
                println! ("Please enter a number between 1 and 100");
                continue;
            }
        }
    };

    let mut rng = rand::rng();
    let mut birthdays = Vec::new();

    for _ in 0..num {
        let day_of_year = rng.random_range(1..=365);
        let date = NaiveDate::from_yo_opt(2023, day_of_year).unwrap();
        birthdays.push((date.day(), date.month()));
    }

    let mut counts: HashMap<(u32, u32), usize> = HashMap::new();
    for &b in &birthdays {
        *counts.entry(b).or_insert(0) += 1;
    }

    print! ("Here are {} birthdays:   ", num);
    for (i, &(day, month)) in birthdays.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        let month_name = month_names[(month - 1) as usize];
        if counts[&(day, month)] > 1 {
            print! ("🎉{} {}🎉", ordinal(day), month_name);
        } else {
            print! ("{} {}", ordinal(day), month_name);
        }
    }

    println!();
    io::stdout().flush().unwrap();

    let unique: HashSet<_> = birthdays.iter().collect();
    if unique.len() != birthdays.len() {
        println! ("🎉 We have a match! 🎉");
    } else {
        println! ("No matches this time :(");
    }
}