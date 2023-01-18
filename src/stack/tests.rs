mod len {
    use std::cmp::min;

    use crate::stack::Stack;

    const SIZE: usize = 5;

    #[test]
    fn empty() {
        let list: Stack<SIZE, i64, false> = Stack::default();

        assert_eq!(list.len(), 0);
    }

    #[test]
    fn partial_size_push() {
        let mut list: Stack<SIZE, i64, false> = Stack::default();

        for i in 0..5 {
            assert!(list.push(i).is_ok());
            assert_eq!(list.len(), (i + 1) as usize);
        }
    }

    #[test]
    fn overflow_push() {
        let mut list: Stack<SIZE, i64, true> = Stack::default();

        for i in 0..10 {
            assert!(list.push(i).is_ok());

            assert_eq!(list.len(), min(i as usize + 1, SIZE));
        }
    }
}

mod push {
    use crate::{error::Error, list::List, stack::Stack};

    const SIZE: usize = 5;

    #[test]
    fn empty() {
        let mut list: Stack<SIZE, i64, false> = Stack::default();

        let expect = vec![].try_into().unwrap();
        assert_eq!(list, expect);

        assert!(list.push(1).is_ok());
        let expect = vec![1].try_into().unwrap();
        assert_eq!(list, expect);
    }

    #[test]
    fn fill() {
        let mut list: Stack<SIZE, i64, false> = Stack::default();

        let expect = vec![].try_into().unwrap();
        assert_eq!(list, expect);

        assert!(list.push(1).is_ok());
        let expect = vec![1].try_into().unwrap();
        assert_eq!(list, expect);

        assert!(list.push(2).is_ok());
        let expect = vec![1, 2].try_into().unwrap();
        assert_eq!(list, expect);

        assert!(list.push(3).is_ok());
        let expect = vec![1, 2, 3].try_into().unwrap();
        assert_eq!(list, expect);

        assert!(list.push(4).is_ok());
        let expect = vec![1, 2, 3, 4].try_into().unwrap();
        assert_eq!(list, expect);

        assert!(list.push(5).is_ok());
        let expect = vec![1, 2, 3, 4, 5].try_into().unwrap();
        assert_eq!(list, expect);
    }

    #[test]
    fn overflow() {
        let mut stack: Stack<SIZE, i64, true> = Stack::default();

        assert!(stack.push(1).is_ok());

        assert!(stack.push(2).is_ok());

        assert!(stack.push(3).is_ok());

        assert!(stack.push(4).is_ok());

        assert!(stack.push(5).is_ok());

        assert!(stack.push(6).is_ok());
        let expect = [2, 3, 4, 5, 6].try_into().unwrap();
        assert_eq!(stack, expect);
    }

    #[test]
    fn no_overflow() {
        let mut list: List<SIZE, i64, false> = vec![1, 2, 3, 4, 5].try_into().unwrap();

        assert_eq!(Err(Error::Overflow), list.push_back(10 as i64));
    }
}

mod peek {
    use crate::stack::Stack;

    const SIZE: usize = 5;

    #[test]
    fn empty() {
        let stack: Stack<SIZE, i64, false> = Stack::default();

        assert_eq!(stack.peek(), None)
    }

    #[test]
    fn partially_filled() {
        let stack: Stack<SIZE, i64, false> = vec![1, 2, 3].try_into().unwrap();

        assert_eq!(stack.peek(), Some(&3));
    }

    #[test]
    fn filled() {
        let stack: Stack<SIZE, i64, false> = vec![1, 2, 3, 4, 5].try_into().unwrap();

        assert_eq!(stack.peek(), Some(&5));
    }

    #[test]
    fn overflow() {
        let mut stack: Stack<SIZE, i64, true> = vec![1, 2, 3, 4, 5].try_into().unwrap();

        assert!(stack.push(6).is_ok());

        assert_eq!(stack.peek(), Some(&6));
    }
}

mod pop {
    use crate::stack::Stack;

    const SIZE: usize = 5;

    #[test]
    fn empty() {
        let mut stack: Stack<SIZE, i64, false> = Stack::default();

        assert_eq!(stack.pop(), None)
    }

    #[test]
    fn partially_filled() {
        let mut stack: Stack<SIZE, i64, false> = vec![1, 2, 3].try_into().unwrap();

        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn filled() {
        let mut stack: Stack<SIZE, i64, false> = vec![1, 2, 3, 4, 5].try_into().unwrap();

        assert_eq!(stack.pop(), Some(5));
    }

    #[test]
    fn overflow() {
        let mut stack: Stack<SIZE, i64, true> = vec![1, 2, 3, 4, 5].try_into().unwrap();

        assert!(stack.push(6).is_ok());

        assert_eq!(stack.pop(), Some(6));
    }
}

mod read {
    use crate::{error::Error, stack::Stack};

    const SIZE: usize = 5;
    #[test]
    fn empty() {
        let stack: Stack<SIZE, i64, false> = Stack::default();

        assert_eq!(Err(Error::IndexOutOfRange), stack.read(0));
        assert_eq!(Err(Error::IndexOutOfRange), stack.read(1));
        assert_eq!(Err(Error::IndexOutOfRange), stack.read(2));
    }

    #[test]
    fn partially_filled() {
        let stack: Stack<SIZE, i64, false> = vec![1, 2, 3].try_into().unwrap();

        assert_eq!(Ok(&3), stack.read(0));
        assert_eq!(Ok(&2), stack.read(1));
        assert_eq!(Ok(&1), stack.read(2));
        assert_eq!(Err(Error::IndexOutOfRange), stack.read(3));
    }

    #[test]
    fn filled() {
        let stack: Stack<SIZE, i64, false> = vec![1, 2, 3, 4, 5].try_into().unwrap();

        assert_eq!(Ok(&5), stack.read(0));
        assert_eq!(Ok(&4), stack.read(1));
        assert_eq!(Ok(&3), stack.read(2));
        assert_eq!(Ok(&2), stack.read(3));
        assert_eq!(Ok(&1), stack.read(4));

        assert_eq!(Err(Error::IndexOutOfRange), stack.read(5));
    }

    #[test]
    fn overflow() {
        let mut stack: Stack<SIZE, i64, true> = vec![1, 2, 3, 4, 5].try_into().unwrap();

        assert!(stack.push(6).is_ok());

        assert_eq!(Ok(&6), stack.read(0));
        assert_eq!(Ok(&5), stack.read(1));
        assert_eq!(Ok(&4), stack.read(2));
        assert_eq!(Ok(&3), stack.read(3));
        assert_eq!(Ok(&2), stack.read(4));

        assert_eq!(Err(Error::IndexOutOfRange), stack.read(5));
    }
}

mod display {
    use crate::stack::Stack;

    const SIZE: usize = 5;

    #[test]
    fn empty() {
        let list: Stack<SIZE, i64, false> = Stack::default();

        let actual = list.to_string();

        assert_eq!(actual, "[]");
    }
    #[test]
    fn one_element() {
        let list: Stack<SIZE, i64, false> = Stack::try_from(vec![1]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[1]");
    }

    #[test]
    fn partially_filled() {
        let list: Stack<SIZE, i64, false> = Stack::try_from(vec![1, 2, 3]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[1, 2, 3]");
    }

    #[test]
    fn filled() {
        let list: Stack<SIZE, i64, false> = Stack::try_from(vec![1, 2, 3, 4, 5]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[1, 2, 3, 4, 5]");
    }

    #[test]
    fn overflow() {
        let list: Stack<SIZE, i64, true> = Stack::try_from(vec![1, 2, 3, 4, 5, 6]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[2, 3, 4, 5, 6]");
    }
}

mod debug {
    use crate::stack::Stack;

    const SIZE: usize = 5;

    #[test]
    fn empty() {
        let list: Stack<SIZE, i64, false> = Stack::default();

        let actual = format!("{list:?}");

        assert_eq!("Stack { : List { : CyclicList { list: [None, None, None, None, None], start: 0, end: 0, size: 0 } } }", actual)
    }

    #[test]
    fn partially_filled() {
        let list: Stack<SIZE, i64, false> = Stack::try_from(vec![1, 2, 3]).unwrap();

        let actual = format!("{list:?}");

        assert_eq!("Stack { : List { : CyclicList { list: [Some(1), Some(2), Some(3), None, None], start: 0, end: 2, size: 3 } } }", actual);
    }

    #[test]
    fn filled() {
        let list: Stack<SIZE, i64, false> = Stack::try_from(vec![1, 2, 3, 4, 5]).unwrap();

        let actual = format!("{list:?}");

        assert_eq!("Stack { : List { : CyclicList { list: [Some(1), Some(2), Some(3), Some(4), Some(5)], start: 0, end: 4, size: 5 } } }", actual);
    }

    #[test]
    fn overflow() {
        let mut list: Stack<SIZE, i64, true> = Stack::try_from(vec![1, 2, 3, 4, 5]).unwrap();

        assert!(list.push(6).is_ok());
        let actual = format!("{list:?}");
        assert_eq!("Stack { : List { : CyclicList { list: [Some(6), Some(2), Some(3), Some(4), Some(5)], start: 1, end: 0, size: 5 } } }", actual)
    }
}
