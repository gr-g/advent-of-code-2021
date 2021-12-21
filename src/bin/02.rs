fn solve(input: &str) -> (usize, usize) {
    let commands: Vec<_> = input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(c, n)| (c, n.parse::<usize>().unwrap()))
        .collect();

    let mut pos = (0, 0);
    for &(c, n) in &commands {
        match c {
            "forward" => pos.0 += n,
            "up" => pos.1 -= n,
            "down" => pos.1 += n,
            _ => panic!("Invalid instruction"),
        }
    }

    let mut aim = 0;
    let mut pos2 = (0, 0);
    for &(c, n) in &commands {
        match c {
            "forward" => { pos2.0 += n; pos2.1 += n * aim; },
            "up" => aim -= n,
            "down" => aim += n,
            _ => panic!("Invalid instruction"),
        }
    }

    (pos.0 * pos.1, pos2.0 * pos2.1)
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
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
forward 5
down 5
forward 8
up 3
down 8
forward 2"), (150, 900));
    }
}
