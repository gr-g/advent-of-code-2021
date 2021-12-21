use advent_of_code_2021::graph::UnweightedGraph;
use advent_of_code_2021::grid::{consts::*, Pos, SimpleGrid};
use std::cmp::Reverse;

struct Cave(SimpleGrid);

impl UnweightedGraph<Pos> for Cave {
    fn edges(&self, node: &Pos) -> Vec<Pos> {
        let mut v = Vec::new();
        let node_value = self.0.get_pos(node).unwrap();

        // Add edges to nearby positions that have a higher value, but less than 9.
        for d in [UP, DOWN, LEFT, RIGHT] {
            let next_node = node.go(d);
            if self.0.get_pos(&next_node).filter(|c| *c < &b'9' && *c > node_value).is_some() {
                v.push(next_node);
            }
        }
        v
    }
}

fn solve(input: &str) -> (u32, usize) {
    let g = Cave(SimpleGrid::create_from(input));

    //println!("{}", g.0);

    let mut risk_level = 0;
    let mut basins = vec![];

    for (pos, value) in g.0.entries_pos() {
        if [UP, DOWN, LEFT, RIGHT]
            .into_iter()
            .all(|d| value < g.0.get_pos(&pos.go(d)).unwrap_or(&b'9'))
        {
            // The curent position is a low point
            risk_level += (*value - b'0') as u32 + 1;

            basins.push(g.shortest_paths(pos, &[]).len());
        }
    }

    basins.sort_by_key(|b| Reverse(*b));

    (risk_level, basins[0]*basins[1]*basins[2])
}

fn main() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();
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
2199943210
3987894921
9856789892
8767896789
9899965678"), (15, 1134));
    }
}
