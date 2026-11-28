# H116 pointwise characteristic-polynomial source gate

Protocol `H116-POINTWISE-CHARPOLY-001` was committed at `e36bd4e1e5` before
either implementation was written or any frozen cell was executed. This gate
seals the source that may execute that protocol. [pcc:unverified]

## Frozen source

```text
36dbefbbd7d7efba055e94d1deaa427a63dc13a94ed6561bc38e1784a3efba4a  POINTWISE_CHARPOLY_PROTOCOL.json
7062d182cf4f381f3220db9cb156605b48cd3ebecca23aa741aad4af8f93c227  h116_pointwise_charpoly.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
```

The Rust producer reuses H116's existing polynomial-basis field, deterministic
Rabin test and affine curve operations. The Python verifier has a separate
field and group implementation and selects the first irreducible polynomial by
trial division through all possible monic factors. It does not import or invoke
the Rust implementation. [pcc:unverified]

## Pre-execution checks

```text
PATH=/workspace/.h116-cargo/bin:$PATH \
RUSTUP_HOME=/workspace/.h116-rustup \
CARGO_HOME=/workspace/.h116-cargo \
rustc --edition=2021 -O -D warnings h116_pointwise_charpoly.rs \
  -o /tmp/h116_pointwise_charpoly
/tmp/h116_pointwise_charpoly --selftest
h116 pointwise charpoly selftest ok

python3 h116_pointwise_charpoly_verify.py --selftest
h116 pointwise charpoly verifier selftest ok
```

The self-tests cover field laws, Frobenius order and curve identity behavior on
fixtures outside the frozen eight-cell panel. They do not expose affected panel
data. The first permitted panel execution is the exact Rust source above,
followed by the exact independent Python replay. Every output row must be
retained even if the producer exits nonzero after writing the complete table.
[pcc:unverified]

This source gate is a reproducibility boundary. It is not evidence that the
positive equations pass, and it does not expand the finite experiment into a
generic proof or an ECDLP complexity claim. [pcc:unverified]
