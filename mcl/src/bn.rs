use crate::ffi::*;
use libc::{c_int, size_t};
use std::os::raw::{c_char, c_void};

use std::ops::{Add, Mul, Sub, Div};
use mcl_derive::*;

pub trait RawSerializable {
    fn serialize_raw(&self) -> Vec<u8>;
    fn deserialize_raw(&mut self, bytes: &[u8]) -> Result<usize, ()>;
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

#[derive(Default, Debug, Clone, Copy, ScalarGroup, Object)]
pub struct Fp {
    inner: MclBnFp,
}
set_by_csprng_impl![Fp, MclBnFp, mclBnFp_setByCSPRNG];

#[derive(Default, Debug, Clone, Copy, ScalarGroup, Object)]
pub struct Fp2 {
    inner: MclBnFp2,
}

#[derive(Default, Debug, Clone, Copy, ScalarGroup, Object)]
pub struct Fr {
    inner: MclBnFr,
}
set_by_csprng_impl![Fr, MclBnFr, mclBnFr_setByCSPRNG];
str_conversions_impl![Fr, MclBnFr, mclBnFr_getStr, mclBnFr_setStr];

#[derive(Default, Debug, Clone, AdditiveGroup, Object)]
pub struct G1 {
    inner: MclBnG1,
}
str_conversions_impl![G1, MclBnG1, mclBnG1_getStr, mclBnG1_setStr];

#[derive(Default, Debug, Clone, AdditiveGroup, Object)]
pub struct G2 {
    inner: MclBnG2,
}
str_conversions_impl![G2, MclBnG2, mclBnG2_getStr, mclBnG2_setStr];

#[derive(Default, Debug, Clone, MultiplicativeGroup, Object)]
pub struct GT {
    inner: MclBnGT,
}
str_conversions_impl![GT, MclBnGT, mclBnGT_getStr, mclBnGT_setStr];

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
    fn test_serde_raw() {
        run_test(|| {
            let a = Fr::from_str("123", Base::Dec);
            let mut after = Fr::default();
            after.deserialize_raw(&a.serialize_raw()).unwrap();
            assert_eq!(a, after);
        });
    }
}
