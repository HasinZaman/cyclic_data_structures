use std::marker::PhantomData;

use crate::list::List;

pub struct ListIter<'a, T, L> where L: 'a + List<T> {
    pointer: usize,
    list: L,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, L> ListIter<'a, T, L> where L: 'a + List<T> {
    pub fn new(list: L) -> Self {
        Self {
            pointer: 0,
            list: list,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, L> Iterator for ListIter<'a, T, L> where L: 'a + List<T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        let tmp = self.list.get(self.pointer).ok();

        self.pointer+=1;

        tmp
    }
}

impl<'a, T, L> ExactSizeIterator for ListIter<'a, T, L> where L: 'a + List<T> {
    fn len(&self) -> usize {
        self.list.len() - self.pointer
    }
}
