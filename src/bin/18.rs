use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Number {
    Value(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn create_from(s: &str) -> Self {
        if s.get(..1) == Some("[") && s.get(s.len()-1..) == Some("]") {
            let mut depth = 0;
            for (pos, c) in s.char_indices() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 1 => {
                        return Number::Pair(
                            Box::new(Number::create_from(&s[1..pos])),
                            Box::new(Number::create_from(&s[pos+1..s.len()-1])),
                        );
                    },
                    _ => {},
                }
            }
        }
        Number::Value(s.parse().unwrap())
    }

    fn magnitude(&self) -> usize {
        match self {
            Number::Value(v) => *v,
            Number::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn leftmost_value(&mut self) -> &mut usize {
        match self {
            Number::Value(v) => v,
            Number::Pair(l, _) => l.leftmost_value(),
        }
    }

    fn rightmost_value(&mut self) -> &mut usize {
        match self {
            Number::Value(v) => v,
            Number::Pair(_, r) => r.rightmost_value(),
        }
    }

    fn explode(&mut self, depth: usize, left: Option<&mut Number>, right: Option<&mut Number>) -> bool {
        match self {
            Number::Value(_) => {
                false
            },
            Number::Pair(l, r) => {
                if depth == 0 {
                    if let Some(n) = left { *n.rightmost_value() += l.magnitude(); };
                    if let Some(n) = right { *n.leftmost_value() += r.magnitude(); };
                    *self = Number::Value(0);
                    true
                } else {
                    l.explode(depth - 1, left, Some(r)) || r.explode(depth - 1, Some(l), right)
                }
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Value(v) => {
                if *v >= 10 {
                    *self = Number::Pair(
                        Box::new(Number::Value(*v/2)),
                        Box::new(Number::Value((*v+1)/2))
                    );
                    true
                } else {
                    false
                }
            },
            Number::Pair(l, r) => {
                l.split() || r.split()
            },
        }
    }

    fn reduce(&mut self) {
        if self.explode(4, None, None) {
            //println!("after explode: {}", self);
            self.reduce();
        }
        if self.split() {
            //println!("after split: {}", self);
            self.reduce();
        }
    }

    fn add(&self, other: &Number) -> Self {
        let mut n = Number::Pair(Box::new(self.clone()), Box::new(other.clone()));
        //println!("after addition: {}", n);
        n.reduce();
        n
    }
}

fn solve(input: &str) -> (usize, usize) {
    let numbers: Vec<_> = input
        .lines()
        .map(|s| Number::create_from(s))
        .collect();

    let (first, rest) = numbers.split_first().unwrap();
    let sum = rest.iter().fold(first.clone(), |acc, n| { acc.add(n) });

    let max_pairwaise_magnitude = numbers
        .iter()
        .flat_map(|i| { numbers.iter().map(|j| i.add(j).magnitude()) })
        .max()
        .unwrap();

    (sum.magnitude(), max_pairwaise_magnitude)
}

fn main() {
    let input = std::fs::read_to_string("input/18.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Value(v) => { write!(f, "{}", v)?; },
            Number::Pair(l, r) => { write!(f, "[{},{}]", l, r)?; },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let mut n = Number::create_from("[[[[[9,8],1],2],3],4]");
        n.explode(4, None, None);
        assert_eq!(n, Number::create_from("[[[[0,9],2],3],4]"));

        let mut n = Number::create_from("[7,[6,[5,[4,[3,2]]]]]");
        n.explode(4, None, None);
        assert_eq!(n, Number::create_from("[7,[6,[5,[7,0]]]]"));

        let mut n = Number::create_from("[[6,[5,[4,[3,2]]]],1]");
        n.explode(4, None, None);
        assert_eq!(n, Number::create_from("[[6,[5,[7,0]]],3]"));

        let mut n = Number::create_from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        n.explode(4, None, None);
        assert_eq!(n, Number::create_from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        let mut n = Number::create_from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        n.explode(4, None, None);
        assert_eq!(n, Number::create_from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn example02() {
        let n1 = Number::create_from("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = Number::create_from("[1,1]");
        assert_eq!(n1.add(&n2), Number::create_from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn example03() {
        let mut sum = Number::create_from("[1,1]");
        sum = sum.add(&Number::create_from("[2,2]"));
        sum = sum.add(&Number::create_from("[3,3]"));
        sum = sum.add(&Number::create_from("[4,4]"));
        assert_eq!(sum, Number::create_from("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
        sum = sum.add(&Number::create_from("[5,5]"));
        assert_eq!(sum, Number::create_from("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
        sum = sum.add(&Number::create_from("[6,6]"));
        assert_eq!(sum, Number::create_from("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    fn example04() {
        let mut sum = Number::create_from("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        sum = sum.add(&Number::create_from("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"));
        assert_eq!(sum, Number::create_from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
        sum = sum.add(&Number::create_from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"));
        assert_eq!(sum, Number::create_from("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"));
        sum = sum.add(&Number::create_from("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"));
        assert_eq!(sum, Number::create_from("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"));
        sum = sum.add(&Number::create_from("[7,[5,[[3,8],[1,4]]]]"));
        assert_eq!(sum, Number::create_from("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"));
        sum = sum.add(&Number::create_from("[[2,[2,2]],[8,[8,1]]]"));
        assert_eq!(sum, Number::create_from("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"));
        sum = sum.add(&Number::create_from("[2,9]"));
        assert_eq!(sum, Number::create_from("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"));
        sum = sum.add(&Number::create_from("[1,[[[9,3],9],[[9,0],[0,7]]]]"));
        assert_eq!(sum, Number::create_from("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"));
        sum = sum.add(&Number::create_from("[[[5,[7,4]],7],1]"));
        assert_eq!(sum, Number::create_from("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"));
        sum = sum.add(&Number::create_from("[[[[4,2],2],6],[8,7]]"));
        assert_eq!(sum, Number::create_from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    }

    #[test]
    fn example05() {
        assert_eq!(Number::create_from("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(Number::create_from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(Number::create_from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(Number::create_from("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(Number::create_from("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(Number::create_from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
    }

    #[test]
    fn example06() {
        assert_eq!(solve("\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"), (4140, 3993));
    }
}
