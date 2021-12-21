use std::collections::HashMap;

fn advance(pos: u16, steps: u16) -> u16 {
    (pos + steps - 1) % 10 + 1
}

fn play_deterministic(start1: u16, start2: u16) -> (u16, u16, u16)  {
    let (mut score1, mut p1) = (0, start1);
    let (mut score2, mut p2) = (0, start2);
    let mut die = 0;

    loop {
        p1 = advance(p1, (die+1)+(die+2)+(die+3));
        die += 3;
        score1 += p1;
        //println!("after {} rolls: {} - {}", die, score1, score2);
        if score1 >= 1000 {
            break;
        }
        p2 = advance(p2, (die+1)+(die+2)+(die+3));
        die += 3;
        score2 += p2;
        //println!("after {} rolls: {} - {}", die, score1, score2);
        if score2 >= 1000 {
            break;
        }
    }

    (die, score1, score2)
}

fn play_dirac(start1: u16, start2: u16) -> (u64, u64) {
    // Keep track of the number of universes where the game status is
    // (score1, pos1, score2, pos2) after each turn.
    let mut universes1 = HashMap::new(); // status before player 1 turn
    universes1.insert((0, start1, 0, start2), 1);
    let mut universes2 = HashMap::new(); // status before player 2 turn

    let mut wins1 = 0;
    let mut wins2 = 0;

    loop {
        for ((score1, p1, score2, p2), n) in universes1.drain() {
            // The sum of 3 dice is 3 in 1 case, 4 in 3 cases, 5 in 6 cases, ...
            for (d, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let p1 = advance(p1, d);
                if score1 + p1 >= 21 {
                    wins1 += n * times;
                } else {
                    *universes2.entry((score1 + p1, p1, score2, p2)).or_insert(0) += n * times;
                }
            }
        }
        //print!("explored {} universes: ", wins1 + wins2 + universes2.values().sum::<u64>());
        //println!("P1 wins in {}, P2 wins in {}, playing in {}", wins1, wins2, universes2.values().sum::<u64>());
        if universes2.is_empty() {
            break;
        }
        for ((score1, p1, score2, p2), n) in universes2.drain() {
            for (d, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let p2 = advance(p2, d);
                if score2 + p2 >= 21 {
                    wins2 += n * times;
                } else {
                    *universes1.entry((score1, p1, score2 + p2, p2)).or_insert(0) += n * times;
                }
            }
        }
        //print!("explored {} universes: ", wins1 + wins2 + universes1.values().sum::<u64>());
        //println!("P1 wins in {}, P2 wins in {}, playing in {}", wins1, wins2, universes1.values().sum::<u64>());
        if universes1.is_empty() {
            break;
        }
    }

    (wins1, wins2)
}

fn solve(input: &str) -> (u64, u64) {
    let input1 = input.lines().nth(0).unwrap();
    let start1 = input1.strip_prefix("Player 1 starting position: ").unwrap().parse().unwrap();
    let input2 = input.lines().nth(1).unwrap();
    let start2 = input2.strip_prefix("Player 2 starting position: ").unwrap().parse().unwrap();

    let (rolls, score1, score2) = play_deterministic(start1, start2);
    let (wins1, wins2) = play_dirac(start1, start2);

    (rolls as u64 * u16::min(score1, score2) as u64, u64::max(wins1, wins2))
}

fn main() {
    let input = std::fs::read_to_string("input/21.txt").unwrap();
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
Player 1 starting position: 4
Player 2 starting position: 8"), (739785, 444356092776315));
    }
}
