#[cfg(nightly)]
use core::intrinsics;

#[cfg(not(nightly))]
use core::hint;

/// Hints to the compiler that this path will never be taken
///
/// Immediate undefiend behavior if this is reached
#[cold]
#[inline(always)]
pub const unsafe fn unreachable() -> ! {
    #[cfg(nightly)]
    unsafe {
        intrinsics::unreachable();
    }

    #[cfg(not(nightly))]
    unsafe {
        hint::unreachable_unchecked();
    }
}

/// Hints to the compiler that it is safe to assume this will always be true
///
/// Immediate undefiend behavior if `b` is false
#[inline(always)]
pub const unsafe fn assume(b: bool) {
    #[cfg(nightly)]
    unsafe {
        intrinsics::assume(b);
    }

    #[cfg(not(nightly))]
    unsafe {
        hint::assert_unchecked(b);
    }
}

/// Hints that this path is cold (unlikely to be taken)
#[cold]
#[inline(always)]
pub const fn cold_path() {
    #[cfg(nightly)]
    intrinsics::cold_path();

    #[cfg(not(nightly))]
    hint::cold_path();
}

/// Hints to the compiler this value is likely to be true
#[inline(always)]
pub const fn likely(b: bool) -> bool {
    #[cfg(nightly)]
    {
        intrinsics::likely(b)
    }

    #[cfg(not(nightly))]
    if b {
        true
    } else {
        cold_path();
        false
    }
}

/// Hints to the compiler this value is likely to be false
#[inline(always)]
pub const fn unlikely(b: bool) -> bool {
    #[cfg(nightly)]
    {
        intrinsics::unlikely(b)
    }

    #[cfg(not(nightly))]
    if b {
        cold_path();
        true
    } else {
        false
    }
}
