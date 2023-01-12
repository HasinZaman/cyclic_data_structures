use super::List;

pub struct Iter<'a, const SIZE: usize, T, const WRITE_OVER: bool> {
    pointer: usize,
    list: &'a List<SIZE, T, WRITE_OVER>,
}

impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> Iter<'a, SIZE, T, WRITE_OVER> {
    pub fn new(list: &'a List<SIZE, T, WRITE_OVER>) -> Self {
        Self {
            pointer: 0,
            list: &list
        }
    }
}

impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> Iterator for Iter<'a, SIZE, T, WRITE_OVER> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        if self.pointer > self.list.len() {
            return None;
        }

        let tmp = self.list.get(self.pointer as isize);

        self.pointer+=1;

        tmp
    }
}

impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> ExactSizeIterator for Iter<'a, SIZE, T, WRITE_OVER> {
    fn len(&self) -> usize {
        self.list.len() - self.pointer
    }
}
