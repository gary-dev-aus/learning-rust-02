use crate::deck::card::Card;

pub fn check_value(hand: &Vec<Card>) -> u32 {
    let mut value: u32 = 0;
    let mut aces: u32 = 0;

    for card in hand {
        match card.rank.as_str() {
            "Ace" => aces += 1,
            "Jack" => value += 10,
            "Queen" => value += 10,
            "King" => value += 10,
            _ => value += card.rank.parse::<u32>().unwrap(),
        }
    }

    for _ in 0..aces {
        if value + 11 > 21 {
            value += 1;
        } else {
            value += 11;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::card::Suit;

    #[test]
    fn test_check_value() {
        let hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: "Ace".to_string(),
                id: "acehearts".to_string(),
            },
            Card {
                suit: Suit::Spades,
                rank: 2.to_string(),
                id: "2spades".to_string(),
            },
        ];

        assert_eq!(check_value(&hand), 13);
    }
}
