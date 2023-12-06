use nom::{
    branch::alt,
    character::complete::{char, line_ending, none_of, one_of},
    multi::{many1, separated_list0},
    IResult, Parser,
};

type Point = (i32, i32);

pub struct Num {
    pub num: u32,
    pub length: i32,
    pub pos: Point,
}

pub struct Symbol {
    pub sym: char,
    pub pos: Point,
}

pub struct Board {
    pub symbols: Vec<Symbol>,
    pub nums: Vec<Num>,
}

enum Node {
    Symbol(char),
    Number(u32, i32),
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<Node>>> {
    many1(alt((
        char('.').map(|_| None),
        many1(one_of("0123456789")).map(|c| {
            Some(Node::Number(
                str::parse(&c.iter().collect::<String>()).unwrap(),
                c.len() as i32,
            ))
        }),
        none_of("\n\r").map(|c| Some(Node::Symbol(c))),
    )))
    .parse(input)
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Board {
    let (_, points) = separated_list0(line_ending, parse_line)
        .parse(input)
        .expect("Parsing failed");

    let mut symbols: Vec<Symbol> = vec![];
    let mut nums: Vec<Num> = vec![];

    for (y, row) in points.into_iter().enumerate() {
        let mut x = 0;
        for cell in row.into_iter() {
            x += match cell {
                Some(cell) => match cell {
                    Node::Symbol(c) => {
                        symbols.push(Symbol {
                            sym: c,
                            pos: (x, y as i32),
                        });
                        1
                    }
                    Node::Number(num, length) => {
                        nums.push(Num {
                            num,
                            length,
                            pos: (x, y as i32),
                        });
                        length
                    }
                },
                None => 1,
            }
        }
    }

    Board { symbols, nums }
}

fn is_point_adjacent_to_rect(
    rect_topleft: &Point,
    rect_bottomright: &Point,
    point: &Point,
) -> bool {
    (point.1 >= rect_topleft.1 && point.1 <= rect_bottomright.1)
        && (point.0 >= rect_topleft.0 && point.0 <= rect_bottomright.0)
}

#[aoc(day3, part1)]
pub fn part1(board: &Board) -> u32 {
    board
        .nums
        .iter()
        .filter(|num| {
            let top_left = (num.pos.0 - 1, num.pos.1 - 1);
            let bottom_right = (num.pos.0 + num.length, num.pos.1 + 1);

            board
                .symbols
                .iter()
                .any(|symbol| is_point_adjacent_to_rect(&top_left, &bottom_right, &symbol.pos))
        })
        .fold(0, |sum, num| {
            sum + num.num
        })
}

#[aoc(day3, part2)]
pub fn part2(board: &Board) -> u32 {
    board
        .symbols
        .iter()
        .filter(|sym| sym.sym == '*')
        .map(|symbol| {
            let adjacent_nums: Vec<_> = board
                .nums
                .iter()
                .filter(|num| {
                    let top_left = (num.pos.0 - 1, num.pos.1 - 1);
                    let bottom_right = (num.pos.0 + num.length, num.pos.1 + 1);

                    is_point_adjacent_to_rect(&top_left, &bottom_right, &symbol.pos)
                })
                .map(|num| num.num)
                .collect();

            if adjacent_nums.len() == 2 {
                Some(adjacent_nums[0] * adjacent_nums[1])
            } else {
                None
            }
        })
        .filter(|maybe_nums| maybe_nums.is_some())
        .map(|maybe_nums| maybe_nums.unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse() {
        let board = input_generator(indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
        });

        assert_eq!(board.nums.len(), 10);
        assert_eq!(board.symbols.len(), 6);
        assert_eq!(board.nums[3].pos, (6, 2));
        assert_eq!(board.nums[3].length, 3);
        assert_eq!(board.symbols[2].pos, (3, 4));
    }

    #[test]
    fn test_part1() {
        let board = input_generator(indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
        });

        assert_eq!(part1(&board), 4361);
    }

    #[test]
    fn test_part2() {
        let board = input_generator(indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592+....
            ......755.
            ...$.*....
            .664.598..
            "
        });

        assert_eq!(part2(&board), 467835);
    }
}
