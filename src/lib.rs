use std::{array, mem::{MaybeUninit, self}, ptr, ops::{Index, IndexMut}, fmt::{Display, Debug}};

use iterator::ListIter;
use list::List;

pub mod iterator;
mod list;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Error{
    IndexOutOfRange,
    Overflow
}

#[derive(Clone)]
pub struct CyclicList<const SIZE: usize, T: Sized, const WRITEOVER: bool>{
    list: [T; SIZE],
    start: usize,
    end: usize,

}

impl<const SIZE: usize, T, const WRITEOVER: bool> CyclicList<SIZE, T, WRITEOVER> {
    pub unsafe fn new(initializer: fn()->T) -> Self{
        let list: [T; SIZE] = {
            let mut list: [T; SIZE] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
        
            for dst in &mut list[..] {
                unsafe {
                    ptr::write(
                        dst,
                        initializer()
                    );
                }
            }
        
            unsafe {
                mem::transmute::<_, [T; SIZE]>(list)
            }
        };

        Self {
            list,
            start: 0,
            end: 0
        }
    }

    fn increment_start(&self) -> usize {
        (self.start+1)%SIZE
    }
    
    fn increment_end(&self) -> usize {
        (self.end+1)%SIZE
    }
    fn decrement_end(&self) -> usize {
        if let Some(val) = self.end.checked_sub(1) {
            return val
        }
        SIZE-1
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> PartialEq for CyclicList<SIZE, T, WRITEOVER> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {

        if self.len() != other.len() {
            return false;
        }

        let tmp = self.iter()
            .zip(other.iter())
            .all(|(l1, l2)| l1 == l2);

        return tmp;
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> List<T> for CyclicList<SIZE, T, WRITEOVER> {
    fn len(&self) -> usize {
        if self.end < self.start {
            return SIZE - self.start + self.end
        }

        self.end - self.start
    }

    fn push(&mut self, elem: T) -> Result<&Self, Error> {
        if self.len() == SIZE && !WRITEOVER {
            return Err(Error::Overflow)
        }

        if self.len() == 0 {
            self.list[self.end] = elem;

            self.end = self.increment_end();

            return Ok(self)
        }
        
        self.end = self.increment_end();
        
        //pushing new value
        self.list[self.end] = elem;

        //if end pointer loops over to start pointer
        if self.start == self.end {
            //dropping first value & incrementing start pointer
            self.start = self.increment_start();
        }

        Ok(self)
    }

    fn get(&self, index: usize) -> Result<&T, Error> {
        if self.len() <= index{
            return Err(Error::IndexOutOfRange)
        }

        Ok(&self.list[(self.start + index) % SIZE])
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut T, Error> {
        if self.len() <= index{
            return Err(Error::IndexOutOfRange)
        }

        Ok(&mut self.list[(self.start + index) % SIZE])
    }

    fn pop(&mut self) -> Option<&mut T> {
        if self.len() == 0 {
            return None
        }
        let pop_index = self.end;

        self.end = self.decrement_end();

        Some(&mut self.list[pop_index])
    }

    fn iter(&self) -> ListIter<T, Self> where Self: Sized {
        ListIter::new(self)
    }

    fn iter_mut(&mut self) -> ListIter<T, Self> where Self: Sized {
        ListIter::new(self)
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> Display for CyclicList<SIZE, T, WRITEOVER> where T: Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = (&self).iter();

        let tmp = iter.fold(String::from(""), |acc, val| format!("{},{}", acc, val));

        write!(f, "[{}]", tmp)
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> Debug for CyclicList<SIZE, T, WRITEOVER> where T: Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CyclicList").field("list", &self.list).field("start", &self.start).field("end", &self.end).finish()
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> Default for CyclicList<SIZE, T, WRITEOVER> where T: Default {
    fn default() -> Self {
        let list: [T; SIZE] = array::from_fn(|_| T::default());
        
        Self {
            list,
            start: 0,
            end: 0
        }
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> Index<usize> for CyclicList<SIZE, T, WRITEOVER> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.len() <= index{
            panic!("{:?}", Error::IndexOutOfRange);
        }

        &self.list[(self.start + index) % SIZE]
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> IndexMut<usize> for CyclicList<SIZE, T, WRITEOVER> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.len() <= index{
            panic!("{:?}", Error::IndexOutOfRange);
        }

        &mut self.list[(self.start + index) % SIZE]
    }
}
