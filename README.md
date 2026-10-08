# Stable Intrinsics (`stable_intr`)

Replaces nightly-only `core::intrinsics` with stable functions. These intrinsics have roughly the same codegen.

## Usage

Use `nontemporal_store` for fast, uncached, unsynchronized writes:

```rust
const LEN: usize = 64;

let a = 123u32;
let mut b = 4;

let mut arr = MaybeUninit::<[u32; LEN]>::uninit();
let ptr = arr.as_mut_ptr().cast::<u32>();

for i in 0..LEN {
    let ptr = unsafe { ptr.add(i) };

    let val = unsafe { disjoint_bitor(a, b) };

    unsafe {
        nontemporal_store(ptr, val);
    }

    let new_b = b.wrapping_shl(16);
    b = select_unpredictable(new_b == 0, 4, new_b);
}

nontemporal_fence();

let arr = unsafe { arr.assume_init() };

for (i, &actual) in arr.iter().enumerate() {
    let b = if i % 2 == 0 { 4 } else { 4 << 16 };
    let expected = 123 | b;

    assert_eq!(
        actual, expected,
        "incorrect value at index {i}: got {actual}, expected {expected}"
    );
}

println!("{arr:?}");
```

Use a generic `disjoint_bitor` in const contexts on stable Rust:

```rust
use stable_intr::disjoint_bitor;

const C: u8 = unsafe { disjoint_bitor(123, 4) };
const C: u128 = unsafe { disjoint_bitor(123, 4) };
const C: u32 = unsafe { disjoint_bitor(123, 4) };
const C: usize = unsafe { disjoint_bitor(123, 4) };
```

## Compile Error on Nightly Build

Rust makes to promise of stability on nightly builds, so if this crate fails to compile:

1. try disabling `nightly` feature to stop this crate from using `core::intrinsics`
2. try disabling other unused features to avoid compiling unnecessary intrinsics
3. just use stable rust

### Nightly Detection vs Feature

This crate automatically detects nightly Rust and will not use `core::intrinsics` if it detects another toolchain.

To let this crate use those intrinsics even on nightly, it requires you to explicitly enable the `nightly` feature. Without this, it treats the build as stable and compiles safe fallbacks.
