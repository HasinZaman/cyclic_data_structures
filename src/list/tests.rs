mod len{
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

            match i as usize / SIZE {
                0 => {
                    assert_eq!(list.len(), (i + 1) as usize);
                },
                _=> {
                    assert_eq!(list.len(), SIZE);
                }
            };
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

            match i as usize / SIZE {
                0 => {
                    assert_eq!(list.len(), (i + 1) as usize);
                },
                _=> {
                    assert_eq!(list.len(), SIZE);
                }
            };
        }
    }
}

mod insert_at{
    use crate::list::List;

    const SIZE: usize = 5;
    
    #[test]
    fn push_back() {
        todo!()
    }
    
    #[test]
    fn push_front() {
        todo!()
    }

    #[test]
    fn insert_middle() {
        let mut list: List<SIZE, usize, false> = vec![1,2,3].try_into().unwrap();

        assert!(list.insert_at(10, 1).is_ok());

        assert_eq!(list, vec![1,10,2,3].try_into().unwrap());
        
        assert!(list.insert_at(10, 3).is_ok());

        assert_eq!(list, vec![1,10,2,10,3].try_into().unwrap());
    }

    #[test]
    fn overflow() {
        let mut list: List<SIZE, usize, true> = vec![1,2,3,4,5].try_into().unwrap();

        assert!(list.insert_at(10, 0).is_ok());

        assert_eq!(list, vec![10,1,2,3,4].try_into().unwrap());
        
        assert!(list.insert_at(10, 5).is_ok());

        assert_eq!(list, vec![10,1,2,3,10].try_into().unwrap());
        
        assert!(list.insert_at(10, 2).is_ok());

        assert_eq!(list, vec![1,2,10,3,10].try_into().unwrap());
    }

    #[test]
    fn no_overflow(){
        todo!()
    }

    #[test]
    fn index_out_of_range(){
        todo!()
    }
}

mod push_back{
    use crate::list::List;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn fill(){
        todo!()
    }

    fn overflow(){
        todo!()
    }

    fn no_overflow(){
        todo!()
    }
}

mod push_front{
    use crate::list::List;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        todo!()
    }

    #[test]
    fn fill(){
        todo!()
    }

    fn overflow(){
        todo!()
    }

    fn no_overflow(){
        todo!()
    }
}

mod get{
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

mod Display{
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

mod Debug{
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

mod Index{
    #[test]
    fn index_out_of_range(){
        todo!()
    }

    #[test]
    fn get(){
        todo!()
    }
}

mod IndexMut{
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