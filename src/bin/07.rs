fn fuel_cost_1(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|p| (p - target).abs())
        .sum()
}

fn fuel_cost_2(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|p| (p - target).abs() * ((p - target).abs() + 1) / 2)
        .sum()
}

fn solve(input: &str) -> (i32, i32) {
    let mut positions: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // When the fuel cost is the distance to the target, the total cost
    // as a function of the target is a piecewise-linear function
    // with a minimum corresponding to the median of the sample.
    positions.sort();
    let median = positions[positions.len() / 2];
    let fuel_cost_1 = fuel_cost_1(&positions, median);

    // When the fuel cost is distance * (distance + 1) / 2, the total cost
    // as a function of the target is a piecewise-quadratic function
    // with a minimum in the range (mean-1/2, mean+1/2), therefore the
    // integer target which achieves the minimum is floor(mean) or ceil(mean).
    let mean_floor = positions.iter().sum::<i32>() / positions.len() as i32;
    let fuel_cost_2 = i32::min(
        fuel_cost_2(&positions, mean_floor),
        fuel_cost_2(&positions, mean_floor+1)
    );

    (fuel_cost_1, fuel_cost_2)
}

fn main() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();
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
        assert_eq!(solve("16,1,2,0,4,2,7,1,2,14"), (37, 168));
    }
}
