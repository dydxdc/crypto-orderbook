use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use orderbook::orderbook::LinkedList;
use std::collections::{LinkedList as StdLinkedList, VecDeque};

const N: usize = 200_000; // keep memory reasonable

fn build_custom(n: usize) -> LinkedList<u64, u64> {
    let mut list = LinkedList::new();
    for i in 0..n as u64 {
        // key == value for simplicity
        let _ = list.push_back(i, i);
    }
    list
}

fn build_std(n: usize) -> StdLinkedList<u64> {
    let mut list = StdLinkedList::new();
    for i in 0..n as u64 {
        list.push_back(i);
    }
    list
}

fn build_vecdeque(n: usize) -> VecDeque<u64> {
    let mut dq = VecDeque::with_capacity(n);
    for i in 0..n as u64 {
        dq.push_back(i);
    }
    dq
}

fn bench_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan_all");
    group.throughput(Throughput::Elements(N as u64));

    group.bench_function("ours_linkedlist_iter", |b| {
        b.iter_batched(
            || build_custom(N),
            |list| {
                let sum: u64 = list.iter().map(|(_, v)| *v).sum();
                black_box(sum)
            },
            BatchSize::LargeInput,
        )
    });

    group.bench_function("std_linkedlist_iter", |b| {
        b.iter_batched(
            || build_std(N),
            |list| {
                let sum: u64 = list.into_iter().sum();
                black_box(sum)
            },
            BatchSize::LargeInput,
        )
    });

    group.bench_function("vecdeque_iter", |b| {
        b.iter_batched(
            || build_vecdeque(N),
            |dq| {
                let sum: u64 = dq.into_iter().sum();
                black_box(sum)
            },
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_scan);
criterion_main!(benches);


