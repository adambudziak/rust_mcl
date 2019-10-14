//! Module implementing the high level structures for manipulating
//! MCL points and scalars.
//!
//! # Examples
//! ```
//! // Schnorr identification scheme.
//! // Prover wants to show to the Verifier that he knows the secret key,
//! use mcl::{init, bn::{Fr, G1}};
//! 
//! // Always initialize the library first.
//! init::init_curve(init::Curve::Bls12_381);
//!
//! // choose the generators for both of the groups
//! let g = G1::hash_and_map(b"something").unwrap();
//!
//! // setup the keys
//! let sk = Fr::from_csprng();
//! let pk = &g * &sk;
//!
//! // initialize ephemerals (done by the Prover)
//! let x = Fr::from_csprng();
//! let commitment = &g * &x;
//!
//! // generate challenge (done by the Verifier)
//! let c = Fr::from_csprng();
//!
//! // compute the response (done by the Prover)
//! let s = x + &sk * &c;
//!
//! // verify the proof (done by the Verifier)
//! assert_eq!(&g * s, &commitment + pk * &c);
//! ```

use crate::{ffi::*, traits::*, common::Base};

use std::ops::{Add, Mul, Sub, Div};
use mcl_derive::*;

#[derive(Object, ScalarPoint, Random)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Fp {
    inner: MclBnFp,
}

#[derive(Object, ScalarPoint)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Fp2 {
    inner: MclBnFp2,
}

#[derive(Object, ScalarPoint, Formattable, Random)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Fr {
    inner: MclBnFr,
}

#[derive(Object, AdditivePoint, Formattable)]
#[derive(Default, Debug, Clone)]
pub struct G1 {
    inner: MclBnG1,
}

#[derive(Object, AdditivePoint, Formattable)]
#[derive(Default, Debug, Clone)]
pub struct G2 {
    inner: MclBnG2,
}

#[derive(Object, MultiplicativePoint, Formattable)]
#[derive(Default, Debug, Clone)]
pub struct GT {
    inner: MclBnGT,
}

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
            after.deserialize_raw(&a.serialize_raw().unwrap()).unwrap();
            assert_eq!(a, after);
        });
    }
}
