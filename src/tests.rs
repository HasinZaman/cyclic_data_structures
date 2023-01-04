
//bench mark against
// other cyclic list: https://docs.rs/cyclic_list/latest/cyclic_list/list/struct.List.html#method.iter_mut
// Vectors
// Linked list

// new
// len(&self) -> usize;
// insert_at
mod push{
    use crate::{CyclicList, list::List, Error};

    #[test]
    fn push() {
        let mut list: CyclicList<5, i64, false> = CyclicList::default();

        assert!(list.push(23).is_ok());
        assert_eq!(list, CyclicList::try_from(vec![23]).unwrap());
        println!("{:?}", list);
        assert_eq!(list.len(), 1);
        
        assert!(list.push(23).unwrap().push(25).is_ok());
        assert_eq!(list,CyclicList::try_from(vec![23,23,25]).unwrap());
        assert_eq!(list.len(), 3);
    }
    #[test]
    fn push_no_overflow() {
        let mut list: CyclicList<5, i64, false> = CyclicList::try_from(vec![1,2,3,4,5]).unwrap();

        assert_eq!(list.push(6), Err(Error::Overflow));
        assert_eq!(list,CyclicList::try_from(vec![1,2,3,4,5]).unwrap());
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn push_overflow() {
        let mut list: CyclicList<5, i64, true> = CyclicList::try_from(vec![1,2,3,4,5]).unwrap();

        assert!(list.push(6).is_ok());
        assert_eq!(
            list,CyclicList::try_from(vec![2,3,4,5,6]).unwrap());
        assert_eq!(list.len(), 5);
    }

}

// get(&self, index: usize) -> Result<&T, Error>;
// get_mut(&mut self, index: usize) -> Result<&mut T, Error>;
// pop(&mut self) -> Option<&mut T>;
mod pop{
    use crate::{CyclicList, list::List};

    #[test]
    fn empty_pop() {
        let mut list: CyclicList<5, i64, false> = CyclicList::default();
        
        let val = list.pop();

        assert_eq!(val, None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_one() {
        let mut list: CyclicList<5, i64, false> = CyclicList::try_from(vec![1]).unwrap();
        
        let val = list.pop();
        assert_eq!(val, Some(&mut 1));

        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_serial() {
        let mut list: CyclicList<5, i64, false> = CyclicList::try_from(vec![1,2,3,4,5]).unwrap();
        
        for i in (1..=5).rev(){
            let mut expected = i;

            let val = list.pop();
            assert_eq!(val, Some(&mut expected));

            assert_eq!(list.len(), i as usize - 1);
        }
        
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_from_overflow() {
        let mut list: CyclicList<5, i64, true> = CyclicList::default();

        for i in 1..=7{
            list.push(i).unwrap();
        }

        let mut size = 5;
        for i in (3..=7).rev(){
            size -= 1;
            
            let mut expected = i;

            let val = list.pop();
            assert_eq!(val, Some(&mut expected));

            assert_eq!(list.len(), size);
        }
        
        assert_eq!(list.len(), 0);
    }
}


// remove_at

mod iter{
    use crate::{CyclicList, list::List};

    #[test]
    fn empty_iter() {
        let list: CyclicList<5, i64, false> = CyclicList::default();
        
        let iter = list.iter();

        assert_eq!(iter.count(), 0);

    }

    #[test]
    fn no_overflow_iter() {
        let source = [1,2,3,4,5];
        let list: CyclicList<5, i64, false> = CyclicList::from(source);
        
        let iter = list.iter();

        for (actual, expected) in iter.zip(source.iter()) {
            assert_eq!(actual, expected);
        }        
    }

    #[test]
    fn overflow_iter() {
        let source = vec![1,2,3,4,5,6,7];
        let list: CyclicList<5, i64, true> = {
            let mut list = CyclicList::default();

            for i in source{
                list.push(i).unwrap();
            }

            list
        };
        
        let iter = list.iter();

        for (actual, expected) in iter.zip([3,4,5,6,7].iter()) {
            assert_eq!(actual, expected);
        }        
    }
}
// iter_mut(&mut self) -> ListIter<T, Self> where Self: Sized;
// PartialEq
// Display
// Debug
// Default
// Index<usize>
// IndexMut<usize>
// TryFrom<Vec<T>>
// From<[T; ARRAY_SIZE]>
// CyclicList<SIZE, T, WRITEOVER>
