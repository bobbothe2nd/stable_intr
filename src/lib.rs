//! Reimplementations of [`core::intrinsics`] for stable Rust
//!
//! This crate aims to provide roughly the same codegen as using actual intrinsics regardless of whether your using stable or nightly rust.

#![cfg_attr(all(feature = "nightly", nightly), feature(core_intrinsics))]
#![cfg_attr(all(feature = "nightly", nightly), allow(internal_features))]

#![forbid(missing_docs)]

#![no_std]

#[cfg(feature = "disjoint_bitor")]
mod disjoint_bitor;
#[cfg(feature = "disjoint_bitor")]
pub use disjoint_bitor::*;

#[cfg(feature = "hints")]
mod hint;
#[cfg(feature = "hints")]
pub use hint::*;

#[cfg(feature = "nontemporal")]
mod nontemporal;
#[cfg(feature = "nontemporal")]
pub use nontemporal::*;

#[cfg(any(feature = "raw_eq", feature = "ptr_swap", feature = "prefetch"))]
mod ptr;

#[cfg(any(feature = "raw_eq", feature = "ptr_swap", feature = "prefetch"))]
pub use ptr::*;

/// Same as [`core::intrinsics::breakpoint`] with stable implementations for various architectures
#[inline(always)]
#[cfg(feature = "breakpoint")]
pub fn breakpoint() {
    #[cfg(all(feature = "nightly", nightly))]
    core::intrinsics::breakpoint();

    #[cfg(not(all(feature = "nightly", nightly)))]
    {
        use core::arch::asm;

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe { asm!("int3", options(nomem, nostack)) };

        #[cfg(target_arch = "arm")]
        unsafe { asm!("bkpt", options(nomem, nostack)) };

        #[cfg(target_arch = "aarch64")]
        unsafe { asm!("brk #0xf000", options(nomem, nostack)) };

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe { asm!("ebreak", options(nomem, nostack)) };

        #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
        unsafe { asm!("break", options(nomem, nostack)) };
    }
}

/// Like [`core::mem::transmute`] but with support for independently sized types
///
/// # Safety
///
/// Equally as unsafe as `transmute` but with no restriction on independently sized types
#[inline(always)]
#[cfg(feature = "transmute")]
pub const unsafe fn transmute_independent<Src, Dst>(src: Src) -> Dst {
    const {
        assert!(size_of::<Src>() == size_of::<Dst>(), "cannot transmute between types of different sizes")
    }

    unsafe {
        transmute_unchecked::<Src, Dst>(src)
    }
}

/// Like [`core::mem::transmute`] but without the size checks
///
/// # Safety
///
/// Even more unsafe than `transmute` because it's undefined behavior if `Src` and `Dst have different sizes`
#[inline(always)]
#[cfg(feature = "transmute")]
pub const unsafe fn transmute_unchecked<Src, Dst>(src: Src) -> Dst {
    #[cfg(all(feature = "nightly", nightly))]
    unsafe {
        core::intrinsics::transmute_unchecked::<Src, Dst>(src)
    }

    #[cfg(not(all(feature = "nightly", nightly)))]
    {
        use core::mem::ManuallyDrop;

        union Transmute<Src, Dst> {
            t: ManuallyDrop<Src>,
            u: ManuallyDrop<Dst>,
        }

        unsafe {
            core::hint::assert_unchecked(size_of::<Src>() == size_of::<Dst>());
        }

        let transmutee = Transmute {
            t: ManuallyDrop::new(src),
        };

        let transmuted = unsafe { transmutee.u };

        ManuallyDrop::into_inner(transmuted)
    }
}
