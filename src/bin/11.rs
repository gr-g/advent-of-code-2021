use advent_of_code_2021::grid::{consts::*, SimpleGrid};

fn step(g: &mut SimpleGrid) {
    let mut flashes = vec![];
    for (pos, e) in g.entries_mut_pos() {
        *e = match *e {
            b'9' => { flashes.push(pos); b'0' },
            _ => { *e + 1 },
        };
    }

    while let Some(pos) = flashes.pop() {
        for d in [UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT] {
            let new_pos = pos.go(d);
            if let Some(e) = g.get_mut_pos(&new_pos) {
                *e = match *e {
                    b'0' => { b'0' },
                    b'9' => { flashes.push(new_pos); b'0' },
                    _ => { *e + 1 },
                };
            }
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut g = SimpleGrid::create_from(input);
    //println!("{}", g);

    let mut n_steps = 0;
    let mut n_flashes = 0;

    while n_steps < 100 {
        step(&mut g);
        n_steps += 1;
        n_flashes += g.values().filter(|e| **e == b'0').count();
        //println!("{}", g);
    }

    while !g.values().all(|e| *e == b'0') {
        step(&mut g);
        n_steps += 1;
        //println!("{}", g);
    }

    (n_flashes, n_steps)
}

fn main() {
    let input = std::fs::read_to_string("input/11.txt").unwrap();
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
        let mut g = SimpleGrid::create_from("\
11111
19991
19191
19991
11111");
        step(&mut g);
        assert_eq!(g.to_string().trim(), "\
34543
40004
50005
40004
34543");
        step(&mut g);
        assert_eq!(g.to_string().trim(), "\
45654
51115
61116
51115
45654");
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"), (1656, 195));
    }
}
