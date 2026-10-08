use stable_intr::{disjoint_bitor, nontemporal_fence, nontemporal_store, select_unpredictable};

const LEN: usize = 1 << 18;

fn main() {
    let a = 123u32;
    let mut b = 4;

    let mut arr = Box::<[u32]>::new_uninit_slice(LEN);
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

    println!("{:?}", &arr[..32]);
}
