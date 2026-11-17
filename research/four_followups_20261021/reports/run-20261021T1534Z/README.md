# Kummer ordinary-addition applicability gate

- **Run ID:** `20261021T1534Z-kummer-ambiguity`.
- **Source:** `f6a440f0eaef939094062012b7db4aee9f550529` (committed before independent replay).
- **Command:** `cargo run --release --locked --manifest-path research/four_followups_20261021/Cargo.toml --bin kummer-ambiguity-gate -q > research/four_followups_20261021/results/kummer-ambiguity.jsonl`.
- **Input:** the exact 21 Scholten source models from the frozen projective-control archive, digest in the companion factor-base run report.
- **Requested/completed:** 21/21 source-order checks, Jacobian addition identities, equal unsigned Kummer inputs and unequal sum outputs. This is an exact algebraic applicability gate, not a timed DLP or a differential Kummer collector.
- **Raw:** [`kummer-ambiguity.jsonl`](../../results/kummer-ambiguity.jsonl), one coordinate witness per curve.
- **Visual:** [`kummer-ambiguity.svg`](../../kummer-ambiguity.svg) and the [PDF report](../../REPORT.pdf).
- **Remaining:** construct a differential walk with known differences and a sign/lift-safe target relation, cross-check its arithmetic, then run and price the full 1,260-target cohort.
