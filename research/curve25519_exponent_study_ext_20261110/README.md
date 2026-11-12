# Exponent study, extension: m = 3 and m = 4 at larger |V| under raised limits (2026-11-12)

**Class.** An engine measurement in counted units: F_p multiplications of the F4 row reduction,
`F4Report.field_ops`, from `curve25519_family f4sub`, with the same frozen binary, seeds and cell
format as the parent study
[`../curve25519_exponent_study_20261108/`](../curve25519_exponent_study_20261108/README.md). It
measures the cost T(E,m,L) of one symmetrised decomposition system against the base size |V|. It is
not an end-to-end solve, not an ecbench session, and not a leaderboard or `benchmarks/` row. No pcc
claim was registered, so every measured number below is an unverified assertion. Rho remains ahead
on every track [pcc:unverified].

**Question (owner, 2026-11-10).** The parent study left the m = 3 and m = 4 verdicts
"undetermined" because its limits (3.5 GB, 1,200 s per target) censored the series before the
preregistered estimator could apply. This extension raises the limits to 12 GB of virtual memory per
process and 10,800 s per target, adds the even divisors of p - 1 at k = 20 (the parent used odd d
only), and reruns the parent's censored cells. The estimator and the lever thresholds are the
parent's (section 5 there): a lever at m = 4 needs the fitted exponent s of T in |V| below 0.5 with
upper 95 % bound below 0.5; at m = 3 it needs s below 0 with upper bound below 0.

**Answer.** The verdicts at m = 3 and m = 4 stay "undetermined" under the preregistered estimator,
and no reading meets a lever rule [pcc:unverified]. Four new cells completed, all at k = 20, m = 3
(d = 12, 14, 18, 28). Every larger cell, and every m = 4 cell at or beyond d = 12, exhausted 12 GB
of memory before its first target finished; none was censored by time [pcc:unverified]. The binding
limit on this route at these sizes is the memory of the F4 matrix, not wall time.

- **Preregistered estimator: still not applicable.** It needs at least 4 complete cells with d >= 9
  spanning 4x in |V|. At k = 20, m = 3 there are now 8 such cells (d = 9 to 28) but they span
  3.1x; the span gate needed d = 36, which censored. At m = 4 only d = 9 is complete above the
  boundary cells. At k = 22, m = 3 nothing changed (6 cells, 3.7x) [pcc:unverified].
- **Other readings (not verdicts).** The d >= 9 fit at k = 20, m = 3, computed despite the span
  gate, is now s = 6.08 [1.69, 10.47] over 8 cells; in the parent it was 5.56 [-8.75, 19.88] over
  4 cells. The extension cells therefore move the lower bound of this reading above 0, so the
  d >= 9 cells alone now exclude s <= 0 at 95 % at m = 3, k = 20 [pcc:unverified]. The sensitivity
  fit over all 10 complete cells (d = 3 to 28) gives 6.87 [5.01, 8.73]; the four even cells alone
  give 7.15 [1.38, 12.92]; the odd cells with d >= 9 alone give 5.56 [-8.76, 19.88]
  [pcc:unverified]. Every point estimate is far above both thresholds (0 and 0.5), and no upper
  bound is below them.
- **Cost does not follow |V| alone.** The even cells are much dearer than their odd neighbours:
  d = 18 cost 2.43e10 field operations against 2.20e8 at d = 19 (110x), and d = 12 cost 7.75e8
  against 1.48e8 at d = 9 (5x), while d = 14 cost the same as d = 12 and d = 28 cost 0.77x of
  d = 27 [pcc:unverified]. The symmetrised system has one chain variable per prime factor of d
  per summand (d = 19: 1; d = 14: 2; d = 12, 18, 28: 3), so the number of prime factors of d,
  with multiplicity, matters as much as |V| = d. A single power law in |V| is a coarse model for
  this series, which is one more reason the span gate was not met.

## 1. Predictions (frozen in [`PROTOCOL.md`](PROTOCOL.md) before data) and outcomes

| Prediction | Outcome | Basis |
|---|---|---|
| P1: k = 20, m = 3, d = 57 censors again at 10,800 s | **not testable** | the chain stopped at d = 36 (memory), so d = 57 was never rerun under the chain rule |
| P2: >= 4 even m = 3 cells complete, the estimator applies at k = 20, m = 3, s > 3 with lower bound > 0 | **failed** | 4 even cells completed (12, 14, 18, 28), but the estimator does not apply: 8 cells with d >= 9 span 3.1x < 4x |
| P3: k = 20, m = 4, d = 19 censors under 12 GB / 10,800 s | passed | memory allocation failure after 1,027 s |
| P4: m = 4 cells d = 12, 14, 18 complete, all two-point slopes above 2 | **failed** | d = 12 exhausted 12 GB after 6,226 s; d = 14 and 18 were not attempted (chain rule) |
| P5: k = 22, m = 3, d = 55 censors again (memory) | passed | memory allocation failure after 5,286 s |
| P6: binary sha256 matches `data/frozen.sha256` at launch; rerun cells' first target_x equals the parent's (610025 for d = 57) | passed in part | the hash matched at each launch and in every cell record; the d = 57 part is not testable (not rerun); the rerun cells d = 19 (m = 4) and d = 55 (k = 22) aborted before the engine recorded a target, so no target_x comparison exists |

Failed predictions stay failed. P2 and P4 both failed on the resource side: the memory limit, not
the measured slopes, decided them. The slope part of P2 (s > 3, lower bound > 0) holds in every
reading that has a lower bound above 0, but that is not the preregistered estimator.

## 2. Cells

Seeds are the parent's (20261108 + 1000 m + d), 6 targets per cell, G = 0, form `sym`, one process
at a time with `RAYON_NUM_THREADS=1`. T is the mean of `field_ops` over the 6 targets; the six values
agree to within 1e-5 relative in every complete cell. All 24 completed targets were inconsistent
systems (a uniform target is a sum of three base points of a base this small with negligible
probability), as in the parent.

| k | m | d | prime factors | |V| | T (field ops) | solving degree | max columns | wall | outcome |
|--:|--:|--:|---|--:|--:|--:|--:|--:|---|
| 20 | 3 | 12 | 2·2·3 | 12 | 7.748e8 | 7 | 7,759 | 16 s | complete |
| 20 | 3 | 14 | 2·7 | 14 | 7.766e8 | 10 | 10,651 | 18 s | complete |
| 20 | 3 | 18 | 2·3·3 | 18 | 2.4275e10 | 7 | 13,616 | 298 s | complete |
| 20 | 3 | 28 | 2·2·7 | 28 | 2.0161e11 | 11 | 54,949 | 3,693 s | complete |
| 20 | 3 | 36 | 2·2·3·3 | 36 | | | | 4,521 s | censored: memory, 12 GB |
| 20 | 4 | 12 | 2·2·3 | 12 | | | | 6,226 s | censored: memory, 12 GB |
| 20 | 4 | 19 | 19 | 19 | | | | 1,027 s | censored: memory, 12 GB (parent: memory, 3.5 GB) |
| 22 | 3 | 55 | 5·11 | 55 | | | | 5,286 s | censored: memory, 12 GB (parent: memory, 3.5 GB) |

Every censor is an `exit -6` with "memory allocation of N bytes failed" on stderr under
`ulimit -v 12000000`; no target reached the 10,800 s cap. Cells d = 38 to 133 at k = 20, m = 3,
d = 21 to 57 at k = 20, m = 4, and d = 75, 93 at k = 22, m = 3 were not attempted, by the chain rule.

Parent cells used in the fits (unchanged): k = 20, m = 3: d = 3, 7, 9, 19, 21, 27; k = 20, m = 4:
d = 3, 7, 9; k = 22, m = 3: d = 3, 5, 11, 15, 25, 31, 33, 41. Full numbers: `data/analysis.json`.

## 3. Fits

The estimator is the parent's section 5: OLS of ln T on ln |V| over complete cells with d >= 9,
applied only with at least 4 cells spanning 4x, two-sided 95 % t-interval.

| group | preregistered fit (d >= 9, gate) | d >= 9 despite the gate | all complete cells (sensitivity) |
|---|---|---|---|
| k = 20, m = 3 | not applicable: 8 cells, span 3.1x | 6.08 [1.69, 10.47], 8 cells | 6.87 [5.01, 8.73], 10 cells, span 9.3x |
| k = 20, m = 4 | not applicable: 1 cell | (fewer than 3 cells) | not applicable: 3 cells, span 3.0x |
| k = 22, m = 3 | not applicable: 6 cells, span 3.7x | 4.51 [-1.27, 10.28], 6 cells | 5.76 [3.77, 7.75], 8 cells, span 13.7x |

Parent values for comparison: k = 20, m = 3 d >= 9 despite the gate 5.56 [-8.75, 19.88] (4 cells);
all cells 6.44 [3.49, 9.39] (6 cells). The k = 22 row is the parent's, since d = 55 censored again.

Two-point local slopes at k = 20, m = 3 in d order: 9 to 12: 5.8; 12 to 14: 0.0; 14 to 18: 13.7;
18 to 19: -87.0; 19 to 21: 25.4; 21 to 27: 18.1; 27 to 28: -7.4. The sign changes at each parity
change of d.

Under the paper's model (omega = 2) the relation and linear-algebra phases balance at total
exponent 2/(m + 1 - s). With s >= 4.5 in every reading, the denominator is at most -0.5 at m = 3
and at most 0.5 at m = 4, so no balance below the generic 1/2 exists at either m. This
extrapolation is derived from a fixed small p, not measured.

## 4. Protocol, freeze and changes after the freeze

- [`PROTOCOL.md`](PROTOCOL.md) and `scripts/run_ext.py` were frozen in commit `0afe01c59d`
  (2026-11-10T15:38Z), before any cell. `data/frozen.sha256` is the parent's file (protocol,
  parent scripts and binary hashes); the binary line is the one this study checks.
- Post-freeze fix before any data (`4424e037d8`, 15:39Z): the runner's launch check read the first
  line of `frozen.sha256` (the parent protocol's hash) instead of the binary's line and refused to
  start. The fix selects the line ending in `curve25519_family`. No cell had run.
- `scripts/analyse_ext.py` and `scripts/figure.py` were written after the freeze, while the last
  chain was running (7 of 8 cells done). `analyse_ext.py` reimplements the parent's frozen
  estimator on the union of parent and extension cells and encodes the P1 to P6 rules of the
  protocol; its "other readings" (fit despite the gate, even/odd split) are not preregistered and
  are labelled so above. The union rule: an extension cell replaces the parent cell with the same
  key and d; the parent's censored d = 57 was not rerun and is kept as the parent's.
- A smoke test of an even divisor (d = 4, m = 2, 1 target, about 30 s) ran before the freeze to
  check that the engine accepts even d; it is not a study cell.

## 5. Host, restarts and other disclosures

- **Host.** The same shared 4-core, 15 GB container as the parent; no EKS, Conductor, S3 checkpoint
  or benchmark record (`benchrec`) because none of them is reachable from this runtime; this is a
  deviation from the repository's default execution path, disclosed as in the parent. Load
  averages (1 min) before and after each cell are in each cell's `ext` block; they were 1.0 to 1.5
  for most cells, 3.0 before d = 18 and 4.6 before k = 22 d = 55. Wall time is not a claimed
  quantity; only `field_ops` is, and memory censoring does not depend on load.
- **Three container restarts** killed the detached runner (`data/env.txt`): 2026-11-10 19:40Z
  during d = 18 (k = 20, m = 3), 2026-11-11 20:12Z during d = 12 (k = 20, m = 4), and 2026-11-12
  01:43Z during d = 55 (k = 22, m = 3). The runner writes a cell file only when the cell ends, so no
  partial data exists; each interrupted cell was rerun from scratch with the same seed. The second
  check-in trigger reached the session about 20 hours late, which is why the run spanned two days.
  After the second restart a tracked background waiter kept the session active between cells.
- **Binary.** `/home/user/bin-archive/curve25519_family-dc2c30a4`, sha256
  `dc2c30a4242d80281622b2cb42cbaf9cb457d041ffbc9a4d35794bb893bf3faf`, equal to the parent's frozen
  hash at every launch (`data/run.log`, "binary ok") and recorded in every cell's `ext.bin_sha256`.
  It is not committed to Git; the parent's artifact-store upload covers its raw records, not the
  binary.
- **Memory accounting.** `ulimit -v 12000000` limits virtual address space, so the engine may have
  failed below 12 GB of resident memory. The failure point is therefore an upper bound on what the
  matrix needs at that cell, not a measurement of it.

## 6. What this changes and what is next

- The parent's open item "the m = 3 and m = 4 exponent cells need more memory and time than the
  local container allows" is now quantified: 12 GB does not reach the span gate at m = 3
  (d = 36 fails) and does not reach a second non-boundary cell at m = 4 (d = 12 fails). Time was
  never the limit at these sizes.
- Applying the preregistered estimator at m = 3 needs one of: more memory for d = 36 (k = 20), or a
  host where k = 22 d = 55 fits; at m = 4 it needs d = 36 or beyond, which is out of reach of this
  engine's dense F4 at any plausible memory. The alternative is to change the engine (sparse
  elimination, or a Gröbner engine with a degree bound), which is an engine change, not a
  measurement, and would need its own freeze.
- Nothing here is an improvement over the index-calculus baseline; the study is a closure
  measurement on the exponent question, reported with its gap to rho as in the parent
  [pcc:unverified].

## 7. Files

| File | Content |
|---|---|
| `PROTOCOL.md` | frozen protocol (cells, limits, estimator, predictions P1 to P6) |
| `scripts/run_ext.py` | frozen sequential, resumable runner |
| `scripts/analyse_ext.py`, `data/analysis.json` | estimator on the cell union, prediction decisions, other readings |
| `scripts/figure.py`, `exponent_ext.svg`, `exponent_ext.png` | figure: T against |V| at m = 3 and 4, k = 20 and 22, parent and extension cells, censored cells marked |
| `cells/*.json` | the 8 cell records (parent format plus an `ext` block with limits, load, wall, binary hash, exit) |
| `data/run.log`, `data/env.txt`, `data/frozen.sha256` | run log, launch and restart notes, frozen hashes |
| `REPORT.typ`, `REPORT.pdf` | the report (pandoc to Typst, compiled with the typst Python module) |
| `validator-report.json` | independent validation (see "Validation") |

## Validation

Pending: filled in from `validator-report.json` once the independent validator has run.
