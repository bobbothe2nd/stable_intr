/// Specialized trait for nontemporal stores
///
/// # Safety
///
/// Implementors must emit a valid nontemporal store for `dst` when `dst`
/// satisfies the method's safety requirements.
pub unsafe trait NontemporalStore: Copy + 'static {
    /// emits a `nontemporal` store
    ///
    /// # Safety
    ///
    /// `dst` must be valid for writing a `Self` and properly aligned.
    unsafe fn nontemporal_store(self, dst: *mut Self);
}

/// Emits a `nontemporal` store, which gives a hint to the CPU that the data should not be held in cache.
///
/// # Safety
///
/// `ptr` must be valid for writes of a `T`, properly aligned for `T`, and, on x86 fast paths, suitably aligned for the selected NT store.
///
/// Nontemporal stores have weaker ordering semantics than ordinary stores.
/// Use [`nontemporal_fence`] when an ordering/completion boundary is required.
#[inline(always)]
pub unsafe fn nontemporal_store<T: NontemporalStore>(ptr: *mut T, val: T) {
    #[cfg(nightly)]
    unsafe {
        core::intrinsics::nontemporal_store(ptr, val);
    }

    #[cfg(not(nightly))]
    unsafe {
        val.nontemporal_store(ptr);
    }
}

/// Establishes an ordering/completion boundary for preceding nontemporal stores.
///
/// Ensures that preceding nontemporal stores are globally visible before
/// subsequent stores and memory operations are issued.
#[inline(always)]
pub unsafe fn nontemporal_fence() {
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "sse",
    ))]
    unsafe {
        core::arch::x86_64::_mm_sfence();
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
unsafe impl NontemporalStore for i32 {
    #[inline(always)]
    unsafe fn nontemporal_store(self, dst: *mut Self) {
        unsafe {
            core::arch::x86_64::_mm_stream_si32(dst, self);
        }
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
unsafe impl NontemporalStore for i64 {
    #[inline(always)]
    unsafe fn nontemporal_store(self, dst: *mut Self) {
        unsafe {
            core::arch::x86_64::_mm_stream_si64(dst, self);
        }
    }
}

unsafe impl NontemporalStore for u32
where 
    i32: NontemporalStore,
{
    #[inline(always)]
    unsafe fn nontemporal_store(self, dst: *mut Self) {
        unsafe {
            self.cast_signed().nontemporal_store(dst.cast());
        }
    }
}

unsafe impl NontemporalStore for u64
where 
    i64: NontemporalStore,
{
    #[inline(always)]
    unsafe fn nontemporal_store(self, dst: *mut Self) {
        unsafe {
            self.cast_signed().nontemporal_store(dst.cast());
        }
    }
}
