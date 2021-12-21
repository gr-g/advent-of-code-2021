enum Outcome {
    Valid,
    Incomplete(u64),
    Corrupted(u64),
}

fn parse(line: &str) -> Outcome {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => { if stack.pop() != Some('(') { return Outcome::Corrupted(3); } },
            ']' => { if stack.pop() != Some('[') { return Outcome::Corrupted(57); } },
            '}' => { if stack.pop() != Some('{') { return Outcome::Corrupted(1197); } },
            '>' => { if stack.pop() != Some('<') { return Outcome::Corrupted(25137); } },
            _ => panic!(),
        }
    }

    if !stack.is_empty() {
        let mut score = 0;
        while let Some(s) = stack.pop() {
            match s {
                '(' => { score = 5 * score + 1; },
                '[' => { score = 5 * score + 2; },
                '{' => { score = 5 * score + 3; },
                '<' => { score = 5 * score + 4; },
                _ => panic!(),
            }
        }
        return Outcome::Incomplete(score);
    }
    Outcome::Valid
}

fn solve(input: &str) -> (u64, u64) {
    let mut syntax_error_score = 0;
    let mut autocomplete_scores = vec![];

    for line in input.lines() {
        match parse(line) {
            Outcome::Valid => {},
            Outcome::Corrupted(s) => { syntax_error_score += s; },
            Outcome::Incomplete(s) => { autocomplete_scores.push(s); },
        }
    }

    autocomplete_scores.sort();
    let middle_score = autocomplete_scores[autocomplete_scores.len()/2];

    (syntax_error_score, middle_score)
}

fn main() {
    let input = std::fs::read_to_string("input/10.txt").unwrap();
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
        assert_eq!(solve("\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"), (26397, 288957));
    }
}
