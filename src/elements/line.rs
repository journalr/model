use std::any::Any;
use std::collections::linked_list::{Iter, IterMut};
use std::collections::LinkedList;

use rstar::{RTreeObject, AABB};

use crate::Element;
use crate::Point;

#[derive(Default, PartialEq, Eq, Debug, Clone, Hash)]
pub struct Line {
    points: LinkedList<Point>,
}

impl Line {
    pub fn from_iter<T, P>(points: T) -> Self
    where
        P: Into<Point>,
        T: IntoIterator<Item=P> + Iterator<Item=P>,
    {
        Line {
            points: LinkedList::from_iter(points.map(|p| -> Point { p.into() }).into_iter()),
        }
    }

    pub fn push_back(&mut self, point: Point) {
        self.points.push_back(point);
    }

    pub fn iter(&self) -> Iter<Point> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Point> {
        self.into_iter()
    }
}

impl FromIterator<Point> for Line {
    fn from_iter<T: IntoIterator<Item=Point>>(points: T) -> Self
    {
        Line::from_iter(points.into_iter())
    }
}

impl RTreeObject for Line {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        if self.points.is_empty() {
            return AABB::from_point(Point { x: 0, y: 0 });
        }
        AABB::from_points(self.points.iter())
    }
}

impl Element for Line {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = &'a Point;
    type IntoIter = Iter<'a, Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter()
    }
}

impl<'a> IntoIterator for &'a mut Line {
    type Item = &'a mut Point;
    type IntoIter = IterMut<'a, Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_line() {
        let line = Line::default();
        assert_eq!(line.points.len(), 0);
    }

    #[test]
    fn from_iter_stores_points() {
        let points = vec![(5, 7), (1, 2)];
        let line = Line::from_iter(points.iter());
        assert_eq!(line.points.len(), points.len());
        assert_eq!(*line.points.front().unwrap(), (5, 7));
        assert_eq!(*line.points.back().unwrap(), (1, 2));
    }

    #[test]
    fn can_add_point() {
        let points = vec![(5, 7), (1, 2)];
        let mut line = Line::from_iter(points.iter());
        line.push_back((3, 4).into());
        assert_eq!(line.points.len(), points.len() + 1);
        let mut iter = line.iter();
        assert_eq!(*iter.next().unwrap(), (5, 7));
        assert_eq!(*iter.next().unwrap(), (1, 2));
        assert_eq!(*iter.next().unwrap(), (3, 4));
        assert!(iter.next().is_none());
    }

    #[test]
    fn correct_empty_envelope() {
        let line = Line::default();
        assert_eq!(line.envelope().lower(), (0, 0));
        assert_eq!(line.envelope().upper(), (0, 0));
    }


    #[test]
    fn correct_filled_envelope() {
        let points = vec![(5, 7), (1, 2)];
        let line = Line::from_iter(points.iter());
        assert_eq!(line.envelope().lower(), Point { x: 1, y: 2 });
        assert_eq!(line.envelope().upper(), Point { x: 5, y: 7 });
    }

    #[test]
    fn can_iterate_over_points() {
        let points = vec![(5, 7), (4, 3)];
        let line = Line::from_iter(points.iter());
        let mut iter = line.iter();
        assert_eq!(*iter.next().unwrap(), (5, 7));
        assert_eq!(*iter.next().unwrap(), (4, 3));
        assert!(iter.next().is_none());
    }

    #[test]
    fn can_iterate_and_modify_points() {
        let points = vec![(5, 7), (4, 3), (2, 1)];
        let mut line = Line::from_iter(points.iter());
        let mut modify_iter = line.iter_mut();
        *modify_iter.nth(1).unwrap() = (8, 9).into();
        let mut iter = line.iter();
        assert_eq!(*iter.next().unwrap(), (5, 7));
        assert_eq!(*iter.next().unwrap(), (8, 9));
        assert_eq!(*iter.next().unwrap(), (2, 1));
        assert!(iter.next().is_none());
    }
    
    #[test]
    fn can_cast_to_any() {
        let points = vec![(5, 7), (4, 3), (2, 1)];
        let line = Line::from_iter(points.iter());
        let any = line.as_any();
        let line_ref = any.downcast_ref::<Line>().unwrap();
        assert!(line_ref.iter().eq(points.iter()))
    }

    #[test]
    fn can_cast_to_any_mut() {
        let points = vec![(5, 7), (4, 3), (2, 1)];
        let mut line = Line::from_iter(points.iter());
        let any = line.as_any_mut();
        let line_ref = any.downcast_mut::<Line>().unwrap();
        assert!(line_ref.iter().eq(points.iter()));
        line_ref.push_back((3, 4).into());
        assert_eq!(*line_ref.iter().nth(3).unwrap(), (3, 4))
    }
}
