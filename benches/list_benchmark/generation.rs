use criterion::{black_box, Criterion};
use cyclic_data_types::list::List;
use std::collections::linked_list::LinkedList;
use ringbuffer::{AllocRingBuffer, RingBufferWrite};
use smallvec::SmallVec;
use circular_queue::CircularQueue;
use arraydeque::{ArrayDeque, Wrapping};
use arrayvec::ArrayVec;
use ring_queue::Ring;

macro_rules! list_generation_benchmark{
    ($SIZE: literal, $c: expr, $small_vec: literal) => {
        //cyclic list
        $c.bench_function(
            &format!("Cyclic List (default) (no write_over) (no optimization) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: List<$SIZE, usize, false> = List::default();

                        for i in 0..$SIZE {
                            let _ = black_box(list.push_back(black_box(i)));
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("Cyclic List (default) (write_over) (no optimization) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: List<$SIZE, usize, true> = List::default();

                        for i in 0..$SIZE {
                            let _ = black_box(list.push_back(black_box(i)));
                        }
                    }
                )

            }
        );

        $c.bench_function(
            &format!("Cyclic List (default) (no write_over) (optimization) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: List<$SIZE, usize, false> = List::default();

                        for i in 0..$SIZE {
                            let _ = list.push_back(black_box(i));
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("Cyclic List (default) (write_over) (optimization) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: List<$SIZE, usize, true> = List::default();

                        for i in 0..$SIZE {
                            let _ = list.push_back(black_box(i));
                        }
                    }
                )

            }
        );

        //vec
        $c.bench_function(
            &format!("vec (new) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list = Vec::new();

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )

            }
        );
        $c.bench_function(
            &format!("vec (from_alloc) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list = Vec::with_capacity($SIZE);

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );

        //linked list
        $c.bench_function(
            &format!("linked_list - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list = LinkedList::new();

                        for i in 0..$SIZE {
                            black_box(list.push_back(black_box(i)));
                        }
                    }
                )
            }
        );

        //ring buffer
        $c.bench_function(
            &format!("Ring buffer - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list = AllocRingBuffer::with_capacity($SIZE);

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );

        //small vec
        $c.bench_function(
            &format!("small vec (new) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: SmallVec<[_; $small_vec]> = SmallVec::new();

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("small vec (with_capacity) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );

        //circular queue
        $c.bench_function(
            &format!("circular queue - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list = CircularQueue::with_capacity($SIZE);

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );

        //arrayvec
        $c.bench_function(
            &format!("arrayvec (push) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: ArrayVec<usize, $SIZE> = ArrayVec::new();

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("arrayvec (try push) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: ArrayVec<usize, $SIZE> = ArrayVec::new();

                        for i in 0..$SIZE {
                            let _ = black_box(list.try_push(black_box(i)));
                        }
                    }
                )
            }
        );

        //ring queue
        $c.bench_function(
            &format!("ring queue (new) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list = Ring::new();

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );
        $c.bench_function(
            &format!("ring queue (with_capacity) - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list = Ring::with_capacity($SIZE);

                        for i in 0..$SIZE {
                            black_box(list.push(black_box(i)));
                        }
                    }
                )
            }
        );

    };
    ($SIZE: literal, $c: expr, $small_vec: literal, $array_deque: literal) => {
        list_generation_benchmark!($SIZE,$c,$small_vec);
        //array deque
        $c.bench_function(
            &format!("array deque - generate {}", $SIZE),
            |b| {

                b.iter(
                    || {
                        let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

                        for i in 0..$SIZE {
                            black_box(list.push_back(black_box(i)));
                        }
                    }
                )
            }
        );
    }
    
}
pub fn list_generation_benchmark(c: &mut Criterion) {
    list_generation_benchmark!(8usize, c, 8, 10);
    list_generation_benchmark!(16usize, c, 16, 100);
    list_generation_benchmark!(32usize, c, 32, 100);
    list_generation_benchmark!(64usize, c, 64, 100);
    list_generation_benchmark!(128usize, c, 128, 128);
    list_generation_benchmark!(256usize, c, 256, 1024);
    list_generation_benchmark!(512usize, c, 512, 1024);
    list_generation_benchmark!(1024usize, c, 1024, 1024);
    list_generation_benchmark!(2048usize, c, 2048);
    list_generation_benchmark!(4096usize, c, 4096);
    list_generation_benchmark!(8192usize, c, 8192);
    list_generation_benchmark!(16384usize, c, 16384);
}