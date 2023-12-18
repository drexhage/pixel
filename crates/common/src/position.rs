use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[cfg(feature = "serde")]
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn zero() -> Self {
        Position::new(0, 0)
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        f64::sqrt(i32::pow(self.x - other.x, 2) as f64 + i32::pow(self.y - other.y, 2) as f64)
    }

    pub fn interpolate(a: &Position, b: &Position) -> Vec<Position> {
        let (x0, y0) = (a.x, a.y);
        let (x1, y1) = (b.x, b.y);
        let dx = i32::abs(x0 - x1);
        let dy = i32::abs(y0 - y1);
        let two_dx = 2 * dx;
        let two_dy = 2 * dy;
        let mut result = vec![];
        if dy > dx {
            let mut d_hat = two_dx - dy;
            let (y_start, y_end, x_start, x_end) = if y0 < y1 {
                (y0, y1, x0, x1)
            } else {
                (y1, y0, x1, x0)
            };
            let mut x = x_start;
            let incr = if x_start < x_end { 1 } else { -1 };

            for y in y_start..y_end {
                result.push((x, y).into());
                if d_hat <= 0 {
                    d_hat += two_dx;
                } else {
                    d_hat += two_dx - two_dy;
                    x += incr;
                }
            }
        } else {
            let mut d_hat = two_dy - dx;
            let (x_start, x_end, y_start, y_end) = if x0 < x1 {
                (x0, x1, y0, y1)
            } else {
                (x1, x0, y1, y0)
            };
            let mut y = y_start;
            let incr = if y_start < y_end { 1 } else { -1 };

            for x in x_start..x_end {
                result.push((x, y).into());
                if d_hat <= 0 {
                    d_hat += two_dy;
                } else {
                    d_hat += two_dy - two_dx;
                    y += incr;
                }
            }
        }
        result.push(*b);
        result
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos[{},{}]", self.x, self.y)
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<Position> for (i32, i32) {
    fn from(value: Position) -> Self {
        (value.x, value.y)
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position::new(x, y)
    }
}

// Custom serde for smaller serialization size (`[1, 2]` instead of `{"x":1,"y":2}`)
#[cfg(feature = "serde")]
impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PositionVisitor;

        impl<'de> Visitor<'de> for PositionVisitor {
            type Value = Position;

            fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }

            fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: SeqAccess<'de>,
            {
                let x: i32 = access.next_element().unwrap().unwrap();
                let y: i32 = access.next_element().unwrap().unwrap();
                Ok(Position { x, y })
            }
        }

        deserializer.deserialize_seq(PositionVisitor)
    }
}

#[cfg(all(feature = "serde", test))]
mod test {
    use crate::Position;

    #[test]
    fn deserialize() {
        let json = "[12,2]";
        let reconstructed: Position = serde_json::from_str(json).unwrap();
        assert_eq!(reconstructed.x, 12);
        assert_eq!(reconstructed.y, 2);
    }

    #[test]
    fn serialize() {
        let pos = Position { x: 2, y: 1 };
        let json = serde_json::to_string(&pos).unwrap();
        assert_eq!(json, "[2,1]")
    }

    #[test]
    fn interpolate() {
        let interpolated = Position::interpolate(&(2, 0).into(), &(5, 2).into());
        assert_eq!(
            interpolated,
            vec![(2, 0).into(), (3, 1).into(), (4, 1).into(), (5, 2).into()]
        );
    }
}
