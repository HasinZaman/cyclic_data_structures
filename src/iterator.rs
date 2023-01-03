use super::CyclicList;

pub struct ListIter<'a, const SIZE: usize, T: Sized, const WRITEOVER: bool> {
    pointer: usize,
    list: &'a CyclicList<SIZE, T, WRITEOVER>,
}

impl<'a, const SIZE: usize, T: Sized, const WRITEOVER: bool> ListIter<'a, SIZE, T, WRITEOVER> {
    pub fn new(list: &'a CyclicList<SIZE, T, WRITEOVER>) -> Self {
        Self {
            pointer: 0,
            list: list
        }
    }
}

impl<'a, const SIZE: usize, T: Sized, const WRITEOVER: bool> Iterator for ListIter<'a, SIZE, T, WRITEOVER> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        let tmp = self.list.get(self.pointer).ok();

        self.pointer+=1;

        tmp
    }
}

impl<'a, const SIZE: usize, T: Sized, const WRITEOVER: bool> ExactSizeIterator for ListIter<'a, SIZE, T, WRITEOVER> {
    fn len(&self) -> usize {
        self.list.len() - self.pointer
    }
}
