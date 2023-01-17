
mod display {
    use crate::queue::Queue;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let list: Queue<SIZE, i64, false> = Queue::default();

        let actual = list.to_string();

        assert_eq!(actual, "[]");
    }
    #[test]
    fn one_element(){
        let list: Queue<SIZE, i64, false> = Queue::try_from(vec![1]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[1]");
    }

    #[test]
    fn partially_filled(){
        let list: Queue<SIZE, i64, false> = Queue::try_from(vec![1,2,3]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[1, 2, 3]");
    }

    #[test]
    fn filled(){
        let list: Queue<SIZE, i64, false> = Queue::try_from(vec![1,2,3,4,5]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[1, 2, 3, 4, 5]");
    }

    #[test]
    fn overflow(){
        let list: Queue<SIZE, i64, true> = Queue::try_from(vec![1,2,3,4,5, 6]).unwrap();

        let actual = list.to_string();

        assert_eq!(actual, "[2, 3, 4, 5, 6]");
    }
}

mod debug {
    use crate::queue::Queue;

    const SIZE: usize = 5;

    #[test]
    fn empty(){
        let list: Queue<SIZE, i64, false> = Queue::default();

        let actual = format!("{list:?}");

        assert_eq!("Queue { : List { : CyclicList { list: [None, None, None, None, None], start: 0, end: 0, size: 0 } } }", actual)
    }

    #[test]
    fn partially_filled(){
        let list: Queue<SIZE, i64, false> = Queue::try_from(vec![1,2,3]).unwrap();

        let actual = format!("{list:?}");

        assert_eq!("Queue { : List { : CyclicList { list: [Some(1), Some(2), Some(3), None, None], start: 0, end: 2, size: 3 } } }", actual);
    }

    #[test]
    fn filled(){
        let list: Queue<SIZE, i64, false> = Queue::try_from(vec![1,2,3,4,5]).unwrap();

        let actual = format!("{list:?}");

        assert_eq!("Queue { : List { : CyclicList { list: [Some(1), Some(2), Some(3), Some(4), Some(5)], start: 0, end: 4, size: 5 } } }", actual);
    }

    #[test]
    fn overflow(){
        let mut list: Queue<SIZE, i64, true> = Queue::try_from(vec![1,2,3,4,5]).unwrap();

        assert!(list.enqueue(6).is_ok());
        let actual = format!("{list:?}");
        assert_eq!("Queue { : List { : CyclicList { list: [Some(6), Some(2), Some(3), Some(4), Some(5)], start: 1, end: 0, size: 5 } } }", actual)
    }
}
