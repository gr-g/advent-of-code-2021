fn solve(input: &str) -> (usize, usize) {
    let values: Vec<_> = input
        .lines()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let increases = values
        .windows(2)
        .filter(|v| v[1] > v[0])
        .count();

    let increases3 = values
        .windows(4)
        .filter(|v| v[3] > v[0])
        .count();

    (increases, increases3)
}

fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();
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
199
200
208
210
200
207
240
269
260
263"), (7, 5));
    }
}
