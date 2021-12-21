use std::collections::HashSet;

fn is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_lowercase())
}

struct CaveSystem<'a> {
    connections: Vec<(&'a str, &'a str)>,
    start: &'a str,
    end: &'a str,
}

impl CaveSystem<'_> {
    fn neighbors(&self, cave: &str) -> Vec<&str> {
        let mut neighbors = vec![];
        for &(c1, c2) in &self.connections {
            if c1 == cave {
                neighbors.push(c2);
            }
            if c2 == cave {
                neighbors.push(c1);
            }
        }
        neighbors
    }

    fn paths(&self, start: &str, visited: &HashSet<&str>, revisits: usize) -> Vec<String> {
        if start == self.end {
            // Base case.
            return vec![start.to_string()];
        }

        let mut solutions = vec![];
        let mut visited = visited.clone();
        visited.insert(start);

        // Explore caves connected to 'start'.
        for n in self.neighbors(start) {
            if !is_small(n) || !visited.contains(n) {
                // n is not small or is an unvisited small cave:
                // consider all paths from n.
                for mut s in self.paths(n, &visited, revisits) {
                    s.insert(0, ',');
                    s.insert_str(0, start);
                    solutions.push(s);
                }
            } else if n != self.start && revisits > 0 {
                // n is a visited small cave: consider all paths from n
                // with 1 less revisits.
                for mut s in self.paths(n, &visited, revisits - 1) {
                    s.insert(0, ',');
                    s.insert_str(0, start);
                    solutions.push(s);
                }
            }
        }

        //println!("Solutions from {} (seen: {:?}, revisits: {}): {:#?}", start, visited, revisits, solutions);
        solutions
    }
}

fn solve(input: &str) -> (usize, usize) {
    let caves = CaveSystem{
        connections: input
            .lines()
            .map(|s| s.split_once('-').unwrap())
            .collect(),
        start: "start",
        end: "end",
    };

    // Paths with 0 small caves revisited.
    let n_paths_0 = caves.paths(caves.start, &HashSet::new(), 0).len();

    // Paths with up to 1 small caves revisited.
    let n_paths_1 = caves.paths(caves.start, &HashSet::new(), 1).len();

    (n_paths_0, n_paths_1)
}

fn main() {
    let input = std::fs::read_to_string("input/12.txt").unwrap();
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
start-A
start-b
A-c
A-b
b-d
A-end
b-end"), (10, 36));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"), (19, 103));
    }

    #[test]
    fn example03() {
        assert_eq!(solve("\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"), (226, 3509));
    }
}
