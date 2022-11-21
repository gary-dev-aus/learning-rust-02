use crate::deck::card::Card;

// discard card(hand, index of card to discard) returns discarded card
pub fn discard_card(hand: &mut Vec<Card>, index: usize) -> Card {
    hand.remove(index)
}

// return discarded card to discard pile
pub fn discard_to_pile(hand: &mut Vec<Card>, index: usize, discard_pile: &mut Vec<Card>) {
    discard_pile.push(discard_card(hand, index));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::card::Suit;

    #[test]
    fn test_discard_card() {
        let mut hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: 1.to_string(),
                id: "1hearts".to_string(),
            },
            Card {
                suit: Suit::Spades,
                rank: 2.to_string(),
                id: "2spades".to_string(),
            },
        ];

        let discarded_card = discard_card(&mut hand, 1);

        assert_eq!(
            discarded_card,
            Card {
                suit: Suit::Spades,
                rank: 2.to_string(),
                id: "2spades".to_string(),
            }
        );
    }

    #[test]
    fn test_discard_to_pile() {
        let mut hand = vec![
            Card {
                suit: Suit::Clubs,
                rank: 1.to_string(),
                id: "1hearts".to_string(),
            },
            Card {
                suit: Suit::Spades,
                rank: 2.to_string(),
                id: "2spades".to_string(),
            },
        ];

        let mut discard_pile: Vec<Card> = vec![];
        discard_to_pile(&mut hand, 1, &mut discard_pile);

        assert_eq!(
            discard_pile,
            vec![Card {
                suit: Suit::Spades,
                rank: 2.to_string(),
                id: "2spades".to_string(),
            }]
        );
    }
}
