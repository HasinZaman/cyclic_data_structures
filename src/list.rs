use crate::{iterator::{ListIter}, Error};


pub trait List<T>{
    fn len(&self) -> usize;
    fn push(&mut self, elem: T) -> Result<&Self, Error>;
    fn get(&self, index: usize) -> Result<&T, Error>;
    fn get_mut(&mut self, index: usize) -> Result<&mut T, Error>;
    fn pop(&mut self) -> Option<&mut T>;
    fn iter(&self) -> ListIter<T, Self> where Self: Sized;
    fn iter_mut(&mut self) -> ListIter<T, Self> where Self: Sized;
}