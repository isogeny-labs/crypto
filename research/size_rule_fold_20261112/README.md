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

## Results `[pcc:unverified]`

Run 2026-11-12 16:12 to 16:16 Z, record
`benchmarks/records/20261112T1612Z-size-rule-fold-e2e.json` (measured; raw
`results.tar.gz`, `results.json`), binary SHA-256 in `results/ic-sha256.txt`
(a frozen copy of commit 4840612771's build). Full table `RESULTS.md`, graph
`results.svg/png`, `REPORT.pdf`. Every one of the 24 configurations
recovered its planted logarithm in every repeat, and so did every matched
rho run.

| instance (unknowns) | rho S | pair table S | `inherited-f4` S | `-legacy` S | `fes-then-inherited` S (engine it ran) |
|:--|--:|--:|--:|--:|--:|
| Koblitz 13 (14) | 2.89 | 197.8 | 138.0 (117.8–167.6) | 138.0 | **101.9 (83.7–116.3)** (`fes-f2`) |
| random 15 (16) | 2.28 | 473.7 | 278.8 | 278.8 | 278.8 (`inherited-f4`) |
| Koblitz 17 (18) | 1.73 | 493.2 | 304.5 | 304.5 | 304.5 (`inherited-f4`) |
| Koblitz 19 (20) | 1.35 | 1,063.3 | 465.5 | 465.5 | 465.5 (`inherited-f4`) |
| random 21 (22) | 1.24 | 1,975.8 | 4,763.3 | 4,763.3 | 4,763.3 (`inherited-f4`) |
| Koblitz 23 (24) | 0.88 | 3,986.3 | 2,051.2 | 2,051.2 | 2,051.2 (`inherited-f4`) |

Mean over 3 repeats (range where it matters).

- **The fold changes no S**: `inherited-f4` and `inherited-f4-legacy` agree
  to every digit on all six instances, as the identical word-XOR counts
  require.
- **The size rule is the best algebraic row on every instance.** At degree
  13 it runs `fes-f2` and gives S 101.9 against `inherited-f4`'s 138.0 (0.74×;
  35.3× the matched rho instead of 47.8×), the three repeats not overlapping
  (83.7–116.3 against 117.8–167.6). On the other five instances it runs
  `inherited-f4` and has exactly its S. The pair table stays best on random
  21. The expectation held; the falsifier did not fire.
- **The degree-13 figures differ from the previous run** (fes-f2 94.7 and
  inherited-f4 123.1 over 2 repeats): the third repeat of this run is the
  dearer one for both engines (116.3 and 167.6), and the matched rho mean
  moved from 2.76 to 2.89 with its own random runs. The ratio is 0.77× there
  and 0.74× here.
- **Wall inside the pipeline is not a paired comparison.** The sweep runs
  the configurations one after another on shared per-thread caches, so the
  folded `inherited-f4` (run first) is below `-legacy` per call on four
  instances (0.89 to 0.92×) and above it at Koblitz 13 and 17 (1.05 and
  1.08×). The paired per-call measurement of the same kernels is
  `../inherited_f4_next_20261112/` run 1 (0.86 to 0.94× on every cell).

No leaderboard row: no replay certificate or validator report (validation
gate); a validated isolab run of the degree-13 configuration is being
prepared separately (`../fes_k13_validated_20261112/`).

## Sources checked

KB (`research-kb`) unavailable in this session. `src/cryptanalysis/ic_framework/mod.rs`
(solver pricing: word-XOR units priced by count),
`src/cryptanalysis/ic_framework/solvers.rs`,
`research/inherited_f4_next_20261112/README.md` and `RESULTS_AB.md`.
