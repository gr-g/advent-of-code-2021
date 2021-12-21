//use std::collections::BTreeMap;
//use std::collections::HashSet;
//use std::collections::HashMap;

/*struct Node {
    n_children: usize,
    n_metadata: usize,
    children: Vec<Node>,
    metadata: Vec<String>,
}*/

/*impl Node {
    fn create_from(s: &str) -> Node {
        let (id, offset_size) = s.split_once(" @ ").unwrap();
        let (offset, size) = offset_size.split_once(": ").unwrap();
        let (offset_x, offset_y) = offset.split_once(",").unwrap();
        let (offset_x, offset_y) = (offset_x.parse().unwrap(), offset_y.parse().unwrap());
        let (size_x, size_y) = size.split_once("x").unwrap();
        let (size_x, size_y) = (size_x.parse().unwrap(), size_y.parse().unwrap());

        Node {
            n_children: id[1..].parse().unwrap(),
            n_metadata: (offset_x, offset_y),
            children: (size_x, size_y)
            metadata: (offset_x, offset_y),
        }
    }
}*/

fn solve(input: &str) -> (usize, usize) {
    //let pairs: Vec<_> = input
    //    .lines()
    //    .map(|s| s.split_once(", ").unwrap())
    //    .map(|(n1, n2)| (n1.parse::<i64>().unwrap(), n2.parse::<i64>().unwrap()))
    //    .collect();


    (0, 0)
}

fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();
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
"), (0, 0));
    }
}
