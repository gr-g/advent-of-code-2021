use std::collections::HashMap;

struct PolymerElements {
    frequency: HashMap<u8, usize>, // frequency of individual elements
    pair_frequency: HashMap<(u8, u8), usize>, // frequency of pairs of adjacent elements
}

impl PolymerElements {
    fn from_template(template: &str) -> Self {
        let mut frequency = HashMap::new();
        for e in template.bytes() {
            *frequency.entry(e).or_insert(0) += 1;
        }

        let mut pair_frequency = HashMap::new();
        for pair in template.as_bytes().windows(2) {
            *pair_frequency.entry((pair[0], pair[1])).or_insert(0) += 1;
        }

        PolymerElements { frequency, pair_frequency }
    }

    fn apply_insertions(&mut self, rules: &HashMap<(u8, u8), u8>) {
        let mut new_pair_frequency = HashMap::with_capacity(self.pair_frequency.len());
        for ((e1, e2), n) in self.pair_frequency.drain() {
            if let Some(&e) = rules.get(&(e1, e2)) {
                *self.frequency.entry(e).or_insert(0) += n;
                *new_pair_frequency.entry((e1, e)).or_insert(0) += n;
                *new_pair_frequency.entry((e, e2)).or_insert(0) += n;
            } else {
                *new_pair_frequency.entry((e1, e2)).or_insert(0) += n;
            }
        }
        self.pair_frequency = new_pair_frequency;
    }

    fn result(&self) -> usize {
        //println!("Element frequencies:");
        //for (e, n) in self.frequency.iter() {
        //    println!("{} -> {}", *e as char, n);
        //}

        let most_common = self.frequency.values().max().unwrap();
        let least_common = self.frequency.values().min().unwrap();
        most_common - least_common
    }
}

fn solve(input: &str) -> (usize, usize) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules: HashMap<_, _> = rules
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(s1, s2)| ((s1.as_bytes()[0], s1.as_bytes()[1]), s2.as_bytes()[0]))
        .collect();

    let mut p = PolymerElements::from_template(template);

    for _ in 0..10 {
        p.apply_insertions(&rules);
    }
    let result_10 = p.result();

    for _ in 10..40 {
        p.apply_insertions(&rules);
    }
    let result_40 = p.result();

    (result_10, result_40)
}

fn main() {
    let input = std::fs::read_to_string("input/14.txt").unwrap();
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
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"), (1588, 2188189693529));
    }
}
