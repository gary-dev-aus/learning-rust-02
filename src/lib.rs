mod blackjack;
mod deck;

use blackjack::game::load_blackjack;

pub fn run() {
    load_blackjack()
}
