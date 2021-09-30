use std::iter::Enumerate;
use std::ops::Div;

#[derive(Clone, Debug)]
pub struct Vec2d<T> {
    rows: usize,
    columns: usize,
    items: Vec<T>,
}

impl<T> Vec2d<T> {
    pub fn from_vec(rows: usize, columns: usize, items: Vec<T>) -> Self {
        Vec2d {
            rows,
            columns,
            items,
        }
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        Vec2d {
            rows,
            columns,
            items: Vec::with_capacity(rows * columns),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.columns || y >= self.rows {
            self.items.get(x + y * self.columns)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.columns || y >= self.rows {
            self.items.get_mut(x + y * self.columns)
        } else {
            None
        }
    }

    pub fn iter(&self) -> Vec2dIterator<T> {
        Vec2dIterator {
            x: 0,
            y: 0,
            vec: self,
        }
    }

    pub fn iter_mut(&mut self) -> Vec2dMutIterator<T> {
        Vec2dMutIterator {
            x: 0,
            y: 0,
            vec: self,
        }
    }
}

#[derive(Default)]
struct Vec2dIterator<'a, T> {
    x: usize,
    y: usize,
    vec: &'a Vec2d<T>,
}

impl<'a, T> Iterator for Vec2dIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.x, self.y);
        if let Some(_item) = item {
            let next_index = ((self.x + 1) + self.y * self.vec.columns);

            self.x = (next_index % self.vec.columns);
            self.y = (next_index / self.vec.columns);
        }
        item
    }
}

struct Vec2dMutIterator<'a, T> {
    x: usize,
    y: usize,
    vec: &'a Vec2d<T>,
}

impl<'a, T> Iterator for Vec2dMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get_mut(self.x, self.y);
        if let Some(_item) = item {
            let next_index = ((self.x + 1) + self.y * self.vec.columns);

            self.x = (next_index % self.vec.columns);
            self.y = (next_index / self.vec.columns);
        }
        item
    }
}
