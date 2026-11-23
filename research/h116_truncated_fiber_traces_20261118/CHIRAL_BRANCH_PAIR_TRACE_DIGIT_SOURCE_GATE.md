# H116 chiral branch-pair genus-one trace-digit source gate

Protocol `H116-CHIRAL-BRANCH-PAIR-TRACE-DIGIT-001`, ordered branch-pair
quartics, trace-residue convention, complete affine `F_3` rule family, ternary
recursion, inherited branch/resultant controls, factored/expanded quartic
evaluation, square-root census, character-sum/projective count and Hasse
controls, shuffle statistic, sources and launch manifest were frozen before
evaluating any branch-pair trace against a scalar digit.

```text
8e9d7c887c7af86812909044d76a94feed1035c9960f626d6fdf2264753ad050  CHIRAL_BRANCH_PAIR_TRACE_DIGIT_PROTOCOL.md
9a2af62c930ea013c802e628698323d18c7303cee30aea1f1d4ff996be99b678  CHIRAL_BRANCH_PAIR_TRACE_DIGIT_LAUNCH.json
aa90384f38dbcdaa9ac4f8f589fd5d4f0330e0afa806ad43893aefc141e89eb6  h116_chiral_branch_residual_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812 trace_rules=243
4230ca457dee6afcd5ab5faa084178f1d770c174e622f271f947388f9c65d151  /tmp/h116_chiral_branch_pair_trace_digit
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target trace tuple, selected affine rule
or shuffle score existed when this gate was written.  The first permissible
run is from the first pushed commit containing these exact bytes.
