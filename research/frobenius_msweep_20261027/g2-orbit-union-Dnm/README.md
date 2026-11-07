# Cell G2: per-slot-shift orbit-union bases vs the exact class count D(n,m)

Status: prepared, not run. (Amendment 2026-10-27, PROTOCOL.md section 14: ladder NOT_EVALUABLE on l = 5 instrument failures; contrast saturation limit stated.) (Revision 3, after two independent reviews; see PROTOCOL.md section 13: mod-4 null for a = 0, calibrated z, direct frob-minus-frobperm contrast, targets merged under negation, coded ladder criterion.)

Lead re-examined: LEAD-ICB-frobenius-search-symmetry (campaign 2026-10-27, branch claude/frobenius-symmetry-m-sweep-20261027).
Std-only Rust crate (own `[workspace]`, not part of the root build). Reuses the GF(2^n) exp/log field and the binary
Weierstrass point arithmetic of `research/twist_obstruction_20261026/src/main.rs` (restricted to F_q, no F_{q^2}).

* `PROTOCOL.md` frozen design: claim classes, arms (frob, frobperm nuisance-matched control, rsub, rset, skew, geom, plain, plainbig), parity-aware null, decision rule, 52-cell grid, power/resolution, runtime, exclusions, disclosures.
* `RUN_CARD.md` the token command, exact later commands, expected wall time, output paths.
* `run_cells.sh` driver (gate-checked, clean-tree-checked; do not launch before the checkpoint exists).
* `src/` field, curve, combinatorics (D(n,m), diagonal classes, parity and mod-4 weights), block families, exact enumerator, runner, frozen analysis.
* `tests/identities.rs` 30 tests (rev 3 adds: mod-4 sign-count structure and weight, exact zero for a type-2-only block, z calibration at two D, contrast verdicts, audit/skew/incomplete failures, ladder): field/curve laws, an independent table-free group-law check, D(n,m) values, free action and Burnside,
  Vandermonde and the factor-n relation, halving criterion, parity null vs exact zero on a ker-Tr family, enumerator vs brute force
  (m = 2..5), per-slot gauge identity, frobperm properties, distinct-orbit targets, pure-class identity, planted decomposition, row round trip,
  runner plumbing on a tiny fixture, detector self-test, and pinned verdicts on planted synthetic fixtures.

Test: `CARGO_TARGET_DIR=<dir> cargo test --release` (30 passed). No experiment, sweep or benchmark has been run; the planning table in
PROTOCOL.md section 6 is closed-form arithmetic printed by `g2dnm plan` and a test.
