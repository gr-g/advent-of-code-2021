// Each segment in the seven-segment display is associated to an index
// in 0..7 according to the following pattern:
//
//   0000
//  1    2
//  1    2
//   3333
//  4    5
//  4    5
//   6666
//
static DIGITS: [(char, [bool; 7]); 10] = [
    ('0', [true , true , true , false, true , true , true ]),
    ('1', [false, false, true , false, false, true , false]),
    ('2', [true , false, true , true , true , false, true ]),
    ('3', [true , false, true , true , false, true , true ]),
    ('4', [false, true , true , true , false, true , false]),
    ('5', [true , true , false, true , false, true , true ]),
    ('6', [true , true , false, true , true , true , true ]),
    ('7', [true , false, true , false, false, true , false]),
    ('8', [true , true , true , true , true , true , true ]),
    ('9', [true , true , true , true , false, true , true ]),
];

fn derive_wiring(patterns: &str) -> Option<[char; 7]> {
    // Deduce the segment associated to each symbol by counting how many
    // times the symbol appears in the digits '0'-'9'.
    // Disambiguate segment 0 and segment 2: segment 2 appears in digit '1'.
    // Disambiguate segment 3 and segment 6: segment 3 appears in digit '4'.
    let symbols = "abcdefg";

    let symbols_in_1 = patterns
        .split(' ')
        .find(|pattern| pattern.len() == 2)?;

    let symbols_in_4 = patterns
        .split(' ')
        .find(|pattern| pattern.len() == 4)?;

    let mut wiring = [' '; 7];

    for s in symbols.chars()  {
        let count = patterns.chars().filter(|x| *x == s).count();
        let is_in_1 = symbols_in_1.contains(s);
        let is_in_4 = symbols_in_4.contains(s);
        let decoded_segment = match (count, is_in_1, is_in_4) {
            (8, false,     _) => 0,
            (6, _    ,     _) => 1,
            (8, true ,     _) => 2,
            (7, _    , true ) => 3,
            (4, _    , _    ) => 4,
            (9, _    , _    ) => 5,
            (7, _    , false) => 6,
            _ => return None,
        };
        wiring[decoded_segment] = s;
    }
    Some(wiring)
}

fn digit_from_pattern(wiring: &[char; 7], pattern: &str) -> Option<char> {
    let mut decoded_pattern = [false; 7];
    for s in pattern.chars() {
        // Find the segment associated to this symbol.
        let index = wiring.iter().position(|w| *w == s)?;
        decoded_pattern[index] = true;
    }

    DIGITS.iter()
          .find(|(_, pattern)| *pattern == decoded_pattern)
          .map(|(digit, _)| *digit)
}

fn output_value(patterns: &str, display: &str) -> u32 {
    // Derive the wiring from the patterns
    let wiring = derive_wiring(patterns).unwrap();

    // Apply the wiring to interpret the display output.
    let decoded_number: String = display
        .split(' ')
        .map(|pattern| digit_from_pattern(&wiring, pattern).unwrap())
        .collect();

    decoded_number.parse::<u32>().unwrap()
}

fn solve(input: &str) -> (usize, u32) {
    let entries: Vec<_> = input
        .lines()
        .map(|s| s.split_once(" | ").unwrap())
        .collect();

    let mut count_digits_1478 = 0;
    for (_, display) in &entries {
        for pattern in display.split(' ') {
            if [2,3,4,7].contains(&pattern.len()) {
                count_digits_1478 += 1;
            }
        }
    }

    let output_values_sum = entries.iter()
        .map(|(patterns, display)| output_value(patterns, display))
        .sum();

    (count_digits_1478, output_values_sum)
}

fn main() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(derive_wiring("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"), Some(['d', 'e', 'a', 'f', 'g', 'b', 'c']));
        assert_eq!(output_value("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", "cdfeb fcadb cdfeb cdbaf"), 5353);
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"), (26, 61229));
    }
}
