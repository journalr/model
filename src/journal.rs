use crate::Page;

use std::collections::linked_list::{Iter, IterMut};
use std::collections::LinkedList;

pub struct Journal {
    pages: LinkedList<Page>,
}

impl Journal {
    pub fn insert(&mut self, at: usize, page: Page) {
        let mut tail = self.pages.split_off(at);
        self.pages.push_back(page);
        self.pages.append(&mut tail);
    }

    pub fn push_back(&mut self, page: Page) {
        self.pages.push_back(page);
    }

    pub fn remove(&mut self, at: usize) -> Option<Page> {
        let mut tail = self.pages.split_off(at);
        let page = tail.pop_front();
        self.pages.append(&mut tail);
        page
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    pub fn iter(&self) -> Iter<Page> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Page> {
        self.into_iter()
    }
}

impl Default for Journal {
    fn default() -> Self {
        let mut journal = Journal {
            pages: LinkedList::new(),
        };
        journal.pages.push_back(Page::default());
        journal
    }
}

impl<'a> IntoIterator for &'a Journal {
    type Item = &'a Page;
    type IntoIter = Iter<'a, Page>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.iter()
    }
}

impl<'a> IntoIterator for &'a mut Journal {
    type Item = &'a mut Page;
    type IntoIter = IterMut<'a, Page>;

    fn into_iter(self) -> Self::IntoIter {
        self.pages.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Line;

    #[test]
    fn has_by_default_one_empty_page() {
        let journal = Journal::default();
        assert_eq!(journal.len(), 1);
        assert_eq!(journal.iter().next().unwrap().len(), 0)
    }

    #[test]
    fn can_check_emptiness() {
        let mut journal = Journal::default();
        assert_eq!(journal.is_empty(), false);
        journal.remove(0);
        assert_eq!(journal.is_empty(), true);
    }

    #[test]
    fn can_push_back_page() {
        let mut journal = Journal::default();
        journal.push_back(Page::default());
        assert_eq!(journal.len(), 2);
    }

    #[test]
    fn can_insert_page_at_front() {
        let mut journal = Journal::default();
        let mut page = Page::default();
        page.insert::<Line>(Line::default(), 0);
        journal.insert(0, page);
        assert_eq!(journal.len(), 2);
    }

    #[test]
    fn can_insert_page_in_between_existing_pages() {
        let mut journal = Journal::default();
        journal.push_back(Page::default());
        journal.push_back(Page::default());

        let mut page = Page::default();
        page.insert(Line::from_iter(vec![(0, 1)].iter()), 0);
        journal.insert(1, page);
        assert_eq!(journal.len(), 4);
        assert_eq!(journal.iter().nth(0).unwrap().len(), 0);
        assert_eq!(journal.iter().nth(1).unwrap().len(), 1);
        assert_eq!(journal.iter().nth(2).unwrap().len(), 0);
    }

    #[test]
    fn can_iterate_and_modify_pages() {
        let mut journal = Journal::default();
        let first_page = journal.iter_mut().next().unwrap();
        first_page.insert(Line::from_iter(vec![(0, 1)].iter()), 0);
        assert_eq!(journal.iter().next().unwrap().len(), 1);
    }
}
