use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: String,
    pub id: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {:?}", self.rank, self.suit)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub fn display_card_vector(hand: &Vec<Card>) -> String {
    let mut string_array: Vec<String> = vec![];

    for card in hand {
        string_array.push(format!("{}", card));
    }

    let string = string_array.join(", ");

    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::card::Suit;

    #[test]
    fn test_display_hand() {
        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: 1.to_string(),
                id: "1clubs".to_string(),
            },
            Card {
                suit: Suit::Spades,
                rank: 2.to_string(),
                id: "2spades".to_string(),
            },
        ];

        assert_eq!(display_card_vector(&hand), "1 of Clubs, 2 of Spades");
    }
}
