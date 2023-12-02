#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += get_first_digit(line.chars()) * 10;
        sum += get_first_digit(line.chars().rev());
    }

    return sum;
}

fn get_first_digit<I>(chars: I) -> u32
where
    I: Iterator<Item = char>,
{
    for chr in chars {
        if chr.is_numeric() {
            if let Some(num) = chr.to_digit(10) {
                return num;
            }
        }
    }

    return 0;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let first = get_first_digit_or_digit_word(|| line.chars(), false);
        let second = get_first_digit_or_digit_word(|| line.chars().rev(), true);
        sum += first * 10 + second;
    }

    return sum;
}

fn get_first_digit_or_digit_word<I, F>(chars_builder: F, reverse: bool) -> u32
where
    I: Iterator<Item = char>,
    F: Fn() -> I,
{
    for (i, chr) in chars_builder().enumerate() {
        if chr.is_numeric() {
            if let Some(num) = chr.to_digit(10) {
                return num;
            }
        } else if reverse {
            let mut maybe_num = None;

            iter_digit_words_with_last_char(chr, |word| {
                let res = parse_digit_word(word, chars_builder().skip(i), reverse);
                if res {
                    maybe_num = Some(word.number);
                }
                res
            });

            if let Some(num) = maybe_num {
                return num;
            }
        } else {
            let mut maybe_num = None;

            iter_digit_words_with_first_char(chr, |word| {
                let res = parse_digit_word(word, chars_builder().skip(i), reverse);
                if res {
                    maybe_num = Some(word.number);
                }
                res
            });

            if let Some(num) = maybe_num {
                return num;
            }
        }
    }

    panic!("No digits found in line")
}

struct NumberWord {
    pub word: &'static str,
    pub number: u32,
}

const NUMBER_WORDS: [NumberWord; 10] = [
    NumberWord {
        word: "zero",
        number: 0,
    },
    NumberWord {
        word: "one",
        number: 1,
    },
    NumberWord {
        word: "two",
        number: 2,
    },
    NumberWord {
        word: "three",
        number: 3,
    },
    NumberWord {
        word: "four",
        number: 4,
    },
    NumberWord {
        word: "five",
        number: 5,
    },
    NumberWord {
        word: "six",
        number: 6,
    },
    NumberWord {
        word: "seven",
        number: 7,
    },
    NumberWord {
        word: "eight",
        number: 8,
    },
    NumberWord {
        word: "nine",
        number: 9,
    },
];

fn parse_digit_word<I>(word: &NumberWord, mut actual_chars: I, reverse: bool) -> bool
where
    I: Iterator<Item = char>,
{
    if reverse {
        word.word
            .chars()
            .rev()
            .all(|chr| actual_chars.next() == Some(chr))
    } else {
        word.word
            .chars()
            .all(|chr| actual_chars.next() == Some(chr))
    }
}

fn iter_digit_words_with_first_char<F>(first_chr: char, mut func: F) -> bool
where
    F: FnMut(&NumberWord) -> bool,
{
    match first_chr {
        // zero
        'z' => func(&NUMBER_WORDS[0]),
        // one
        'o' => func(&NUMBER_WORDS[1]),
        // two, three
        't' => func(&NUMBER_WORDS[2]) || func(&NUMBER_WORDS[3]),
        // four, five
        'f' => func(&NUMBER_WORDS[4]) || func(&NUMBER_WORDS[5]),
        // six, seven
        's' => func(&NUMBER_WORDS[6]) || func(&NUMBER_WORDS[7]),
        // eight
        'e' => func(&NUMBER_WORDS[8]),
        // nine
        'n' => func(&NUMBER_WORDS[9]),
        _ => false,
    }
}

fn iter_digit_words_with_last_char<F>(last_chr: char, mut func: F) -> bool
where
    F: FnMut(&NumberWord) -> bool,
{
    match last_chr {
        // zero, two
        'o' => func(&NUMBER_WORDS[0]) || func(&NUMBER_WORDS[2]),
        // one, three, five, nine
        'e' => {
            func(&NUMBER_WORDS[1])
                || func(&NUMBER_WORDS[3])
                || func(&NUMBER_WORDS[5])
                || func(&NUMBER_WORDS[9])
        }
        // four
        'r' => func(&NUMBER_WORDS[4]),
        // six
        'x' => func(&NUMBER_WORDS[6]),
        // seven
        'n' => func(&NUMBER_WORDS[7]),
        // eight
        't' => func(&NUMBER_WORDS[8]),
        _ => false,
    }
}
