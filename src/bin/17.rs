fn solve(input: &str) -> (i64, usize) {
    let (str_x, str_y) = input.trim().strip_prefix("target area: ").unwrap().split_once(", ").unwrap();
    let (str_x_min, str_x_max) = str_x.strip_prefix("x=").unwrap().split_once("..").unwrap();
    let (str_y_min, str_y_max) = str_y.strip_prefix("y=").unwrap().split_once("..").unwrap();
    let target_x_min = str_x_min.parse::<i64>().unwrap();
    let target_x_max = str_x_max.parse::<i64>().unwrap();
    let target_y_min = str_y_min.parse::<i64>().unwrap();
    let target_y_max = str_y_max.parse::<i64>().unwrap();

    assert!(target_y_max < 0);
    assert!(target_x_min > 0);

    // High trajectories with initial vertical velocity dy always
    // pass through y=dy -> y=0 -> y=-dy-1.
    // Max height is achieved when dy = -target_y_min - 1.
    let max_y = ((-target_y_min - 1) * (-target_y_min)) / 2;

    let mut n_solutions = 0;

    for start_dy in (target_y_min)..=(-target_y_min-1) {
        for start_dx in 1..=target_x_max {
            let (mut x, mut y) = (0, 0);
            let (mut dx, mut dy) = (start_dx, start_dy);
            while y >= target_y_min && x <= target_x_max && (dx > 0 || x >= target_x_min) {
                if x >= target_x_min && y <= target_y_max {
                    //println!("Target hit with initial velocity: ({}, {})", start_dx, start_dy);
                    n_solutions += 1;
                    break;
                }
                x += dx;
                y += dy;
                if dx > 0 { dx -= 1; }
                dy -= 1;
            }
        }
    }

    (max_y, n_solutions)
}

fn main() {
    let input = std::fs::read_to_string("input/17.txt").unwrap();
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
        assert_eq!(solve("target area: x=20..30, y=-10..-5"), (45, 112));
    }
}
