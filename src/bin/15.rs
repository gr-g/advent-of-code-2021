use advent_of_code_2021::graph::Graph;
use advent_of_code_2021::grid::{consts::*, Pos, SimpleGrid};

struct Cave(SimpleGrid);

impl Graph<Pos> for Cave {
    fn edges(&self, node: &Pos) -> Vec<(Pos, usize)> {
        let mut v = Vec::new();

        // Add edges to nearby positions
        for d in [UP, DOWN, LEFT, RIGHT] {
            let next_node = node.go(d);
            if let Some(&value) = self.0.get_pos(&next_node) {
                v.push((next_node, (value - b'0') as usize));
            }
        }
        v
    }
}

fn solve(input: &str) -> (usize, usize) {
    let g = Cave(SimpleGrid::create_from(input));
    let (rows, cols) = (g.0.rows(), g.0.cols());

    //println!("{}", g.0);

    let start = g.0.top_left();
    let end = g.0.bottom_right();
    let risk = g.shortest_paths(start)[&end];

    let (rows5, cols5) = (rows * 5, cols * 5);
    let mut g5 = Cave(SimpleGrid::new(rows5, cols5));
    for r in 0..rows5 {
        for c in 0..cols5 {
            let g_value = g.0.get(r % rows, c % cols).unwrap();
            g5.0.set(r, c, (g_value + (r/rows) as u8 + (c/cols) as u8 - b'1') % 9 + b'1');
        }
    }

    //println!("{}", g5.0);

    let end5 = g5.0.bottom_right();
    let risk5 = g5.shortest_paths(start)[&end5];

    (risk, risk5)
}

fn main() {
    let input = std::fs::read_to_string("input/15.txt").unwrap();
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
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"), (40, 315));
    }
}
