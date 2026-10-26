# Cell G2: per-slot-shift orbit-union bases vs the exact class count D(n,m)

Status: prepared, not run.

Lead re-examined: LEAD-ICB-frobenius-search-symmetry (campaign 2026-10-27, branch claude/frobenius-symmetry-m-sweep-20261027).
Std-only Rust crate (own `[workspace]`, not part of the root build). Reuses the GF(2^n) exp/log field and the binary
Weierstrass point arithmetic of `research/twist_obstruction_20261026/src/main.rs` (restricted to F_q, no F_{q^2}).

* `PROTOCOL.md` frozen design, claim classes, arms, controls, metric, decision rule, power/resolution, runtime estimate, exclusions.
* `RUN_CARD.md` exact later commands, expected wall time, output paths, preconditions (checkpoint token).
* `run_cells.sh` driver (gate-checked; do not launch before the checkpoint exists).
* `src/` field, curve, combinatorics (D(n,m), diagonal classes), block families, exact enumerator, runner, frozen analysis.
* `tests/identities.rs` 13 tests: field/curve laws, D(n,m) values, free action and Burnside by direct count, Vandermonde and
  the factor-n relation, enumerator vs brute force, per-slot gauge identity (exact for Frobenius blocks, detectably broken
  for random blocks), pure-class identity, planted decomposition, detector self-test on synthetic counts, analysis pipeline.

Test: `CARGO_TARGET_DIR=<dir> cargo test --release` (13 passed). No experiment, sweep or benchmark has been run; the
planning table in PROTOCOL.md section 6 is closed-form arithmetic printed by a test.
