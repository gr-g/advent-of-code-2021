fn step(fish_count: &mut [u64; 9]) {
    let spawn = fish_count[0];
    for t in 0..8 {
        fish_count[t] = fish_count[t+1];
    }
    fish_count[6] += spawn;
    fish_count[8] = spawn;
}

fn solve(input: &str) -> (u64, u64) {
    // Store in fish_count[t] the number of fishes with timer set to t.
    let mut fish_count = [0u64; 9];

    for timer in input.trim().split(',') {
        let timer = timer
            .parse::<usize>().ok()
            .filter(|t| *t < 9)
            .expect("Invalid input");
        fish_count[timer] += 1;
    }

    for _ in 0..80 {
        step(&mut fish_count);
    }
    let n_80 = fish_count.iter().sum();

    for _ in 80..256 {
        step(&mut fish_count);
    }
    let n_256 = fish_count.iter().sum();

    (n_80, n_256)
}

fn main() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
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
        assert_eq!(solve("3,4,3,1,2"), (5934, 26984457539));
    }
}
