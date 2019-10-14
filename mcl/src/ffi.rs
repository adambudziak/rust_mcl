//! Bindings to all functions (WIP) defined in https://github.com/herumi/mcl/blob/master/api.md

use libc::{c_int, size_t};
use std::os::raw::{c_char, c_void};

pub const BN254: i32 = 0;
pub const BLS12_381: i32 = 5;
pub const MCLBN_FR_UNIT_SIZE: i32 = 4;
pub const MCLBN_FP_UNIT_SIZE: i32 = 6;

pub const FR_SIZE: i32 = MCLBN_FR_UNIT_SIZE;
pub const G1_SIZE: i32 = MCLBN_FP_UNIT_SIZE * 3;
pub const G2_SIZE: i32 = MCLBN_FP_UNIT_SIZE * 6;
pub const GT_SIZE: i32 = MCLBN_FP_UNIT_SIZE * 12;

pub const SEC_SIZE: i32 = FR_SIZE * 2;
pub const PUB_SIZE: i32 = G1_SIZE + G2_SIZE;
pub const G1_CIPHER_SIZE: i32 = G1_SIZE * 2;
pub const G2_CIPHER_SIZE: i32 = G2_SIZE * 2;
pub const GT_CIPHER_SIZE: i32 = GT_SIZE * 4;

pub const MCLBN_COMPILED_TIME_VAR: i32 = (MCLBN_FR_UNIT_SIZE * 10) + MCLBN_FP_UNIT_SIZE;


#[link(name = "mclbn384_256")]
extern "C" {
    pub fn mclBn_init(curve: c_int, compiledTimeVar: c_int) -> c_int;
    pub fn mclBnFr_setStr(x: *mut MclBnFr, buf: *const c_char, bufSize: size_t, ioMode: c_int)
        -> c_int;
    pub fn mclBnG1_setStr(x: *mut MclBnG1, buf: *const c_char, bufSize: size_t, ioMode: c_int)
        -> c_int;
    pub fn mclBnG2_setStr(x: *mut MclBnG2, buf: *const c_char, bufSize: size_t, ioMode: c_int)
        -> c_int;
    pub fn mclBnGT_setStr(x: *mut MclBnGT, buf: *const c_char, bufSize: size_t, ioMode: c_int)
        -> c_int;
    pub fn mclBnFr_getStr(
        buf: *mut c_char,
        maxBufSize: size_t,
        x: *const MclBnFr,
        ioMode: c_int,
    ) -> size_t;
    pub fn mclBnG1_getStr(
        buf: *mut c_char,
        maxBufSize: size_t,
        x: *const MclBnG1,
        ioMode: c_int,
    ) -> size_t;
    pub fn mclBnG2_getStr(
        buf: *mut c_char,
        maxBufSize: size_t,
        x: *const MclBnG2,
        ioMode: c_int,
    ) -> size_t;
    pub fn mclBnGT_getStr(
        buf: *mut c_char,
        maxBufSize: size_t,
        x: *const MclBnGT,
        ioMode: c_int,
    ) -> size_t;

    // Hash and map
    pub fn mclBnG1_hashAndMapTo(x: *mut MclBnG1, buf: *const c_void, bufSize: size_t) -> c_int;
    pub fn mclBnG2_hashAndMapTo(x: *mut MclBnG2, buf: *const c_void, bufSize: size_t) -> c_int;

    // Arithmetic operations
    // Multiplication
    pub fn mclBnFr_mul(z: *mut MclBnFr, x: *const MclBnFr, y: *const MclBnFr);
    pub fn mclBnFp_mul(z: *mut MclBnFp, x: *const MclBnFp, y: *const MclBnFp);
    pub fn mclBnFp2_mul(z: *mut MclBnFp2, x: *const MclBnFp2, y: *const MclBnFp2);
    pub fn mclBnGT_mul(z: *mut MclBnGT, x: *const MclBnGT, y: *const MclBnGT);

    // Addition
    pub fn mclBnFr_add(z: *mut MclBnFr, x: *const MclBnFr, y: *const MclBnFr);
    pub fn mclBnFp_add(z: *mut MclBnFp, x: *const MclBnFp, y: *const MclBnFp);
    pub fn mclBnFp2_add(z: *mut MclBnFp2, x: *const MclBnFp2, y: *const MclBnFp2);

    // Division
    pub fn mclBnFr_div(z: *mut MclBnFr, x: *const MclBnFr, y: *const MclBnFr);
    pub fn mclBnFp_div(z: *mut MclBnFp, x: *const MclBnFp, y: *const MclBnFp);
    pub fn mclBnFp2_div(z: *mut MclBnFp2, x: *const MclBnFp2, y: *const MclBnFp2);

    // Substraction
    pub fn mclBnFr_sub(z: *mut MclBnFr, x: *const MclBnFr, y: *const MclBnFr);
    pub fn mclBnFp_sub(z: *mut MclBnFp, x: *const MclBnFp, y: *const MclBnFp);
    pub fn mclBnFp2_sub(z: *mut MclBnFp2, x: *const MclBnFp2, y: *const MclBnFp2);
    pub fn mclBnG1_sub(z: *mut MclBnG1, x: *const MclBnG1, y: *const MclBnG1);
    pub fn mclBnG2_sub(z: *mut MclBnG2, x: *const MclBnG2, y: *const MclBnG2);

    // Inversion
    pub fn mclBnFr_inv(z: *mut MclBnFr, x: *const MclBnFr);
    pub fn mclBnFp_inv(z: *mut MclBnFp, x: *const MclBnFp);
    pub fn mclBnFp2_inv(z: *mut MclBnFp2, x: *const MclBnFp2);

    // Square
    pub fn mclBnFr_sqr(z: *mut MclBnFr, x: *const MclBnFr);
    pub fn mclBnFp_sqr(z: *mut MclBnFp, x: *const MclBnFp);
    pub fn mclBnFp2_sqr(z: *mut MclBnFp2, x: *const MclBnFp2);

    // Negation
    pub fn mclBnFr_neg(z: *mut MclBnFr, x: *const MclBnFr);
    pub fn mclBnFp_neg(z: *mut MclBnFp, x: *const MclBnFp);
    pub fn mclBnFp2_neg(z: *mut MclBnFp2, x: *const MclBnFp2);
    pub fn mclBnG1_neg(z: *mut MclBnG1, x: *const MclBnG1);
    pub fn mclBnG2_neg(z: *mut MclBnG2, x: *const MclBnG2);

    // Point multiplication by scalar
    pub fn mclBnG1_mul(z: *mut MclBnG1, x: *const MclBnG1, y: *const MclBnFr);
    pub fn mclBnG2_mul(z: *mut MclBnG2, x: *const MclBnG2, y: *const MclBnFr);

    // Point addition
    pub fn mclBnG1_add(z: *mut MclBnG1, x: *const MclBnG1, y: *const MclBnG1);
    pub fn mclBnG2_add(z: *mut MclBnG2, x: *const MclBnG2, y: *const MclBnG2);

    // Point double
    pub fn mclBnG1_dbl(z: *mut MclBnG1, x: *const MclBnG1);
    pub fn mclBnG2_dbl(z: *mut MclBnG2, x: *const MclBnG2);

    // GT arithmetic
    pub fn mclBnGT_pow(z: *mut MclBnGT, x: *const MclBnGT, y: *const MclBnFr);

    // equality functions
    pub fn mclBnG1_isEqual(x: *const MclBnG1, y: *const MclBnG1) -> c_int;
    pub fn mclBnG2_isEqual(x: *const MclBnG2, y: *const MclBnG2) -> c_int;
    pub fn mclBnGT_isEqual(x: *const MclBnGT, y: *const MclBnGT) -> c_int;
    pub fn mclBnFp_isEqual(x: *const MclBnFp, y: *const MclBnFp) -> c_int;
    pub fn mclBnFr_isEqual(x: *const MclBnFr, y: *const MclBnFr) -> c_int;
    pub fn mclBnFp2_isEqual(x: *const MclBnFp2, y: *const MclBnFp2) -> c_int;

    // pairing
    pub fn mclBn_pairing(z: *mut MclBnGT, x: *const MclBnG1, y: *const MclBnG2);

    pub fn mclBnFr_setByCSPRNG(x: *mut MclBnFr);
    pub fn mclBnFp_setByCSPRNG(x: *mut MclBnFp);

    // Set to zero
    pub fn mclBnFp_clear(x: *mut MclBnFp);
    pub fn mclBnFr_clear(x: *mut MclBnFr);
    pub fn mclBnFp2_clear(x: *mut MclBnFp2);
    pub fn mclBnG1_clear(x: *mut MclBnG1);
    pub fn mclBnG2_clear(x: *mut MclBnG2);
    pub fn mclBnGT_clear(x: *mut MclBnGT);

    // serialization
    // ret byte count, 0 == error
    pub fn mclBnFr_serialize(buf: *mut c_void, maxBufSize: size_t, x: *const MclBnFr) -> size_t;
    pub fn mclBnG1_serialize(buf: *mut c_void, maxBufSize: size_t, x: *const MclBnG1) -> size_t;
    pub fn mclBnG2_serialize(buf: *mut c_void, maxBufSize: size_t, x: *const MclBnG2) -> size_t;
    pub fn mclBnGT_serialize(buf: *mut c_void, maxBufSize: size_t, x: *const MclBnGT) -> size_t;
    pub fn mclBnFp_serialize(buf: *mut c_void, maxBufSize: size_t, x: *const MclBnFp) -> size_t;
    pub fn mclBnFp2_serialize(buf: *mut c_void, maxBufSize: size_t, x: *const MclBnFp2) -> size_t;

    // deserialization
    // ret byte count, 0 == error
    pub fn mclBnFr_deserialize(x: *mut MclBnFr, buf: *const c_void, bufSize: size_t) -> size_t;
    pub fn mclBnG1_deserialize(x: *mut MclBnG1, buf: *const c_void, bufSize: size_t) -> size_t;
    pub fn mclBnG2_deserialize(x: *mut MclBnG2, buf: *const c_void, bufSize: size_t) -> size_t;
    pub fn mclBnGT_deserialize(x: *mut MclBnGT, buf: *const c_void, bufSize: size_t) -> size_t;
    pub fn mclBnFp_deserialize(x: *mut MclBnFp, buf: *const c_void, bufSize: size_t) -> size_t;
    pub fn mclBnFp2_deserialize(x: *mut MclBnFp2, buf: *const c_void, bufSize: size_t) -> size_t;
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MclBnFp {
    d: [u64; MCLBN_FP_UNIT_SIZE as usize],
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MclBnFp2 {
    d: [MclBnFp; 2],
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MclBnFr {
    d: [u64; MCLBN_FR_UNIT_SIZE as usize],
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MclBnG1 {
    x: MclBnFp,
    y: MclBnFp,
    z: MclBnFp,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MclBnG2 {
    x: MclBnFp2,
    y: MclBnFp2,
    z: MclBnFp2,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MclBnGT {
    d: [MclBnFp; 12],
}
