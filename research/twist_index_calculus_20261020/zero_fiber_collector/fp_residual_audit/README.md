# Finite prime-field residual census

The [fixed protocol](PROTOCOL.md) covers all 42 complete original zero-fiber
case-arms, with phi and generic doubling controls. It consumes the original
hash-bound records and public coordinates; no collector or F4 is rerun.

The [native auditor](native/src/main.rs) reconstructs the quartic field,
public point pool, signed scalar-action orbits and folded columns. It
recomputes every peeled residual, retains infinity and classifies finite
prime-field x coordinates. Saved field roots receive independent equation,
component and distinct-cubic-root checks. Exact physical-set preservation,
projective rows, zero rows, independent RREF and full-rank factor-log point
verification are required. Violations remain in the output.

This is a counterfactual audit using existing complete records. Its skipped
system and historical F4-row counts do not measure a deployed filter,
additional relation yield, a full cold price or a twist-specific speedup.
The original mixed-component comparison and all three active panels retain
their workload, controls and acceptance criteria.

Run the separate native gate with `build-native.sh FRESH_GATE FRESH_TARGET`.
After a committed source/input freeze, use `run.sh FRESH_RUN PASSING_GATE`
directly under a single managed owner; it owns common reservations for the
first/A/A processes and compares their complete output trees. Each process
retains the 8 GiB and 1,200-second envelope. Inspect terminal receipts before
any follow-up; no timeout observation authorizes another owner.

Native gate v1 passes all eight tests, with zero failures; formatting,
separate release build and source seals pass. Binary SHA-256:
`d0f56b52bddfe3ef9a0731c4fdaeb65daa9e9d311e71a60e851406840005137f`.
The gate is `../runs/fp-residual-audit-native-gate-v1/`, with durable metadata
copied under `../results/fp-residual-audit-v1/native-gate/`.

Production has not run at this source freeze. The final descriptive report,
graph/PDF, native totals and raw archive must be added after execution. The
existing conditional component proof and its full group/embedding gaps are
described in `../../joint_image_elimination/reports/run-20261024T0445Z-square/`.
