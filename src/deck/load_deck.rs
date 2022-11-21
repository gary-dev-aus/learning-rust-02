use crate::deck::card::{Card, Suit};

pub fn load_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    let suits = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    let ranks: Vec<String> = [
        "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    for suit in suits {
        for rank in &ranks {
            let id = format!("{}{:?}", rank, suit).to_lowercase();

            deck.push(Card {
                suit,
                rank: rank.to_string(),
                id,
            });
        }
    }

    deck
}
