#![cfg_attr(all(feature = "nightly", nightly), feature(core_intrinsics))]
#![cfg_attr(all(feature = "nightly", nightly), allow(internal_features))]

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

mod ptr;
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

/// Like [`core::mem::transmute`] but without the size checks
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

            ManuallyDrop::into_inner(Transmute {
                t: ManuallyDrop::new(src),
            }.u)
        }
    }
}
