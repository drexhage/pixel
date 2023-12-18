use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::{Position, Size};

#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle {
    pub size: Size,
    pub position: Position,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rectangle {
            size: Size { width, height },
            position: Position { x, y },
        }
    }

    pub fn of(position: Position, size: Size) -> Self {
        Rectangle { size, position }
    }

    pub fn intersect(a: &Rectangle, b: &Rectangle) -> Self {
        let upper_x = i32::max(a.position.x, b.position.x);
        let upper_y = i32::max(a.position.y, b.position.y);
        let lower_x = i32::min(
            a.position.x + a.size.width as i32,
            b.position.x + b.size.width as i32,
        );
        let lower_y = i32::min(
            a.position.y + a.size.height as i32,
            b.position.y + b.size.height as i32,
        );
        let width = (lower_x - upper_x) as u32;
        let height = (lower_y - upper_y) as u32;

        Rectangle {
            size: Size::new(width, height),
            position: Position::new(upper_x, upper_y),
        }
    }

    pub fn intersectn(rectangles: &[&Rectangle]) -> Self {
        let mut result = Rectangle::intersect(rectangles[0], rectangles[1]);
        for rectangle in rectangles {
            result = Rectangle::intersect(&result, rectangle);
        }
        result
    }

    /// All points of the rectangle in global coordinates
    pub fn points(&self) -> Vec<Position> {
        let mut result = vec![];
        for x in 0i32..self.size.width as i32 {
            for y in 0i32..self.size.height as i32 {
                result.push(Position::new(x + self.position.x, y + self.position.y));
            }
        }
        result
    }

    pub fn bounding(a: &Rectangle, b: &Rectangle) -> Self {
        let upper_x = i32::min(a.position.x, b.position.x);
        let upper_y = i32::min(a.position.y, b.position.y);
        let lower_x = i32::max(
            a.position.x + a.size.width as i32,
            b.position.x + b.size.width as i32,
        );
        let lower_y = i32::max(
            a.position.y + a.size.height as i32,
            b.position.y + b.size.height as i32,
        );
        let width = (lower_x - upper_x) as u32;
        let height = (lower_y - upper_y) as u32;

        Rectangle {
            size: Size::new(width, height),
            position: Position::new(upper_x, upper_y),
        }
    }

    pub fn bounding_all(list: Vec<Rectangle>) -> Self {
        list.into_iter()
            .reduce(|a, b| Rectangle::bounding(&a, &b))
            .unwrap_or((0, 0, 0, 0).into())
    }
}

impl Add<&Position> for &Rectangle {
    type Output = Rectangle;

    fn add(self, rhs: &Position) -> Self::Output {
        Rectangle {
            size: self.size,
            position: self.position + *rhs,
        }
    }
}

impl Sub<&Position> for &Rectangle {
    type Output = Rectangle;

    fn sub(self, rhs: &Position) -> Self::Output {
        Rectangle {
            size: self.size,
            position: self.position - *rhs,
        }
    }
}

impl From<(i32, i32, u32, u32)> for Rectangle {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Rectangle {
            size: (w, h).into(),
            position: (x, y).into(),
        }
    }
}

impl From<&Rectangle> for (u32, u32, u32, u32) {
    fn from(value: &Rectangle) -> Self {
        (
            value.position.x as u32,
            value.position.y as u32,
            value.size.width,
            value.size.height,
        )
    }
}

impl From<Vec<Rectangle>> for Rectangle {
    fn from(value: Vec<Rectangle>) -> Self {
        Rectangle::bounding_all(value)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle<x={}, y={}, w={}, h={}>",
            self.position.x, self.position.y, self.size.width, self.size.height
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{Position, Rectangle};

    #[test]
    fn intersection() {
        let a: Rectangle = (10, 10, 10, 10).into();
        let b: Rectangle = (5, 5, 10, 10).into();
        assert_eq!(Rectangle::intersect(&a, &b), (10, 10, 5, 5).into());

        let a: Rectangle = (5, 5, 10, 10).into();
        let b: Rectangle = (10, 0, 10, 20).into();
        assert_eq!(Rectangle::intersect(&a, &b), (10, 5, 5, 10).into());

        let a: Rectangle = (0, 1, 2, 2).into();
        let b: Rectangle = (0, 0, 2, 2).into();
        assert_eq!(Rectangle::intersect(&a, &b), (0, 1, 2, 1).into());
    }

    #[test]
    fn points() {
        let a: Rectangle = (0, 1, 2, 1).into();
        println!("{:?}", a);
        let expected: Vec<Position> = vec![(0, 1).into(), (1, 1).into()];
        assert_eq!(a.points(), expected);
    }

    #[test]
    fn bounding() {
        let a: Rectangle = (10, 10, 10, 10).into();
        let b: Rectangle = (5, 5, 10, 10).into();
        assert_eq!(Rectangle::bounding(&a, &b), (5, 5, 15, 15).into());

        let a: Rectangle = (5, 5, 10, 10).into();
        let b: Rectangle = (10, 0, 10, 20).into();
        assert_eq!(Rectangle::bounding(&a, &b), (5, 0, 15, 20).into());
    }
}
