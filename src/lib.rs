use std::{array, mem::{MaybeUninit, self}, ptr, ops::{Index, IndexMut}, fmt::{Display, Debug}};

pub use iterator::ListIter;
pub use list::List;

pub mod iterator;
pub mod list;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub enum Error{
    IndexOutOfRange,
    Overflow,
    InvalidSize
}

#[derive(Clone)]
pub struct CyclicList<const SIZE: usize, T: Sized, const WRITEOVER: bool>{
    list: [T; SIZE],
    start: usize,
    end: usize,
    empty: bool,
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
            end: 0,
            empty: true,
        }
    }

    fn increment_start(&self) -> usize {
        (self.start+1)%SIZE
    }
    fn decrement_start(&self) -> usize {
        if let Some(val) = self.start.checked_sub(1) {
            return val
        }
        SIZE-1
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
            .all(|(l1, l2)| {
                l1 == l2
            });

        return tmp;
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> List<T> for CyclicList<SIZE, T, WRITEOVER> {
    fn len(&self) -> usize {
        if self.empty {
            return 0;
        }

        if self.end < self.start {
            return SIZE - self.start + self.end + 1
        }

        self.end - self.start + 1
    }

    fn insert_at(&mut self, elem: T, index: usize) -> Result<&mut Self, Error> where T: Clone {
        //todo implement push from beginning and end
        if self.len()+1 <= SIZE && WRITEOVER {
            return Err(Error::Overflow)
        }

        if self.len() < index  {
            return Err(Error::IndexOutOfRange)
        }

        if self.len() == index {
            return self.push(elem);
        }

        if index == 0 {
            self.start = self.decrement_start();

            if self.start == self.end {
                self.end = self.decrement_end();
            }

            self.list[self.start] = elem;

            return Ok(self)
        }

        self.end = self.increment_end();

        if self.start == self.end {
            self.start = self.increment_start();
        }

        //shift everything from index to end
        for i in (index..self.len()).rev() {
            self.list[i] = self.list[i-1].clone();
        }

        //adding value at index
        self.list[index] = elem;

        Ok(self)
    }

    fn push(&mut self, elem: T) -> Result<&mut Self, Error> {
        if self.len() + 1 > SIZE && !WRITEOVER {
            return Err(Error::Overflow)
        }

        match (self.len(), self.empty) {
            (0, true) => {
                self.empty = false;
            },
            (_val, true) => {
                panic!("How did i break the list");
            }
            _ => {
                self.end = self.increment_end();

                //if end pointer loops over to start pointer
                if self.start == self.end {
                    //dropping first value & incrementing start pointer
                    self.start = self.increment_start();
                }
            },
        }
        
        //pushing new value
        self.list[self.end] = elem;


        

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

        if self.end != self.start {
            self.end = self.decrement_end();
        }
        else {
            self.empty = true;
        }
        

        Some(&mut self.list[pop_index])
    }

    fn remove_front(&mut self) -> Option<&mut T> {
        if self.len() == 0 {
            return None
        }

        let pop_index = self.start;

        if self.end != self.start {
            self.start = self.increment_start();
        }
        else {
            self.empty = true;
        }
        

        Some(&mut self.list[pop_index])
    }

    fn remove_at(&mut self, index: usize) -> Result<Option<T>, Error> where T: Clone {
        if self.len() <= index  {
            return Err(Error::IndexOutOfRange)
        }

        let val = Some(self[index].clone());

        if index == 0 {
            self.start = self.increment_start();

            return Ok(val)
        }
        
        for i in index..(self.len()-1) {
            self.list[i] = self.list[i+1].clone();
        }

        self.end = self.increment_end();

        Ok(val)
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
        f.debug_struct("CyclicList").field("list", &self.list).field("start", &self.start).field("end", &self.end).field("size", &self.len()).finish()
    }
}

impl<const SIZE: usize, T, const WRITEOVER: bool> Default for CyclicList<SIZE, T, WRITEOVER> where T: Default {
    fn default() -> Self {
        let list: [T; SIZE] = array::from_fn(|_| T::default());
        
        Self {
            list,
            start: 0,
            end: 0,
            empty: true
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

impl <const SIZE: usize, T, const WRITEOVER: bool> TryFrom<Vec<T>>  for CyclicList<SIZE, T, WRITEOVER> where T: Default{
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if SIZE < value.len() {
            return Err(Error::InvalidSize)
        }

        let mut list: CyclicList<SIZE, T, WRITEOVER> = CyclicList::default();

        for val in value {
            if let Err(err) = list.push(val) {
                return Err(err)
            }
        }

        Ok(list)
    }
}

impl <const SIZE: usize, T, const WRITEOVER: bool> From<[T; SIZE]> for CyclicList<SIZE, T, WRITEOVER> where T: Default + Clone{
    fn from(value: [T; SIZE]) -> Self {
        let mut list = Self::default();

        for i in 0..value.len() {
            let _ =list.push(value[i].clone());
        }

        list
    }
}


impl<const SIZE: usize, T, const WRITEOVER: bool> Into<Vec<T>> for CyclicList<SIZE, T, WRITEOVER> where T: Clone{
    fn into(self) -> Vec<T> {

        self.iter()
            .map(|val| val.clone())
            .collect()
    }
}