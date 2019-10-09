use mcl::{init, bn::*};

fn main() {
    // Always initialize the library first.
    init::init_curve(init::Curve::Bls12_381);

    // choose the generators for both of the groups
    let g = G1::hash_and_map(b"something").unwrap();
    let g2 = G2::hash_and_map(b"something else").unwrap();

    // setup the keys
    let a = Fr::from_csprng();
    let A = &g2 * &a;

    // initialize ephemerals (done by the Prover)
    let x = Fr::from_csprng();
    let X = &g2 * &x;

    // generate challenge (done by the Verifier)
    let c = Fr::from_csprng();

    // compute the response (done by the Prover)
    let mut U = X.get_str(Base::Dec);
    U.push_str(&c.get_str(Base::Dec));
    let U = G1::hash_and_map(U.as_bytes()).unwrap();

    let s = x + &a * &c;

    let S = &U * s;

    let e1 = GT::from_pairing(&U, &(X + A * c));
    let e2 = GT::from_pairing(&S, &g2);
    assert_eq!(e1, e2);
}
