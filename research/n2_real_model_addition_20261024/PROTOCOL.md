# Protocol: a cheaper genus-2 real-model addition for the H2 glued index calculus

Frozen 2026-10-24, before any run of `glue2` or the recorded `glue2check`.
Development runs of the formulas (the operation counts of `glue2check` while
the formulas were being written, and the replay of the 28-bit row) were seen
before this freeze; they are the source of the cost figures in the prediction
below and they are not results of this study.

## Question

Lead 2 of the round-4 report: the H2 route (gluing `E/F_{p^2}` to `E^sigma`, an
index calculus on the genus-2 `Jac(C)/F_p`) needs a Jacobian operation of about
22 M + I to reach parity with rho at `h = 1`. The validated baseline
(VAL-20261024-wd4) is 41 M + I per addition and 51 M + I per doubling, and
`IC/rho = 1.78 / 1.80 / 1.88` at 40 bits, `h = 1`, `I = 3M`, native work at
0 / 0.1 / 1 GAE. How far do cheaper explicit formulas move that distance, with
the algorithm unchanged?

## Class of the change

Engineering: same algorithm (the large-prime first-cycle index calculus with
the r-adding walk), same instances, same random stream, same walks; only the
group-law primitive changes. If the formulas are right the walks are identical
and the steps, relations and answers of the two arms are equal row by row. A
stage-level cost is reported as such; it becomes a pipeline result only through
the end-to-end `S` of the paired arms below.

## Baseline (named before optimising)

Arm `Jglue` of `H2-glue-run-20261018T2240Z` (tool `n2_routes.rs` at the commit
the run names; the unchanged functions `arm_glue`, `radd_fast`, `radd`,
`rmul_fast` of this tree), with its protocol, receipts and end-to-end cost:
`glue.csv`, certified by `validation/validation_report.yaml` (VAL-20261024-wd4).
The matched rho is arm `R` of the same file.

## Arms

- `R`: rho, unchanged (`arm_rho`).
- `Jglue`: the baseline, unchanged (`arm_glue`, `radd_fast`).
- `Jglue2`: `arm_glue2`, a line-for-line copy of `arm_glue` whose group
  operations are `radd_fast2` / `rmul_fast2`: the second-generation formulas of
  `n2_routes.rs` (`radd2_pre`, `rdbl2_pre`, `rpost2`). They are exact
  re-derivations; no literature formula is used and no literature count is
  quoted as a fact.

The `glue2` mode runs the three arms on every target. `Jglue2` is started from
the random state `Jglue` started from, and the stream continues from the state
`Jglue` ended in, so the `R` and `Jglue` rows of `glue2.csv` equal those of
`glue.csv` (H2) except for the trailing seconds. That is the replay control and
the pairing: instance, target, rho run and walk are the same.

## Instances and sizes (same seeds as H2)

`log2 #E in {28, 32, 36, 40}` and `h in {1, 4}`, 4 instances (seeds
`splitmix(0x6c750000 ^ (bits << 40) ^ (h << 8) ^ inst)`), 10 targets per
instance: 40 targets per cell, exactly H2's. 44 bits (`h = 1, 4`, same seed
rule, same counts) is added; H2 has no baseline row there, so at 44 bits the
baseline is the `Jglue` row of the same run. Single thread (`N2_THREADS=1`).
Every answer is verified by `k P = Q` on `E(F_{p^2})` before it is counted.

## Formula verification (before any claim of cost)

Run `glue2check`, recorded:

1. Exhaustive: every ordered pair of reduced divisors (including the zero
   divisor, doublings, equal-`u` pairs and negations) of three random
   squarefree monic sextics over each of `F_7`, `F_11`, `F_13`, plus a
   non-monic (twisted) sextic over each: `radd_fast2 == radd` on every pair.
2. Random: on glued instances at `log2 #E in {20, 28, 36, 40, 44}`, 10,000
   additions, 5,000 doublings and 20,000 special pairs (negation, zero,
   repeated operand) per size, each compared with the generic law `radd`, and
   the batched version on groups of `W in {4, 16, 64}`.
3. Positive control of the detector: the old `radd_fast` on the non-monic
   models. It assumes a monic sextic, so the check must flag it (it did in
   development: 399 of 729 pairs). The new formulas defer to `radd` there.
4. Counted cost per operation: add, double, walk add `D + R_j`
   (`R_j` one of the 32 fixed multipliers), old against new.

Unit tests: `rustc --edition 2021 --test` (the n2_routes tests, including
`real_model_second_generation_matches_generic`), `rustfmt --edition 2021
--check`, `clippy-driver --edition 2021 -D warnings`.

## Replay control

`N2_SIZES=28 N2_HS=1 n2_routes <dir> glue` must reproduce the 28-bit,
`h = 1` rows of `H2-glue-run-20261018T2240Z/glue.csv` in every column except
the trailing seconds. Recorded as a run directory with the diff.

## Cost model (frozen, unchanged from H2 plus the §5a pricing of Erratum V)

- Counted `F_p` operations, as `Counts`: M (squarings count as M), I, native.
- Pricing band (AGENTS.md §5a), 12 cells: `I in {3M, 2 log2 p M}` x Jacobian
  operation priced as measured or as one `E(F_{p^2})` addition x native
  operations at 0, 0.1 and 1 GAE (one GAE = the row's own measured
  `E(F_{p^2})` addition at the cell's `I`). The split test is priced at one
  Legendre symbol per walk step, at one `I` (Addendum H3). The same pricing is
  applied to every arm, rho included.
- Gate's three points: `I = 3M`, measured Jacobian cost, native work at 0, 0.1
  and 1 GAE.
- Ranking: two arms are ranked only where their order holds in every one of
  the 12 cells. Otherwise "unresolved". Tool: `n2_routes <dir> gaeband <csv>`
  (the `Jglue*` arms get the Legendre term).
- Distance to rho: reported on every row as the ratio `IC/rho` at the three
  gate points; never a pass/fail.
- Mixed addition: reported if a derivation finds D2-only precomputation that
  the counters can show; otherwise "no gain, same as general addition".

## Prediction (before the run)

The formulas cost 30 M + 1 I per addition and 39 M + 1 I per doubling in the
development counts (old: 41 + 1, 51 + 1). The walk is unchanged, so the
`Jglue2` counters are those of `Jglue` with the per-operation costs replaced
(adds and doubles split from `jac_M` and `jac_ops` of H2 at 41 / 51). Predicted
`IC/rho` at the three gate points (w = 0, 0.1, 1), baseline then new:

| bits, h | baseline `Jglue/R` | predicted `Jglue2/R` |
|:--|:--|:--|
| 28, 1 | 1.497 / 1.521 / 1.611 | 1.168 / 1.232 / 1.474 |
| 32, 1 | 1.692 / 1.721 / 1.818 | 1.324 / 1.407 / 1.683 |
| 36, 1 | 1.918 / 1.946 / 2.035 | 1.502 / 1.597 / 1.892 |
| 40, 1 | 1.777 / 1.802 / 1.876 | 1.393 / 1.480 / 1.745 |
| 40, 4 | 2.681 / 2.694 / 2.733 | 2.101 / 2.207 / 2.534 |

(The baseline column is H2's `glue.csv` priced by the `gae_band` rule; the 40-bit
`h = 1` row reproduces the validated 1.78 / 1.80 / 1.88.) The prediction is
nearly arithmetic because the walks are identical; what the run adds is the
proof that they are identical, the verification, and the recomputed bands.
Expected: an engineering improvement of about 22-25% of the IC cost at every
size, the IC still above rho at the gate's three points (no parity: the formula
needs about 22 M + I and has 30 M + I), and the 12-cell band still unresolved
at `h = 1` with fewer cells where rho is cheaper.

## Decision rules

- **Improvement** (a success per the owner's rule): all answers verified, the
  `Jglue2` rows equal the `Jglue` rows in steps, relations and found `k`, and
  `Jglue2` costs less than `Jglue` at each of the gate's three points and in
  every one of the 12 cells at a cell (bits, h). Reported with the delta against
  the baseline at the same points. Anything else is recorded as is.
- **Not an improvement**: a change in counts that comes from a relabelling
  (for example two multiplications that the old code evaluated twice). The
  report separates the 2 M of duplicate products in the old `radd_fast`
  (measured: removing them gives 39 M + I / 49 M + I) from the structural
  saving.
- **Beating rho**: only an end-to-end cold `S` below the matched rho's in every
  one of the 12 cells, robust at growing `n`, independently replayed. Not
  expected here.
- **Null**: if the walks differ, or a check fails, the arm is reported as
  invalid and the cause recorded.

## Out of scope for this freeze

Parallel lockstep walks with a shared (Montgomery) inversion change the walk
(different random stream), so they are not paired with `Jglue`. If run, they
are an arm of their own, fixed by an amendment appended and dated here before
that run. The batched-inversion cost per operation of independent additions is
measured by `glue2check` as a stage diagnostic only.

## Run bookkeeping

Each run directory `results/<item>-run-<UTC>/` holds `env.txt` (git HEAD,
sha256 of `examples/n2_routes.rs` and of the binary, the command, the sha256
of this file, rustc, host), the CSVs and logs. Seeds are fixed in the tool.
