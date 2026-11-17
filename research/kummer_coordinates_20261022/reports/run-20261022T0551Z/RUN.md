# Coordinate gate run, 2026-10-22 05:51 UTC

The protocol was frozen before the coordinate candidate. The public cohort
archive digest is
`550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
Commands from the repository root, after a locked offline release build:

```
research/kummer_coordinates_20261022/target/release/kummer-coordinate-gate
research/kummer_coordinates_20261022/target/release/validate
```

Validator stdout:

```
validated_models=21 model_exception_pass=1008 model_exception_zero=168 validated_targets=1260 coordinate_pass=8813 coordinate_zero=7 coordinate_fail=0 source_fail=0 signed_oracle_fail=0
```

The early one-permutation shortcut and its 1,008 inapplicable cases are
preserved as a diagnostic file. The final computation uses all 105 fitted
two-torsion projective actions, yet still records 168 model zero outputs
and seven target zero outputs. Every one has exact coordinate and Jacobian
expected-class witnesses. No fast Recover, collector, or cost run took place.
The released producer was rerun after saving SHA-256 digests of all ten raw
JSONL files; every file matched byte-for-byte. The separate validator
recomputed the generic, exceptional and cohort counts from raw records and
cross-checked the prior reference-gate workload and labels.
