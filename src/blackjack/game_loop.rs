use crate::{
    blackjack::{check_value::check_value, game::load_blackjack},
    deck::{
        card::{display_card_vector, Card},
        draw::draw_card,
    },
};

pub fn blackjack_loop(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    let mut state = String::new();

    while state == "" {
        println!("Hit or Stand? (h/s)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "h" | "" => {
                println!("Hit!");
                hand.push(draw_card(deck));
                println!("Hand: {}", display_card_vector(&hand));
            }
            "s" => {
                println!("Stand!");
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }

        let value = check_value(&hand);
        match value {
            21 => {
                println!("Blackjack!");
                state = "blackjack".to_string();
            }
            22..=31 => {
                println!("Bust!");
                state = "bust".to_string();
            }
            _ => {}
        }
    }

    if state == "blackjack".to_string() || state == "bust".to_string() {
        println!("Game Over!");
    }

    println!("Would you like to play again? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "y" | "" => {
            println!("Restarting game...");
            load_blackjack();
        }
        "n" => {
            println!("Goodbye!");
        }
        _ => {
            println!("Invalid input!");
        }
    }
}
