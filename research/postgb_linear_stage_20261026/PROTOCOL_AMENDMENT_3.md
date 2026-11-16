# Protocol amendment 3: the fast arm's counter, and the matched rho reference

Frozen 2026-10-27, after the R2 cards and before any R3 card. It changes
nothing in `PROTOCOL.md` or amendments 1 and 2 except what is stated here.

## What happened

**The counter.** A review of the PR found that `fast_stage` did not count all of
its multiplications from the kernels that ran them:
- The powering step of the root finder (`Powering::square`) was charged
  `3d²/2` per squaring. That is a formula: the kernels skip zero coefficients
  and switch to Karatsuba above length 256.
- `times_x` was not charged at all.
- `pmul_school` charged `|a|·|b|`, including the rows it skips.

So the fast arm's `muls`, and the `lin_muls` that `JV_LIN=fast` adds to S,
were partly a formula and not a count.

The fix (`src/cryptanalysis/zero_dim_la.rs`) counts every product the kernels
execute: one per nonzero coefficient times the row it scales. The powering
charges the difference, and `times_x` charges `d` when it reduces. Nothing
else in the arm changed. The fix affects no solution, no root and no wall
time beyond the cost of the counter.

**The rho reference.** The rho reference that `jv_cover --exp dlp` runs in the
same process (`rho_e`) has no negation map. The strongest rho on this curve
uses the negation map: `y² = x³ + a₂x² + a₄x` with `α ∉ F_q` has only the
automorphisms ±1, and no Frobenius acts on it. `rho_e_neg`
(`JV_RHO_NEG=1`, commit `39a72dbe5e`) adds it, after the clock.

## Changes

1. **R3-C, the corpus counts.** One process of the fixed `postgb_la_bench`
   runs the whole corpus, every family as in R2-L1–L6:

   `--families jv-nagao,gaudry-s4,gaudry-d4,gaudry-s5,dense --per 8 --reps 1 --arms sparse,fast`

   It is one card, with a 7000 s timeout, and writes `runs/R3-C.json`.
   - Its counted multiplications, for both arms in the same run, replace
     the R2 counts in every counted figure and in P4's counted clause.
   - Its timings are not used; wall time stays R2's.
   - Its solution sets must equal R2's on every system, or the study
     fails the correctness gate.
2. **R3-D1 and R3-D2, the DLP counts and the reference.** The fixed `jv_cover`
   runs R2-D1's and R2-D2's command, with `JV_RHO_NEG=1` and
   `JV_DUMP_INSTANCE=<card>.instances.jsonl`, writing `runs/R3-D1.json` and
   `runs/R3-D2.json`.
   - Their S, `lin_muls`, S/rho and S/rho_neg replace R2's counted figures.
   - The linear stage's wall time stays R2-D's (`lin_ms`).
   - Relations, the solved logarithms and the plain rho runs must equal
     R2-D's.
   - The §5a matched reference is `rho_e_neg` in the same process: S of
     each arm against the mean S of its negation-map rho runs.
3. **Native analysis.** `analysis/summary.json` is written by the Rust
   example `examples/postgb_la_summary.rs`, which replaces `scripts/summary.py`
   (AGENTS.md, "Implementation language: no Python"). It uses the same
   measures and the same bootstrap (4,000 resamples over systems), with
   Rust's `StdRng` seeded 20261027, so interval endpoints may move in the
   last digits. `scripts/plots.py` only lays out `summary.json`.
4. **The main session runs the three R3 cards** with `tools/task_card.py`, as
   under amendment 2, on the current host. The host is recorded by `lscpu`.
   Counts do not depend on the host.

## Unchanged

- Arms, corpus, seeds, rounds and wall-time measures.
- Predictions P1–P5, scored as before, with P4's counted clause now on R3
  counts.
- The decision rule and the dense-arm cap.
