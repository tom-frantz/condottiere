use std::iter::Enumerate;
use std::ops::Div;
use std::slice::IterMut;

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
        if x < self.columns || y < self.rows {
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
}

#[derive(Clone, Debug)]
pub struct Vec2dInterpolated<T> {
    rows: usize,
    columns: usize,
    items: Vec<T>,
}

impl<T> Vec2dInterpolated<T> {
    pub fn from_vec(rows: usize, columns: usize, items: Vec<T>) -> Self {
        Vec2dInterpolated {
            rows,
            columns,
            items,
        }
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        Vec2dInterpolated {
            rows,
            columns,
            items: Vec::with_capacity(rows * columns),
        }
    }

    // TODO interpolate properly with the triangle library
    // pub fn get(&self, x: f64, y: f64) -> Option<&T> {
    //     if x < self.columns as f64 || y < self.rows as f64 {
    //         self.items.get(x + y * self.columns as f64)
    //     } else {
    //         None
    //     }
    // }
    //
    // pub fn get_mut(&mut self, x: f64, y: f64) -> Option<&mut T> {
    //     if x >= self.columns as f64 || y >= self.rows as f64 {
    //         self.items.get_mut(x + y * self.columns as f64)
    //     } else {
    //         None
    //     }
    // }
    //
    // fn interpolate(start: f64, end: f64, percent: f64) -> f64 {
    //     return (end - start) * percent + start;
    // }
}

#[cfg(test)]
mod tests {
    use triangle::{Point, Triangle};

    #[test]
    fn testing() {
        let point1 = Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let point2 = Point {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        };
        let sqrt_075 = (3.0_f64 / 4.0).sqrt();
        let point3 = Point {
            x: 0.5,
            y: sqrt_075,
            z: 1.0,
        };

        let triangle = Triangle::new(point1, point2, point3);
        assert!(triangle.has_point(Point {
            x: 0.5,
            y: 0.25,
            z: 2.0
        }));
        println!(
            "{:?}",
            triangle.cartesian_to_barycentric(&triangle.centroid())
        );
    }
}
