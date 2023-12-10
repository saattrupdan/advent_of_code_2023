// --- Part Two ---
// To make things a little more interesting, the Elf introduces one additional rule.
// Now, J cards are jokers - wildcards that can act like whatever card would make the
// hand the strongest type possible.
//
// To balance this, J cards are now the weakest individual cards, weaker even than 2.
// The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
//
// J cards can pretend to be whatever card is best for the purpose of determining hand
// type; for example, QJJQ2 is now considered four of a kind. However, for the purpose
// of breaking ties between two hands of the same type, J is always treated as J, not
// the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than
// Q.
//
// Now, the above example goes very differently:
//
// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
// 32T3K is still the only one pair; it doesn't contain any jokers, so its strength
// doesn't increase.
// KK677 is now the only two pair, making it the second-weakest hand.
// T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets
// rank 4, and KTJJT gets rank 5.
// With the new joker rule, the total winnings in this example are 5905.
//
// Using the new joker rule, find the rank of every hand in your set. What are the new
// total winnings?

use std::cmp::Ordering;

fn main() {
    let data = include_str!("../../data/day7.txt");

    let hands = data
        .lines()
        .map(|x| x.split(" ").next().unwrap())
        .collect::<Vec<&str>>();

    let bids = data
        .lines()
        .map(|x| x.split(" ").last().unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(higher_rank);

    let ranks = hands
        .iter()
        .map(|x| sorted_hands.iter().rev().position(|y| x == y).unwrap() + 1)
        .collect::<Vec<usize>>();

    let answer = ranks
        .iter()
        .zip(bids.iter())
        .fold(0, |acc, (rank, bid)| acc + rank * bid);

    println!("The answer for the second task is: {}", answer);
}

fn higher_suit(suit1: char, suit2: char) -> Ordering {
    let suits: Vec<char> = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    suits
        .iter()
        .position(|x| *x == suit1)
        .unwrap()
        .cmp(&suits.iter().position(|x| *x == suit2).unwrap())
}

fn higher_rank(hand1: &&str, hand2: &&str) -> Ordering {
    let type1 = get_type(hand1);
    let type2 = get_type(hand2);

    // First check if the types are different, and if so, return whether hand1 has a
    // better type than hand2
    if type1 != type2 {
        return type1.cmp(&type2);
    }

    // Otherwise, the types are the same, so we need to compare the cards
    for char_idx in 0..hand1.len() {
        let suit_order = higher_suit(
            hand1.chars().nth(char_idx).unwrap(),
            hand2.chars().nth(char_idx).unwrap(),
        );
        if suit_order != Ordering::Equal {
            return suit_order;
        }
    }

    // If we get here, the hands are equal, so return false
    return Ordering::Equal;
}

fn get_type(hand: &str) -> u8 {
    let suits: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

    // Get unique characters in the hand, aside from the jokers
    let mut sorted_chars = hand.chars().collect::<Vec<char>>();
    sorted_chars.sort();

    let mut unique_non_joker_chars = sorted_chars.clone();
    unique_non_joker_chars.dedup();
    unique_non_joker_chars.retain(|x| *x != 'J');

    let joker_indices = hand
        .chars()
        .enumerate()
        .filter_map(|(idx, x)| if x == 'J' { Some(idx) } else { None })
        .collect::<Vec<usize>>();

    let mut best_rank = 6;
    let mut possible_hands: Vec<String> = vec![hand.to_string()];
    let mut old_possible_hands: Vec<String>;
    let mut new_hand: String;

    for joker_idx in joker_indices {
        old_possible_hands = possible_hands.clone();
        possible_hands.clear();
        for old_possible_hand in old_possible_hands {
            for non_joker_char in suits.iter() {
                new_hand = old_possible_hand.clone().to_string();
                new_hand.replace_range(joker_idx..joker_idx + 1, &non_joker_char.to_string());
                possible_hands.push(new_hand);
            }
        }
    }

    for possible_hand in possible_hands {
        sorted_chars = possible_hand.chars().collect::<Vec<char>>();
        sorted_chars.sort();

        // Check for five of a kind
        if possible_hand
            .chars()
            .all(|x| x == possible_hand.chars().nth(0).unwrap())
        {
            best_rank = best_rank.min(0);
        }
        // Check for four of a kind
        else if sorted_chars[0..4].iter().all(|x| x == &sorted_chars[0])
            || sorted_chars[1..5].iter().all(|x| x == &sorted_chars[1])
        {
            best_rank = best_rank.min(1);
        }
        // Check for full house
        else if (sorted_chars[0..3].iter().all(|x| x == &sorted_chars[0])
            && sorted_chars[3..5].iter().all(|x| x == &sorted_chars[3]))
            || (sorted_chars[0..2].iter().all(|x| x == &sorted_chars[0])
                && sorted_chars[2..5].iter().all(|x| x == &sorted_chars[2]))
        {
            best_rank = best_rank.min(2);
        }
        // Check for three of a kind
        else if sorted_chars[0..3].iter().all(|x| x == &sorted_chars[0])
            || sorted_chars[1..4].iter().all(|x| x == &sorted_chars[1])
            || sorted_chars[2..5].iter().all(|x| x == &sorted_chars[2])
        {
            best_rank = best_rank.min(3);
        }
        // Check for two pair
        else if (sorted_chars[0..2].iter().all(|x| x == &sorted_chars[0])
            && sorted_chars[2..4].iter().all(|x| x == &sorted_chars[2]))
            || (sorted_chars[0..2].iter().all(|x| x == &sorted_chars[0])
                && sorted_chars[3..5].iter().all(|x| x == &sorted_chars[3]))
            || (sorted_chars[1..3].iter().all(|x| x == &sorted_chars[1])
                && sorted_chars[3..5].iter().all(|x| x == &sorted_chars[3]))
        {
            best_rank = best_rank.min(4);
        }
        // Check for one pair
        else if sorted_chars[0..2].iter().all(|x| x == &sorted_chars[0])
            || sorted_chars[1..3].iter().all(|x| x == &sorted_chars[1])
            || sorted_chars[2..4].iter().all(|x| x == &sorted_chars[2])
            || sorted_chars[3..5].iter().all(|x| x == &sorted_chars[3])
        {
            best_rank = best_rank.min(5);
        }
    }

    return best_rank;
}
