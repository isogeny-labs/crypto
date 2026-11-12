# Size rule `fes-then-inherited` and the `inherited-f4` fold, end to end (2026-11-12)

Owner's ask (2026-11-12): "do the rest of this stuff. Keep working on these
things", after the previous round (PR #3979:
[`../m4gb_update_signatures_20261112/`](../m4gb_update_signatures_20261112/),
[`../inherited_f4_next_20261112/`](../inherited_f4_next_20261112/)). This study
lands two follow-ups that round named.

## What changed (`src/cryptanalysis/ic_framework/solvers.rs`)

1. **Fold.** The engine `inherited-f4` now runs with the next kernels (SIMD
   row XOR, fused tallies, Four Russians table reuse, exact fused builders):
   the same rows, decisions and word-XOR count, measured at 0.86 to 0.94× the
   wall per call in `../inherited_f4_next_20261112/` (run 1). The previous
   kernels stay runnable as `inherited-f4-legacy`, the A/B arm.
   `inherited-f4-next` stays registered as an alias of `inherited-f4`, so the
   earlier sweeps reproduce.
2. **Size rule.** A new solver `fes-then-inherited` runs `fes-f2` on quadratic
   systems of at most `fes_max_vars` unknowns (default 14, at most 64
   equations) and `inherited-f4` on everything else. The cost record says
   which engine ran (`size_rule_fes`). The default 14 was fixed from the
   per-call word-XOR counts before this run: at 14 unknowns `fes-f2` counts
   about 33 thousand against `inherited-f4`'s 49 to 52 thousand; at 16
   unknowns 132 thousand against 102 to 124 thousand
   (`../inherited_f4_next_20261112/RESULTS_AB.md`). Both engines count word
   XORs, so S stays on one pricing basis.

Test: `ic_framework::solvers::tests::size_rule_and_inherited_fold_match_the_reference`
(both branches of the rule against the exhaustive reference; folded and
legacy `inherited-f4` give the same solutions and count). The module's 14
tests pass.

## Protocol

`run_e2e.sh` under `tools/benchrec.py run`: `ic bench --sweep` on the six
instances of the e2e ladder (Koblitz 13, 17, 19, 23; random 15, 21; same
subspace bases), the algebraic oracle with `inherited-f4`,
`inherited-f4-legacy` and `fes-then-inherited`, the pair table as the
reference row, 3 repeats, 60 s per call, 16 matched rho runs. Local
container, one thread, not isolated; no leaderboard row.

## Pre-stated expectation and falsifier

- `inherited-f4` and `inherited-f4-legacy`: identical S on every instance
  (same count), the folded engine with the lower wall per call.
- `fes-then-inherited`: S equal to `fes-f2`'s at degree 13 (14 unknowns: the
  rule picks `fes-f2`, which gave 94.7 against 123.1 in the previous run), and
  equal to `inherited-f4`'s on the other five instances (16 unknowns and up:
  the rule picks `inherited-f4`). So the size rule's row is the best
  algebraic row on every instance where the two engines differ.
- Falsifier: folded and legacy S differ on any instance; the rule's S at
  degree 13 not below `inherited-f4`'s; or the rule's S on any other instance
  differing from `inherited-f4`'s beyond run noise.

## Results

Filled after the run.
