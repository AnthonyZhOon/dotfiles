use crate::custom_error::AocError;
use std::cmp::Ordering;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut hands = [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    _input.trim().split('\n').for_each(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        let (value, score) = score_hand(cards);
        hands[value].push((score, bid.parse::<usize>().unwrap()))
    });
    for bucket in hands.iter_mut() {
        bucket.sort_by(order_hands)
    }
    Ok(hands
        .iter()
        .flatten()
        .zip(1usize..)
        // .inspect(|x| println!("{x:?}"))
        .map(|(&(_, score), rank)| score * rank)
        .sum::<usize>()
        .to_string())
}

type Hand = [usize; 5];
fn order_hands<'a, 'b, T>((this, _): &'a (Hand, T), (other, _): &'b (Hand, T)) -> Ordering {
    for i in 0..this.len() {
        match this[i].cmp(&other[i]) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => continue,
        }
    }
    Ordering::Equal
}

fn score_hand(hand: &str) -> (usize, [usize; 5]) {
    let mut set: [usize; 15] = [0; 15];
    let mut arr = [0; 5];
    for (i, char) in hand.chars().enumerate() {
        let idx: usize = {
            match char {
                x if char.is_ascii_digit() => x.to_digit(10).unwrap() as usize,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Unexpected char"),
            }
        };
        set[idx] += 1;
        arr[i] = idx;
    }
    let [x, y] = set.into_iter().fold([0, 0], |top, x| {
        if x > top[0] {
            [x, top[0]]
        } else if x > top[1] {
            [top[0], x]
        } else {
            top
        }
    });
    let kind = {
        match x {
            x if x >= 4 => x + 1,
            x if x == 3 && y == 2 => 4,
            x if x == 3 => 3,
            x if x == 2 && y == 2 => 2,
            x => x - 1,
        }};
    // if y == 2 {
    //     println!("{y}, {:?}", arr.clone())
    // }
    (kind, arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
