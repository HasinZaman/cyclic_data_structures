mod len{
    use std::cmp::min;

    use crate::list::List;

    const SIZE: usize = 5;

    #[test]
    fn empty() {
        let list: List<SIZE, i64, false>  = List::default();

        assert_eq!(list.len(), 0);
    }
    
    #[test]
    fn partial_size_push_back() {
        let mut list: List<SIZE, i64, false>  = List::default();

        for i in 0..5 {
            assert!(list.push_back(i).is_ok());
            assert_eq!(list.len(), (i + 1) as usize);
        }
    }
    
    #[test]
    fn overflow_push_back() {
        let mut list: List<SIZE, i64, true>  = List::default();

        for i in 0..10 {
            assert!(list.push_back(i).is_ok());

            assert_eq!(list.len(), min(i as usize, SIZE));
        }
    }

    #[test]
    fn partial_size_push_front() {
        let mut list: List<SIZE, i64, false>  = List::default();

        for i in 0..5 {
            assert!(list.push_front(i).is_ok());
            assert_eq!(list.len(), (i + 1) as usize);
        }
    }
    
    #[test]
    fn overflow_push_front() {
        let mut list: List<SIZE, i64, true>  = List::default();

        for i in 0..10 {
            assert!(list.push_front(i).is_ok());

            assert_eq!(list.len(), min(i as usize, SIZE));
        }
    }
}

mod insert_at{
    use std::cmp::min;

    use crate::{list::List, error::Error};

    const SIZE: usize = 5;
    
    #[test]
    fn push_back() {
        todo!();
    }
    
    #[test]
    fn push_front() {
        todo!();
    }

    #[test]
    fn insert_middle() {
        todo!();
    }

    #[test]
    fn no_overflow(){
        todo!();
    }

    #[test]
    fn index_out_of_range(){
        todo!();
    }
}

mod push_back{
    use std::cmp::min;

    use crate::{list::List, error::Error};

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let mut list: List<SIZE, i64, false> = List::default();

        assert!(list.push_back(1).is_ok());
        assert_eq!(list, vec![1].try_into().unwrap());
    }

    #[test]
    fn fill(){
        let mut list: List<SIZE, i64, false> = List::default();
        let mut expected = vec![];

        for i in 0..SIZE{
            assert!(list.push_back(i as i64).is_ok());
            expected.push(i as i64);


            assert_eq!(list.len(), i + 1);
            assert_eq!(list, expected.clone().try_into().unwrap());
        }
    }

    #[test]
    fn overflow(){
        let mut list: List<SIZE, i64, true> = List::default();
        let mut expected = vec![];

        for i in 0..SIZE+1{
            assert!(list.push_back(i as i64).is_ok());
            match i {
                0..=SIZE => {
                    expected.push(i as i64);
                }
                _=> {
                    expected.remove(0);
                    expected.push(i as i64);
                }
            }
            
            assert_eq!(list.len(), min(i + 1, SIZE));
            assert_eq!(list, expected.clone().try_into().unwrap());
        }
    }

    #[test]
    fn no_overflow(){
        let mut list: List<SIZE, i64, false> = vec![1,2,3,4,5].try_into().unwrap();

        assert_eq!(Err(Error::Overflow), list.push_back(10 as i64));
    }
}

mod push_front{
    use std::cmp::min;

    use crate::{list::List, error::Error};

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let mut list: List<SIZE, i64, false> = List::default();

        assert!(list.push_front(1).is_ok());
        assert_eq!(list, vec![1].try_into().unwrap());
    }

    #[test]
    fn fill(){
        let mut list: List<SIZE, i64, false> = List::default();
        let mut expected = vec![];

        for i in 0..SIZE{
            assert!(list.push_front(i as i64).is_ok());
            expected.insert(0, i as i64);


            assert_eq!(list.len(), i + 1);
            assert_eq!(list, expected.clone().try_into().unwrap());
        }
    }

    #[test]
    fn overflow(){
        let mut list: List<SIZE, i64, true> = List::default();
        let mut expected = vec![];

        for i in 0..SIZE+1{
            assert!(list.push_front(i as i64).is_ok());
            match i {
                0..=SIZE => {
                    expected.push(i as i64);
                }
                _=> {
                    expected.pop();
                    expected.push(i as i64);
                }
            }
            
            assert_eq!(list.len(), min(i + 1, SIZE));
            assert_eq!(list, expected.clone().try_into().unwrap());
        }
    }

    #[test]
    fn no_overflow(){
        let mut list: List<SIZE, i64, false> = vec![1,2,3,4,5].try_into().unwrap();

        assert_eq!(Err(Error::Overflow), list.push_front(10 as i64));
    }
}

mod get{
    use crate::{list::List};

    const SIZE: usize = 5;
    #[test]
    fn get(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get(0), Some(&0));
        assert_eq!(list.get(1), Some(&1));
        assert_eq!(list.get(2), Some(&2));
        assert_eq!(list.get(3), Some(&3));

        assert!(list.push_back(4).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get(0), Some(&0));
        assert_eq!(list.get(1), Some(&1));
        assert_eq!(list.get(2), Some(&2));
        assert_eq!(list.get(3), Some(&3));
        assert_eq!(list.get(4), Some(&4));

        assert!(list.push_back(5).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), Some(&4));
        assert_eq!(list.get(4), Some(&5));
    }

    #[test]
    fn get_overflow(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get(0 + list.len() as isize), Some(&0));
        assert_eq!(list.get(1 + list.len() as isize), Some(&1));
        assert_eq!(list.get(2 + list.len() as isize), Some(&2));
        assert_eq!(list.get(3 + list.len() as isize), Some(&3));

        assert!(list.push_back(4).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get(0 + list.len() as isize), Some(&0));
        assert_eq!(list.get(1 + list.len() as isize), Some(&1));
        assert_eq!(list.get(2 + list.len() as isize), Some(&2));
        assert_eq!(list.get(3 + list.len() as isize), Some(&3));
        assert_eq!(list.get(4 + list.len() as isize), Some(&4));

        assert!(list.push_back(5).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get(0 + list.len() as isize), Some(&1));
        assert_eq!(list.get(1 + list.len() as isize), Some(&2));
        assert_eq!(list.get(2 + list.len() as isize), Some(&3));
        assert_eq!(list.get(3 + list.len() as isize), Some(&4));
        assert_eq!(list.get(4 + list.len() as isize), Some(&5));
    }

    #[test]
    fn get_from_back(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get(-1), Some(&3));
        assert_eq!(list.get(-2), Some(&2));
        assert_eq!(list.get(-3), Some(&1));
        assert_eq!(list.get(-4), Some(&0));

        assert!(list.push_back(4).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get(-1), Some(&4));
        assert_eq!(list.get(-2), Some(&3));
        assert_eq!(list.get(-3), Some(&2));
        assert_eq!(list.get(-4), Some(&1));
        assert_eq!(list.get(-5), Some(&0));

        assert!(list.push_back(5).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get(-1), Some(&5));
        assert_eq!(list.get(-2), Some(&4));
        assert_eq!(list.get(-3), Some(&3));
        assert_eq!(list.get(-4), Some(&2));
        assert_eq!(list.get(-5), Some(&1));
    }

    #[test]
    fn get_from_back_overflow(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get(-1 + -1 * list.len() as  isize), Some(&3));
        assert_eq!(list.get(-2 + -1 * list.len() as  isize), Some(&2));
        assert_eq!(list.get(-3 + -1 * list.len() as  isize), Some(&1));
        assert_eq!(list.get(-4 + -1 * list.len() as  isize), Some(&0));

        assert!(list.push_back(4).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get(-1 + -1 * list.len() as  isize), Some(&4));
        assert_eq!(list.get(-2 + -1 * list.len() as  isize), Some(&3));
        assert_eq!(list.get(-3 + -1 * list.len() as  isize), Some(&2));
        assert_eq!(list.get(-4 + -1 * list.len() as  isize), Some(&1));
        assert_eq!(list.get(-5 + -1 * list.len() as  isize), Some(&0));

        assert!(list.push_back(5).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get(-1 + -1 * list.len() as  isize), Some(&5));
        assert_eq!(list.get(-2 + -1 * list.len() as  isize), Some(&4));
        assert_eq!(list.get(-3 + -1 * list.len() as  isize), Some(&3));
        assert_eq!(list.get(-4 + -1 * list.len() as  isize), Some(&2));
        assert_eq!(list.get(-5 + -1 * list.len() as  isize), Some(&1));
    }

    #[test]
    fn empty(){
        let list: List<SIZE, i64, false> = List::default();

        for i in (-1 * SIZE as isize)..(SIZE as isize){
            assert_eq!(None, list.get(i));
        }
    }
}

mod get_mut{
    use crate::{list::List};

    const SIZE: usize = 5;
    #[test]
    fn get(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get_mut(0), Some(&mut 0));
        assert_eq!(list.get_mut(1), Some(&mut 1));
        assert_eq!(list.get_mut(2), Some(&mut 2));
        assert_eq!(list.get_mut(3), Some(&mut 3));

        assert!(list.push_back(4).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get_mut(0), Some(&mut 0));
        assert_eq!(list.get_mut(1), Some(&mut 1));
        assert_eq!(list.get_mut(2), Some(&mut 2));
        assert_eq!(list.get_mut(3), Some(&mut 3));
        assert_eq!(list.get_mut(4), Some(&mut 4));

        assert!(list.push_back(5).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get_mut(0), Some(&mut 1));
        assert_eq!(list.get_mut(1), Some(&mut 2));
        assert_eq!(list.get_mut(2), Some(&mut 3));
        assert_eq!(list.get_mut(3), Some(&mut 4));
        assert_eq!(list.get_mut(4), Some(&mut 5));
    }

    #[test]
    fn get_overflow(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get_mut(0 + list.len() as isize), Some(&mut 0));
        assert_eq!(list.get_mut(1 + list.len() as isize), Some(&mut 1));
        assert_eq!(list.get_mut(2 + list.len() as isize), Some(&mut 2));
        assert_eq!(list.get_mut(3 + list.len() as isize), Some(&mut 3));

        assert!(list.push_back(4).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get_mut(0 + list.len() as isize), Some(&mut 0));
        assert_eq!(list.get_mut(1 + list.len() as isize), Some(&mut 1));
        assert_eq!(list.get_mut(2 + list.len() as isize), Some(&mut 2));
        assert_eq!(list.get_mut(3 + list.len() as isize), Some(&mut 3));
        assert_eq!(list.get_mut(4 + list.len() as isize), Some(&mut 4));

        assert!(list.push_back(5).is_ok());

        println!("{:?}", list);
        assert_eq!(list.get_mut(0 + list.len() as isize), Some(&mut 1));
        assert_eq!(list.get_mut(1 + list.len() as isize), Some(&mut 2));
        assert_eq!(list.get_mut(2 + list.len() as isize), Some(&mut 3));
        assert_eq!(list.get_mut(3 + list.len() as isize), Some(&mut 4));
        assert_eq!(list.get_mut(4 + list.len() as isize), Some(&mut 5));
    }

    #[test]
    fn get_from_back(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get_mut(-1), Some(&mut 3));
        assert_eq!(list.get_mut(-2), Some(&mut 2));
        assert_eq!(list.get_mut(-3), Some(&mut 1));
        assert_eq!(list.get_mut(-4), Some(&mut 0));

        assert!(list.push_back(4).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get_mut(-1), Some(&mut 4));
        assert_eq!(list.get_mut(-2), Some(&mut 3));
        assert_eq!(list.get_mut(-3), Some(&mut 2));
        assert_eq!(list.get_mut(-4), Some(&mut 1));
        assert_eq!(list.get_mut(-5), Some(&mut 0));

        assert!(list.push_back(5).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get_mut(-1), Some(&mut 4));
        assert_eq!(list.get_mut(-2), Some(&mut 3));
        assert_eq!(list.get_mut(-3), Some(&mut 2));
        assert_eq!(list.get_mut(-4), Some(&mut 1));
        assert_eq!(list.get_mut(-5), Some(&mut 5));
    }

    #[test]
    fn get_from_back_overflow(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        assert_eq!(list.get_mut(-1 + -1 * list.len() as  isize), Some(&mut 3));
        assert_eq!(list.get_mut(-2 + -1 * list.len() as  isize), Some(&mut 2));
        assert_eq!(list.get_mut(-3 + -1 * list.len() as  isize), Some(&mut 1));
        assert_eq!(list.get_mut(-4 + -1 * list.len() as  isize), Some(&mut 0));

        assert!(list.push_back(4).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get_mut(-1 + -1 * list.len() as  isize), Some(&mut 4));
        assert_eq!(list.get_mut(-2 + -1 * list.len() as  isize), Some(&mut 3));
        assert_eq!(list.get_mut(-3 + -1 * list.len() as  isize), Some(&mut 2));
        assert_eq!(list.get_mut(-4 + -1 * list.len() as  isize), Some(&mut 1));
        assert_eq!(list.get_mut(-5 + -1 * list.len() as  isize), Some(&mut 0));

        assert!(list.push_back(5).is_ok());
        println!("{:?}", list);
        assert_eq!(list.get_mut(-1 + -1 * list.len() as  isize), Some(&mut 5));
        assert_eq!(list.get_mut(-2 + -1 * list.len() as  isize), Some(&mut 4));
        assert_eq!(list.get_mut(-3 + -1 * list.len() as  isize), Some(&mut 3));
        assert_eq!(list.get_mut(-4 + -1 * list.len() as  isize), Some(&mut 2));
        assert_eq!(list.get_mut(-5 + -1 * list.len() as  isize), Some(&mut 1));
    }

    #[test]
    fn empty(){
        let mut list: List<SIZE, i64, false> = List::default();

        for i in (-2 * SIZE as isize)..(2 * SIZE as isize){
            assert_eq!(None, list.get_mut(i));
        }
    }

    #[test]
    fn update(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        *list.get_mut(0).unwrap() = 4;

        assert_eq!(list.get(0), Some(&4));
        
        *list.get_mut(-1).unwrap() = 5;

        assert_eq!(list.get(-1), Some(&5));
        
        *list.get_mut(4 + list.len() as isize).unwrap() = 6;

        assert_eq!(list.get(-1), Some(&6));
    }
}

mod remove_back{
    use crate::list::List;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let mut list: List<SIZE, usize, true> = List::default();

        assert_eq!(None, list.remove_back());
    }

    #[test]
    fn get_last(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        assert_eq!(Some(3), list.remove_back());
        assert_eq!(Some(2), list.remove_back());
        assert_eq!(Some(1), list.remove_back());
        assert_eq!(Some(0), list.remove_back());
        assert_eq!(None, list.remove_back());

        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,4].try_into().unwrap();
        assert!(list.push_back(5).is_ok());

        assert_eq!(Some(5), list.remove_back());
        assert_eq!(Some(4), list.remove_back());
        assert_eq!(Some(3), list.remove_back());
        assert_eq!(Some(2), list.remove_back());
        assert_eq!(Some(1), list.remove_back());
        assert_eq!(None, list.remove_back());
    }
}

mod remove_front{
    use crate::list::List;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let mut list: List<SIZE, usize, true> = List::default();

        assert_eq!(None, list.remove_front());
    }

    #[test]
    fn get_last(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        assert_eq!(Some(0), list.remove_front());
        assert_eq!(Some(1), list.remove_front());
        assert_eq!(Some(2), list.remove_front());
        assert_eq!(Some(3), list.remove_front());
        assert_eq!(None, list.remove_front());

        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,4].try_into().unwrap();
        assert!(list.push_back(5).is_ok());

        assert_eq!(Some(1), list.remove_front());
        assert_eq!(Some(2), list.remove_front());
        assert_eq!(Some(3), list.remove_front());
        assert_eq!(Some(4), list.remove_front());
        assert_eq!(Some(5), list.remove_front());
        assert_eq!(None, list.remove_front());
    }
}

mod remove_at{
    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn remove_middle(){
        todo!()
    }

    #[test]
    fn remove_front(){
        todo!()
    }

    #[test]
    fn remove_back(){
        todo!()
    }
}

mod iter{
    use crate::list::List;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let list: List<SIZE, i64, false> = List::default();

        let mut iter = list.iter();

        assert_eq!(None, iter.next());

        
        let iter = list.iter();

        assert_eq!(0, iter.len());
    }

    #[test]
    fn iter(){
        let expected: Vec<i64>= vec![1,2,3,4,5];
        let list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();

        let mut actual = list.iter();
        let mut expected = expected.iter();

        for (actual, expected) in (&mut actual).zip(&mut expected){
            assert_eq!(actual, expected);
        }

        if let (None, None) = (actual.next(), expected.next()) {
            assert!(true);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn iter_size(){
        let list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();
        
        let mut iter = list.iter();
        let mut i1 = SIZE;


        assert_eq!(i1, iter.len());

        while let Some(_) = iter.next() {
            i1 = i1 - 1;
        
            assert_eq!(i1, iter.len());
        }
        assert_eq!(0, iter.len());
    }
    
    #[test]
    fn iter_overflow(){
        let expected: Vec<i64>= vec![3,4,5,6,7];
        let mut list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();

        assert!(list.push_back(6).is_ok());
        assert!(list.push_back(7).is_ok());

        let mut actual = list.iter();
        let mut expected = expected.iter();

        for (actual, expected) in (&mut actual).zip(&mut expected){
            assert_eq!(actual, expected);
        }

        if let (None, None) = (actual.next(), expected.next()) {
            assert!(true);
        }
        else {
            assert!(false);
        }

    }

    #[test]
    fn iter_overflow_size(){
        let mut list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();
        
        assert!(list.push_back(6).is_ok());
        assert!(list.push_back(7).is_ok());
        
        let mut iter = list.iter();
        let mut i1 = SIZE;


        assert_eq!(i1, iter.len());

        while let Some(_) = iter.next() {
            i1 = i1 - 1;
        
            assert_eq!(i1, iter.len());
        }
        assert_eq!(0, iter.len());
    }
}

mod iter_mut{
    use crate::list::List;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let mut list: List<SIZE, i64, false> = List::default();

        let mut iter = list.iter_mut();

        assert_eq!(None, iter.next());

        
        let iter = list.iter_mut();

        assert_eq!(0, iter.len());
    }

    #[test]
    fn iter(){
        let expected: Vec<i64>= vec![1,2,3,4,5];
        let list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();

        let mut actual = list.iter();
        let mut expected = expected.iter();

        for (actual, expected) in (&mut actual).zip(&mut expected){
            assert_eq!(actual, expected);
        }

        if let (None, None) = (actual.next(), expected.next()) {
            assert!(true);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn iter_size(){
        let mut list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();
        
        let mut iter = list.iter_mut();
        let mut i1 = SIZE;


        assert_eq!(i1, iter.len());

        while let Some(_) = iter.next() {
            i1 = i1 - 1;
        
            assert_eq!(i1, iter.len());
        }
        assert_eq!(0, iter.len());
    }
    
    #[test]
    fn iter_overflow(){
        let expected: Vec<i64>= vec![3,4,5,6,7];
        let mut list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();

        assert!(list.push_back(6).is_ok());
        assert!(list.push_back(7).is_ok());

        let mut actual = list.iter_mut();
        let mut expected = expected.iter();

        for (actual, expected) in (&mut actual).zip(&mut expected){
            assert_eq!(actual, expected);
        }

        if let (None, None) = (actual.next(), expected.next()) {
            assert!(true);
        }
        else {
            assert!(false);
        }

    }

    #[test]
    fn iter_overflow_size(){
        let mut list: List<SIZE, i64, false> = List::try_from(vec![1,2,3,4,5]).unwrap();
        
        assert!(list.push_back(6).is_ok());
        assert!(list.push_back(7).is_ok());
        
        let mut iter = list.iter_mut();
        let mut i1 = SIZE;


        assert_eq!(i1, iter.len());

        while let Some(_) = iter.next() {
            i1 = i1 - 1;
        
            assert_eq!(i1, iter.len());
        }
        assert_eq!(0, iter.len());
    }

    #[test]
    fn update_val(){
        todo!()
    }
}

mod display{
    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn partially_filled(){
        todo!()
    }

    #[test]
    fn filled(){
        todo!()
    }

    #[test]
    fn overflow(){
        todo!()
    }
}

mod debug{
    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn partially_filled(){
        todo!()
    }

    #[test]
    fn filled(){
        todo!()
    }

    #[test]
    fn overflow(){
        todo!()
    }
}

mod index{
    #[test]
    fn index_out_of_range(){
        todo!()
    }

    #[test]
    fn get(){
        todo!()
    }
}

mod index_mut{
    #[test]
    fn index_out_of_range(){
        todo!()
    }

    #[test]
    fn update(){
        todo!()
    }
}

mod try_from_vec{
    #[test]
    fn empty(){
        todo!()
    }
    
    #[test]
    fn smaller_than_list(){
        todo!()
    }
    
    #[test]
    fn equal_to_list(){
        todo!()
    }

    #[test]
    fn no_overflow(){
        todo!()
    }

    #[test]
    fn overflow(){
        todo!()
    }
}

mod try_from_linked_list{
    #[test]
    fn empty(){
        todo!()
    }
    
    #[test]
    fn smaller_than_list(){
        todo!()
    }
    
    #[test]
    fn equal_to_list(){
        todo!()
    }

    #[test]
    fn no_overflow(){
        todo!()
    }

    #[test]
    fn overflow(){
        todo!()
    }
}

mod try_from_iter{
    #[test]
    fn empty(){
        todo!()
    }
    
    #[test]
    fn smaller_than_list(){
        todo!()
    }
    
    #[test]
    fn equal_to_list(){
        todo!()
    }

    #[test]
    fn no_overflow(){
        todo!()
    }

    #[test]
    fn overflow(){
        todo!()
    }
}

mod from_array{
    #[test]
    fn fill(){
        todo!()
    }
}

mod from_iter{
    #[test]
    fn smaller_overflow(){
        todo!()
    }

    #[test]
    fn smaller_no_overflow(){
        todo!()
    }

    #[test]
    fn greater_overflow(){
        todo!()
    }

    #[test]
    fn greater_no_overflow(){
        todo!()
    }
}

mod swap_write_over{

    #[test]
    fn true_to_false() {
        todo!()
    }

    #[test]
    fn false_to_true() {
        todo!()
    }
}