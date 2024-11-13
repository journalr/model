use crate::TimedElement;

pub struct SelectionIter<'a> {
    iter: Box<dyn Iterator<Item = &'a TimedElement> + 'a>,
}

impl<'a> SelectionIter<'a> {
    pub fn new(iterator: Box<dyn Iterator<Item = &'a TimedElement> + 'a>) -> Self {
        SelectionIter { iter: iterator }
    }
}

impl<'a> Iterator for SelectionIter<'a> {
    type Item = &'a TimedElement;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::element::mocks::MockElement;

    #[test]
    fn next_forwards_to_internal_iterator() {
        let timed_element = TimedElement::new(MockElement::new(), 0, 0);
        let vec = vec![&timed_element];

        let mut iter = SelectionIter::new(Box::new(vec.iter().cloned()));
        let element_ref: Option<<SelectionIter as Iterator>::Item> = iter.next();
        assert_eq!(element_ref, Some(&timed_element));
        assert_eq!(iter.next(), None);
    }
}
