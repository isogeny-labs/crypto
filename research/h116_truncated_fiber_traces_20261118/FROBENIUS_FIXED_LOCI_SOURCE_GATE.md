# H116 Frobenius fixed-loci source gate

Protocol `H116-FROBENIUS-FIXED-LOCI-001` was committed at `32361f0abe`
before either affected implementation existed. This gate seals the only source
paths permitted to generate the orbit-census result. [pcc:unverified]

```text
0191f93cc986b025224fcad313e8be85842f1599a223f92aa2e94aefb2a2772d  FROBENIUS_FIXED_LOCI_PROTOCOL.json
f8d3100fc0e19033688e2f6ee6cc81d3f9c7e442eb64758f3a6fe337e1ae0fb4  h116_frobenius_fixed_loci.rs
f77eb1051abcf101123456f81d189e5f3123a1b62f5fc324cfac04d10c7058c4  h116_frobenius_fixed_loci_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
```

The Rust producer uses the previously sealed exact field/group module. The
Python verifier reuses only the independently implemented Python field/group
path from the parent panel and reconstructs every point, orbit and
characteristic value without invoking Rust. [pcc:unverified]

## Pre-execution checks

```text
RUSTUP_HOME=/workspace/.h116-rustup \
CARGO_HOME=/workspace/.h116-cargo \
/workspace/.h116-cargo/bin/rustc --edition=2021 -O -D warnings \
  h116_frobenius_fixed_loci.rs -o /tmp/h116_frobenius_fixed_loci
/tmp/h116_frobenius_fixed_loci --selftest
h116 Frobenius fixed-loci selftest ok

python3 h116_frobenius_fixed_loci_verify.py --selftest
h116 Frobenius fixed-loci verifier selftest ok
```

The compiler is `rustc 1.100.0 (0ef1bc386 2026-11-09)`. Compilation with
warnings denied and both self-tests pass. `rustfmt` is not installed in the
pinned toolchain; no formatting success is claimed. The self-tests use no
frozen experiment cell. [pcc:unverified]

The first permitted affected execution is one producer run against the
retained parent TSV followed by one independent Python replay. Every emitted
row is retained on failure. This gate establishes source identity only and
predicts no orbit counts or characteristic outcome. [pcc:unverified]
