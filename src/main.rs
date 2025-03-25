use std::io;
use std::io::Write;

use rules::show_rules;


mod card_operations;
mod ai;
mod rules;
include!("card_operations.rs");

// Developing a black-jack game

fn main() {
    print!("Are you familiar with the rules of BlackJack? (y/n): ");
    io::stdout().flush().unwrap();
    let mut familiar = String::new();
    io::stdin().read_line(&mut familiar).expect("Failed to read line");
    if familiar.trim().to_lowercase() != "y" {
        show_rules();
    }

    loop {
        let mut cards = vec!["A","K","Q","J","10","9","8","7","6","5","4","3","2"];
        let mut player_cards: Vec<&str> = vec![];
        let mut dealer_cards: Vec<&str> = vec![];

        shuffle_cards(&mut cards);
        
        // Initial deal
        player_cards.push(cards.pop().unwrap());
        dealer_cards.push(cards.pop().unwrap());
        player_cards.push(cards.pop().unwrap());
        dealer_cards.push(cards.pop().unwrap());

        println!("\nYour cards: ");
        show_cards(&player_cards);
        println!("Your hand value: {}", ai::calculate_hand_value(&player_cards));
        println!("Dealer's up card: {}", dealer_cards[0]);

        // Player's turn
        loop {
            let player_value = ai::calculate_hand_value(&player_cards);
            if player_value > 21 {
                println!("Bust! You lose!");
                break;
            }

            print!("Hit or Stand? (h/s): ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");

            match choice.trim().to_lowercase().as_str() {
                "h" => {
                    player_cards.push(cards.pop().unwrap());
                    println!("Your cards: ");
                    show_cards(&player_cards);
                    println!("Your hand value: {}", ai::calculate_hand_value(&player_cards));
                },
                "s" => break,
                _ => continue,
            }
        }

        let player_value = ai::calculate_hand_value(&player_cards);
        if player_value <= 21 {
            // Dealer's turn
            println!("\nDealer's cards: ");
            show_cards(&dealer_cards);

            while ai::dealer_should_hit(&dealer_cards) {
                dealer_cards.push(cards.pop().unwrap());
                println!("Dealer hits:");
                show_cards(&dealer_cards);
            }

            let dealer_value = ai::calculate_hand_value(&dealer_cards);

            // Determine winner
            if dealer_value > 21 {
                println!("Dealer busts! You win!");
            } else if dealer_value > player_value {
                println!("Dealer wins!");
            } else if dealer_value < player_value {
                println!("You win!");
            } else {
                println!("Push!");
            }
        }

        print!("\nPlay again? (y/n): ");
        io::stdout().flush().unwrap();
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }
}

