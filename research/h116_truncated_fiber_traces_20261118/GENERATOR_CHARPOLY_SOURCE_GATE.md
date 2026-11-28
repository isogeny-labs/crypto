# H116 generator characteristic-certificate source gate

Parent protocol `H116-GENERATOR-CHARPOLY-001` and pre-data amendment
`H116-GENERATOR-CHARPOLY-002` were committed before generator search. This
gate seals the only sources permitted to compute the affected generator data.
[pcc:unverified]

```text
e43661b2fb99f0a9b635047c2fa6977d2f723cb73879125dae72362b8eb986ff  GENERATOR_CHARPOLY_PROTOCOL.json
0344381af2e625a98742baecf660a63eb50fffb0bd26cd979d178daa349e87d7  GENERATOR_CHARPOLY_PROTOCOL_AMENDMENT_V2.json
6c10efe27dbf9afc06da0d8fb2f4cd2b95897ed94bdc541a1fb502a6f9756564  h116_generator_charpoly.rs
25805327d359b41fd8657e281d5938e2d31bcc5a344d6777528e9f52098f9ac6  h116_generator_charpoly_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
```

The Rust producer and Python verifier select generators independently. Both
use exact divisor reduction for every point order and confirm each emitted
generator order by repeated addition. The Python path reuses only the already
sealed independent field/group implementation from the parent panel; it does
not invoke Rust. [pcc:unverified]

## Pre-execution checks

```text
rustc --edition=2021 -O -D warnings h116_generator_charpoly.rs \
  -o /tmp/h116_generator_charpoly
/tmp/h116_generator_charpoly --selftest
h116 generator charpoly selftest ok

python3 h116_generator_charpoly_verify.py --selftest
h116 generator charpoly verifier selftest ok
```

These self-tests use no frozen cell. The first permitted affected execution is
the exact producer against the retained parent TSV, followed by the exact
Python replay. Every row is retained on any failure. [pcc:unverified]

This gate establishes reproducibility only. It predicts no generator count or
compression ratio and does not convert a finite certificate into an
algebraic-closure, rational-map or ECDLP result. [pcc:unverified]
