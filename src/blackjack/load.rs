use crate::deck::{
    card::{display_card_vector, Card},
    draw::{draw_card, draw_hand},
    load_deck::load_deck,
    shuffle::shuffle_deck,
};

// Print out game state and deal initial hand
pub fn load_blackjack() {
    println!("Loading Blackjack...");

    let mut deck = load_deck();
    deck = shuffle_deck(&mut deck);
    let mut hand: Vec<Card> = vec![];

    println!("Dealing hand to player...");
    hand = draw_hand(&mut deck, 2, hand);

    println!("Hand: {}", display_card_vector(&hand));

    blackjack_loop(&mut deck, &mut hand)
}

pub fn blackjack_loop(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    loop {
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
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_load_blackjack() {
//         let mut deck =
//     }
// }
