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
        let mut list: List<SIZE, i64, true> = List::default();
        let mut expected = vec![];

        for i in 1..=10{
            assert!(list.insert_at(i, list.len()).is_ok());
            println!("{}\n{:?}", i, list);
            match i as usize {
                0..=SIZE => {
                    expected.push(i);
                },
                _=>{
                    expected.remove(0);
                    expected.push(i);
                }
            }

            assert_eq!(list.len(), min(i as usize, SIZE));
            assert_eq!(
                list,
                List::<SIZE, i64, true>::try_from(expected.clone()).unwrap()
            );
        }
    }
    
    #[test]
    fn push_front() {
        let mut list: List<SIZE, i64, true> = List::default();
        let mut expected = vec![];

        for i in 1..=10{
            assert!(list.insert_at(i, 0).is_ok());
            println!("{}\n{:?}", i, list);
            match i as usize {
                0..=SIZE => {
                    expected.insert(0, i);
                },
                _=>{
                    expected.pop();
                    expected.insert(0, i);
                }
            }

            assert_eq!(list.len(), min(i as usize, SIZE));
            assert_eq!(
                list,
                List::<SIZE, i64, true>::try_from(expected.clone()).unwrap()
            );
        }
    }

    #[test]
    fn insert_middle() {
        let mut list: List<SIZE, i64, true> = vec![1,2,3].try_into().unwrap();
        let mut expected = vec![1,2,3];

        for i in 1..=10{
            assert!(list.insert_at(10, 1).is_ok());
            println!("{}\n{:?}", i, list);
            match list.len() {
                0..=SIZE => {
                    expected.insert(1, 10);
                    assert_eq!(list.len(), i as usize + 3);
                },
                _=>{
                    expected.pop();
                    expected.insert(1, 10);
                }
            }

            assert_eq!(list.len(), min(i as usize + 3, SIZE));
            assert_eq!(
                list,
                List::<SIZE, i64, true>::try_from(expected.clone()).unwrap()
            );
        }
    }

    #[test]
    fn no_overflow(){
        let mut list: List<SIZE, i64, false> = vec![1,2,3,4,5].try_into().unwrap();
        
        assert_eq!(Err(Error::Overflow), list.insert_at(10, 1));
    }

    #[test]
    fn index_out_of_range(){
        let mut list: List<SIZE, i64, false> = vec![1,].try_into().unwrap();
        
        assert_eq!(Err(Error::IndexOutOfRange), list.insert_at(10, 4));
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
    use crate::{list::List, error::Error};

    const SIZE: usize = 5;
    #[test]
    fn get(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        for i in 0..list.len(){
            println!("get at({i})");
            assert_eq!(list.get(i as isize), Some(&i));
        }

        assert!(list.push_back(4).is_ok());

        println!("{:?}", list);
        for i in 0..list.len(){
            println!("get at({i})");
            assert_eq!(list.get(i as isize), Some(&i));
        }

        assert!(list.push_back(5).is_ok());

        println!("{:?}", list);
        for i in 0..list.len(){
            println!("get at({i})");
            assert_eq!(list.get(i as isize), Some(&(i + 1)));
        }
    }

    #[test]
    fn get_overflow(){
        let mut list: List<SIZE, usize, true> = vec![0,1,2,3,].try_into().unwrap();

        println!("{:?}", list);
        for i in list.len()..list.len()*2{
            println!("get at({i})");
            assert_eq!(list.get(i as isize), Some(&(i%list.len())));
        }

        assert!(list.push_back(4).is_ok());

        println!("{:?}", list);
        for i in list.len()..list.len()*2{
            println!("get at({i})");
            assert_eq!(list.get(i as isize), Some(&(i%list.len())));
        }

        assert!(list.push_back(5).is_ok());

        println!("{:?}", list);
        for i in list.len()..list.len()*2{
            println!("get at({i})");
            assert_eq!(list.get(i as isize), Some(&((i+1)%list.len())));
        }
    }

    #[test]
    fn get_from_back(){
        todo!();
    }

    #[test]
    fn get_from_back_overflow(){
        todo!()
    }

    #[test]
    fn empty(){
        let list: List<SIZE, i64, false> = List::default();

        for i in 0..SIZE{
            assert_eq!(None, list.get(i as isize));
        }
    }
}

mod get_mut{
    
    #[test]
    fn get(){
        todo!()
    }

    #[test]
    fn get_overflow(){
        todo!()
    }

    #[test]
    fn get_from_back(){
        todo!()
    }

    #[test]
    fn get_from_back_overflow(){
        todo!()
    }

    #[test]
    fn update(){
        todo!()
    }
}

mod remove_back{
    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn get_last(){
        todo!()
    }
}

mod remove_front{
    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn get_last(){
        todo!()
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
    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn iter(){
        todo!()
    }
    
    #[test]
    fn iter_over_overflow(){
        todo!()
    }
}

mod iter_mut{
    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn iter(){
        todo!()
    }
    
    #[test]
    fn iter_over_overflow(){
        todo!()
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