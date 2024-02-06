use crate::custom_error::AocError;
use std::cmp::Ordering;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut hands = [vec![], vec![], vec![], vec![], vec![], ];
    _input.split('\n').for_each(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        let (value, score) = score_hand(cards);
        hands[value].push((score, bid.parse::<usize>().unwrap()))
    });
    for bucket in hands.iter_mut() {
        bucket.sort_by(order_hands)
    }
    hands.iter().flatten().zip(1..).map(|()|)


    Ok(1.to_string())
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

fn score_hand(hand: &str) -> (usize, [usize;5]) {
    let mut set: [usize; 15] = [0; 15];
    let mut arr = [0; 5];
    for (i, char) in hand.chars().enumerate() {
        
        let idx: usize = {match char {
            x if char.is_ascii_digit() => x.to_digit(10).unwrap() as usize,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unexpected char")
        }};
        set[idx] += 1;
        arr[i] = idx;
    }
    (set.into_iter().max().unwrap(), arr)
}

fn rank_card(x: char) -> u8 {
    match x {
        x if x.is_ascii_digit() => x.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        _ => panic!("Unexpected char")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
