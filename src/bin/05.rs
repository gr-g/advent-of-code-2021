use std::collections::HashMap;

#[derive(Debug, Clone)]
struct HydrothermalVent {
    start: (i16, i16),
    end: (i16, i16),
}

impl HydrothermalVent {
    fn create_from(s: &str) -> Self {
        let (p1, p2) = s.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();

        HydrothermalVent {
            start: (x1.parse().unwrap(), y1.parse().unwrap()),
            end: (x2.parse().unwrap(), y2.parse().unwrap()),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
}

fn vent_frequency_map(vents: &[HydrothermalVent]) -> HashMap<(i16, i16), usize> {
    let mut vent_frequency_map = HashMap::new();

    for v in vents {
        let ((x1, y1), (x2, y2)) = (v.start, v.end);
        let n_points = i16::max((x2-x1).abs(), (y2-y1).abs());
        //println!("vent: {:?} -> {:?}", (x1,y1), (x2,y2));

        for p in 0..=n_points {
            let x = (x1 * (n_points-p) + x2 * p) / n_points;
            let y = (y1 * (n_points-p) + y2 * p) / n_points;
            //println!("passing by: {:?}", (x,y));
            *vent_frequency_map.entry((x,y)).or_insert(0) += 1;
        }
    }

    vent_frequency_map
}

fn solve(input: &str) -> (usize, usize) {
    let vents: Vec<_> = input
        .lines()
        .map(|s| HydrothermalVent::create_from(s))
        .collect();

    let vents_hv: Vec<_> = vents
        .iter()
        .filter(|v| v.is_horizontal() || v.is_vertical())
        .cloned()
        .collect();

    let vent_frequency_hv = vent_frequency_map(&vents_hv);
    let vent_overlaps_hv = vent_frequency_hv.values().filter(|v| **v >= 2).count();

    let vent_frequency = vent_frequency_map(&vents);
    let vent_overlaps = vent_frequency.values().filter(|v| **v >= 2).count();

    (vent_overlaps_hv, vent_overlaps)
}

fn main() {
    let input = std::fs::read_to_string("input/05.txt").unwrap();
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
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"), (5, 12));
    }
}
