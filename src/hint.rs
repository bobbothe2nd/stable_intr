#[cfg(all(feature = "nightly", nightly))]
use core::intrinsics;

#[cfg(not(all(feature = "nightly", nightly)))]
use core::hint;

/// Hints to the compiler that this path will never be taken
///
/// # Safety
///
/// Immediate undefiend behavior if this is reached
#[cold]
#[inline(always)]
pub const unsafe fn unreachable() -> ! {
    #[cfg(all(feature = "nightly", nightly))]
    unsafe {
        intrinsics::unreachable();
    }

    #[cfg(not(all(feature = "nightly", nightly)))]
    unsafe {
        hint::unreachable_unchecked();
    }
}

/// Hints to the compiler that it is safe to assume this will always be true
///
/// # Safety
///
/// Immediate undefiend behavior if `b` is false
#[inline(always)]
pub const unsafe fn assume(b: bool) {
    #[cfg(all(feature = "nightly", nightly))]
    unsafe {
        intrinsics::assume(b);
    }

    #[cfg(not(all(feature = "nightly", nightly)))]
    unsafe {
        hint::assert_unchecked(b);
    }
}

/// Hints that this path is cold (unlikely to be taken)
#[cold]
#[inline(always)]
pub const fn cold_path() {
    #[cfg(all(feature = "nightly", nightly))]
    intrinsics::cold_path();

    #[cfg(not(all(feature = "nightly", nightly)))]
    hint::cold_path();
}

/// Hints to the compiler this value is likely to be true
#[inline(always)]
pub const fn likely(b: bool) -> bool {
    #[cfg(all(feature = "nightly", nightly))]
    {
        intrinsics::likely(b)
    }

    #[cfg(not(all(feature = "nightly", nightly)))]
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
    #[cfg(all(feature = "nightly", nightly))]
    {
        intrinsics::unlikely(b)
    }

    #[cfg(not(all(feature = "nightly", nightly)))]
    if b {
        cold_path();
        true
    } else {
        false
    }
}

/// Returns either `true_val` or `false_val` depending on the value of `condition`, with a hint to the compiler that `condition` is unlikely to be correctly predicted by a CPU’s branch predictor.
#[inline(always)]
pub fn select_unpredictable<T>(condition: bool, true_val: T, false_val: T) -> T {
    #[cfg(all(feature = "nightly", nightly))]
    {
        intrinsics::select_unpredictable(condition, true_val, false_val)
    }

    #[cfg(not(all(feature = "nightly", nightly)))]
    {
        hint::select_unpredictable(condition, true_val, false_val)
    }
}
