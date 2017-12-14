
use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct Grid<T>
    where T: Copy
{
    size_x: usize,
    size_y: usize,
    default: T,
    cells: Vec<T>,
}

impl<T> Grid<T>
    where T: Copy
{
    pub fn new(size_x: usize, size_y: usize, default: T) -> Grid<T> {
        let mut cells = Vec::with_capacity(size_x * size_y);
        for _ in 0..(size_x * size_y) {
            cells.push(default);
        }
        return Grid {
            size_x: size_x,
            size_y: size_y,
            default: default,
            cells: cells,
        };
    }

    pub fn get(&self, x: isize, y: isize) -> T {
        if (x < 0) || (y < 0) {
            return self.default;
        }
        let x = x as usize;
        let y = y as usize;
        if (x >= self.size_x) || (y >= self.size_y) {
            return self.default;
        }
        return self.cells[y * self.size_x + x];
    }

    pub fn set(&mut self, x: isize, y: isize, val: T) {
        let x = x as usize;
        let y = y as usize;
        self.cells[y * self.size_x + x] = val;
    }

    pub fn set2(&mut self, x: usize, y: usize, val: T) {
        self.cells[y * self.size_x + x] = val;
    }

    pub fn iter(&self) -> Iter<T> {
        return self.cells.iter();
    }
}
