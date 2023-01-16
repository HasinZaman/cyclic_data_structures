//! The list module contains a series of structs to create cyclic lists and their utility functionalities.
//! 
//! Cyclic List Example
//! ```text
//! 1 ↔ 2 ↔ ... ↔ n
//! ↑             ↑
//! └─────────────┘
//! ```
//! Note: Even though the diagram uses arrow to denote the relationship between nodes/elements; implying a linked node structure - however, the implementation in this crate uses arrays.
//! 
//! As result, a [`List<T>`] struct can be sized (assuming that `T` is also sized). Allowing the `List` to exist on the stack.
use std::{ops::{Index, IndexMut}, fmt::{Display, Debug}, collections::LinkedList, mem};
use std::convert::TryFrom;

use std::iter::FromIterator;

use crate::{CyclicList, error::Error};

use self::iterator::Iter;

mod iterator;

#[cfg(test)]
mod tests;

/// `List` is the `struct` used to define the state of a cyclic List
/// 
/// # Generics
/// List types are derived using 3 generics.
/// 
/// 1. `const SIZE: usize`
/// 
/// SIZE is a generic constant [^note] that defines the maximum size of the list
/// 
/// 2. `T: Sized`
/// 
/// T is the type of element stored in the list
/// 
/// 3. `const WRITE_OVER: bool>`
/// 
/// WRITE_OVER is a generic constant [^note] that is used to determine if elements should be over written on overflow
/// 
/// # Creating Lists
/// 
/// List can be created in a couple of ways.
/// 
/// 1. Empty list
/// 
/// Empty list are created using the [`Default`] trait implementation for List.
/// 
/// ```
/// # use cyclic_data_types::list::List;
/// # const SIZE: usize = 5;
/// let list: List<SIZE, i64, false> = List::default();
/// 
/// assert_eq!(list.len(), 0);
/// ```
/// 2. From Array
/// 
/// List can can also be from arrays. The maximum size of the list is the same size as the array. This is done using the [`From<[SIZE; T]`] trait implementation for List.
/// 
/// ```
/// # use cyclic_data_types::list::List;
/// # const SIZE: usize = 5;
/// let list: List<SIZE, i64, false> = [1i64,2i64,3i64,4i64,5i64].into();
/// #
/// # assert_eq!(list.len(), 5);
/// # assert_eq!(list[0usize], 1i64);
/// # assert_eq!(list[1usize], 2i64);
/// # assert_eq!(list[2usize], 3i64);
/// # assert_eq!(list[3usize], 4i64);
/// # assert_eq!(list[4usize], 5i64);
/// ```
/// 
/// 3. From Vectors, Linked Lists and Iterators
/// 
/// Since collections (Vectors, Linked Lists and Iterators) cannot guarantee a size at compile time - the conversion is not always guaranteed to succeed. This occurs when collection is larger than the List variant. As a result, the new cyclic list cannot be created without resulting in a [`Error::Overflow`]. This can be resolved by either making sure the collection is at max the same size as the cyclic list variant or the cyclic list variant permits `WRITE_OVER`.
/// 
/// Therefore, the [`TryFrom`] trait implementation of List must be used.
/// 
/// Example of a successful conversion
/// ```
/// # use cyclic_data_types::list::List;
/// const SIZE: usize = 5;
/// let list: List<SIZE, i64, false> = List::try_from(vec![1i64,2i64,3i64,4i64,5i64])
///     .unwrap();
/// #
/// # assert_eq!(list.len(), 5);
/// # assert_eq!(list, [1i64,2i64,3i64,4i64,5i64].into());
/// ```
/// ```
/// # use cyclic_data_types::list::List;
/// const SIZE: usize = 5;
/// let list: List<SIZE, i64, true> = vec![1i64,2i64,3i64,4i64,5i64,6i64].try_into()
///     .unwrap();
/// #
/// # assert_eq!(list.len(), 5);
/// # assert_eq!(list, [2i64,3i64,4i64,5i64,6i64].into());
/// ```
/// Example of a failed conversion
/// ```
/// # use cyclic_data_types::list::List;
/// # use cyclic_data_types::error::Error;
/// const SIZE: usize = 5;
/// let list: Result<List<SIZE, i64, false>, Error> = List::try_from(vec![1i64,2i64,3i64,4i64,5i64,6i64]);
/// 
/// assert_eq!(list, Err(Error::Overflow))
/// ```
/// 
/// [note]: [Generic Constraints](https://rust-lang.github.io/rfcs/2000-const-generics.html)
#[derive(Eq, PartialEq, Default)]
pub struct List<const SIZE: usize, T: Sized, const WRITE_OVER: bool>{
    list: CyclicList<SIZE, T, WRITE_OVER>
}

impl<const SIZE: usize, T, const WRITE_OVER: bool> List<SIZE, T, WRITE_OVER> {
    /// Returns the number of elements in the list
    /// 
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list: List<SIZE, i64, false>  = List::default();
    /// 
    /// assert_eq!(list.len(), 0);
    /// 
    /// assert!(list.push_back(1).is_ok());
    /// 
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    // pub fn insert_at(&mut self, elem: T, index: usize) -> Result<&mut Self, Error> where T: Clone {
        
    //     todo!();
    // }

    /// Pushes a new element to the back of the list. This operation is done in `O(1)`.
    /// 
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, false> = List::default();
    /// 
    /// assert!(list.push_back(1).is_ok());
    /// ```
    /// 
    /// If the list is full - the list has two options based on the `WRITE_OVER` flag.
    /// 1. `WRITE_OVER = true`
    /// 
    /// The first element is written over and dropped. Meaning the first element is no longer in the list.
    /// 
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, true> = [1,2,3,4,5].into();
    /// 
    /// assert!(list.push_back(6).is_ok());
    /// 
    /// assert_eq!(list, [2,3,4,5,6].into());
    /// ```
    /// 
    /// 2. `WRITE_OVER = false`
    /// 
    /// The new element isn't added to the list. Resulting in no change to the state of the list.
    /// 
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// 
    /// assert!(list.push_back(6).is_err());
    /// 
    /// assert_eq!(list, [1,2,3,4,5].into());
    /// ```
    /// 
    /// # Returns
    /// * Self if the push was successful
    /// * [Error] if List is full and the `WRITE_OVER` flag is set to `false`
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
            },
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

        self.list[end] = Some(elem);

        Ok(self)
    }

    /// Pushes a new element to the front of the list. This operation is done in `O(1)`.
    /// 
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, false> = List::default();
    /// 
    /// assert!(list.push_front(1).is_ok());
    /// ```
    /// 
    /// If the list is full - the list has two options based on the `WRITE_OVER` flag.
    /// 1. `WRITE_OVER = true`
    /// 
    /// The last element is written over and dropped. Meaning the last element is no longer in the list.
    /// 
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, true> = [1,2,3,4,5].into();
    /// 
    /// assert!(list.push_front(0).is_ok());
    /// 
    /// assert_eq!(list, [0,1,2,3,4].into());
    /// ```
    /// 
    /// 2. `WRITE_OVER = false`
    /// 
    /// The new element isn't added to the list. Resulting in no change to the state of the list.
    /// 
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// 
    /// assert!(list.push_front(0).is_err());
    /// 
    /// assert_eq!(list, [1,2,3,4,5].into());
    /// ```
    /// 
    /// # Returns
    /// * Self if the push was successful
    /// * [Error] if List is full and the `WRITE_OVER` flag is set to `false`
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
        let start = self.list.start;
        self.list[start] = Some(elem);

        Ok(self)
    }

    /// returns a reference to an element in the list at provided index.
    /// 
    /// The get element retrieval works similarly to a cyclic list and python lists. Where, the list loop backs to the beginning of the list when the index is greater than the size of the list; and the list can be accessed from the end of the list using negative integers.
    /// 
    /// Therefore, let the following be a cyclic list.
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// #
    /// let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// ```
    /// 
    /// The following is equivalent in retrieving the last element
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// # let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// assert_eq!(list.get(4), Some(&5));
    /// assert_eq!(list.get(4), list.get(9));
    /// assert_eq!(list.get(4), list.get(-1));
    /// ```
    /// 
    /// The following is equivalent in retrieving the first element
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// # let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// assert_eq!(list.get(0), Some(&1));
    /// assert_eq!(list.get(0), list.get(5));
    /// assert_eq!(list.get(0), list.get(-5));
    /// ```
    /// 
    /// # Return
    /// * `None` if the list is empty
    /// * `Some(T)` if the list has at least one element
    /// 
    /// # Note
    /// 
    /// If certain constraints on the index is met. Then the following element retrievals are recommended over `.get(index)`.
    /// 
    /// If, i ∈ [0, list.len()) then, `list.get(i)` = `list[i as usize]`
    /// If, i ∈ [-1 * list.len(), list.len()) then, `list.get(i)` = `list[i as isize]`
    /// 
    /// This is because the `Index<usize>` and `Index<isize>` are defined with let checks - with the assumption that certain guarantees on the index are provided on run time.
    /// 
    /// Such that, im(`Self::Index<usize>`) ⊆ im(`Self::Index<isize>`) ⊆ im(`Self::get(&self, isize)`).
    pub fn get(&self, index: isize) -> Option<&T> {
        if self.len() == 0 {
            return None
        }

        match index {
            ..=-1 => {
                return Some(&self[-1*((index + 1).abs() % self.len() as isize) - 1])
            },
            _ => {
                return Some(&self[index % self.len() as isize])
            }
        }
    }

    /// returns a reference to an element in the list at provided index of the underlying array. get_unchecked is faster than all the other retrieval methods - however, provides less safety & features in exchange.
    /// 
    /// # Returns
    /// The method returns None if the underlying array has not value at the given index. Otherwise, the method returns a reference to the items at the array index.
    pub unsafe fn get_unchecked(&self, index: usize) -> Option<&T> {
        self.list.get_unchecked(index)
    }
    

    /// returns a mutable reference to an element in the list at provided index.
    /// 
    /// The get_mut element retrieval works similarly to a cyclic list and python lists. Where, the list loop backs to the beginning of the list when the index is greater than the size of the list; and the list can be accessed from the end of the list using negative integers.
    /// 
    /// Therefore, let the following be a cyclic list.
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// #
    /// let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// ```
    /// 
    /// The following is equivalent in retrieving last element
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// # let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// assert_eq!(list.get_mut(4), Some(&mut 5));
    /// assert_eq!(list.get_mut(9), Some(&mut 5));
    /// assert_eq!(list.get_mut(-1), Some(&mut 5));
    /// ```
    /// 
    /// The following is equivalent in the retrieving first element
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// # let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// assert_eq!(list.get_mut(0), Some(&mut 1));
    /// assert_eq!(list.get_mut(5), Some(&mut 1));
    /// assert_eq!(list.get_mut(-5), Some(&mut 1));
    /// ```
    /// 
    /// The following can be used to update an element
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// # let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// *list.get_mut(0).unwrap() = 6;
    /// 
    /// assert_eq!(list.get(0), Some(&6));
    /// ```
    /// 
    /// # Return
    /// * `None` if the list is empty
    /// * `Some(T)` if the list has at least one element
    /// 
    /// # Note
    /// 
    /// If certain constraints on the index is met. Then the following element retrievals are recommended over `.get(index)`.
    /// 
    /// If, i ∈ [0, list.len()) then, `list.get(i)` = `list[i as usize]`
    /// If, i ∈ [-1 * list.len(), list.len()) then, `list.get(i)` = `list[i as isize]`
    /// 
    /// This is because the `Index<usize>` and `Index<isize>` are defined with let checks - with the assumption that certain guarantees on the index are provided on run time.
    /// 
    /// Such that, im(`Self::Index<usize>`) ⊆ im(`Self::Index<isize>`) ⊆ im(`Self::get_mut(&self, isize)`).
    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if self.len() == 0 {
            return None
        }

        let len = self.len() as isize;

        match index {
            ..=-1 => {
                return Some(&mut self[-1*((index + 1).abs() % len as isize) - 1])
            },
            _ => {
                return Some(&mut self[index % len])
            }
        }
    }

    /// returns a reference to an element in the list at provided index of the underlying array. get_unchecked is faster than all the other retrieval methods - however, provides less safety & features in exchange.
    /// 
    /// # Returns
    /// The method returns None if the underlying array has not value at the given index. Otherwise, the method returns a reference to the items at the array index.
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> Option<&mut T> {
        self.list.get_unchecked_mut(index)
    }
    

    /// Removes the last element from the list and returns removed element. This occurs in `O(1)`
    ///
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// 
    /// assert_eq!(list.remove_back(), Some(5));
    /// 
    /// assert_eq!(list.len(), 4);
    /// assert_eq!(list, vec![1,2,3,4].try_into().unwrap());
    /// ```
    /// 
    /// # Return
    /// * `Some(last element in list)` if `list.len()` > 0
    /// * `None` if `list.len()` = 0
    pub fn remove_back(&mut self) -> Option<T> {
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
        
        let mut value = None;

        mem::swap(&mut self.list[pop_index], &mut value);

        Some(value.unwrap())
    }

    /// Removes the first element from the list and returns removed element. This occurs in `O(1)`
    ///
    /// ```
    /// # use cyclic_data_types::list::List;
    /// # const SIZE: usize = 5;
    /// let mut list : List<SIZE, i64, false> = [1,2,3,4,5].into();
    /// 
    /// assert_eq!(list.remove_front(), Some(1));
    /// 
    /// assert_eq!(list.len(), 4);
    /// assert_eq!(list, vec![2,3,4,5].try_into().unwrap());
    /// ```
    /// 
    /// # Return
    /// * `Some(last element in list)` if `list.len()` > 0
    /// * `None` if `list.len()` = 0
    pub fn remove_front(&mut self) -> Option<T> {
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
        
        let mut value = None;

        mem::swap(&mut self.list[pop_index], &mut value);

        Some(value.unwrap())
    }

    // pub fn remove_at(&mut self, index: usize) -> Result<T, Error> where T: Clone {
        
    //     todo!();
    // }

    /// Creates an iterator object that iterates over the elements in the list
    pub fn iter(&self) -> Iter<SIZE, T, WRITE_OVER> where Self: Sized {
        Iter::new(self)
    }

    /// Creates a iterator object over the list. In which, each element in the iterator can be updated.
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

impl<const SIZE: usize, T, const W: bool> Index<usize> for List<SIZE, T, W> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.len() <= index {
            panic!("{:?}", Error::IndexOutOfRange);
        }
        
        self.list[(self.list.start + index) % SIZE].as_ref().unwrap()
    }
}

impl<const SIZE: usize, T, const W: bool> IndexMut<usize> for List<SIZE, T, W> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.len() <= index {
            panic!("{:?}", Error::IndexOutOfRange);
        }

        let start = self.list.start;
        self.list[(start + index) % SIZE].as_mut().unwrap()
    }
}

impl<const S: usize, T, const W: bool> Index<isize> for List<S, T, W> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        if 0 <= index {
            return &self[index as usize];
        }

        if self.len() < index.abs() as usize {
            panic!("{:?}", Error::IndexOutOfRange);
        }


        //index exists between [-1*self.len(), -1]
        &self[self.len() - index.abs() as usize]
    }
}

impl<const S: usize, T, const W: bool> IndexMut<isize> for List<S, T, W> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        if 0 <= index {
            return &mut self[index as usize];
        }

        if self.len() < index.abs() as usize {
            panic!("{:?}", Error::IndexOutOfRange);
        }


        //index exists between [-1*self.len(), -1]
        let len = self.len();
        &mut self[len - index.abs() as usize]
    }
}

impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Vec<T>> for List<LIST_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if LIST_SIZE < value.len() && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        let mut list = Self::default();

        for element in value{
            if let Err(err) = list.push_back(element) {
                return Err(err);
            }
        }

        Ok(list)
    }
}

impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<LinkedList<T>> for List<LIST_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: LinkedList<T>) -> Result<Self, Self::Error> {
        if LIST_SIZE < value.len() && !WRITE_OVER {
            return Err(Error::Overflow)
        }

        let mut list = Self::default();

        for element in value{
            list.push_back(element).unwrap();
        }

        Ok(list)
    }
}

impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> FromIterator<T> for List<LIST_SIZE, T, WRITE_OVER> where T: Default {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut list :List<LIST_SIZE, T, WRITE_OVER> = List::default();

        for elem in iter {
            list.push_back(elem).unwrap();
        }

        list
    }
}

//generic generator
impl<const LIST_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Box<dyn Iterator<Item = T>>> for List<LIST_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Box<dyn Iterator<Item = T>>) -> Result<Self, Self::Error> {
        let mut list = Self::default();

        for element in value{
            if let Err(err) = list.push_back(element) {
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