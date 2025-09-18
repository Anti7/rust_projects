use std::io;
use rand::Rng;

fn main() {
    let secret: String = (0..3)
        .map(|_| rand::thread_rng().gen_range(0..10).to_string())
        .collect();

    // debug
    // println! ("{secret}");
    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .unwrap();
        let guess = guess.trim();

        if guess.len() != 3 || !guess.chars().all(|c| c.is_ascii_digit()) {
            println! ("Please enter a 3-digit number");
            continue;
        }

        let mut feedback: Vec<char> = vec!['⬛'; 3];
        let mut used_secret = vec![false; 3];
        let mut used_guess = vec![false; 3];

        for (i, (g, s)) in guess.chars().zip(secret.chars()).enumerate() {
            if g == s {
                feedback[i] = '🟩';
                used_secret[i] = true;
                used_guess[i] = true;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if used_guess[i] {
                continue;
            }
            if let Some((j, _)) = secret.chars().enumerate()
                .find(|(j, s)| *s == g && !used_secret[*j])
            {
                feedback[i] = '🟨';
                used_secret[j] = true;
            }
        }

        println!("{}", feedback.iter().collect::<String>());

        if guess == secret {
            println! ("You guessed it!");
            break;
        }
    }
}