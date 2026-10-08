#[cfg(nightly)]
use core::intrinsics;

#[cfg(not(nightly))]
use core::{ptr::{self, from_ref}, slice};

/// Swaps the values at two mutable locations, without deinitializing either one.
#[inline(always)]
#[cfg(feature = "ptr_swap")]
pub const unsafe fn typed_swap_nonoverlapping<T>(x: *mut T, y: *mut T) {
    #[cfg(nightly)]
    unsafe {
        intrinsics::typed_swap_nonoverlapping(x, y);
    }

    #[cfg(not(nightly))]
    unsafe {
        ptr::swap(x, y);
    }
}

/// Compares raw bytes of `a` with `b` for equality and returns the result
///
/// # Safety
///
/// All bytes of both `a` and `b` must be initialized
#[inline(always)]
#[cfg(feature = "raw_eq")]
pub const unsafe fn raw_eq<T>(a: &T, b: &T) -> bool {
    #[cfg(nightly)]
    unsafe {
        intrinsics::raw_eq(a, b)
    }

    #[cfg(not(nightly))]
    unsafe {
        let a = slice::from_raw_parts(from_ref(a).cast::<u8>(), size_of::<T>());
        let b = slice::from_raw_parts(from_ref(b).cast::<u8>(), size_of::<T>());

        let mut i = 0;

        while i < size_of::<T>() {
            if a[i] != b[i] {
                return false;
            }

            i += 1;
        }

        true
    }
}

/// Hints to insert a prefetch instruction for the given address
#[inline(always)]
#[cfg(feature = "prefetch")]
pub unsafe fn prefetch_read_data<T, const LOCALITY: i32>(data: *const T) {
    const {
        assert!(LOCALITY <= 3, "invalid `LOCALITY` outside cache hierarcy");
    }

    #[cfg(nightly)]
    {
        intrinsics::prefetch_read_data::<T, LOCALITY>(data);
    }

    #[cfg(all(not(nightly), target_arch = "x86_64", target_feature = "sse"))]
    unsafe {
        use core::arch::x86_64::{_MM_HINT_T0, _MM_HINT_T1, _MM_HINT_T2};

        match LOCALITY {
            0 => {}
            1 => core::arch::x86_64::_mm_prefetch::<{ _MM_HINT_T2 }>(data.cast()),
            2 => core::arch::x86_64::_mm_prefetch::<{ _MM_HINT_T1 }>(data.cast()),
            3 => core::arch::x86_64::_mm_prefetch::<{ _MM_HINT_T0 }>(data.cast()),
            _ => crate::unreachable(),
        }
    }
}

/// ints to insert a prefetch instruction for te given address indicating anticipation to write
#[inline(always)]
#[cfg(feature = "prefetch")]
pub unsafe fn prefetch_write_data<T, const LOCALITY: i32>(data: *const T) {
    const {
        assert!(LOCALITY <= 3, "invalid `LOCALITY` outside cache hierarcy");
    }

    #[cfg(nightly)]
    {
        intrinsics::prefetch_write_data::<T, LOCALITY>(data);
    }

    #[cfg(all(not(nightly), target_arch = "x86_64", target_feature = "sse"))]
    unsafe {
        use core::arch::x86_64::{_MM_HINT_ET0, _MM_HINT_ET1, _MM_HINT_T2};

        match LOCALITY {
            0 => {}
            1 => core::arch::x86_64::_mm_prefetch::<{ _MM_HINT_T2 }>(data.cast()),
            2 => core::arch::x86_64::_mm_prefetch::<{ _MM_HINT_ET1 }>(data.cast()),
            3 => core::arch::x86_64::_mm_prefetch::<{ _MM_HINT_ET0 }>(data.cast()),
            _ => crate::unreachable(),
        }
    }
}
