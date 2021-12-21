use advent_of_code_2021::grid::SimpleGrid;

struct InfiniteImage {
    pixels: SimpleGrid,
    background: u8,
}

impl InfiniteImage {
    fn create_from(s: &str) -> Self {
        InfiniteImage {
            pixels: SimpleGrid::create_from(s),
            background: b'.',
        }
    }

    fn get(&self, row: isize, col: isize) -> &u8 {
        if row < 0 || col < 0 {
            &self.background
        } else {
            self.pixels.get(row as usize, col as usize).unwrap_or(&self.background)
        }
    }

    fn enhance(&self, algorithm: &[u8]) -> InfiniteImage {
        assert!(algorithm.len() == 512);

        let rows = self.pixels.rows() + 2;
        let cols = self.pixels.cols() + 2;
        let mut pixels = SimpleGrid::new(rows, cols);

        for r in 0..rows as isize {
            for c in 0..cols as isize {
                let mut index = 0;
                for (wr, wc) in [(r-2,c-2), (r-2,c-1), (r-2,c),
                                 (r-1,c-2), (r-1,c-1), (r-1,c),
                                 (  r,c-2), (  r,c-1), (  r,c)] {
                    index <<= 1;
                    if self.get(wr, wc) == &b'#' {
                        index += 1;
                    }
                }
                pixels.set(r as usize, c as usize, algorithm[index]);
            }
        }

        let mut background = algorithm[0];
        if self.background == b'#' {
            background = algorithm[511];
        }

        InfiniteImage{ pixels, background }
    }

    fn lit_pixels(&self) -> usize {
        assert!(self.background != b'#');
        self.pixels.values().filter(|c| **c == b'#').count()
    }
}

fn solve(input: &str) -> (usize, usize) {
    let (algorithm, image) = input.split_once("\n\n").unwrap();

    let algorithm = algorithm.as_bytes();
    let mut image = InfiniteImage::create_from(image);
    //println!("background: {}\n{}", image.background as char, image.pixels);

    for _ in 0..2 {
        image = image.enhance(algorithm);
        //println!("background: {}\n{}", image.background as char, image.pixels);
    }
    let lit_pixels_2 = image.lit_pixels();

    for _ in 2..50 {
        image = image.enhance(algorithm);
    }
    let lit_pixels_50 = image.lit_pixels();

    (lit_pixels_2, lit_pixels_50)
}

fn main() {
    let input = std::fs::read_to_string("input/20.txt").unwrap();
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
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"), (35, 3351));
    }
}
