# hecdlp_sge_coord_20261116: structured Gaussian elimination before the coordinate Wiedemann

SMOKE only. Not a verdict, no frozen protocol, all figures `[pcc:unverified]`.

## What was built

* `IcExt::sge_coord` (src/cryptanalysis/genus_ladder.rs). With `IcParams::sge = Some(sp)` the
  peeled core is reduced by the existing `sge_reduce`, then `wiedemann_coord(.., square = true)`
  runs on a random square row subset of the reduced matrix, with the target column mapped by the
  reducer's renumbering (`rct`). The answer is certified by group arithmetic as for the coord path.
  A failed certificate (or a singular subset) falls back, same attempt, counted, to the baseline
  `wiedemann` on the SAME reduced matrix (the merge is not repeated or double charged). Without
  `prm.sge` the flag does nothing. Default `IcExt` is unchanged (test
  `ext_default_is_run_ic_count_for_count` passes).
* Counting: `sge_reduce` charges the merge (`la_sge`, included in `la`). Merged rows have non-unit
  entries; `ic_control::mat_vec` charges one mul per non-(+-1) entry per product and one add per
  entry, so `generic_out` (reported by `SgeStats`) is what the solve pays for in mults.
  `skip_wiedemann_draws` is not used on this path (the coord solve draws from the stream directly,
  as `coord_square` does); the relation phase does not depend on it (relations equal arm w's in the test).
* Binary arm `s` (src/bin/genus_ladder.rs): arm w + merge + coord solve; `--sge W,F` repeatable,
  default 4,1.5; runs only merged. Record arm name `..+coord+unitT+sgecoord+sge(W,F)`.
  Existing `w` with `--sge` is the older "merge then baseline Wiedemann" (`..+coord+unitT+sge(W,F)`).
* One-line edit outside my files: `src/cryptanalysis/genus_ladder_sieve.rs` has an exhaustive
  `IcExt { .. }` literal; I added `sge_coord: false` so the crate compiles. Nothing else touched there.

## Tests (release, `cargo test --lib genus_ladder::tests`: 14 passed)

* `sge_coord_certifies_and_cuts_la`: p in {131,257}, g in {3,4}, SgeParams (4,1.5) and (8,3):
  certified, 0 bad relations, secret recovered, relations equal arm w's, la.mul accounting adds up,
  and where the core shrank by >= 20 % and neither run fell back, LA mults < arm w's (6 runs).
  Examples (cols in->out; la_wied mul/add vs arm w): p=257 g=4 (8,3): 107->41, 10262/41034 vs 29540/138348.

## Smoke (genus 4, --max-h 2, --cc 2.0, --restart-sqrt, 3 curves/prime, solver seeds 1-2, 1 thread)

`--curve-seeds 2053=40/60/109,4099=72/247/418 --arms ws --sge 4,1.5 --sge 6,2 --sge 8,3`;
raw `smoke.jsonl`, binary sha `binary.sha256`, table `summary.txt` (mean of 6 runs, all 84 certified,
0 LA fallbacks, one attempt each). LA totals (graph+peel+merge+solve) in native ops:

| p | arm | core cols -> reduced | LA mul | LA add |
|---|---|---|---|---|
| 2053 | w | 574 | 9.32e5 | 5.81e6 |
| 2053 | s (4,1.5) | 422 | 5.63e5 | 4.13e6 |
| 2053 | s (6,2) | 352 | 4.41e5 | 3.81e6 |
| 2053 | s (8,3) | 312 | 3.97e5 | 3.73e6 |
| 4099 | w | 980 | 2.75e6 | 1.88e7 |
| 4099 | s (4,1.5) | 763 | 1.79e6 | 1.44e7 |
| 4099 | s (6,2) | 654 | 1.42e6 | 1.35e7 |
| 4099 | s (8,3) | 575 | 1.24e6 | 1.36e7 |

Merge cost itself: <= 42 mul, <= 1e5 add per run (negligible). Merge + baseline Wiedemann
(`w+sge`) is worse than w (e.g. 4099: 5.4e6 mul at (4,1.5)), as expected: the random-projection solve
is where arm w's gain came from. Relation-phase mults (1.65e7 at 4099, 6.7e6 at 2053) are still
5-10x the LA mults here, so SGE moves total native work by roughly 5 % at these sizes; LA share grows
with p (N ~ p^{3/4}, LA ~ N^2).

## Open issues / unverified

* Smoke is 6 runs per cell on 6 instance/seed pairs (same relations for w and s): no intervals, no
  matched-rho pricing (AGENTS.md 5a) done here, no validation-gate artefacts.
* A random square subset of the merged matrix being nonsingular was never a failure in the smoke, but
  with a thin surplus (core rows barely above cols) fallbacks may rise; not measured.
* Did not verify against the literature in this session (no KB queries run): SGE/"filtering" as in NFS
  (Cavallar 2000, Lenstra-Lenstra-style merging; Odlyzko 1985 for early structured elimination) and
  block-Wiedemann/BM cost (Coppersmith 1994, Thome 2002) are cited from memory; the cost claims here
  are only the counted figures above.
* Only genus 4 smoked; genus 3 covered by unit tests only.
