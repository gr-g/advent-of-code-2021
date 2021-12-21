struct BingoBoard {
    numbers: [usize; 25],
    marked: [bool; 25],
    won: bool,
}

enum BingoOutcome {
    NotWon,
    Won(usize),
    AlreadyWon,
}

impl BingoBoard {
    fn create_from(s: &str) -> Self {
        let mut numbers = [0; 25];
        let marked = [false; 25];
        let won = false;

        for (i, n) in s.split_ascii_whitespace().enumerate() {
            numbers[i] = n.parse().unwrap();
        }

        BingoBoard { numbers, marked, won }
    }

    fn call(&mut self, n: usize) -> BingoOutcome {
        if self.won {
            return BingoOutcome::AlreadyWon;
        }

        for i in 0..25 {
            if !self.marked[i] && self.numbers[i] == n {
                self.marked[i] = true;
                if self.wins() {
                    //println!("Bingo! Score: {} * {} = {}", self.sum_of_unmarked(), n, self.sum_of_unmarked() * n);
                    self.won = true;
                    return BingoOutcome::Won(self.sum_of_unmarked() * n)
                }
            }
        }

        BingoOutcome::NotWon
    }

    fn wins(&self) -> bool {
        self.marked[0..5].iter().all(|m| *m) ||
        self.marked[5..10].iter().all(|m| *m) ||
        self.marked[10..15].iter().all(|m| *m) ||
        self.marked[15..20].iter().all(|m| *m) ||
        self.marked[20..25].iter().all(|m| *m) ||
        self.marked[0..].iter().step_by(5).all(|m| *m) ||
        self.marked[1..].iter().step_by(5).all(|m| *m) ||
        self.marked[2..].iter().step_by(5).all(|m| *m) ||
        self.marked[3..].iter().step_by(5).all(|m| *m) ||
        self.marked[4..].iter().step_by(5).all(|m| *m)
    }

    fn sum_of_unmarked(&self) -> usize {
        self.numbers.iter()
            .zip(self.marked.iter())
            .filter(|(_, m)| !*m)
            .map(|(n, _)| n)
            .sum()
    }
}

fn solve(input: &str) -> (usize, usize) {
    let (called_numbers, boards) = input.split_once("\n\n").unwrap();

    let called_numbers: Vec<_> = called_numbers
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut boards: Vec<_> = boards
        .split("\n\n")
        .map(|s| BingoBoard::create_from(s))
        .collect();

    let n_boards = boards.len();

    let mut first_winning_score = 0;
    let mut last_winning_score = 0;
    let mut playing_boards = n_boards;

    for n in called_numbers {
        //println!("Called number: {}", n);
        for b in &mut boards {
            if let BingoOutcome::Won(score) = b.call(n) {
                if playing_boards == n_boards {
                    first_winning_score = score;
                }
                playing_boards -= 1;
                if playing_boards == 0 {
                    last_winning_score = score;
                }
            }
        }
    }

    (first_winning_score, last_winning_score)
}

fn main() {
    let input = std::fs::read_to_string("input/04.txt").unwrap();
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
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"), (188*24, 148*13));
    }
}
