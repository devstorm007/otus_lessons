use std::borrow::Borrow;
use std::ops::{Add, Deref};

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {}

impl Point<()> {}

impl<T> Point<T> {} // blanket implementation for all types

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Point<T> {
    type Output = Point<T>;
    fn add(self, other: T) -> Point<T> {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<&T> for Point<T> {
    type Output = Point<T>;
    fn add(self, other: &T) -> Point<T> {
        Self {
            x: self.x + *other,
            y: self.y + *other,
        }
    }
}

impl Add<Point<i32>> for Point<i8> {
    type Output = Point<f32>;
    fn add(self, other: Point<i32>) -> Point<f32> {
        Point {
            x: self.x as f32 + other.x as f32,
            y: self.y as f32 + other.y as f32,
        }
    }
}

// turbofish - explicit type defining
// apparently: В плане ::<> по сравнению с <> позволяет компилятору не проходить по коду второй раз
//some::<Type>()
