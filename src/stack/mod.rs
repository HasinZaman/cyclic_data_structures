//! The list module contains a series of structs to create stacks and their utility functionalities using cyclic lists.

use std::{fmt::{Display, Debug}, collections::LinkedList};

use crate::{list::List, error::Error};

/// `Stack` is the `struct` used to define the state of a stack using cyclic [`List`]
/// 
/// # Generics
/// List types are derived using 3 generics.
/// 
/// 1. `const SIZE: usize`
/// 
/// SIZE is a generic constant [^note] that defines the maximum size of the stack
/// 
/// 2. `T: Sized`
/// 
/// T is the type of element stored in the stack
/// 
/// 3. `const WRITE_OVER: bool>`
/// 
/// 
/// # Creating Stacks
/// 
/// Stacks can be created in a couple of ways.
/// 
/// 1. Empty Stack
/// 
/// Empty Stack are created using the [`Default`] trait implementation for List.
/// 
/// ```
/// # use cyclic_data_types::stack::Stack;
/// # const SIZE: usize = 5;
/// let list: Stack<SIZE, i64, false> = Stack::default();
/// 
/// assert_eq!(list.len(), 0);
/// ```
/// 
/// WRITE_OVER is a generic constant [^note] that is used to determine if elements should be over written on overflow
/// [note]: [Generic Constraints](https://rust-lang.github.io/rfcs/2000-const-generics.html)
#[derive(Default, PartialEq)]
pub struct Stack<const SIZE: usize, T, const WRITE_OVER: bool> {
    stack: List<SIZE, T, WRITE_OVER>
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> Stack<SIZE, T, WRITE_OVER> {
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn push(&mut self, elem: T) -> Result<&mut Self, Error>{
        match self.stack.push_back(elem){
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.stack.get(-1)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.remove_back()
    }
    pub fn read(&self, index: usize) -> Result<&T, Error> {
        let len = self.stack.len();

        if len <= index {
            return Err(Error::IndexOutOfRange)
        }

        return Ok(&self.stack[-1 * index as isize - 1])
    }
}

impl<const S: usize, T, const W: bool> Display for Stack<S, T, W> where T: Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stack)
    }
}

impl<const S: usize, T, const W: bool> Debug for Stack<S, T, W> where T: Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field("", &self.stack)
            .finish()
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Vec<T>> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if STACK_SIZE < value.len() && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        let mut stack = Self::default();

        for element in value{
            if let Err(err) = stack.push(element) {
                return Err(err);
            }
        }

        Ok(stack)
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<LinkedList<T>> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: LinkedList<T>) -> Result<Self, Self::Error> {
        if STACK_SIZE < value.len() && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        let mut stack = Self::default();

        for element in value{
            stack.push(element).unwrap();
        }

        Ok(stack)
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> FromIterator<T> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Default {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut list :Stack<STACK_SIZE, T, WRITE_OVER> = Stack::default();

        for elem in iter {
            list.push(elem).unwrap();
        }

        list
    }
}

//generic generator
impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Box<dyn Iterator<Item = T>>> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Box<dyn Iterator<Item = T>>) -> Result<Self, Self::Error> {
        let mut list = Self::default();

        for element in value{
            if let Err(err) = list.push(element) {
                return Err(err);
            }
        }

        Ok(list)
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> From<[T; STACK_SIZE]> for Stack<STACK_SIZE, T, WRITE_OVER> {
    fn from(value: [T; STACK_SIZE]) -> Self {
        Stack{
            stack: value.into(),
        }
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> From<List<STACK_SIZE, T, WRITE_OVER>> for Stack<STACK_SIZE, T, WRITE_OVER>{
    fn from(value: List<STACK_SIZE, T, WRITE_OVER>) -> Self {
        Self {
            stack: value 
        }
    }
}

impl<const STACK_SIZE: usize, T> From<Stack<STACK_SIZE, T, true>> for Stack<STACK_SIZE, T, false> {
    fn from(value: Stack<STACK_SIZE, T, true>) -> Self {
        Self{
            stack: value.stack.into(),
        }
    }
}

impl<const STACK_SIZE: usize, T> From<Stack<STACK_SIZE, T, false>> for Stack<STACK_SIZE, T, true> {
    fn from(value: Stack<STACK_SIZE, T, false>) -> Self {
        Self{
            stack: value.stack.into(),
        }
    }
}