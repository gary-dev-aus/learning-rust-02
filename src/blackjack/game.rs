use crate::{
    blackjack::game_loop::blackjack_loop,
    deck::{
        card::{display_card_vector, Card},
        draw::draw_hand,
        load_deck::load_deck,
        shuffle::shuffle_deck,
    },
};

pub struct Player {
    pub hand: Vec<Card>,
    pub name: String,
}

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
