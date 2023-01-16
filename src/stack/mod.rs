//! The list module contains a series of structs to create stacks and their utility functionalities using cyclic lists.
//! 
//! It is recommended to use [`Vec`] over [`Stack`] for most applications. As [`Vec`] has better - if not similar performance to the [`Stack`]. It is therefore, [`Stack`] should only be used when the stack should strictly be limited to a given size and or life convince of life features provided by the [`Stack`].

use std::{fmt::{Display, Debug}, collections::LinkedList, ops::{Deref, DerefMut}};

use crate::{list::List, error::Error};

#[cfg(test)]
mod tests;

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
/// let stack: Stack<SIZE, i64, false> = Stack::default();
/// 
/// assert_eq!(stack.len(), 0);
/// ```
/// 
/// 2. From Array
/// 
/// Stack can also be derived from arrays. The maximum size of the stack is the same size as the array. This is done using the [`From<[SIZE; T]`] trait implementation for List.
/// 
/// ```
/// # use cyclic_data_types::stack::Stack;
/// # const SIZE: usize = 5;
/// let stack: Stack<SIZE, i64, false> = [1i64,2i64,3i64,4i64,5i64].into();
/// #
/// # assert_eq!(stack.len(), 5);
/// # assert_eq!(stack.read(4).unwrap(), &1i64);
/// # assert_eq!(stack.read(3).unwrap(), &2i64);
/// # assert_eq!(stack.read(2).unwrap(), &3i64);
/// # assert_eq!(stack.read(1).unwrap(), &4i64);
/// # assert_eq!(stack.read(0).unwrap(), &5i64);
/// ```
/// 
/// 3. From Vectors, Linked Lists and Iterators
/// 
/// Since collections (Vectors, Linked Lists and Iterators) cannot guarantee a size at compile time - the conversion is not always guaranteed to succeed. This occurs when collection is larger than the stack variant. As a result, the new stack cannot be created without resulting in a [`Error::Overflow`]. This can be resolved by either making sure the collection is at max the same size as the stack variant or the cyclic list variant permits `WRITE_OVER`.
/// 
/// Therefore, the [`TryFrom`] trait implementation of stack must be used.
/// 
/// Example of a successful conversion
/// ```
/// # use cyclic_data_types::stack::Stack;
/// const SIZE: usize = 5;
/// let stack: Stack<SIZE, i64, false> = Stack::try_from(vec![1i64,2i64,3i64,4i64,5i64])
///     .unwrap();
/// #
/// # assert_eq!(stack.len(), 5);
/// # assert_eq!(stack, [1i64,2i64,3i64,4i64,5i64].into());
/// ```
/// ```
/// # use cyclic_data_types::stack::Stack;
/// const SIZE: usize = 5;
/// let stack: Stack<SIZE, i64, true> = vec![1i64,2i64,3i64,4i64,5i64,6i64].try_into()
///     .unwrap();
/// #
/// # assert_eq!(stack.len(), 5);
/// # assert_eq!(stack, [2i64,3i64,4i64,5i64,6i64].into());
/// ```
/// Example of a failed conversion
/// ```
/// # use cyclic_data_types::stack::Stack;
/// # use cyclic_data_types::error::Error;
/// const SIZE: usize = 5;
/// let stack: Result<Stack<SIZE, i64, false>, Error> = Stack::try_from(vec![1i64,2i64,3i64,4i64,5i64,6i64]);
/// 
/// assert_eq!(stack, Err(Error::Overflow))
/// ```
/// 
/// WRITE_OVER is a generic constant [^note] that is used to determine if elements should be over written on overflow
/// [note]: [Generic Constraints](https://rust-lang.github.io/rfcs/2000-const-generics.html)
#[derive(Default, PartialEq)]
pub struct Stack<const SIZE: usize, T, const WRITE_OVER: bool> (List<SIZE, T, WRITE_OVER>);

impl<const SIZE: usize, T, const WRITE_OVER: bool> Stack<SIZE, T, WRITE_OVER> {
    /// Returns the number of elements in the list.
    /// 
    /// ```
    /// # use cyclic_data_types::stack::Stack;
    /// # const SIZE: usize = 5;
    /// let mut stack: Stack<SIZE, i64, false> = Stack::default();
    /// 
    /// assert_eq!(stack.len(), 0);
    /// 
    /// assert!(stack.push(1).is_ok());
    /// 
    /// assert_eq!(stack.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Pushes an element to the end of the stack.
    /// 
    /// ```
    /// # use cyclic_data_types::stack::Stack;
    /// # const SIZE: usize = 5;
    /// 
    /// let mut stack: Stack<SIZE, i64, false> = Stack::default();
    /// 
    /// assert!(stack.push(1).is_ok());
    /// 
    /// assert!(stack.push(2).is_ok());
    /// 
    /// # assert_eq!(stack.len(), 2);
    /// # assert_eq!(stack.read(0).unwrap(), &2);
    /// # assert_eq!(stack.read(1).unwrap(), &1);
    /// ```
    pub fn push(&mut self, elem: T) -> Result<&mut Self, Error>{
        match self.0.push_back(elem){
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
    
    /// Returns a reference to the top most element of the stack.
    /// 
    /// ```
    /// # use cyclic_data_types::stack::Stack;
    /// # const SIZE: usize = 5;
    /// 
    /// let mut stack: Stack<SIZE, i64, false> = vec![1,2,3].try_into().unwrap();
    /// 
    /// # assert_eq!(stack.len(), 3);
    /// assert_eq!(stack.peek(), Some(&3));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.0.get(-1)
    }
    
    /// Returns the top most element of the stack - after removing said element from the stack.
    /// 
    /// ```
    /// # use cyclic_data_types::stack::Stack;
    /// # const SIZE: usize = 5;
    /// 
    /// let mut stack: Stack<SIZE, i64, false> = vec![1,2,3].try_into().unwrap();
    /// 
    /// # assert_eq!(stack.len(), 3);
    /// assert_eq!(stack.pop(), Some(3));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.0.remove_back()
    }
    
    /// Returns a reference to an element in the stack using an index (relative to the top of the stack).
    /// 
    /// ```
    /// # use cyclic_data_types::stack::Stack;
    /// # const SIZE: usize = 5;
    /// 
    /// let mut stack: Stack<SIZE, i64, false> = vec![1,2].try_into().unwrap();
    /// 
    /// # assert_eq!(stack.len(), 2);
    /// assert_eq!(stack.read(0).unwrap(), &2);
    /// assert_eq!(stack.read(1).unwrap(), &1);
    /// ```
    /// 
    /// # Return
    /// read returns [`Error`] if the index is out range of the stack. Otherwise the method returns a reference of element at index.
    pub fn read(&self, index: usize) -> Result<&T, Error> {
        let len = self.0.len();

        if len <= index {
            return Err(Error::IndexOutOfRange)
        }

        return Ok(&self.0[-1 * index as isize - 1])
    }
}

impl<const S: usize, T, const W: bool> Display for Stack<S, T, W> where T: Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const S: usize, T, const W: bool> Debug for Stack<S, T, W> where T: Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field("", &self.0)
            .finish()
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Vec<T>> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Ok(
            Stack(
                {
                    match value.try_into() {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(err)
                        },
                    }
                }
            )
        )
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<LinkedList<T>> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: LinkedList<T>) -> Result<Self, Self::Error> {
        Ok(
            Stack(
                {
                    match value.try_into() {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(err)
                        },
                    }
                }
            )
        )
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> FromIterator<T> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Default {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        Stack(iter.into_iter().collect())
    }
}

//generic generator
impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Box<dyn Iterator<Item = T>>> for Stack<STACK_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Box<dyn Iterator<Item = T>>) -> Result<Self, Self::Error> {
        Ok(
            Stack(
                {
                    match value.try_into() {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(err)
                        },
                    }
                }
            )
        )
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> From<[T; STACK_SIZE]> for Stack<STACK_SIZE, T, WRITE_OVER> {
    fn from(value: [T; STACK_SIZE]) -> Self {
        Stack(value.into())
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> From<List<STACK_SIZE, T, WRITE_OVER>> for Stack<STACK_SIZE, T, WRITE_OVER>{
    fn from(value: List<STACK_SIZE, T, WRITE_OVER>) -> Self {
        Self(value)
    }
}

impl<const STACK_SIZE: usize, T> From<Stack<STACK_SIZE, T, true>> for Stack<STACK_SIZE, T, false> {
    fn from(value: Stack<STACK_SIZE, T, true>) -> Self {
        Self(value.0.into())
    }
}

impl<const STACK_SIZE: usize, T> From<Stack<STACK_SIZE, T, false>> for Stack<STACK_SIZE, T, true> {
    fn from(value: Stack<STACK_SIZE, T, false>) -> Self {
        Self(value.0.into())
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> Deref for Stack<STACK_SIZE, T, WRITE_OVER> {
    type Target = List<STACK_SIZE, T, WRITE_OVER>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const STACK_SIZE: usize, T, const WRITE_OVER: bool> DerefMut for Stack<STACK_SIZE, T, WRITE_OVER> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}