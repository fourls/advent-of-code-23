use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, space1, u32, u8},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult, Parser,
};

pub type Round = (u32, u32, u32);

pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

enum Color {
    RED,
    GREEN,
    BLUE,
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    separated_list0(
        char(','),
        tuple((
            space1,
            u8,
            space1,
            alt((
                tag("red").map(|_| Color::RED),
                tag("green").map(|_| Color::GREEN),
                tag("blue").map(|_| Color::BLUE),
            )),
        ))
        .map(|(_, count, _, color)| (count, color)),
    )
    .map(|turns| {
        turns.into_iter().fold(
            (0, 0, 0),
            |mut round: Round, (count, color): (u8, Color)| {
                *match color {
                    Color::RED => &mut round.0,
                    Color::GREEN => &mut round.1,
                    Color::BLUE => &mut round.2,
                } += count as u32;
                round
            },
        )
    })
    .parse(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    tuple((
        delimited(tag("Game "), u32, char(':')),
        separated_list0(char(';'), parse_round),
    ))
    .map(|(id, rounds)| Game { id, rounds })
    .parse(input)
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let games = separated_list0(line_ending, parse_game).parse(input);

    let (_, games) = games.unwrap();
    return games;
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[aoc(day2, part1)]
pub fn part1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|game| {
            !game
                .rounds
                .iter()
                .any(|round| round.0 > MAX_RED || round.1 > MAX_GREEN || round.2 > MAX_BLUE)
        })
        .fold(0, |id_sum, game| id_sum + game.id)
}

#[aoc(day2, part2)]
pub fn part2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            game.rounds.iter().fold((0, 0, 0), |max, curr| {
                (
                    if max.0 < curr.0 { curr.0 } else { max.0 },
                    if max.1 < curr.1 { curr.1 } else { max.1 },
                    if max.2 < curr.2 { curr.2 } else { max.2 },
                )
            })
        })
        .map(|min_set| min_set.0 * min_set.1 * min_set.2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse() {
        let games = input_generator(indoc! {
            "
            Game 1: 1 green, 2 blue; 13 red, 2 blue, 3 green; 4 green, 14 red
            Game 2: 2 blue, 11 green; 4 blue, 12 red, 4 green; 7 red, 1 blue, 9 green; 10 green, 12 red, 6 blue
            Game 3: 1 blue, 12 green, 2 red; 9 red, 16 green; 1 red, 10 green, 1 blue; 1 red, 14 green
            Game 4: 8 green, 18 blue; 4 green, 14 blue, 2 red; 3 blue, 5 green, 11 red
            Game 5: 7 red, 15 blue, 1 green; 13 blue; 18 red, 2 green, 9 blue; 19 blue, 5 green, 10 red; 9 green, 2 blue, 7 red
            Game 6: 1 red, 8 blue, 2 green; 1 blue, 3 red, 5 green; 2 green, 3 red, 2 blue; 1 blue, 4 green
            "
        });

        assert_eq!(games.len(), 6);
        assert_eq!(games[0].rounds.len(), 3);
        assert_eq!(games[0].rounds[1].0, 13);
        assert_eq!(games[0].rounds[1].1, 3);
        assert_eq!(games[0].rounds[1].2, 2);
    }

    #[test]
    fn test_part1() {
        let input = input_generator(indoc! {
            "
            Game 1: 1 green, 2 blue; 13 red, 2 blue, 3 green; 4 green, 14 red
            Game 2: 2 blue, 11 green; 4 blue, 12 red, 4 green; 7 red, 1 blue, 9 green; 10 green, 12 red, 6 blue
            Game 3: 1 blue, 12 green, 2 red; 9 red, 16 green; 1 red, 10 green, 1 blue; 1 red, 14 green
            Game 4: 8 green, 18 blue; 4 green, 14 blue, 2 red; 3 blue, 5 green, 11 red
            Game 5: 7 red, 15 blue, 1 green; 13 blue; 18 red, 2 green, 9 blue; 19 blue, 5 green, 10 red; 9 green, 2 blue, 7 red
            Game 6: 1 red, 8 blue, 2 green; 1 blue, 3 red, 5 green; 2 green, 3 red, 2 blue; 1 blue, 4 green
            "
        });

        assert_eq!(part1(&input), 2 + 6);
    }

    #[test]
    fn test_part2_power_set() {
        let input = input_generator(indoc! {
            "
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            "
        });

        // (9, 16, 1)

        assert_eq!(part2(&input), 1560);
    }

    #[test]
    fn test_part2_sum() {
        let input = input_generator(indoc! {
            "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
        });

        assert_eq!(part2(&input), 2286);
    }
}
