# Amendment 1 (2026-11-15): guard identity 2 after any narrowing

Written and committed before the amendment's runs. Prompted by the independent validator
(VAL-20261115-b1e7a3, `../validation/validator-report.json`, discrepancy D1).

**Defect.** Identity 2 (V_i^T V_0 from the three-term recurrence) needs V_{i-1}^T M V_0 = 0, which for i >= 4 relies
on long-range M-orthogonality. Narrowing in the middle of a run breaks that, and the frozen `src/la2.rs` only
disables the recurrence after narrowing at iteration 0 (`narrowed0`). The validator drove `block_lanczos` directly
on random small-field matrices: 1,922 of 2,990 mid-run-narrowing calls (all at ell <= 101) used an inexact value and
24 returned a different kernel vector. In this study's 60 units and the validator's 56 replays, narrowing happened
only in the final iteration, so identity 2 was never reused after a narrowing (oracle build: 0 inexact uses).

**Change.** `amendment-1/src/la2.rs` differs from `../src/la2.rs` in three lines: the flag becomes `narrowed_any`,
set on every narrowing, and the recurrence is used only while it is false. Identity 1 is unchanged (exact in every
path, validator). `#[path]` includes in `src/v3.rs` gain one `../` for the deeper directory.

**Check (pre-registered).** Build BLR1 from `amendment-1/src` with `rustc --edition 2021 -O v3.rs --cfg combo`, run
the 30 BLR units of `../units.tsv` (k = 18..22, seeds 1–6, `ic <q> <seed> D 3.0 1.7`), and compare every field
except those ending wall/cpu, load1 and s.controls* with the committed BLR records in
`../runs-blreuse-20261115.tar.gz`. Pass iff every unit is identical (then every number of the study stands for the
guarded code). Any difference: the study's numbers are re-derived from BLR1 records and the difference is reported.
