use mcl::{init, bn::*};

fn main() {
    // Always initialize the library first.
    init::init_curve(init::Curve::Bls12_381);

    // choose the generators for both of the groups
    let g = G1::hash_and_map(b"something").unwrap();

    // setup the keys
    let sk = Fr::from_csprng();
    let pk = &g * &sk;

    // initialize ephemerals (done by the Prover)
    let x = Fr::from_csprng();
    let commitment = &g * &x;

    // generate challenge (done by the Verifier)
    let c = Fr::from_csprng();

    // compute the response (done by the Prover)
    let s = x + &sk * &c;

    // verify the proof (done by the Verifier)
    assert_eq!(&g * s, &commitment + pk * &c);

}
