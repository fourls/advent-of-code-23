use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32},
    error::ErrorKind,
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

pub struct Race {
    pub duration: u32,
    pub record_distance: u32,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Race> {
    let (_, result) = tuple::<_, _, (_, ErrorKind), _>((
        tag("Time:"),
        space1,
        separated_list1(space1, u32),
        line_ending,
        tag("Distance:"),
        space1,
        separated_list1(space1, u32),
    ))
    .map(|(_, _, times, _, _, _, distances)| {
        times
            .into_iter()
            .zip(distances)
            .map(|(duration, record_distance)| Race {
                duration,
                record_distance,
            })
            .collect()
    })
    .parse(input)
    .unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    pub fn test_parse() {
        let races = input_generator(indoc! {
            "
            Time:      7  15   30
            Distance:  9  40  200
            "
        });

        assert_eq!(races.len(), 3);
        assert_eq!(races[1].duration, 15);
        assert_eq!(races[1].record_distance, 40);
    }
}
