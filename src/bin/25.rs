use advent_of_code_2021::grid::SimpleGrid;

// Advance the state by one step and return true if there was any change.
fn step(g: &mut SimpleGrid) -> bool {
    let mut changed = false;

    // Advance towards east.
    for r in 0..g.rows() {
        let mut c = 0;
        let mut c_end = g.cols() - 1;
        if g.get(r, c_end) == Some(&b'>') && g.get(r, c) == Some(&b'.') {
            // Wrap around the edge of the map.
            changed = true;
            g.set(r, c_end, b'.');
            g.set(r, c, b'>');
            c += 1;
            c_end -= 1;
        }
        while c < c_end {
            if g.get(r, c) == Some(&b'>') && g.get(r, c+1) == Some(&b'.') {
                changed = true;
                g.set(r, c, b'.');
                g.set(r, c+1, b'>');
                c += 2;
            } else {
                c += 1;
            }
        }
    }

    // Advance towards south.
    for c in 0..g.cols() {
        let mut r = 0;
        let mut r_end = g.rows() - 1;
        if g.get(r_end, c) == Some(&b'v') && g.get(r, c) == Some(&b'.') {
            // Wrap around the edge of the map.
            changed = true;
            g.set(r_end, c, b'.');
            g.set(r, c, b'v');
            r += 1;
            r_end -= 1;
        }
        while r < r_end {
            if g.get(r, c) == Some(&b'v') && g.get(r+1, c) == Some(&b'.') {
                changed = true;
                g.set(r, c, b'.');
                g.set(r+1, c, b'v');
                r += 2;
            } else {
                r += 1;
            }
        }
    }

    changed
}

fn solve(input: &str) -> usize {
    let mut g = SimpleGrid::create_from(input);

    let mut t = 1;
    while step(&mut g) {
        t += 1;
        //println!("{}", g);
    }

    t
}

fn main() {
    let input = std::fs::read_to_string("input/25.txt").unwrap();
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
        let mut g = SimpleGrid::create_from("...>>>>>...\n");
        step(&mut g);
        assert_eq!(g.to_string().as_str(), "...>>>>.>..\n");
        step(&mut g);
        assert_eq!(g.to_string().as_str(), "...>>>.>.>.\n");
    }

    #[test]
    fn example02() {
        let mut g = SimpleGrid::create_from("\
..........
.>v....v..
.......>..
..........
");
        step(&mut g);
        assert_eq!(g.to_string().as_str(), "\
..........
.>........
..v....v>.
..........
");
    }


    #[test]
    fn example03() {
        let mut g = SimpleGrid::create_from("\
...>...
.......
......>
v.....>
......>
.......
..vvv..
");
        step(&mut g);
        assert_eq!(g.to_string().as_str(), "\
..vv>..
.......
>......
v.....>
>......
.......
....v..
");
        step(&mut g);
        assert_eq!(g.to_string().as_str(), "\
....v>.
..vv...
.>.....
......>
v>.....
.......
.......
");
        step(&mut g);
        assert_eq!(g.to_string().as_str(), "\
......>
..v.v..
..>v...
>......
..>....
v......
.......
");
        step(&mut g);
        assert_eq!(g.to_string().as_str(), "\
>......
..v....
..>.v..
.>.v...
...>...
.......
v......
");
    }

    #[test]
    fn example04() {
        assert_eq!(solve("\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"), 58);
    }
}
