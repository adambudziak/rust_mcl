MCL bindings for Rust
=====================

MCL is a pairing-friendly crypto library accessible at [herumi/mcl](https://github.com/herumi/mcl).

The library is written in C and C++ and exposes a C API that is used by the FFI defined in this crate.

The aim of this library is to implement the whole api defined in [api.md](https://github.com/herumi/mcl/blob/master/api.md)
and expose both a low-level unsafe FFI API as well as a high-level idiomatic and safe Rust API.

## TODO
- [ ] Implement the missing functions of the `api.md`.
- [ ] Write documentation (hard to do as most of the code is generated using macros).
- [ ] Describe the features.
- [ ] Perform benchmarks (e.g. against zkcrypto pairing library).
- [ ] Add a ton of unittests. 
