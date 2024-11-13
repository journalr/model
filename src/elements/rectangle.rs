use std::any::Any;
use std::cmp::{max, min};

use rstar::{RTreeObject, AABB};

use crate::Element;
use crate::Point;

#[derive(Debug)]
pub struct Rectangle {
    lower: Point,
    upper: Point,
}

impl Rectangle {
    fn get_real_lower_and_upper(lower: Point, upper: Point) -> (Point, Point) {
        let real_lower = Point {
            x: min(lower.x, upper.x),
            y: min(lower.y, upper.y),
        };
        let real_upper = Point {
            x: max(lower.x, upper.x),
            y: max(lower.y, upper.y),
        };

        (real_lower, real_upper)
    }

    pub fn new(lower: Point, upper: Point) -> Self {
        let (real_lower, real_upper) = Self::get_real_lower_and_upper(lower, upper);
        Rectangle {
            lower: real_lower,
            upper: real_upper,
        }
    }

    pub fn lower(&self) -> Point {
        self.lower
    }

    pub fn upper(&self) -> Point {
        self.upper
    }
}

impl RTreeObject for Rectangle {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.lower, self.upper)
    }
}

impl Element for Rectangle {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resorts_corners() {
        for rectangle in [
            Rectangle::new((2, 1).into(), (1, 0).into()),
            Rectangle::new((2, 0).into(), (1, 1).into()),
            Rectangle::new((1, 0).into(), (2, 1).into()),
            Rectangle::new((1, 1).into(), (2, 0).into()),
        ] {
            assert_eq!(rectangle.lower(), (1, 0));
            assert_eq!(rectangle.upper(), (2, 1));
        }
    }

    #[test]
    fn correct_envelope() {
        let lower = Point { x: 0, y: 0 };
        let upper = Point { x: 1, y: 1 };
        let rectangle = Rectangle::new(lower, upper);
        assert_eq!(rectangle.envelope().lower(), lower);
        assert_eq!(rectangle.envelope().upper(), upper);
    }

    #[test]
    fn correct_lower() {
        let lower = Point { x: 2, y: 0 };
        let upper = Point { x: 1, y: 1 };
        let rectangle = Rectangle::new(lower, upper);
        assert_eq!(rectangle.lower(), Point { x: 1, y: 0 });
    }

    #[test]
    fn correct_upper() {
        let lower = Point { x: 2, y: 0 };
        let upper = Point { x: 1, y: 1 };
        let rectangle = Rectangle::new(lower, upper);
        assert_eq!(rectangle.upper(), Point { x: 2, y: 1 });
    }

    #[test]
    fn can_cast_to_any() {
        let lower = Point { x: 1, y: 0 };
        let upper = Point { x: 2, y: 2 };
        let rectangle = Rectangle::new(lower, upper);
        let any = rectangle.as_any();
        let rectangle_ref = any.downcast_ref::<Rectangle>().unwrap();
        assert_eq!(rectangle_ref.lower(), lower)
    }

    #[test]
    fn can_cast_to_any_mut() {
        let lower = Point { x: 1, y: 0 };
        let upper = Point { x: 2, y: 2 };
        let mut rectangle = Rectangle::new(lower, upper);
        let any = rectangle.as_any_mut();
        let rectangle_ref = any.downcast_mut::<Rectangle>().unwrap();
        assert_eq!(rectangle_ref.lower(), (1, 0));
    }
}
