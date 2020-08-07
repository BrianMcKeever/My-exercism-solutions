use std::cmp::Ordering;
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let best_hands = hands
        .into_iter()
        .copied()
        .fold(Vec::new(), |mut best_hands, cards| {
            if best_hands.is_empty() {
                best_hands.push(cards);
                return best_hands;
            }
            let old = best_hand(best_hands[0]);
            let new = best_hand(cards);
            if old < new {
                best_hands.clear();
                best_hands.push(cards);
            } else if old == new {
                best_hands.push(cards);
            }

            best_hands
        });
    Some(best_hands)
}

#[derive(Debug)]
enum Hand {
    HighCard {
        cards: Vec<Card>,
    },
    Pair {
        rank: char,
        cards: Vec<Card>,
    },
    TwoPair {
        high_rank: char,
        low_rank: char,
        cards: Vec<Card>,
    },
    ThreeOfAKind {
        rank: char,
        cards: Vec<Card>,
    },
    Straight {
        rank: char,
    },
    Flush {
        cards: Vec<Card>,
    },
    FullHouse {
        three: char,
        two: char,
    },
    FourOfAKind {
        rank: char,
        extra: char,
    },
    StraightFlush {
        rank: char,
    },
}

#[derive(Copy, Clone, Debug)]
struct Card {
    rank: char,
    suit: char,
}

impl Card {
    fn new(card: &str) -> Card {
        let chars: Vec<char> = card.chars().collect();
        assert!(card.len() == 2 || card.len() == 3);
        let rank;
        let suit;
        if card.len() == 3 {
            assert!(chars[0] == '1' && chars[1] == '0');
            rank = '0';
            suit = chars[2];
        } else {
            rank = chars[0];
            suit = chars[1];
        }
        assert!(ORDER.contains(&rank));
        assert!(SUITS.contains(&suit));
        Card { rank, suit }
    }
}

static ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', '0', 'J', 'Q', 'K', 'A',
];
static SUITS: [char; 4] = ['C', 'S', 'H', 'D'];

fn best_hand<'a>(hand: &'a str) -> Hand {
    let mut cards = convert_to_cards(hand);
    let num_clubs = cards.iter().filter(|c| c.suit == 'C').count();
    let num_spades = cards.iter().filter(|c| c.suit == 'S').count();
    let num_diamonds = cards.iter().filter(|c| c.suit == 'D').count();
    let num_hearts = cards.iter().filter(|c| c.suit == 'H').count();
    let is_flush = num_clubs == 5 || num_spades == 5 || num_diamonds == 5 || num_hearts == 5;
    let is_straight = determine_straight(&cards);
    cards.sort();
    let duplicates = determine_duplicates(&cards);
    match (is_straight, is_flush, duplicates.len()) {
        (true, true, _) => {
            if cards[4].rank == 'A' && cards[0].rank == '2' {
                Hand::StraightFlush { rank: '5' }
            } else {
                Hand::StraightFlush {
                    rank: cards[4].rank,
                }
            }
        }
        (_, _, 1) if duplicates[0].1 == 4 => Hand::FourOfAKind {
            rank: duplicates[0].0,
            extra: cards
                .iter()
                .find(|x| x.rank != duplicates[0].0)
                .unwrap()
                .rank,
        },
        (_, _, 2) if duplicates[0].1 == 3 || duplicates[1].1 == 3 => {
            if duplicates[0].1 == 3 {
                Hand::FullHouse {
                    three: duplicates[0].0,
                    two: duplicates[1].0,
                }
            } else {
                Hand::FullHouse {
                    three: duplicates[1].0,
                    two: duplicates[0].0,
                }
            }
        }
        (false, true, _) => Hand::Flush { cards },
        (true, false, _) => {
            if cards[4].rank == 'A' && cards[0].rank == '2' {
                Hand::Straight { rank: '5' }
            } else {
                Hand::Straight {
                    rank: cards[4].rank,
                }
            }
        }
        (_, _, 1) if duplicates[0].1 == 3 => Hand::ThreeOfAKind {
            rank: duplicates[0].0,
            cards,
        },
        (_, _, 2) => {
            if duplicates[0].0 < duplicates[1].0 {
                Hand::TwoPair {
                    low_rank: duplicates[0].0,
                    high_rank: duplicates[1].0,
                    cards,
                }
            } else {
                Hand::TwoPair {
                    low_rank: duplicates[1].0,
                    high_rank: duplicates[0].0,
                    cards,
                }
            }
        }
        (_, _, 1) => Hand::Pair {
            rank: duplicates[0].0,
            cards,
        },
        _ => Hand::HighCard { cards },
    }
}

fn determine_duplicates(cards: &Vec<Card>) -> Vec<(char, u8)> {
    let mut result = Vec::new();
    let mut singles = cards.clone();
    singles.sort();
    singles.dedup();
    for single in singles.iter() {
        let mut count = 0;
        for card in cards.iter() {
            if card.rank == single.rank {
                count += 1;
            }
        }
        if count > 1 {
            result.push((single.rank, count));
        }
    }
    result
}
fn determine_straight(cards: &Vec<Card>) -> bool {
    let mut cards: Vec<Card> = cards.clone();
    cards.sort();
    if cards[0].rank == '2' && cards[4].rank == 'A' {
        for i in 1..4 {
            if cards[i].rank != ORDER[i] {
                return false;
            }
        }
        return true;
    }
    for i in 0..ORDER.len() {
        if cards[0].rank == ORDER[i] {
            for j in 0..cards.len() {
                if cards[j].rank != ORDER[j + i] {
                    return false;
                }
            }
            break;
        }
    }
    true
}

fn convert_to_cards(hand: &str) -> Vec<Card> {
    hand.split(' ').map(|s| Card::new(s)).collect()
}

fn hand_order(hand: &Hand) -> u8 {
    match hand {
        Hand::HighCard { .. } => 1,
        Hand::Pair { .. } => 2,
        Hand::TwoPair { .. } => 3,
        Hand::ThreeOfAKind { .. } => 4,
        Hand::Straight { .. } => 5,
        Hand::Flush { .. } => 6,
        Hand::FullHouse { .. } => 7,
        Hand::FourOfAKind { .. } => 8,
        Hand::StraightFlush { .. } => 9,
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (a, b) if hand_order(a) != hand_order(b) => hand_order(a).cmp(&hand_order(b)),
            (Hand::HighCard { cards }, Hand::HighCard { cards: cards2 })
            | (Hand::Flush { cards }, Hand::Flush { cards: cards2 }) => {
                for i in (0..cards.len()).rev() {
                    if cards[i] < cards2[i] {
                        return Ordering::Less;
                    } else if cards[i] > cards2[i] {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }
            (
                Hand::Pair { cards, rank },
                Hand::Pair {
                    rank: rank2,
                    cards: cards2,
                },
            ) => match rank.cmp(rank2) {
                Ordering::Equal => Hand::HighCard {
                    cards: cards.clone(),
                }
                .cmp(&Hand::HighCard {
                    cards: cards2.clone(),
                }),
                o => o,
            },
            (
                Hand::TwoPair {
                    cards,
                    high_rank,
                    low_rank,
                },
                Hand::TwoPair {
                    high_rank: high_rank2,
                    low_rank: low_rank2,
                    cards: cards2,
                },
            ) => match (high_rank.cmp(high_rank2), low_rank.cmp(low_rank2)) {
                (Ordering::Equal, Ordering::Equal) => Hand::HighCard {
                    cards: cards.clone(),
                }
                .cmp(&Hand::HighCard {
                    cards: cards2.clone(),
                }),
                (Ordering::Equal, o) => o,
                (o, _) => o,
            },
            (
                Hand::ThreeOfAKind { rank, cards },
                Hand::ThreeOfAKind {
                    rank: rank2,
                    cards: cards2,
                },
            ) => match rank.cmp(rank2) {
                Ordering::Equal => Hand::HighCard {
                    cards: cards.clone(),
                }
                .cmp(&Hand::HighCard {
                    cards: cards2.clone(),
                }),
                o => o,
            },
            (Hand::Straight { rank }, Hand::Straight { rank: rank2 }) => rank.cmp(rank2),
            (
                Hand::FullHouse { three, two },
                Hand::FullHouse {
                    three: three2,
                    two: two2,
                },
            ) => match (three.cmp(three2), two.cmp(two2)) {
                (Ordering::Equal, o) => o,
                (o, _) => o,
            },
            (
                Hand::FourOfAKind { rank, extra },
                Hand::FourOfAKind {
                    rank: rank2,
                    extra: extra2,
                },
            ) => match rank.cmp(rank2) {
                Ordering::Equal => extra.cmp(extra2),
                _ => rank.cmp(rank2),
            },
            (Hand::StraightFlush { rank }, Hand::StraightFlush { rank: rank2 }) => rank.cmp(rank2),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Hand {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..ORDER.len() {
            let c = ORDER[i];
            match (c == self.rank, c == other.rank) {
                (true, true) => return Ordering::Equal,
                (true, false) => return Ordering::Less,
                (false, true) => return Ordering::Greater,
                (false, false) => {}
            }
        }
        unreachable!();
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Card {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_straight() {
        assert!(determine_straight(&convert_to_cards("4H 5H 6H 7H 8H")));
        assert!(!determine_straight(&convert_to_cards("3H 5H 6H 7H 8H")));
    }

    #[test]
    fn test_convert_to_cards() {
        assert_eq!(
            convert_to_cards("4H 5H 6H 7H 8H"),
            [
                Card {
                    rank: '4',
                    suit: 'H'
                },
                Card {
                    rank: '5',
                    suit: 'H'
                },
                Card {
                    rank: '6',
                    suit: 'H'
                },
                Card {
                    rank: '7',
                    suit: 'H'
                },
                Card {
                    rank: '8',
                    suit: 'H'
                },
            ]
        );
    }

    #[test]
    fn test_hand_cmp() {
        let straight_flush = Hand::StraightFlush { rank: 'A' };
        let four = Hand::FourOfAKind {
            rank: 'A',
            extra: '2',
        };
        assert_eq!(Ordering::Less, four.cmp(&straight_flush));
        assert_eq!(Ordering::Greater, straight_flush.cmp(&four));
    }

    #[test]
    fn test_card_cmp() {
        let ace = Card::new("AH");
        let ten = Card::new("10D");
        let two = Card::new("2S");
        assert_eq!(Ordering::Greater, ace.cmp(&ten));
        assert_eq!(Ordering::Greater, ace.cmp(&two));
        assert_eq!(Ordering::Less, two.cmp(&ace));
        assert_eq!(Ordering::Equal, two.cmp(&two));
    }

    #[test]
    fn test_determine_duplicates() {
        let ace = Card::new("AH");
        let ten = Card::new("10D");
        let two = Card::new("2S");
        assert_eq!(
            Vec::<(char, u8)>::new(),
            determine_duplicates(&vec![ace, two])
        );
        assert_eq!(
            vec![('A', 2)],
            determine_duplicates(&vec![ace, ace.clone()])
        );
        assert_eq!(
            vec![('0', 2), ('A', 2)],
            determine_duplicates(&vec![ace, ace.clone(), ten, ten.clone()])
        );
        assert_eq!(
            vec![('0', 2), ('A', 3)],
            determine_duplicates(&vec![
                ace,
                Card::new("AD"),
                Card::new("AS"),
                ten,
                Card::new("10H")
            ])
        );
        assert_eq!(
            vec![('A', 4)],
            determine_duplicates(&vec![ace, ace.clone(), ace.clone(), ace.clone()])
        );
    }
}
