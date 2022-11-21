use crate::deck::card::Card;

pub fn draw_card(deck: &mut Vec<Card>) -> Card {
    deck.pop().unwrap()
}

pub fn draw_card_to_hand(deck: &mut Vec<Card>, mut hand: Vec<Card>) -> Vec<Card> {
    hand.push(draw_card(deck));
    hand
}

pub fn draw_hand(deck: &mut Vec<Card>, hand_size: u32, mut hand: Vec<Card>) -> Vec<Card> {
    for _ in 0..hand_size {
        hand.push(draw_card(deck));
    }

    hand
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::card::Suit;

    #[test]
    fn test_draw_card() {
        let mut deck = vec![
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

        let card = draw_card(&mut deck);

        assert_eq!(card.suit, Suit::Spades);
        assert_eq!(card.rank, "2");
    }

    #[test]
    fn test_draw_card_to_hand() {
        let mut deck = vec![
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

        let hand = vec![Card {
            suit: Suit::Hearts,
            rank: 3.to_string(),
            id: "3hearts".to_string(),
        }];

        let new_hand = draw_card_to_hand(&mut deck, hand);

        assert_eq!(
            new_hand,
            vec![
                Card {
                    suit: Suit::Hearts,
                    rank: 3.to_string(),
                    id: "3hearts".to_string(),
                },
                Card {
                    suit: Suit::Spades,
                    rank: 2.to_string(),
                    id: "2spades".to_string(),
                }
            ]
        );
    }

    #[test]
    fn test_draw_hand() {
        let mut deck = vec![
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
        let hand: Vec<Card> = vec![];
        let hand = draw_hand(&mut deck, 1, hand);

        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].suit, Suit::Spades);
        assert_eq!(hand[0].rank, 2.to_string());
    }
}
