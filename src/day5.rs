use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, none_of, space1, u64},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

pub struct Mapping {
    pub source_start: u64,
    pub dest_start: u64,
    pub length: u64,
}

pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Vec<Mapping>>,
}

fn parse_seeds_section(input: &str) -> IResult<&str, Vec<u64>> {
    tuple((tag("seeds:"), space1, separated_list1(space1, u64)))
        .map(|(_, _, seeds)| seeds)
        .parse(input)
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    tuple((u64, space1, u64, space1, u64))
        .map(|(dest_start, _, source_start, _, length)| Mapping {
            dest_start,
            source_start,
            length,
        })
        .parse(input)
}

fn parse_mapping_section(input: &str) -> IResult<&str, Vec<Mapping>> {
    tuple((
        many1(none_of(":")),
        char(':'),
        line_ending,
        separated_list1(line_ending, parse_mapping),
    ))
    .map(|(_, _, _, maps)| maps)
    .parse(input)
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    let (_, almanac) = tuple((
        parse_seeds_section,
        many1(line_ending),
        separated_list1(many1(line_ending), parse_mapping_section),
    ))
    .map(|(seeds, _, maps)| Almanac { seeds, maps })
    .parse(input)
    .unwrap();

    almanac
}

#[aoc(day5, part1)]
pub fn part1(input: &Almanac) -> u64 {
    let mut min_location = u64::MAX;

    for &seed in input.seeds.iter() {
        let location = input.maps.iter().fold(seed, |from, mappings| {
            let mapping = mappings.iter().find(|mapping| {
                mapping.source_start <= from && (mapping.source_start + mapping.length > from)
            });

            if let Some(mapping) = mapping {
                mapping.dest_start + (from - mapping.source_start)
            } else {
                from
            }
        });

        if min_location > location {
            min_location = location;
        }
    }

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn example() -> Almanac {
        input_generator(indoc! {
            "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            "
        })
    }

    #[test]
    pub fn test_parse() {
        let almanac = example();

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.maps.len(), 7);
        assert_eq!(almanac.maps[2].len(), 4);
        assert_eq!(almanac.maps[2][0].dest_start, 49);
        assert_eq!(almanac.maps[2][0].source_start, 53);
        assert_eq!(almanac.maps[2][0].length, 8);
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(&example()), 35);
    }
}
