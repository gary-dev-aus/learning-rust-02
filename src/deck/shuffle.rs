use crate::deck::card::Card;
use rand::Rng;

pub fn shuffle_deck(deck: &mut Vec<Card>) -> Vec<Card> {
    println!("Shuffling deck...");

    let mut shuffled_deck: Vec<Card> = Vec::new();

    while deck.len() > 0 {
        let random_index = rand::thread_rng().gen_range(0..deck.len());

        shuffled_deck.push(deck.remove(random_index));
    }

    shuffled_deck
}
