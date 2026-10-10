use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use stable_intr::{disjoint_bitor, nontemporal_fence, nontemporal_store, select_unpredictable};
use core::hint::black_box;
use std::mem::MaybeUninit;

const SIZES: &[usize] = &[
    1 << 10,
    1 << 12,
    1 << 14,
    1 << 16,
    1 << 18,
    1 << 20,
    1 << 22,
    1 << 24,
    1 << 26,
    1 << 28,
    1 << 30,
];

#[inline(never)]
fn cached_store(arr: &mut [MaybeUninit<u32>]) {
    let a = black_box(123u32);
    let mut b = black_box(4u32);

    for i in 0..arr.len() {
        let val = unsafe { disjoint_bitor(a, b) };

        unsafe {
            arr.as_mut_ptr().cast::<u32>().add(i).write(val);
        }

        let new_b = b.wrapping_shl(16);
        b = select_unpredictable(new_b == 0, 4, new_b);
    }

    black_box(arr);
}

#[inline(never)]
fn nontemporal_store_bench(arr: &mut [MaybeUninit<u32>]) {
    let a = black_box(123u32);
    let mut b = black_box(4u32);

    let ptr = arr.as_mut_ptr();

    for i in 0..arr.len() {
        let val = unsafe { disjoint_bitor(a, b) };

        unsafe {
            nontemporal_store(ptr.cast::<u32>().add(i), val);
        }

        let new_b = b.wrapping_shl(16);
        b = select_unpredictable(new_b == 0, 4, new_b);
    }

    black_box(arr);
}

fn write_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_only");

    for &size in SIZES {
        let elements = size / size_of::<u32>();

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("cached", format_size(size)),
            &elements,
            |b, &elements| {
                let mut arr = Box::new_uninit_slice(elements);

                b.iter(|| {
                    cached_store(black_box(&mut arr));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("nontemporal", format_size(size)),
            &elements,
            |b, &elements| {
                let mut arr = Box::new_uninit_slice(elements);

                b.iter(|| {
                    nontemporal_store_bench(black_box(&mut arr));
                });
            },
        );
    }

    group.finish();
}

#[inline(never)]
fn cached_store_then_read(arr: &mut [MaybeUninit<u32>]) -> u32 {
    cached_store(arr);

    let mut sum = 0u32;

    for &x in unsafe {
        arr.assume_init_ref().iter()
    } {
        sum = sum.wrapping_add(x);
    }

    black_box(sum)
}

#[inline(never)]
fn nontemporal_store_then_read(arr: &mut [MaybeUninit<u32>]) -> u32 {
    nontemporal_store_bench(arr);

    nontemporal_fence();

    let mut sum = 0u32;

    for &x in unsafe {
        arr.assume_init_ref().iter()
    } {
        sum = sum.wrapping_add(x);
    }

    black_box(sum)
}

fn write_then_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_then_read");

    for &size in SIZES {
        let elements = size / size_of::<u32>();

        group.throughput(Throughput::Bytes((size * 2) as u64));

        group.bench_with_input(
            BenchmarkId::new("cached", format_size(size)),
            &elements,
            |b, &elements| {
                let mut arr = Box::new_uninit_slice(elements);

                b.iter(|| {
                    black_box(cached_store_then_read(black_box(&mut arr)));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("nontemporal", format_size(size)),
            &elements,
            |b, &elements| {
                let mut arr = Box::new_uninit_slice(elements);

                b.iter(|| {
                    black_box(nontemporal_store_then_read(black_box(&mut arr)));
                });
            },
        );
    }

    group.finish();
}

fn format_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB"];

    let unit = (bytes.ilog2() / 10) as usize;
    let value = bytes >> (unit * 10);

    format!("{value} {}", UNITS[unit])
}

criterion_group!(benches, write_only, write_then_read);
criterion_main!(benches);
