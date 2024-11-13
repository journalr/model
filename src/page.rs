use crate::{Element, Point};

use crate::elements::timed_element::TimedElement;
use crate::iterator::SelectionIter;
use rstar::{Envelope, RTree, RTreeObject, SelectionFunction, AABB};

#[derive(Default)]
pub struct Page {
    elements: RTree<TimedElement>,
    t_index: u128,
}

impl Page {
    pub fn insert<T: Element + 'static>(&mut self, element: T, z_index: i32) {
        self.t_index += 1;
        self.elements
            .insert(TimedElement::new(element, z_index, self.t_index));
    }

    pub fn iter(&self) -> SelectionIter {
        SelectionIter::new(Box::new(self.elements.iter()))
    }

    pub fn extract(
        &mut self,
        selection_function: SelectByAddressFunction<TimedElement>,
    ) -> Option<TimedElement> {
        self.elements
            .drain_with_selection_function(selection_function)
            .next()
    }

    pub fn locate_in_envelope(&self, envelope: AABB<Point>) -> SelectionIter {
        SelectionIter::new(Box::new(self.elements.locate_in_envelope(&envelope)))
    }

    pub fn len(&self) -> usize {
        self.elements.size()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.size() == 0
    }
}

pub struct SelectByAddressFunction<T>
where
    T: RTreeObject,
{
    envelope: T::Envelope,
    element_address: *const T,
}

impl<T> SelectByAddressFunction<T>
where
    T: RTreeObject,
{
    pub fn new(envelope: T::Envelope, element_address: &T) -> Self {
        Self {
            envelope,
            element_address,
        }
    }
}

impl<T> SelectionFunction<T> for SelectByAddressFunction<T>
where
    T: RTreeObject,
{
    fn should_unpack_parent(&self, parent_envelope: &T::Envelope) -> bool {
        parent_envelope.contains_envelope(&self.envelope)
    }

    fn should_unpack_leaf(&self, leaf: &T) -> bool {
        core::ptr::eq(self.element_address, leaf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::rectangle::Rectangle;
    use crate::{Line, Point};
    use std::ops::Deref;

    #[test]
    fn add_element() {
        let mut page = Page::default();

        let lower = Point { x: 0, y: 0 };
        let upper = Point { x: 1, y: 1 };
        let element = Rectangle::new(lower, upper);
        page.insert(element, 0);

        assert_eq!(page.len(), 1);
    }

    #[test]
    fn can_iterate() {
        let mut page = Page::default();

        let lower = Point { x: 0, y: 0 };
        let upper = Point { x: 1, y: 1 };
        let element = Rectangle::new(lower, upper);
        page.insert(element, 15);

        let rect = page.iter().next().unwrap();
        assert_eq!(rect.z_index(), 15);
    }

    #[test]
    fn can_extract_element() {
        let mut page = Page::default();
        let points = vec![(1, 2), (2, 3)];
        let line = Line::from_iter(points.iter());
        page.insert(line.clone(), 0);

        let stored_line = page.iter().next().unwrap();
        let selector = SelectByAddressFunction::new(stored_line.envelope(), stored_line);
        let extracted_line = page.extract(selector).unwrap();
        assert_eq!(page.len(), 0);
        assert_eq!(
            *extracted_line.deref().downcast_ref::<Line>().unwrap(),
            line
        );
    }

    #[test]
    fn locate_in_envelope_only_locates_correct_elements() {
        // Given a page
        let mut page = Page::default();

        // With one element outside the selection
        let points_outside = vec![(1, 2), (2, 3)];
        let line_outside = Line::from_iter(points_outside.iter());
        page.insert(line_outside.clone(), 0);

        // And one element partially inside the selection
        let points_intersecting = vec![(1, 2), (3, 5)];
        let line_intersecting = Line::from_iter(points_intersecting.iter());
        page.insert(line_intersecting.clone(), 0);

        // And one element on the boundary but within the selection
        let points_on_boundary = vec![(2, 4), (3, 5)];
        let line_on_boundary = Line::from_iter(points_on_boundary.iter());
        page.insert(line_on_boundary.clone(), 0);

        // And one element inside the selection
        let points_inside = vec![(3, 5), (6, 7)];
        let line_inside = Line::from_iter(points_inside.iter());
        page.insert(line_inside.clone(), 0);

        // When selecting elements
        let mut selection =
            page.locate_in_envelope(AABB::from_corners((2, 4).into(), (7, 8).into()));

        // Then we get the elements that are on the boundary and within the selection envelope
        assert_eq!(
            selection.next().unwrap().downcast_ref::<Line>().unwrap(),
            &line_inside
        );
        assert_eq!(
            selection.next().unwrap().downcast_ref::<Line>().unwrap(),
            &line_on_boundary
        );
    }

    #[test]
    fn can_get_len() {
        let mut page = Page::default();
        assert_eq!(page.len(), 0);
        let points = vec![(1, 2), (2, 3)];
        let line = Line::from_iter(points.iter());
        page.insert(line.clone(), 0);
        assert_eq!(page.len(), 1);
    }

    #[test]
    fn can_check_emptiness() {
        let mut page = Page::default();
        assert_eq!(page.is_empty(), true);
        let points = vec![(1, 2), (2, 3)];
        let line = Line::from_iter(points.iter());
        page.insert(line.clone(), 0);
        assert_eq!(page.is_empty(), false);
    }
}
