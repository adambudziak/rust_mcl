use mcl::{init, bn::*};

fn main() {
    // Always initialize the library first.
    init::init_curve(init::Curve::Bls12_381);

    // choose the generators for both of the groups
    let g = G1::hash_and_map(b"something").unwrap();
    let g2 = G2::hash_and_map(b"something else").unwrap();

    // setup the keys
    let a = Fr::from_csprng();
    let A = &g * &a;

    // initialize params (done by the Prover)
    let x = Fr::from_csprng();
    let X = &g * &x;

    // generate challenge (done by the Verifier)
    let c = Fr::from_csprng();

    // compute the response (done by the Prover)
    let X_and_c = X.get_str(Base::Dec).push_str(&c.get_str(Base::Dec));

    let s = &g2 * (x + &a * &c);

    let e = GT::from_pairing(&g, &s);
    let es = GT::from_pairing(&(X + A * &c), &g2);
}
