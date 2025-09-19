use rand::seq::SliceRandom;
use rand::rng;
use std::io;

struct Card {
    rank: &'static str,
    suit: &'static str,
}

impl Card {
    fn value(&self) -> i32 {
        match self.rank {
            "A" => 11,
            "K" | "Q" | "J" => 10,
            _ => self.rank.parse::<i32>().unwrap(),
        }
    }

    fn ascii_art(&self) -> String {
        format!(
            "┌─────┐\n│{:<2}   │\n│  {}  │\n│   {:>2}│\n└─────┘",
            self.rank, self.suit, self.rank
        )
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["♠", "♥", "♦", "♣"];
        let ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",];

        let mut cards = vec![];
        for &suit in &suits {
            for &rank in &ranks {
                cards.push(Card { rank, suit });
            }
        }

        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

struct Player {
    name: String,
    hand: Vec<Card>,
    balance: i32,
}

impl Player {
    fn new (name: &str, balance: i32) -> Self {
        Player {
            name: name.to_string(),
            hand: vec![],
            balance,
        }
    }

    fn hand_value(&self) -> i32 {
        let mut total = 0;
        let mut aces = 0;
        for card in &self.hand {
            total += card.value();
            if card.rank == "A" {
                aces += 1;
            }
        }

        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        total
    }

    fn show_hand(&self) {
        let arts: Vec<_> = self.hand.iter().map(|c| c.ascii_art()).collect();
        // print out cards
        let lines: Vec<Vec<&str>> = arts.iter().map(|a| a.lines().collect()).collect();
        for row in 0..lines[0].len() {
            for card in &lines {
                print!("{}", card[row]);
            }
            println!();
        }
        println!("{} total: {}", self.name, self.hand_value());
    }

}

fn show_dealer_hidden(hand: &Vec<Card>) {
        if hand.is_empty() {
            return;
        }

        let first_art = hand[0].ascii_art();
        let first_lines: Vec<String> = first_art.lines().map(|s| s.to_string()).collect();

        let back = "┌─────┐\n│░░░░░│\n│░░░░░│\n│░░░░░│\n└─────┘";
        let back_lines: Vec<String> = back.lines().map(|s| s.to_string()).collect();

        let arts: Vec<Vec<String>> = if hand.len() > 1 {
            vec![first_lines, back_lines]
        } else {
            vec![first_lines]
        };

        for row in 0..arts[0].len() {
            for card in &arts {
                print!("{} ", card[row]);
            }
            println!();
        }

        println!("Dealer shows: {}{}", hand[0].rank, hand[0].suit);
    }


fn main() {
    let mut deck = Deck::new();
    let mut player = Player::new("You", 5000);
    let mut dealer = Player::new("Dealer", 0);

    println!("Welcome to Rust Blackjack! You start with ${}", player.balance);

    loop {
        if player.balance <= 0 {
            println!("You're flat broke! Game over.");
            break;
        }

        println!("Enter your bet:  ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let bet: i32 = input.trim().parse().unwrap_or(0);
        if bet <= 0 || bet > player.balance {
            println!("Invalid bet, try again");
            continue;
        }

        player.hand.clear();
        dealer.hand.clear();

        player.hand.push(deck.deal());
        dealer.hand.push(deck.deal());
        player.hand.push(deck.deal());
        dealer.hand.push(deck.deal());

        show_dealer_hidden(&dealer.hand);
        
        println!("\nYour hand:");
        player.show_hand();
        
        // Player turn
        loop {
            println!("Hit or stand? (h/s)");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim().to_lowercase();
            if choice == "h" {
                player.hand.push(deck.deal());
                player.show_hand();
                if player.hand_value() > 21 {
                    println!("Bust! You lose ${}", bet);
                    player.balance -= bet;
                    break;
                }
            } else if choice == "s" {
                break;
            } else {
                println!("Invalid choice.");
            }
        }

        // Dealer turn
        if player.hand_value() <= 21 {
            println!("\nDealer's hand:");
            dealer.show_hand();
            while dealer.hand_value() < 17 {
                println!("Dealer hits...");
                dealer.hand.push(deck.deal());
                dealer.show_hand();
            }

            let player_total = player.hand_value();
            let dealer_total = dealer.hand_value();

            if dealer_total > 21 || player_total > dealer_total {
                println!("You win! +${}", bet);
                player.balance += bet;
            } else if player_total < dealer_total {
                println!("Dealer wins, bad luck. -${}", bet);
                player.balance -= bet;
            } else {
                println!("Draw! Bet returned.");
            }
        }

        println!("Your balance: ${}", player.balance);
        if player.balance > 0 {
            println!("Play another round? (y/n)");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() != "y" {
                break;
            }
        } else {
            println!("You're broke! Game over.");
            break;
        }
        

        if deck.cards.len() < 15 {
            deck = Deck::new();
        }
    }
}