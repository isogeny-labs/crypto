# H116 dual-pair composition source gate

This gate records the source hashes sealed before the first
`H116-DUAL-PAIR-COMPOSITION-001` execution.

```text
916cff75615a65cd56e749a443213b50842c224957513c642f98679de888b1d2  DUAL_PAIR_COMPOSITION_PROTOCOL.json
4221e3f6ea1fe7fe5ad559b050d6bf8a76e8e5493f0f88421ae0ea6deae1b0ec  h116_dual_pair_composition.rs
b89f26b549eec491f29812129885b7ce57d2ddda5ad380bec21da5d1aeaab9d4  h116_dual_pair_composition_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
```

The Rust producer compiled without warnings with Rust 1.100.0 and
`--edition=2021 -O -D warnings`. The independent Python verifier passed
bytecode compilation. Both exact root-multiplicity self-tests passed after the
hashes were sealed. No source changed between this gate and the recorded run.
[pcc:unverified]
