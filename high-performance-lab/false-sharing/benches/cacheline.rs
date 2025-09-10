// #![cfg_attr(test, allow(unused))]
// #![feature(test)]
use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

use criterion::{Criterion, criterion_group, criterion_main};
use crossbeam::utils::CachePadded;

const N: u64 = 1_000_000;
const THREADS: usize = 4;

// 伪共享结构
struct FalseSharing {
    counters: [AtomicU64; THREADS],
}

impl FalseSharing {
    fn new() -> Self {
        Self {
            counters: Default::default(),
        }
    }
}

// 缓存行填充结构
struct PaddedSharing {
    counters: [CachePadded<AtomicU64>; THREADS],
}

impl PaddedSharing {
    fn new() -> Self {
        Self {
            counters: Default::default(),
        }
    }
}

fn run_false_sharing() {
    // 临时值 + 引用同时出现 → Rust 自动延长临时值生命周期
    // Rust 有一个特殊规则：临时值引用提升（temporary lifetime extension）：
    // 当你对一个临时值（FalseSharing::new() 返回的值）做借用（取 & 或
    // &mut）时，Rust 会 自动延长这个临时值的生命周期，直到
    // 最后一个使用该引用的地方结束。
    // 所以即使 FalseSharing::new() 返回的是一个临时值，它也不会在 let data = &...
    // 之后立即被销毁。

    let data = &FalseSharing::new();
    // 你可以在作用域里创建线程，并把对局部变量的引用传进去。
    // Rust 保证这些线程 在 scope 退出前必须结束。
    // 所以，data 的生命周期（&FalseSharing）被“绑定”在这个 scope
    // 的生命周期里，不会出现悬垂引用。
    thread::scope(|s| {
        for i in 0..THREADS {
            s.spawn(move || {
                for _ in 0..N {
                    data.counters[i].fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
}

fn run_padded_sharing() {
    let data = &PaddedSharing::new();
    thread::scope(|s| {
        for i in 0..THREADS {
            s.spawn(move || {
                for _ in 0..N {
                    data.counters[i].fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
}

fn bench_cacheline(c: &mut Criterion) {
    let mut group = c.benchmark_group("cacheline");
    group.bench_function("false_sharing", |b| b.iter(run_false_sharing));
    group.bench_function("padded_sharing", |b| b.iter(run_padded_sharing));
    group.finish();
}

criterion_group!(benches, bench_cacheline);
criterion_main!(benches);
