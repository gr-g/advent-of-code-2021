use std::collections::HashMap;
use std::fmt::Display;

pub mod consts {
    use super::{Pos, Dir};
    pub const ORIGIN: Pos = Pos{ x: 0, y: 0 };
    pub const UP: Dir = Dir{ dx: 0, dy: -1 };
    pub const DOWN: Dir = Dir{ dx: 0, dy: 1 };
    pub const LEFT: Dir = Dir{ dx: -1, dy: 0 };
    pub const RIGHT: Dir = Dir{ dx: 1, dy: 0 };
    pub const UP_LEFT: Dir = Dir{ dx: -1, dy: -1 };
    pub const UP_RIGHT: Dir = Dir{ dx: 1, dy: -1 };
    pub const DOWN_LEFT: Dir = Dir{ dx: -1, dy: 1 };
    pub const DOWN_RIGHT: Dir = Dir{ dx: 1, dy: 1 };
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pos {
    pub x: i16, // steps right from the reference point
    pub y: i16, // steps down from the reference point
}

impl Pos {
    pub fn go( &self, dir: Dir ) -> Pos {
        Pos{ x: self.x + dir.dx, y: self.y + dir.dy }
    }

    pub fn distance( &self, other: &Pos ) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dir {
    pub dx: i16,
    pub dy: i16,
}

impl Dir {
    pub fn add( &self, other: &Dir ) -> Dir {
        Dir{ dx: self.dx + other.dx, dy: self.dy + other.dy }
    }

    pub fn rotate_left( &self ) -> Dir {
        Dir{ dx: self.dy, dy: -self.dx }
    }

    pub fn rotate_right( &self ) -> Dir {
        Dir{ dx: -self.dy, dy: self.dx }
    }

    pub fn reverse( &self ) -> Dir {
        Dir{ dx: -self.dx, dy: -self.dy }
    }

    pub fn times( &self, n: i16 ) -> Dir {
        Dir{ dx: self.dx * n, dy: self.dy * n }
    }
}

// A fixed-size grid of bytes, with values accessed by row/column with
// get()/get_mut()/set(), or by Pos with get_pos()/get_mut_pos()/set_pos().
#[derive(Clone, Debug)]
pub struct SimpleGrid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl SimpleGrid {
    pub fn new( rows: usize, cols: usize ) -> SimpleGrid {
        assert!(rows > 0 && rows < i16::MAX as usize);
        assert!(cols > 0 && cols < i16::MAX as usize);
        SimpleGrid{ rows, cols, data: vec![0; rows*cols] }
    }

    pub fn create_from( s: &str ) -> SimpleGrid {
        let cols = s.find('\n').unwrap();
        let mut rows = 0;
        let mut data = Vec::with_capacity(s.len());

        for line in s.lines() {
            rows += 1;
            data.extend_from_slice(line.as_bytes());
            assert_eq!(data.len(), rows*cols, "input lines have different lengths");
        }

        SimpleGrid{ rows, cols, data }
    }

    pub fn rows( &self ) -> usize {
        self.rows
    }

    pub fn cols( &self ) -> usize {
        self.cols
    }

    pub fn top_left( &self ) -> Pos {
        Pos { x: 0, y: 0 }
    }

    pub fn top_right( &self ) -> Pos {
        Pos { x: self.cols as i16 - 1, y: 0 }
    }

    pub fn bottom_left( &self ) -> Pos {
        Pos { x: 0, y: self.rows as i16 - 1 }
    }

    pub fn bottom_right( &self ) -> Pos {
        Pos { x: self.cols as i16 - 1, y: self.rows as i16 - 1 }
    }

    pub fn get( &self, row: usize, col: usize ) -> Option<&u8> {
        if row < self.rows && col < self.cols {
            self.data.get(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn get_mut( &mut self, row: usize, col: usize ) -> Option<&mut u8> {
        if row < self.rows && col < self.cols {
            self.data.get_mut(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn set( &mut self, row: usize, col: usize, v: u8 ) {
        assert!(row < self.rows && col < self.cols);
        self.data[row * self.cols + col] = v;
    }

    pub fn entries( &self ) -> impl Iterator<Item = ((usize, usize), &u8)> {
        self.data.iter()
            .enumerate()
            .map(|(i, c)| ((i / self.cols, i % self.cols), c))
    }

    pub fn entries_mut( &mut self ) -> impl Iterator<Item = ((usize, usize), &mut u8)> {
        self.data.iter_mut()
            .enumerate()
            .map(|(i, c)| ((i / self.cols, i % self.cols), c))
    }

    pub fn values( &self ) -> impl Iterator<Item = &u8> {
        self.data.iter()
    }

    pub fn get_pos( &self, p: &Pos ) -> Option<&u8> {
        if p.x >= 0 && (p.x as usize) < self.cols && p.y >= 0 && (p.y as usize) < self.rows {
            self.data.get(p.y as usize * self.cols + p.x as usize)
        } else {
            None
        }
    }

    pub fn get_mut_pos( &mut self, p: &Pos ) -> Option<&mut u8> {
        if p.x >= 0 && (p.x as usize) < self.cols && p.y >= 0 && (p.y as usize) < self.rows {
            self.data.get_mut(p.y as usize * self.cols + p.x as usize)
        } else {
            None
        }
    }

    pub fn set_pos( &mut self, p: &Pos, v: u8 ) {
        assert!(p.x >= 0 && (p.x as usize) < self.cols && p.y >= 0 && (p.y as usize) < self.rows);
        self.data[p.y as usize * self.cols + p.x as usize] = v;
    }

    pub fn entries_pos( &self ) -> impl Iterator<Item = (Pos, &u8)> {
        self.data.iter()
            .enumerate()
            .map(|(i, c)| (Pos{ x: (i % self.cols) as i16, y: (i / self.cols) as i16 }, c))
    }

    pub fn entries_mut_pos( &mut self ) -> impl Iterator<Item = (Pos, &mut u8)> {
        self.data.iter_mut()
            .enumerate()
            .map(|(i, c)| (Pos{ x: (i % self.cols) as i16, y: (i / self.cols) as i16 }, c))
    }
}

impl Display for SimpleGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = *self.get(row, col).filter(|v| v.is_ascii_graphic()).unwrap_or(&b' ') as char;
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// A sparse grid of Unicode characters, with values accessed by
// Pos with get()/insert()/remove().
#[derive(Clone, Debug)]
pub struct SparseGrid {
    pub symbols: HashMap<Pos, char>,
}

impl SparseGrid {
    pub fn new() -> Self {
        SparseGrid { symbols: HashMap::new() }
    }
    pub fn create_from( s: &str ) -> Self {
        let mut symbols = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        for c in s.chars() {
            match c {
                '\n' => { y += 1; x = 0; },
                c => {
                    if !c.is_ascii_whitespace() {
                        symbols.insert(Pos { x, y }, c);
                    }
                    x += 1;
                }
            }
        }
        SparseGrid { symbols }
    }

    pub fn x_min( &self ) -> i16 {
        *self.symbols.keys().map(|Pos { x, .. }| x).min().unwrap_or(&0)
    }

    pub fn x_max( &self ) -> i16 {
        *self.symbols.keys().map(|Pos { x, .. }| x).max().unwrap_or(&0)
    }

    pub fn y_min( &self ) -> i16 {
        *self.symbols.keys().map(|Pos { y, .. }| y).min().unwrap_or(&0)
    }

    pub fn y_max( &self ) -> i16 {
        *self.symbols.keys().map(|Pos { y, .. }| y).max().unwrap_or(&0)
    }

    pub fn get(&self, p: &Pos) -> Option<&char> {
        self.symbols.get(p)
    }

    pub fn insert( &mut self, p: Pos, c: char ) -> Option<char> {
        self.symbols.insert(p, c)
    }

    pub fn remove( &mut self, p: &Pos ) -> Option<char> {
        self.symbols.remove(p)
    }

    pub fn find( &self, c: char ) -> Option<&Pos> {
        self.symbols.iter().find(|(_, sym)| **sym == c ).map(|(pos, _)| pos)
    }
}

impl Display for SparseGrid {
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        let x0 = self.x_min();
        let x1 = self.x_max();
        let y0 = self.y_min();
        let y1 = self.y_max();

        for y in y0..=y1 {
            for x in x0..=x1 {
                write!(f, "{}", self.get(&Pos { x, y }).unwrap_or(&' '))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
