use advent_of_code_2021::grid::{Pos, SparseGrid};
use std::collections::HashSet;

fn fold(dots: &HashSet<(i16, i16)>, instruction: &str) -> HashSet<(i16, i16)> {
    let fold_type = &instruction[0..13];
    let fold_position = instruction[13..].parse::<i16>().unwrap();

    match fold_type {
        "fold along x=" => {
            dots.iter()
                .map(|(x, y)| (i16::min(*x, 2 * fold_position - *x), *y ))
                .collect()
            }
        "fold along y=" => {
            dots.iter()
                .map(|(x, y)| (*x, i16::min(*y, 2 * fold_position - *y)))
                .collect()
        },
        _ => panic!(),
    }
}

fn solve(input: &str) -> (usize, String) {
    let (dots, instructions) = input.split_once("\n\n").unwrap();

    let mut dots: HashSet<_> = dots
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()))
        .collect();

    let mut instructions = instructions.lines();

    // Apply the first fold instruction.
    dots = fold(&dots, instructions.next().unwrap());
    let visible_dots = dots.len();

    // Apply the remaining fold instructions.
    for i in instructions {
        dots = fold(&dots, i);
    }

    // Arrange the dots on a grid.
    let mut folded_paper = SparseGrid::new();
    for (x, y) in dots {
        folded_paper.insert(Pos { x, y }, '\u{2588}');
    }

    (visible_dots, folded_paper.to_string())
}

fn main() {
    let input = std::fs::read_to_string("input/13.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {}\n{}", s.0, s.1);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(solve("\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"), (17, "\
█████
█   █
█   █
█   █
█████
".to_string()));
    }
}
