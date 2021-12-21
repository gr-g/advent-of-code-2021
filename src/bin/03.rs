fn solve(input: &str) -> (u32, u32) {
    let diagnostic_report: Vec<_> = input
        .lines()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    // Count the number of bits of the input values
    let n_bits = 16 - diagnostic_report.iter().max().unwrap().leading_zeros();
    assert!(n_bits < 16);

    let mut gamma_rate = 0; // most common bit in each position
    let mut epsilon_rate = 0; // least common bit in each position
    let mut oxygen_rating = 0; // most common bit in each position,
                               // among values with previous bits equal to oxygen
    let mut co2_rating = 0; // least common bit in each position,
                            // among values with previous bits equal to co2

    for d in (0..n_bits).rev() {
        let mut n_zeros = 0; // count of 0s in bit d
        let mut n_ones = 0; // count of 1s in bit d
        let mut n_oxy_zeros = 0; // count of 0s in bit d with previous bits equal to oxygen
        let mut n_oxy_ones = 0; // count of 1s in bit d with previous bits equal to oxygen
        let mut n_co2_zeros = 0; // count of 0s in bit d with previous bits equal to co2
        let mut n_co2_ones = 0; // count of 1s in bit d with previous bits equal to co2

        for v in &diagnostic_report {
            if v & 0b1 << d == 0 {
                n_zeros += 1;
                if v >> (d+1) == oxygen_rating >> (d+1) { n_oxy_zeros += 1; }
                if v >> (d+1) == co2_rating >> (d+1) { n_co2_zeros += 1; }
            } else {
                n_ones += 1;
                if v >> (d+1) == oxygen_rating >> (d+1) { n_oxy_ones += 1; }
                if v >> (d+1) == co2_rating >> (d+1) { n_co2_ones += 1; }
            }
        }

        // Set bit d for gamma_rate and epsilon_rate
        if n_ones >= n_zeros {
            gamma_rate |= 0b1 << d;
        }
        if n_zeros == 0 || (n_zeros > n_ones && n_ones > 0) {
            epsilon_rate |= 0b1 << d;
        }

        // Set bit d for oxygen_rating and co2_rating
        if n_oxy_ones >= n_oxy_zeros {
            oxygen_rating |= 0b1 << d;
        }
        if n_co2_zeros == 0 || (n_co2_zeros > n_co2_ones && n_co2_ones > 0) {
            co2_rating |= 0b1 << d;
        }
    }

    let power_consumption = gamma_rate as u32 * epsilon_rate as u32;
    let life_support_rating = oxygen_rating as u32 * co2_rating as u32;

    (power_consumption, life_support_rating)
}

fn main() {
    let input = std::fs::read_to_string("input/03.txt").unwrap();
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
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"), (22*9, 23*10));
    }
}
