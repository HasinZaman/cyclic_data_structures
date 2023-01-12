use std::{ops::{Index, IndexMut}, fmt::{Display, Debug}, collections::LinkedList};
use std::convert::TryFrom;

use crate::{CyclicList, error::Error};

use self::iterator::Iter;

pub mod iterator;

#[cfg(test)]
mod tests;

#[derive(Eq, PartialEq, Default)]
pub struct List<const SIZE: usize, T: Sized, const WRITE_OVER: bool>{
    list: CyclicList<SIZE, T, WRITE_OVER>
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> List<SIZE, T, WRITE_OVER> {
    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn insert_at(&mut self, elem: T, index: usize) -> Result<&mut Self, Error> where T: Clone {
        if self.len()+1 > SIZE && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        if self.len() < index  {
            return Err(Error::IndexOutOfRange)
        }

        if self.len() == index {
            return self.push_back(elem);
        }

        self.list.end = self.list.increment_end();

        for i in (index..self.len()).rev() {
            self.list[i] = self.list[i-1].clone();
        }

        //adding value at index
        self.list[index] = elem;

        Ok(self)
    }

    pub fn push_back(&mut self, elem: T) -> Result<&mut Self, Error> {
        if self.len() + 1 > SIZE && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        match (self.len(), self.list.empty) {
            (0, true) => {
                self.list.empty = false;
            },
            (_val, true) => {
                panic!("How did i break the list");
            }
            _ => {
                self.list.end = self.list.increment_end();

                //if end pointer loops over to start pointer
                if self.list.start == self.list.end {
                    //dropping first value & incrementing start pointer
                    self.list.start = self.list.increment_start();
                }
            },
        }
        
        //pushing new value
        let end = self.list.end;

        self.list[end] = elem;

        Ok(self)
    }

    pub fn push_front(&mut self, elem: T) -> Result<&mut Self, Error> {
        if self.len() + 1 > SIZE && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        match (self.len(), self.list.empty) {
            (0, true) => {
                self.list.empty = false;
            },
            (_val, true) => {
                panic!("How did i break the list");
            }
            _ => {
                self.list.start = self.list.decrement_start();

                //if start pointer loops over to end pointer
                if self.list.start == self.list.end {
                    self.list.end = self.list.decrement_end();
                }
            },
        }
        self.list[0] = elem;

        Ok(self)
    }

    pub fn get(&self, index: isize) -> &T {
        let list_index : usize;

        let len = self.len() as isize;
        if 0 > index {
            list_index = (len - ((-1 * index) % len)).try_into().unwrap();
        }
        else {
            list_index = (index % len).try_into().unwrap();
        }

        &self.list[list_index]
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        todo!()
    }

    pub fn get_mut(&mut self, index: isize) -> &mut T {
        let list_index : usize;

        let len = self.len() as isize;

        if 0 > index {
            list_index = (len - ((-1 * index) % len)).try_into().unwrap();
        }
        else {
            list_index = (index % len).try_into().unwrap();
        }

        &mut self.list[list_index]
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &T {
        todo!()
    }

    pub fn remove_back(&mut self) -> Option<&mut T> {
        if self.len() == 0 {
            return None
        }
        let pop_index = self.list.end;

        if self.list.end != self.list.start {
            self.list.end = self.list.decrement_end();
        }
        else {
            self.list.empty = true;
        }
        

        Some(&mut self.list[pop_index])
    }

    pub fn remove_front(&mut self) -> Option<&mut T> {
        if self.len() == 0 {
            return None
        }

        let pop_index = self.list.start;

        if self.list.end != self.list.start {
            self.list.start = self.list.increment_start();
        }
        else {
            self.list.empty = true;
        }
        

        Some(&mut self.list[pop_index])
    }

    pub fn remove_at(&mut self, index: usize) -> Result<Option<T>, Error> where T: Clone {
        if self.len() <= index  {
            return Err(Error::IndexOutOfRange)
        }

        let val = Some(self[index].clone());

        if index == 0 {
            self.list.start = self.list.increment_start();

            return Ok(val)
        }
        
        for i in index..(self.len()-1) {
            self.list[i] = self.list[i+1].clone();
        }

        self.list.end = self.list.increment_end();

        Ok(val)
    }

    pub fn iter(&self) -> Iter<SIZE, T, WRITE_OVER> where Self: Sized {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> Iter<SIZE, T, WRITE_OVER> where Self: Sized {
        Iter::new(self)
    }

}

impl<const S: usize, T, const W: bool> Display for List<S, T, W> where T: Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.list)
    }
}

impl<const S: usize, T, const W: bool> Debug for List<S, T, W> where T: Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("", &self.list)
            .finish()
    }
}

impl<const S: usize, T, const W: bool> Index<usize> for List<S, T, W> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

impl<const S: usize, T, const W: bool> IndexMut<usize> for List<S, T, W> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.list[index]
    }
}

impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Vec<T>> for List<LIST_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() < LIST_SIZE && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        let mut list = Self::default();

        for element in value{
            list.push_front(element).unwrap();
        }

        Ok(list)
    }
}

impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<LinkedList<T>> for List<LIST_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: LinkedList<T>) -> Result<Self, Self::Error> {
        if value.len() < LIST_SIZE && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        let mut list = Self::default();

        for element in value{
            list.push_front(element).unwrap();
        }

        Ok(list)
    }
}

//generic generator
impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Box<dyn Iterator<Item = T>>> for List<LIST_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Box<dyn Iterator<Item = T>>) -> Result<Self, Self::Error> {
        let mut list = Self::default();

        for element in value{
            if let Err(err) = list.push_front(element) {
                return Err(err);
            }
        }

        Ok(list)
    }
}

impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> From<[T; LIST_SIZE]> for List<LIST_SIZE, T, WRITE_OVER> {
    fn from(value: [T; LIST_SIZE]) -> Self {
        List{
            list: value.into(),
        }
    }
}

impl<const LIST_SIZE: usize, T> From<List<LIST_SIZE, T, true>> for List<LIST_SIZE, T, false> {
    fn from(value: List<LIST_SIZE, T, true>) -> Self {
        Self{
            list: value.list.into(),
        }
    }
}

impl<const LIST_SIZE: usize, T> From<List<LIST_SIZE, T, false>> for List<LIST_SIZE, T, true> {
    fn from(value: List<LIST_SIZE, T, false>) -> Self {
        Self{
            list: value.list.into(),
        }
    }
}