use crate::{iterator::{ListIter}, Error};


pub trait List<T>{
    fn len(&self) -> usize;
    fn insert_at(&mut self, elem: T, index: usize) -> Result<&mut Self, Error> where T: Clone;
    fn push(&mut self, elem: T) -> Result<&mut Self, Error>;
    fn get(&self, index: usize) -> Result<&T, Error>;
    fn get_mut(&mut self, index: usize) -> Result<&mut T, Error>;
    fn pop(&mut self) -> Option<&mut T>;
    fn remove_front(&mut self) -> Option<&mut T>;
    fn remove_at(&mut self, index: usize) -> Result<Option<T>, Error> where T: Clone;
    fn iter(&self) -> ListIter<T, Self> where Self: Sized;
    fn iter_mut(&mut self) -> ListIter<T, Self> where Self: Sized;
}