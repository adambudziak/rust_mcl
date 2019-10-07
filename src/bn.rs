use crate::ffi::*;
use libc::{c_int, size_t};
use std::ops::{Add, Mul};
use std::os::raw::{c_char, c_void};

macro_rules! add_impl {
    ($t:ty, $u:ty, $fn:ident) => {
        impl Add<$u> for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: $u) -> $t {
                let mut result = <$t>::default();
                unsafe {
                    $fn(&mut result.inner, &self.inner, &other.inner);
                }
                result
            }
        }

        forward_ref_binop! { impl Add, add for $t, $u }
    };
}

macro_rules! mul_impl {
    ($t:ty, $u:ty, $fn:ident) => {
        impl Mul<$u> for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: $u) -> $t {
                let mut result = <$t>::default();
                unsafe {
                    $fn(&mut result.inner, &self.inner, &other.inner);
                }
                result
            }
        }

        forward_ref_binop! { impl Mul, mul for $t, $u }
    };
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Clone`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self.clone(), other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, other.clone())
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self.clone(), other.clone())
            }
        }
    };
}

macro_rules! hash_and_map_impl {
    ($t:ty, $inner:ty, $fn:ident) => {
        impl $t {
            pub fn hash_and_map(buf: &[u8]) -> Result<$t, i32> {
                let mut result = <$t>::default();
                let err = unsafe {
                    $fn(
                        &mut result.inner as *mut $inner,
                        buf.as_ptr() as *const c_void,
                        buf.len(),
                    )
                };
                match err {
                    0 => Ok(result),
                    n => Err(n),
                }
            }
        }
    };
}

macro_rules! str_conversions_impl {
    ($t:ty, $inner:ty, $get_fn:ident, $set_fn:ident) => {
        impl $t {
            pub fn from_str(buffer: &str, io_mode: Base) -> Self {
                let mut result = Self::default();
                result.set_str(buffer, io_mode);
                result
            }

            pub fn set_str(&mut self, buffer: &str, io_mode: Base) {
                let err = unsafe {
                    $set_fn(
                        &mut self.inner as *mut $inner,
                        buffer.as_ptr() as *const c_char,
                        buffer.len() as size_t,
                        io_mode as c_int,
                    )
                };
                assert_eq!(err, 0);
            }

            pub fn get_str(&self, io_mode: Base) -> String {
                let len = 2048;
                let mut buf = vec![0u8; len];
                let bytes = unsafe {
                    $get_fn(
                        buf.as_mut_ptr() as *mut c_char,
                        len as size_t,
                        &self.inner as *const $inner,
                        io_mode as c_int,
                    )
                };
                assert_ne!(bytes, 0);
                String::from_utf8_lossy(&buf[..bytes]).into_owned()
            }
        }
    };
}

macro_rules! is_equal_impl {
    ($t:ty, $inner:ty, $fn:ident) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                unsafe { $fn(&self.inner as *const $inner, &other.inner as *const $inner) == 1 }
            }
        }
    };
}

macro_rules! set_by_csprng_impl {
    ($t:ty, $inner:ty, $fn:ident) => {
        impl $t {
            pub fn from_csprng() -> Self {
                let mut result = <$t>::default();
                unsafe { $fn(&mut result.inner as *mut $inner) };
                result
            }
        }
    };
}

macro_rules! serde_impl {
    ($t:ty, $inner:ty, $ser:ident, $de:ident) => {
        impl $t {
            pub fn serialize(&self) -> Vec<u8> {
                let mut buf = vec![0; 2048];
                unsafe {
                    $ser(
                        buf.as_mut_ptr() as *mut c_void,
                        buf.len() as size_t,
                        &self.inner as *const $inner,
                    )
                };
                buf
            }
            pub fn deserialize(bytes: &[u8]) -> Result<Self, ()> {
                let mut result = Self::default();
                let err = unsafe {
                    $de(
                        &mut result.inner as *mut $inner,
                        bytes.as_ptr() as *const c_void,
                        bytes.len()
                    )
                };
                match err {
                    0 => Err(()),
                    _ => Ok(result)
                }
            }
        }
    };
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Fp {
    inner: MclBnFp,
}
mul_impl![Fp, Fp, mclBnFp_mul];
add_impl![Fp, Fp, mclBnFp_mul];
is_equal_impl![Fp, MclBnFp, mclBnFp_isEqual];
set_by_csprng_impl![Fp, MclBnFp, mclBnFp_setByCSPRNG];
serde_impl![Fp, MclBnFp, mclBnFp_serialize, mclBnFp_deserialize];

#[derive(Default, Debug, Clone, Copy)]
pub struct Fp2 {
    inner: MclBnFp2,
}
mul_impl![Fp2, Fp2, mclBnFp2_mul];
add_impl![Fp2, Fp2, mclBnFp2_mul];
is_equal_impl![Fp2, MclBnFp2, mclBnFp2_isEqual];
serde_impl![Fp2, MclBnFp2, mclBnFp2_serialize, mclBnFp2_deserialize];

#[derive(Default, Debug, Clone, Copy)]
pub struct Fr {
    inner: MclBnFr,
}
mul_impl![Fr, Fr, mclBnFr_mul];
add_impl![Fr, Fr, mclBnFr_mul];
is_equal_impl![Fr, MclBnFr, mclBnFr_isEqual];
set_by_csprng_impl![Fr, MclBnFr, mclBnFr_setByCSPRNG];
str_conversions_impl![Fr, MclBnFr, mclBnFr_getStr, mclBnFr_setStr];
serde_impl![Fr, MclBnFr, mclBnFr_serialize, mclBnFr_deserialize];

#[derive(Default, Debug, Clone)]
pub struct G1 {
    inner: MclBnG1,
}
mul_impl![G1, Fr, mclBnG1_mul];
add_impl![G1, G1, mclBnG1_add];
is_equal_impl![G1, MclBnG1, mclBnG1_isEqual];
hash_and_map_impl![G1, MclBnG1, mclBnG1_hashAndMapTo];
str_conversions_impl![G1, MclBnG1, mclBnG1_getStr, mclBnG1_setStr];
serde_impl![G1, MclBnG1, mclBnG1_serialize, mclBnG1_deserialize];

#[derive(Default, Debug, Clone)]
pub struct G2 {
    inner: MclBnG2,
}
mul_impl![G2, Fr, mclBnG2_mul];
add_impl![G2, G2, mclBnG2_add];
is_equal_impl![G2, MclBnG2, mclBnG2_isEqual];
hash_and_map_impl![G2, MclBnG2, mclBnG2_hashAndMapTo];
str_conversions_impl![G2, MclBnG2, mclBnG2_getStr, mclBnG2_setStr];
serde_impl![G2, MclBnG2, mclBnG2_serialize, mclBnG2_deserialize];

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct GT {
    inner: MclBnGT,
}
mul_impl![GT, GT, mclBnGT_mul];
add_impl![GT, GT, mclBnGT_mul];
is_equal_impl![GT, MclBnGT, mclBnGT_isEqual];
str_conversions_impl![GT, MclBnGT, mclBnGT_getStr, mclBnGT_setStr];
serde_impl![GT, MclBnGT, mclBnGT_serialize, mclBnGT_deserialize];

impl GT {
    pub fn from_pairing(p: &G1, q: &G2) -> GT {
        let mut result = MclBnGT::default();
        unsafe {
            mclBn_pairing(
                &mut result as *mut MclBnGT,
                &p.inner as *const MclBnG1,
                &q.inner as *const MclBnG2,
            );
        }
        GT { inner: result }
    }

    pub fn pow(&self, a: &Fr) -> Self {
        let mut result = MclBnGT::default();
        unsafe {
            mclBnGT_pow(
                &mut result as *mut MclBnGT,
                &self.inner as *const MclBnGT,
                &a.inner as *const MclBnFr,
            );
        }
        GT { inner: result }
    }
}

#[derive(Debug)]
pub enum Base {
    Dec = 10,
    Hex = 16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init;

    fn initialize() {
        init::init_curve(init::Curve::Bls12_381);
    }

    fn run_test(inner: impl FnOnce() -> ()) {
        initialize();
        inner();
    }

    #[test]
    fn test_mcl_bn_fp_str() {
        run_test(|| {
            let fr = Fr::from_str("123", Base::Dec);
            assert_eq!(fr.get_str(Base::Dec), "123".to_string());
        });
    }

    #[test]
    fn test_fp_mul() {
        run_test(|| {
            let a = Fr::from_str("12", Base::Dec);
            let b = Fr::from_str("13", Base::Dec);
            let c = Fr::from_str("156", Base::Dec);
            assert_eq!(a * b, c);
        });
    }

    #[test]
    fn test_g1_mul() {
        run_test(|| {
            let p = G1::hash_and_map(b"this").unwrap();
            let x = Fr::from_str("123", Base::Dec);
            let y = p * x;
            let expected = G1::from_str(
                "1 ea23afffe7e4eaeddbec067563e2387bac5c2354bd58f4346151db670e65c465f947789e5f82de9ba7567d0a289c658 cf01434515162c99815667f4a5515e20d407609702b9bc182155bcf23473960ec4de3b5b552285b3f1656948cfe3260",
                Base::Hex);
            assert_eq!(y, expected);
        });
    }

    #[test]
    fn test_pairing() {
        run_test(|| {
            let a = Fr::from_str("123", Base::Dec);
            let b = Fr::from_str("456", Base::Dec);
            let P = G1::hash_and_map(b"abc").unwrap();
            let Q = G2::hash_and_map(b"abc").unwrap();

            let e1 = GT::from_pairing(&P, &Q);

            let aQ = Q * &a;
            let bP = P * &b;

            let e2 = GT::from_pairing(&bP, &aQ);
            let e1 = e1.pow(&(&a * &b));
            assert_eq!(e1, e2);
        });
    }

    #[test]
    fn test_serde() {
        run_test(|| {
            let a = Fr::from_str("123", Base::Dec);
            let after = Fr::deserialize(&a.serialize()).unwrap();
            assert_eq!(a, after);
        });
    }
}
