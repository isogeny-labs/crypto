# H116 formal-scalar OOS oracle continuation source gate

Status: bound after the retained first-cell selection failure and before any
attempt of cells two through five.

The v1 oracle stopped on `fp107-r127-d3` after exhausting all 11,449 models.
`FORMAL_SCALAR_OOS_FIRST_FAILURE.json` retains that result. The continuation
does not rerun or replace that cell. It changes only orchestration: cells two
through five are attempted once each, and any selection or oracle failure is
serialized rather than terminating the panel early.

Continuation source and binary SHA-256 values:

```text
f75b7073f7b0d93fb253b4518bdfc76a0f219f9a846ca5753df7bb182aa606e4  h116_formal_scalar_oos_oracle.rs
8e315414b765996fa47720a8d9f6cb4767415361cf6c8384489d68899638affa  h116_formal_scalar_oos_oracle_v2
```

The source compiled with the same warning-free Rust command and toolchain as
v1. Curve-selection predicates, ordering, candidate source, map formulas,
controls, sample size and no-replacement rule are unchanged. [pcc:unverified]
