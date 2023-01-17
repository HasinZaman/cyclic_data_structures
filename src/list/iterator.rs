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

        if self.len() <= 0{
            return None;
        }

        let tmp = Some(&self.list[self.pointer]);

        self.pointer+=1;

        tmp
    }
}

impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> ExactSizeIterator for Iter<'a, SIZE, T, WRITE_OVER> {
    fn len(&self) -> usize {
        self.list.len() - self.pointer
    }
}

// impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> DoubleEndedIterator for Iter<'a, SIZE, T, WRITE_OVER> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         match self.pointer.checked_sub(1) {
//             Some(val) => {
//                 self.pointer = val;

//                 Some(&self.list[self.pointer])
//             },
//             None => None,
//         }
//     }
// }

// pub struct IterMut<'a, const SIZE: usize, T, const WRITE_OVER: bool> {
//     pointer: usize,
//     list: &'a mut List<SIZE, T, WRITE_OVER>,
// }

// impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> IterMut<'a, SIZE, T, WRITE_OVER> {
//     pub fn new(list: &'a mut List<SIZE, T, WRITE_OVER>) -> Self {
//         Self {
//             pointer: 0,
//             list: list
//         }
//     }
// }

// impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> Iterator for IterMut<'a, SIZE, T, WRITE_OVER> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.len() <= 0{
//             return None;
//         }

        

//         let tmp = Some(&mut self.list[self.pointer]);

//         self.pointer+=1;

//         tmp
//     }
// }

// impl<'a, const SIZE: usize, T, const WRITE_OVER: bool> ExactSizeIterator for IterMut<'a, SIZE, T, WRITE_OVER> {
//     fn len(&self) -> usize {
//         self.list.len() - self.pointer
//     }
// }
