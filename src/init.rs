use crate::ffi::mcl_bn_init;

pub enum Curve {
    Bls12_381,
}

pub fn init_curve(curve: Curve) {
    match curve {
        Curve::Bls12_381 => mcl_bn_init(5, 46),
    };
}
