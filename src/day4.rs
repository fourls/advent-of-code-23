use std::cmp::min;

use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, space1, u8},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

pub struct Card {
    pub id: u32,
    pub winners: Vec<u32>,
    pub numbers: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    tuple((
        tag("Card"),
        space1,
        u8,
        char(':'),
        space1,
        separated_list1(space1, u8),
        space1,
        char('|'),
        space1,
        separated_list1(space1, u8),
    ))
    .map(|(_, _, id, _, _, winners, _, _, _, numbers)| Card {
        id: id as u32,
        winners: winners.into_iter().map(|num| num as u32).collect(),
        numbers: numbers.into_iter().map(|num| num as u32).collect(),
    })
    .parse(input)
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    let (_, cards) = separated_list0(line_ending, parse_card)
        .parse(input)
        .unwrap();

    cards
}

#[aoc(day4, part1)]
pub fn part1(cards: &Vec<Card>) -> u32 {
    let mut total_score: u32 = 0;

    for card in cards {
        let mut score: u32 = 0;

        for num in &card.numbers {
            if card.winners.contains(&num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        total_score += score;
    }

    total_score
}

#[aoc(day4, part2)]
pub fn part2(cards: &Vec<Card>) -> u32 {
    let mut cache: Vec<Option<u32>> = vec![None; cards.len()];
    let mut sum: u32 = 0;

    for i in 0..cards.len() {
        sum += calculate_card_score(cards, i, &mut cache);
    }

    sum
}

fn calculate_card_score(cards: &Vec<Card>, index: usize, cache: &mut Vec<Option<u32>>) -> u32 {
    if let Some(cached_score) = cache.get(index).unwrap() {
        return *cached_score;
    }

    let card = cards.get(index).unwrap();

    let num_winners = card
        .numbers
        .iter()
        .filter(|num| card.winners.contains(num))
        .count();

    let mut card_score = 1;

    let next_index = index + 1;
    let last_copied_index = min(next_index + num_winners, cards.len() - 1);

    for i in next_index..last_copied_index {
        card_score += calculate_card_score(cards, i, cache);
    }

    *cache.get_mut(index).unwrap() = Some(card_score);

    card_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    pub fn test_parse() {
        let cards = input_generator(indoc! {
            "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        });

        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0].id, 1);
        assert_eq!(cards[0].winners, vec![41, 48, 83, 86, 17]);
        assert_eq!(cards[0].numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    pub fn test_part1() {
        let cards = input_generator(indoc! {
            "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        });

        assert_eq!(part1(&cards), 13);
    }

    #[test]
    pub fn test_part2() {
        let cards = input_generator(indoc! {
            "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        });

        assert_eq!(part2(&cards), 30);
    }
}
