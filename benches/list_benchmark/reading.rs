use criterion::{black_box, Criterion};
use cyclic_data_types::list::List;
use std::collections::linked_list::LinkedList;
use ringbuffer::{RingBufferExt, RingBufferWrite};
use smallvec::SmallVec;
use arraydeque::{ArrayDeque, Wrapping};
use arrayvec::ArrayVec;
use ring_queue::Ring;


macro_rules! list_reading_benchmark{
    ($SIZE: literal, $c: expr, $small_vec: literal) => {
        //cyclic list
        //no optimize
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );


        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );

        //optimize
        $c.bench_function(
            &format!("Cyclic List (no write_over) (optimization) (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = iter.next() {

                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (optimization) (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = iter.next() {

                        }
                    }
                )
            }
        );


        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = iter.next() {

                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = iter.next() {

                        }
                    }
                )
            }
        );

        //no optimize
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            let _ = black_box(list.get(i as isize));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (get negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list.get(-1 * (i as isize) - 1));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            black_box(list.get(i as isize));
                        }
                    }
                )
            }
        );


        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list.get(i as isize));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (get negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list.get((-1 * i as isize) - 1));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            black_box(list.get(i as isize));
                        }
                    }
                )
            }
        );

        //optimize
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            list.get(i as isize);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (optimization) (get negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            list.get(-1 * i as isize - 1);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (optimization) (get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            list.get(i as isize);
                        }
                    }
                )
            }
        );


        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE{
                            list.get(i as isize);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (get negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE{
                            list.get(-1 * i as isize - 1);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev(){
                            list.get(i as isize);
                        }
                    }
                )
            }
        );

        //no optimize
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (index negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list[-1 * (i as isize) - 1]);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (no optimization) (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );


        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (index negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list[(-1 * i as isize) - 1]);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (no optimization) (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );

        //optimize
        $c.bench_function(
            &format!("Cyclic List (no write_over) (optimization) (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            list[i];
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (optimization) (index negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            list[-1 * i as isize - 1];
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (no write_over) (optimization) (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            list[i];
                        }
                    }
                )
            }
        );


        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE{
                            list[i];
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (index negative) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in 0..$SIZE{
                            list[-1 * i as isize - 1];
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("Cyclic List (write_over) (optimization) (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(
                    move || {
                        for i in (0..$SIZE).rev(){
                            list[i];
                        }
                    }
                )
            }
        );

        //vec
        $c.bench_function(
            &format!("vec (get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    || {
                        for i in 0..$SIZE {
                            black_box(list.get(i));
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("vec (get_unchecked)  (front-back) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    || {
                        for i in 0..$SIZE {
                            unsafe {black_box(list.get_unchecked(i))};
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("vec (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    || {
                        for i in 0..$SIZE {
                            black_box(list[i]);
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("vec (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )

            }
        );
        
        $c.bench_function(
            &format!("vec (get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    || {
                        for i in (0..$SIZE).rev() {
                            black_box(list.get(i));
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("vec (get_unchecked)  (back-front) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    || {
                        for i in (0..$SIZE).rev() {
                            unsafe {black_box(list.get_unchecked(i))};
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("vec (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    || {
                        for i in (0..$SIZE).rev() {
                            black_box(list[i]);
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("vec (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list = Vec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )

            }
        );
        
        //linked list
        $c.bench_function(
            &format!("linked_list (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list = LinkedList::new();

                for i in 0..$SIZE {
                    black_box(list.push_back(black_box(i)));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );

        $c.bench_function(
            &format!("linked_list (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list = LinkedList::new();

                for i in 0..$SIZE {
                    black_box(list.push_back(black_box(i)));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );

        //small vec
        $c.bench_function(
            &format!("small vec (get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list.get(i));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("small vec (get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            black_box(list.get(i));
                        }
                    }
                )
            }
        );
        
        $c.bench_function(
            &format!("small vec (unchecked_get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            unsafe {black_box(list.get_unchecked(i))};
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("small vec (unchecked_get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            unsafe {black_box(list.get_unchecked(i))};
                        }
                    }
                )
            }
        );
        
        $c.bench_function(
            &format!("small vec (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("small vec (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );
        
        $c.bench_function(
            &format!("small vec (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("small vec (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );

        //arrayvec
        $c.bench_function(
            &format!("arrayvec (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayVec<usize, $SIZE> = ArrayVec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("arrayvec (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayVec<usize, $SIZE> = ArrayVec::new();

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );

        //ring queue
        $c.bench_function(
            &format!("ring queue (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list = Ring::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in 0..$SIZE {
                            black_box(list[i as isize]);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("ring queue (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list = Ring::with_capacity($SIZE);

                for i in 0..$SIZE {
                    list.push(black_box(i));
                }
                b.iter(
                    move || {
                        for i in (0..$SIZE).rev() {
                            black_box(list[i as isize]);
                        }
                    }
                )
            }
        );

    };
    ($SIZE: literal, $c: expr, $small_vec: literal, $array_deque: literal) => {
        list_reading_benchmark!($SIZE,$c,$small_vec);
        //array deque
        $c.bench_function(
            &format!("array deque (iter) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

                for i in 0..$SIZE {
                    list.push_back(black_box(i));
                }

                b.iter(
                    || {
                        let mut iter = list.iter();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("array deque (iter) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

                for i in 0..$SIZE {
                    list.push_back(black_box(i));
                }

                b.iter(
                    || {
                        let mut iter = list.iter().rev();
                        while let Some(_) = black_box(iter.next()) {

                        }
                    }
                )
            }
        );
        
        $c.bench_function(
            &format!("array deque (get) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

                for i in 0..$SIZE {
                    list.push_back(black_box(i));
                }

                b.iter(
                    || {
                        for i in 0..$SIZE {
                            black_box(list.get(i));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("array deque (get) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

                for i in 0..$SIZE {
                    list.push_back(black_box(i));
                }

                b.iter(
                    || {
                        for i in (0..$SIZE).rev() {
                            black_box(list.get(i));
                        }
                    }
                )
            }
        );
        
        $c.bench_function(
            &format!("array deque (index) (front-back) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

                for i in 0..$SIZE {
                    list.push_back(black_box(i));
                }

                b.iter(
                    || {
                        for i in 0..$SIZE {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("array deque (index) (back-front) - reading {}", $SIZE),
            |b| {
                let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

                for i in 0..$SIZE {
                    list.push_back(black_box(i));
                }

                b.iter(
                    || {
                        for i in (0..$SIZE).rev() {
                            black_box(list[i]);
                        }
                    }
                )
            }
        );
    }
    
}
pub fn list_reading_benchmark(c: &mut Criterion) {
    list_reading_benchmark!(8usize, c, 8, 10);
    list_reading_benchmark!(16usize, c, 16, 100);
    list_reading_benchmark!(32usize, c, 32, 100);
    list_reading_benchmark!(64usize, c, 64, 100);
    list_reading_benchmark!(128usize, c, 128, 128);
    list_reading_benchmark!(256usize, c, 256, 1024);
    list_reading_benchmark!(512usize, c, 512, 1024);
    list_reading_benchmark!(1024usize, c, 1024, 1024);
    list_reading_benchmark!(2048usize, c, 2048);
    list_reading_benchmark!(4096usize, c, 4096);
    list_reading_benchmark!(8192usize, c, 8192);
    list_reading_benchmark!(16384usize, c, 16384);
}