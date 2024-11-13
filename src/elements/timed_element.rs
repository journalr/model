use crate::{Element, Point};
use rstar::{RTreeObject, AABB};
use std::any::Any;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct TimedElement {
    z_index: i32,
    t_index: u128,
    element: Box<dyn Element>,
}

impl TimedElement {
    pub fn new<T>(element: T, z_index: i32, t_index: u128) -> Self
    where
        T: Element + 'static,
    {
        TimedElement {
            z_index,
            t_index,
            element: Box::new(element),
        }
    }

    pub fn z_index(&self) -> i32 {
        self.z_index
    }

    pub fn t_index(&self) -> u128 {
        self.t_index
    }
}

impl Deref for TimedElement {
    type Target = dyn Any;

    fn deref(&self) -> &Self::Target {
        self.element.deref().as_any()
    }
}

impl DerefMut for TimedElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.element.deref_mut().as_any_mut()
    }
}

impl RTreeObject for TimedElement {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        self.element.envelope()
    }
}

impl PartialEq for TimedElement {
    fn eq(&self, other: &Self) -> bool {
        self.t_index() == other.t_index() && self.z_index() == other.z_index()
    }
}

impl PartialOrd for TimedElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.z_index() < other.z_index() {
            return Some(Ordering::Less);
        }
        if self.t_index() < other.t_index() {
            return Some(Ordering::Less);
        }
        if self.z_index() == other.z_index() && self.t_index() == other.t_index() {
            return Some(Ordering::Equal);
        }
        Some(Ordering::Greater)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::element::mocks::MockElement;

    #[test]
    fn correct_z_index() {
        let z_index = 5;
        let element = TimedElement::new(MockElement::new(), z_index, 0);
        assert_eq!(element.z_index(), z_index);
    }

    #[test]
    fn correct_t_index() {
        let t_index = 5;
        let element = TimedElement::new(MockElement::new(), 0, t_index);
        assert_eq!(element.t_index(), t_index);
    }

    #[test]
    fn correct_envelope() {
        let envelope = AABB::from_point((3, 2).into());
        let mut mock = MockElement::new();
        mock.expect_envelope().return_const(envelope);

        let element = TimedElement::new(mock, 0, 0);
        assert_eq!(element.envelope(), envelope)
    }

    #[test]
    fn equal_if_t_index_and_z_index_are_equal() {
        let lhs = TimedElement::new(MockElement::new(), 0, 0);
        let rhs = TimedElement::new(MockElement::new(), 0, 0);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn not_equal_if_t_index_different() {
        let lhs = TimedElement::new(MockElement::new(), 0, 0);
        let rhs = TimedElement::new(MockElement::new(), 0, 1);
        assert_ne!(lhs, rhs);
    }

    #[test]
    fn not_equal_if_z_index_different() {
        let lhs = TimedElement::new(MockElement::new(), 0, 0);
        let rhs = TimedElement::new(MockElement::new(), 1, 0);
        assert_ne!(lhs, rhs);
    }

    #[test]
    fn elements_with_higher_t_index_are_sorted_lesser() {
        let elem_earlier = TimedElement::new(MockElement::new(), 0, 0);
        let elem_later = TimedElement::new(MockElement::new(), 0, 1);
        assert!(elem_earlier < elem_later);
        assert!(elem_later > elem_earlier);
    }

    #[test]
    fn elements_with_same_t_index_but_higher_z_index_are_sorted_lesser() {
        let elem_lower = TimedElement::new(MockElement::new(), 0, 0);
        let elem_higher = TimedElement::new(MockElement::new(), 1, 0);
        assert!(elem_lower < elem_higher);
        assert!(elem_higher > elem_lower);
    }

    #[test]
    fn elements_with_same_t_index_and_same_z_index_are_sorted_equal() {
        let elem_1 = TimedElement::new(MockElement::new(), 0, 0);
        let elem_2 = TimedElement::new(MockElement::new(), 0, 0);
        assert!(elem_1 <= elem_2);
        assert!(elem_1 >= elem_2);
    }

    #[test]
    fn can_deref() {
        let mut mock = MockElement::new();
        mock.expect_action().once().return_const(());

        let elem = TimedElement::new(mock, 0, 0);
        let any: &dyn Any = elem.deref();
        any.downcast_ref::<MockElement>().unwrap().action();
    }

    #[test]
    fn can_deref_mut() {
        let mut mock = MockElement::new();
        mock.expect_action_mut().once().return_const(());

        let mut elem = TimedElement::new(mock, 0, 0);
        let any: &mut dyn Any = elem.deref_mut();
        any.downcast_mut::<MockElement>().unwrap().action_mut();
    }
}
