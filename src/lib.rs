use std::{array, mem::{MaybeUninit, self}, ptr, ops::{Index, IndexMut}, fmt::{Display, Debug}};

use crate::error::Error;

pub mod list;
pub mod stack;
pub mod queue;
pub mod double_queue;


pub mod error;

#[derive(Clone, Eq)]
pub(crate) struct CyclicList<const SIZE: usize, T: Sized, const WRITE_OVER: bool>{
    pub list: [Option<T>; SIZE],
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) empty: bool,
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> CyclicList<SIZE, T, WRITE_OVER> {
    pub(crate) fn new(list: [Option<T>; SIZE], start: usize, end: usize, empty: bool) -> Self {
        Self{
            list,
            start,
            end,
            empty,
        }
    }
    pub(crate) unsafe fn new_empty(initializer: fn()->T) -> Self{
        let list: [Option<T>; SIZE] = {
            let mut list: [Option<T>; SIZE] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
        
            for dst in &mut list[..] {
                unsafe {
                    ptr::write(
                        dst,
                        Some(initializer())
                    );
                }
            }
        
            unsafe {
                mem::transmute::<_, [Option<T>; SIZE]>(list)
            }
        };

        Self {
            list,
            start: 0,
            end: 0,
            empty: true,
        }
    }

    pub(crate) fn len(&self) -> usize {
        if self.empty {
            return 0;
        }

        if self.end < self.start {
            return SIZE - self.start + self.end + 1
        }

        self.end - self.start + 1
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> Option<&T> {
        unsafe {
            self.list.get_unchecked(index).as_ref()
        }
    }
    
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> Option<&mut T> {
        unsafe {
            self.list.get_unchecked_mut(index).as_mut()
        }
    }

    fn increment_start(&self) -> usize {
        (self.start+1)%SIZE
    }
    fn decrement_start(&self) -> usize {
        match self.start.checked_sub(1) {
            Some(start) => start,
            None => SIZE-1,
        }
    }

    fn increment_end(&self) -> usize {
        (self.end+1)%SIZE
    }
    fn decrement_end(&self) -> usize {
        match self.end.checked_sub(1) {
            Some(end) => end,
            None => SIZE-1,
        }
    }
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> PartialEq for CyclicList<SIZE, T, WRITE_OVER> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut p1 = self.start;
        let mut p2= other.start;
        while (p1, p2) != (self.end, other.end) {
            if self[p1] != other[p2] {
                return false;
            }

            p1 = (p1 + 1) % SIZE;
            p2 = (p2 + 1) % SIZE;
        }

        return true
    }
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> Display for CyclicList<SIZE, T, WRITE_OVER> where T: Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 0 {
            return write!(f, "[]");
        }

        let mut str = String::new();

        let mut pointer = self.start;

        while pointer != self.end {
            str.push_str(&format!("{}, ", self[pointer].as_ref().unwrap()));
            pointer = (pointer + 1) % SIZE;
        }
        
        str.push_str(&format!("{}", self[self.end].as_ref().unwrap()));

        write!(f, "[{}]", str)
    }
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> Debug for CyclicList<SIZE, T, WRITE_OVER> where T: Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CyclicList")
            .field("list", &self.list)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("size", &self.len())
            .finish()
    }
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> Default for CyclicList<SIZE, T, WRITE_OVER> where T: Default {
    fn default() -> Self {
        let list: [Option<T>; SIZE] = array::from_fn(|_| None);
        
        Self {
            list,
            start: 0,
            end: 0,
            empty: true
        }
    }
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> Index<usize> for CyclicList<SIZE, T, WRITE_OVER> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        // if self.len() <= index{
        //     panic!("{:?}", Error::IndexOutOfRange);
        // }

        &self.list[index]
    }
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> IndexMut<usize> for CyclicList<SIZE, T, WRITE_OVER> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // if self.len() <= index{
        //     panic!("{:?}", Error::IndexOutOfRange);
        // }

        &mut self.list[index]
    }
}

impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> From<[T; LIST_SIZE]> for CyclicList<LIST_SIZE, T, WRITE_OVER> {
    fn from(value: [T; LIST_SIZE]) -> Self {
        let mut list : [Option<T>; LIST_SIZE] = array::from_fn(|_| None);

        value.into_iter()
            .map(|val| Some(val))
            .enumerate()
            .for_each(|(index, val)| list[index] = val);


        CyclicList{
            list,
            start: 0,
            end: LIST_SIZE - 1,
            empty: false,
        }
    }
}

impl<const LIST_SIZE: usize, T> From<CyclicList<LIST_SIZE, T, true>> for CyclicList<LIST_SIZE, T, false> {
    fn from(value: CyclicList<LIST_SIZE, T, true>) -> Self {
        Self::new(value.list, value.start, value.end, value.empty)
    }
}

impl<const LIST_SIZE: usize, T> From<CyclicList<LIST_SIZE, T, false>> for CyclicList<LIST_SIZE, T, true> {
    fn from(value: CyclicList<LIST_SIZE, T, false>) -> Self {
        Self::new(value.list, value.start, value.end, value.empty)
    }
}