/// Specialized trait for disjoint OR computation via [`disjoint_bitor`]
///
/// # Safety
///
/// Implementors must guarantee that `REPR_ID` identifies a primitive integer
/// representation with exactly the same size, alignment, and valid bit patterns
/// as `Self`, and that `Self` values can be safely reinterpreted as that
/// representation and back.
pub unsafe trait DisjointBitOr: Copy + 'static {
    /// UID of primitive integer representation
    const REPR_ID: u8 = 255;

    /// Computes the bitwise OR of two values with no bits in common
    unsafe fn disjoint_bitor(self, other: Self) -> Self {
        unsafe {
            disjoint_bitor(self, other)
        }
    }
}

macro_rules! transmute_disjoint {
    ($name:ident($a:ident, $b:ident)) => {
        $crate::transmute_unchecked($name($crate::transmute_unchecked($a), $crate::transmute_unchecked($b)))
    };
}

/// Computes the bitwise OR of two values with no bits in common
///
/// # Safety
///
/// Immediate undefined behavior if `a & b != 0`
///
/// `T` must have the representation of a primitive integer or bool
#[inline(always)]
pub const unsafe fn disjoint_bitor<T: DisjointBitOr>(a: T, b: T) -> T {
    unsafe {
        match const { T::REPR_ID } {
            0 => transmute_disjoint!(disjoint_bitor_bool(a, b)),

            1 => transmute_disjoint!(disjoint_bitor_u8(a, b)),
            2 => transmute_disjoint!(disjoint_bitor_i8(a, b)),

            3 => transmute_disjoint!(disjoint_bitor_u16(a, b)),
            4 => transmute_disjoint!(disjoint_bitor_i16(a, b)),

            5 => transmute_disjoint!(disjoint_bitor_u32(a, b)),
            6 => transmute_disjoint!(disjoint_bitor_i32(a, b)),

            7 => transmute_disjoint!(disjoint_bitor_u64(a, b)),
            8 => transmute_disjoint!(disjoint_bitor_i64(a, b)),

            9 => transmute_disjoint!(disjoint_bitor_u128(a, b)),
            10 => transmute_disjoint!(disjoint_bitor_i128(a, b)),

            11 => transmute_disjoint!(disjoint_bitor_usize(a, b)),
            12 => transmute_disjoint!(disjoint_bitor_isize(a, b)),

            _ => crate::unreachable(),
        }
    }
}

macro_rules! def {
    ($name:ident::<$ty:ty, $repr_id:literal>() == $zr:literal) => {
        #[inline(always)]
        #[doc = concat!("computes the bitwise OR of two `", stringify!($ty), "`s with no bits in common")]
        pub const unsafe fn $name(a: $ty, b: $ty) -> $ty {
            unsafe {
                ::core::hint::assert_unchecked(a & b == $zr);
                a | b
            }
        }

        unsafe impl DisjointBitOr for $ty {
            const REPR_ID: u8 = $repr_id;

            #[inline(always)]
            unsafe fn disjoint_bitor(self, other: Self) -> Self {
                #[cfg(all(feature = "nightly", nightly))]
                unsafe {
                    ::core::intrinsics::disjoint_bitor(self, other)
                }

                #[cfg(not(all(feature = "nightly", nightly)))]
                unsafe {
                    $name(self, other)
                }
            }
        }
    };
}

def!(disjoint_bitor_bool::<bool, 0>() == false);

def!(disjoint_bitor_u8::<u8, 1>() == 0);
def!(disjoint_bitor_i8::<i8, 2>() == 0);

def!(disjoint_bitor_u16::<u16, 3>() == 0);
def!(disjoint_bitor_i16::<i16, 4>() == 0);

def!(disjoint_bitor_u32::<u32, 5>() == 0);
def!(disjoint_bitor_i32::<i32, 6>() == 0);

def!(disjoint_bitor_u64::<u64, 7>() == 0);
def!(disjoint_bitor_i64::<i64, 8>() == 0);

def!(disjoint_bitor_u128::<u128, 9>() == 0);
def!(disjoint_bitor_i128::<i128, 10>() == 0);

def!(disjoint_bitor_usize::<usize, 11>() == 0);
def!(disjoint_bitor_isize::<isize, 12>() == 0);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_disjoint_bitor {
        ($name:ident, $ty:ty, $zero:expr, $($value:expr),+ $(,)?) => {
            #[test]
            fn $name() {
                $(
                    let a: $ty = $value;
                    assert_eq!(
                        unsafe { disjoint_bitor(a, $zero) },
                        a,
                    );

                    assert_eq!(
                        unsafe { disjoint_bitor($zero, a) },
                        a,
                    );
                )+

                let a: $ty = 0x55;
                let b: $ty = 0xaa;

                assert_eq!(
                    unsafe { disjoint_bitor(a, b) },
                    (a | b),
                );

                let a: $ty = 0x33;
                let b: $ty = 0xcc;

                assert_eq!(
                    unsafe { disjoint_bitor(a, b) },
                    (a | b),
                );
            }
        };
    }

    test_disjoint_bitor!(u16_, u16, 0, 0, 1, 2, 0x8000, 0xffff);
    test_disjoint_bitor!(i16_, i16, 0, 0, 1, 2, -1, -32768);

    test_disjoint_bitor!(u32_, u32, 0, 0, 1, 2, 0x8000_0000, 0xffff_ffff);
    test_disjoint_bitor!(i32_, i32, 0, 0, 1, 2, -1, i32::MIN);

    test_disjoint_bitor!(u64_, u64, 0, 0, 1, 2, 0x8000_0000_0000_0000, 0xffff_ffff_ffff_ffff);
    test_disjoint_bitor!(i64_, i64, 0, 0, 1, 2, -1, i64::MIN);

    test_disjoint_bitor!(u128_, u128, 0, 0, 1, 2, 1 << 127, u128::MAX);
    test_disjoint_bitor!(i128_, i128, 0, 0, 1, 2, -1, i128::MIN);

    test_disjoint_bitor!(usize_, usize, 0, 0, 1, 2, usize::MAX);
    test_disjoint_bitor!(isize_, isize, 0, 0, 1, 2, -1, isize::MIN);

    #[test]
    fn disjoint_results_match_builtin_or() {
        macro_rules! check {
            ($ty:ty, $a:expr, $b:expr) => {{
                let a = $a as $ty;
                let b = $b as $ty;

                assert_eq!(
                    unsafe { disjoint_bitor(a, b) },
                    a | b,
                );
            }};
        }

        check!(u8,   0x01, 0x80);
        check!(u16,  0x1234, 0x8000);
        check!(u32,  0x0000_ffff, 0xffff_0000);
        check!(u64,  0x0000_0000_ffff_ffff, 0xffff_ffff_0000_0000);
        check!(u128, 0xffff_ffff_ffff_ffff, 0xffff_ffff_ffff_ffffu128 << 64);

        check!(i8,   0x01, -128i8);
        check!(i16,  0x1234, -32768i16);
        check!(i32,  0x0000_ffff, i32::MIN);
        check!(i64,  0x0000_0000_ffff_ffff, i64::MIN);
        check!(i128, 0xffff_ffff, i128::MIN);

        check!(usize, 1, 2);
        check!(isize, 1, isize::MIN);
    }
}
