//! The queue module contains a series of structs to create queues and their utility functionalities using cyclic lists.
//!
//! As a result, the queue inherits the O(1) insertion and deletion for enqueuing & dequeuing.

use std::{
    collections::LinkedList,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::{error::Error, list::List};

#[cfg(test)]
mod tests;

// #[derive(Default, PartialEq)]
// struct PriorityQueue<const SIZE: usize, T, const WRITE_OVER: bool> (List<SIZE, T, WRITE_OVER>) where T: PartialOrd + PartialEq;

/// `Queue` is the `struct` used to define the state of a queue using cyclic [`List`]. As a result, the queue inherits the O(1) insertion and deletion for enqueuing & dequeuing.
///
/// # Generics
/// List types are derived using 3 generics.
///
/// 1. `const SIZE: usize`
///
/// SIZE is a generic constant [^note] that defines the maximum size of the queue
///
/// 2. `T: Sized`
///
/// T is the type of element stored in the queue
///
/// 3. `const WRITE_OVER: bool>`
///
/// # Creating Queue
///
/// Queue can be created in a couple of ways.
///
/// 1. Empty Queue
///
/// Empty Queue are created using the [`Default`] trait implementation for Queue.
///
/// ```
/// # use cyclic_data_types::queue::Queue;
/// # const SIZE: usize = 5;
/// let queue: Queue<SIZE, i64, false> = Queue::default();
///
/// assert_eq!(queue.len(), 0);
/// ```
///
/// 2. From Array
///
/// Queue can also be derived from arrays. The maximum size of the queue is the same size as the array. This is done using the [`From<[SIZE; T]`] trait implementation for List.
///
/// ```
/// # use cyclic_data_types::queue::Queue;
/// # const SIZE: usize = 5;
/// let mut queue: Queue<SIZE, i64, false> = [1i64,2i64,3i64,4i64,5i64].into();
/// #
/// # assert_eq!(queue.len(), 5);
/// # assert_eq!(queue.dequeue(), Some(1i64));
/// # assert_eq!(queue.dequeue(), Some(2i64));
/// # assert_eq!(queue.dequeue(), Some(3i64));
/// # assert_eq!(queue.dequeue(), Some(4i64));
/// # assert_eq!(queue.dequeue(), Some(5i64));
/// # assert_eq!(queue.dequeue(), None);
/// ```
///
/// 3. From Vectors, Linked Lists and Iterators
///
/// Since collections (Vectors, Linked Lists and Iterators) cannot guarantee a size at compile time - the conversion is not always guaranteed to succeed. This occurs when collection is larger than the queue variant. As a result, the new queue cannot be created without resulting in a [`Error::Overflow`]. This can be resolved by either making sure the collection is at max the same size as the queue variant or the cyclic list variant permits `WRITE_OVER`.
///
/// Therefore, the [`TryFrom`] trait implementation of queue must be used.
///
/// Example of a successful conversion
/// ```
/// # use cyclic_data_types::queue::Queue;
/// const SIZE: usize = 5;
/// let queue: Queue<SIZE, i64, false> = Queue::try_from(vec![1i64,2i64,3i64,4i64,5i64])
///     .unwrap();
/// #
/// # assert_eq!(queue.len(), 5);
/// # assert_eq!(queue, [1i64,2i64,3i64,4i64,5i64].into());
/// ```
/// ```
/// # use cyclic_data_types::queue::Queue;
/// const SIZE: usize = 5;
/// let queue: Queue<SIZE, i64, true> = vec![1i64,2i64,3i64,4i64,5i64,6i64].try_into()
///     .unwrap();
/// #
/// # assert_eq!(queue.len(), 5);
/// # assert_eq!(queue, [2i64,3i64,4i64,5i64,6i64].into());
/// ```
/// Example of a failed conversion
/// ```
/// # use cyclic_data_types::queue::Queue;
/// # use cyclic_data_types::error::Error;
/// const SIZE: usize = 5;
/// let queue: Result<Queue<SIZE, i64, false>, Error> = Queue::try_from(vec![1i64,2i64,3i64,4i64,5i64,6i64]);
///
/// assert_eq!(queue, Err(Error::Overflow))
/// ```
///
/// WRITE_OVER is a generic constant [^note] that is used to determine if elements should be over written on overflow
/// [note]: [Generic Constraints](https://rust-lang.github.io/rfcs/2000-const-generics.html)
#[derive(PartialEq)]
pub struct Queue<const SIZE: usize, T, const WRITE_OVER: bool>(List<SIZE, T, WRITE_OVER>);

impl<const SIZE: usize, T, const WRITE_OVER: bool> Queue<SIZE, T, WRITE_OVER> {
    /// Returns the number of elements in the queue.
    ///
    /// ```
    /// # use cyclic_data_types::queue::Queue;
    /// # const SIZE: usize = 5;
    /// let mut queue: Queue<SIZE, i64, false> = Queue::default();
    ///
    /// assert_eq!(queue.len(), 0);
    ///
    /// assert!(queue.enqueue(1).is_ok());
    ///
    /// assert_eq!(queue.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    ///  Pushes an element to the end of the queue.
    ///
    /// ```
    /// # use cyclic_data_types::queue::Queue;
    /// # const SIZE: usize = 5;
    ///
    /// let mut queue: Queue<SIZE, i64, false> = Queue::default();
    ///
    /// assert!(queue.enqueue(1).is_ok());
    ///
    /// assert!(queue.enqueue(2).is_ok());
    ///
    /// # assert_eq!(queue.len(), 2);
    /// ```
    pub fn enqueue(&mut self, elem: T) -> Result<&mut Self, Error> {
        match self.0.push_back(elem) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
    /// Returns a reference to the first element in the queue.
    ///
    /// ```
    /// # use cyclic_data_types::queue::Queue;
    /// # const SIZE: usize = 5;
    ///
    /// let mut queue: Queue<SIZE, i64, false> = vec![1,2,3].try_into().unwrap();
    ///
    /// # assert_eq!(queue.len(), 3);
    /// assert_eq!(queue.peek(), Some(&1));
    /// ```
    pub fn peek(&mut self) -> Option<&T> {
        if self.0.len() == 0 {
            return None;
        }
        Some(&self.0[0usize])
    }

    /// Returns the first element of the queue - after removing said element from the queue.
    ///
    /// ```
    /// # use cyclic_data_types::queue::Queue;
    /// # const SIZE: usize = 5;
    ///
    /// let mut queue: Queue<SIZE, i64, false> = vec![1,2,3].try_into().unwrap();
    ///
    /// # assert_eq!(queue.len(), 3);
    /// assert_eq!(queue.dequeue(), Some(1));
    /// # assert_eq!(queue.len(), 2);
    /// assert_eq!(queue.dequeue(), Some(2));
    /// # assert_eq!(queue.len(), 1);
    /// assert_eq!(queue.dequeue(), Some(3));
    /// # assert_eq!(queue.len(), 0);
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        self.0.remove_front()
    }
}

impl<const S: usize, T, const W: bool> Display for Queue<S, T, W>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const S: usize, T, const W: bool> Debug for Queue<S, T, W>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Queue").field("", &self.0).finish()
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Vec<T>>
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
where
    T: Clone + Default,
{
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Ok(Queue({
            match value.try_into() {
                Ok(value) => value,
                Err(err) => return Err(err),
            }
        }))
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<LinkedList<T>>
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
where
    T: Clone + Default,
{
    type Error = Error;

    fn try_from(value: LinkedList<T>) -> Result<Self, Self::Error> {
        Ok(Queue({
            match value.try_into() {
                Ok(value) => value,
                Err(err) => return Err(err),
            }
        }))
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> FromIterator<T>
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
{
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        Queue(iter.into_iter().collect())
    }
}

//generic generator
impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Box<dyn Iterator<Item = T>>>
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
where
    T: Clone + Default,
{
    type Error = Error;

    fn try_from(value: Box<dyn Iterator<Item = T>>) -> Result<Self, Self::Error> {
        Ok(Queue({
            match value.try_into() {
                Ok(value) => value,
                Err(err) => return Err(err),
            }
        }))
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> From<[T; QUEUE_SIZE]>
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
{
    fn from(value: [T; QUEUE_SIZE]) -> Self {
        Queue(value.into())
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> From<List<QUEUE_SIZE, T, WRITE_OVER>>
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
{
    fn from(value: List<QUEUE_SIZE, T, WRITE_OVER>) -> Self {
        Self(value)
    }
}

impl<const QUEUE_SIZE: usize, T> From<Queue<QUEUE_SIZE, T, true>> for Queue<QUEUE_SIZE, T, false> {
    fn from(value: Queue<QUEUE_SIZE, T, true>) -> Self {
        Self(value.0.into())
    }
}

impl<const QUEUE_SIZE: usize, T> From<Queue<QUEUE_SIZE, T, false>> for Queue<QUEUE_SIZE, T, true> {
    fn from(value: Queue<QUEUE_SIZE, T, false>) -> Self {
        Self(value.0.into())
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> Deref
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
{
    type Target = List<QUEUE_SIZE, T, WRITE_OVER>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> DerefMut
    for Queue<QUEUE_SIZE, T, WRITE_OVER>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> Default for Queue<QUEUE_SIZE, T, WRITE_OVER> {
    fn default() -> Self {
        Self(Default::default())
    }
}
