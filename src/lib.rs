mod blackjack;
mod deck;

use blackjack::load::load_blackjack;

pub fn run() {
    load_blackjack()
}
