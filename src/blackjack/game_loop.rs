use super::{
    check_value::check_value,
    game::{load_blackjack, Game, GameState, PlayerState, PlayerType},
};
use crate::deck::{card::display_card_vector, draw::draw_card};
use std::io;

pub fn blackjack_loop(game: &mut Game) {
    game.game_state = GameState::Playing;

    while game.game_state == GameState::Playing {
        println!(
            "House's hand: {} along with {} unrevealed card{}",
            &game.players[0].hand[0],
            &game.players[0].hand.len() - 1,
            if &game.players[0].hand.len() - 1 > 1 {
                "s"
            } else {
                ""
            }
        );
        for player in game.players.iter_mut() {
            if player.player_state == PlayerState::Playing {
                if player.player_type == PlayerType::Human {
                    println!("Your hand: {}", display_card_vector(&player.hand));
                    println!("Would you like to hit or stand? (H/s)");

                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    match input.trim() {
                        "h" | "" => {
                            player.hand.push(draw_card(&mut game.deck));

                            println!("You drew a {}", player.hand[player.hand.len() - 1]);
                        }
                        "s" => {
                            player.player_state = PlayerState::Stand;
                        }
                        _ => {
                            println!("Please enter 'h' or 's'");
                        }
                    }
                } else {
                    if check_value(&player.hand) < 17 {
                        player.hand.push(draw_card(&mut game.deck));
                    } else {
                        player.player_state = PlayerState::Stand;
                    }
                }

                let value = check_value(&player.hand);
                match value {
                    21 => {
                        println!("Blackjack!");
                        player.player_state = PlayerState::Stand;
                    }
                    22..=31 => {
                        println!("Bust!");
                        println!("You lost because your hand's value was {value}");
                        player.player_state = PlayerState::Bust;
                    }
                    _ => {}
                }

                // Check if player is bust
                if player.player_state == PlayerState::Bust {
                    match player.player_type {
                        super::game::PlayerType::Human => {
                            game.game_state = GameState::HouseWins;
                        }
                        super::game::PlayerType::Computer => {
                            game.game_state = GameState::PlayerWins;
                        }
                    }
                }
            }
        }

        // Check if all players are standing
        let mut stand_count = 0;
        for player in game.players.iter() {
            if player.player_state == PlayerState::Stand {
                stand_count += 1;
            }
        }

        if stand_count == game.players.len() {
            let player_score = check_value(&game.players[1].hand);
            let house_score = check_value(&game.players[0].hand);

            if player_score == house_score {
                game.game_state = GameState::Draw;
            } else if player_score > house_score {
                game.game_state = GameState::PlayerWins;
            } else {
                game.game_state = GameState::HouseWins;
            }
        }
    }

    match game.game_state {
        GameState::PlayerWins => {
            println!("You win!");
        }
        GameState::HouseWins => {
            println!("You lose!");
        }
        GameState::Draw => {
            println!("Draw!");
        }
        _ => {}
    }

    println!("Would you like to play again? (Y/n)");
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
