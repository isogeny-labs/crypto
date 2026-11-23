# H116 chiral branch-resultant parity source gate

Protocol `H116-CHIRAL-BRANCH-RESULTANT-PARITY-001`, residual-quadratic and
pair ordering, resultant-character convention, fixed one-/two-feature affine
family, binary recursion, inherited branch controls, closed/Sylvester
resultant equality, common-root and symmetry controls, shuffle statistic,
sources and launch manifest were frozen before evaluating any branch-resultant
tuple against a scalar label.

```text
d598770653112560f2ab1a347682acefc4f655425e7ddf1ae80a71c1734e986e  CHIRAL_BRANCH_RESULTANT_PARITY_PROTOCOL.md
0702a63acfd86460b9ff73627ebb6d12b80864aa2cc087c22d7cebd33256a874  CHIRAL_BRANCH_RESULTANT_PARITY_LAUNCH.json
87b7516e5f65a44b091ba79209ef91d52baec0bed2dace48eb8c9001564f5186  h116_chiral_branch_residual_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812
d73245618948deb3611032b775cf03240f2c635107f557228981ed7eba22ec22  /tmp/h116_chiral_branch_resultant_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target resultant tuple, selected pair or
shuffle score existed when this gate was written.  The first permissible run
is from the first pushed commit containing these exact bytes.
