use crate::ffi::mclBn_init;

pub enum Curve {
    Bls12_381,
}

/// A high-level wrapper for [`mcl_bn_init`] that chooses appropriate
/// parameters based on the curve specified in a parameter.
pub fn init_curve(curve: Curve) {
    match curve {
        Curve::Bls12_381 => mcl_bn_init(5, 46),
    };
}

/// Initialize the MCL library.
///
/// This function MUST be called before you perform any operation on
/// any type from MCL.
///
/// This function is a synchronized wrapper of [`mclBn_init`]. If you forget to call
/// the underlying function directly or via this wrapper, you'll run into very
/// weird and hard to debug memory bugs.
///
/// This function can be safely called by multiple threads and multiple times,
/// because the current implementation uses [`std::sync::Once`] to assure
/// the FFI endpoint is called only once.
///
/// This also means that at the moment you can't change the elliptic curve using
/// this method by calling it again. For now, the best solution is to unsafely
/// call the FFI endpoint [`mclBn_init`] and pass all the necessary parameters.
///
/// # Note
/// You probably shouldn't call this function unless [`init_curve`] doesn't implement
/// the elliptic curve you need.
///
/// # Examples
/// ```
/// use mcl::init::mcl_bn_init;
/// mcl_bn_init(5, 46); // parameters of BLS12_381
/// ```
///
pub fn mcl_bn_init(curve: i32, compiled_time_var: i32) -> i32 {
    use std::sync::Once;
    static INIT: Once = Once::new();
    static mut VAL: i32 = 0;
    unsafe {
        INIT.call_once(|| {
            VAL = mclBn_init(curve, compiled_time_var);
        });
        VAL
    }
}
