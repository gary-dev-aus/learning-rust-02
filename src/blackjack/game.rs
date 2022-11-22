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
    pub player_type: PlayerType,
    pub player_state: PlayerState,
}

#[derive(PartialEq)]
pub enum PlayerType {
    Human,
    Computer,
}

#[derive(PartialEq)]
pub enum PlayerState {
    Playing,
    Stand,
    Bust,
}

pub struct Game {
    pub deck: Vec<Card>,
    pub players: Vec<Player>,
    pub game_state: GameState,
}

#[derive(PartialEq)]
pub enum GameState {
    Setup,
    Playing,
    PlayerWins,
    HouseWins,
    Draw,
}

// unnecessary until implementing multiple players
// pub struct Blackjack_Config {
//     pub number_of_players: u8,
// }

pub fn load_blackjack() {
    println!("Loading Blackjack...");

    let mut deck = load_deck();
    deck = shuffle_deck(&mut deck);

    let house = Player {
        hand: draw_hand(&mut deck, 2, vec![]),
        name: "House".to_string(),
        player_type: PlayerType::Computer,
        player_state: PlayerState::Playing,
    };

    let player = Player {
        hand: draw_hand(&mut deck, 2, vec![]),
        name: "Player".to_string(),
        player_type: PlayerType::Human,
        player_state: PlayerState::Playing,
    };

    let mut game = Game {
        deck,
        players: vec![house, player],
        game_state: GameState::Setup,
    };

    blackjack_loop(&mut game);
}
