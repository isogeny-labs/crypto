# Protocol: one-solve linear algebra, Lanczos, and excess + clique removal on the genus-5 cover (frozen 2026-11-16, before data)

Lead: `LEAD-TRF-quartic-genus5-humbert-cover`. Follows `../dlp_filter_20261115/` (PR #4065).
There the filtered double-large-prime matrix (`dlpf`) cut Wiedemann work to 0.72–0.77× of full
elimination, but the linear algebra was still 1.6× that of single large primes (`lp`). The
wall-time lead of `dlpf` over `lp` fell from 1.31 to 1.02 between q = 8009 and 24001, because
the linear algebra (63 of 96 s at q = 24001) grows faster than relation collection. Three levers
on that phase are tested here, each in its own arm.

1. **One solve (`w1`).** Every solve so far ran scalar Wiedemann twice under different column
   scalings and compared the results, to detect a kernel of dimension > 1. In 180 of 180 earlier
   runs (`v3`, `v4`) a single round sufficed. `w1` runs one Wiedemann solve, keeps the empty-column
   test and the row check M·x = 0, and relies on the end-to-end check x·P = Q after the descent
   (`e2e_ok`, timed; it does not use x_true). A failed check would cost one more solve plus the
   descent. That repair path is not implemented here: any failure falsifies S3, and the expected
   repair cost is bounded in the report.
2. **Lanczos (`l1`).** Scalar Lanczos (LaMacchia–Odlyzko) on the same A = M′ᵀM′, with the same
   framing (random diagonal scaling, one coordinate fixed, the row check). It needs about n
   iterations against Wiedemann's about 3n (2n for the sequence, n to rebuild the solution), and
   it is run once. Every mulmod in Lanczos is counted, including dot products and vector updates.
   Wiedemann's counter omits its dot products, which biases the counted comparison against Lanczos.
3. **Excess + clique removal (`dlpx`).** The same collection as `dlp:0.5`, but it stops at
   unknowns × 1.10 + 5 full relations instead of unknowns + 5 (HUMBERT_EXCESS = 0.10). After
   singleton pruning, whole trees of rows linked by large primes of degree 2 are removed heaviest
   first (total non-zeros) until rows − columns = 5. Each tree removed lowers the excess by exactly
   1 and takes its non-zeros and large-prime columns with it. Then come the greedy merges of `dlpf`.

## Arms (same instance per (q, seed); one frozen binary, sha256 prefix in `BINARY.sha256`)

All arms run with `HUMBERT_LA_PRECHECK=1`.

| tag | arm | solver | role |
|:--|:--|:--|:--|
| `pre_l1` | `iota` | l1 | hosts the instance's rho run (matched reference) |
| `lp50_w2` | `lp:0.5` | w2 | replay of `v4` lp50 (control, baseline) |
| `dlpf50_w2` | `dlpf:0.5` | w2 | replay of `v4` dlpf50 (control, baseline: current best IC arm) |
| `dlpf50_w1` | `dlpf:0.5` | w1 | isolates the second solve |
| `dlpf50_l1` | `dlpf:0.5` | l1 | isolates Lanczos (against w1) |
| `lp50_l1` | `lp:0.5` | l1 | lp under the new solver |
| `dlpx50_l1` | `dlpx:0.5` | l1 | isolates excess + clique removal (against dlpf50_l1) |
| `dlpx50_l1_aa` | `dlpx:0.5` | l1 | A/A, own process |

Grid: q ∈ {16001, 24001, 32003}, seeds 1–5, twist +1 for odd seeds and −1 for even. One launch
per q (`PRIMES=<q> run_v5.sh <dir>/q<q>`), so each launch stays under the 2 h background limit.
Arm order is rotated per seed (`(seed + k) % 8`). Rho is measured in the `pre_l1` process; every
arm is compared with that instance's expected-steps rho √(πr/2) × measured s/step.

Before data:
- The crate's 7 tests pass. The Wiedemann test now also checks that Lanczos recovers the planted
  kernel vector, and the filter test covers clique removal to a target excess.
- One smoke run of each new configuration at q = 16001, seed 1 solved, passed every check, and
  `dlpf:0.5` w2 reproduced its `v4` counted fields. The smoke is not used in any analysis.
  The predictions below were written before the smoke output was read and were not changed after.
  The smoke binary predates the `summarize-solver` subcommand; the `attack` code is identical.

## Metrics

- Counted (host-independent): relation candidates; LA columns, rows, nnz and mulmods; cliques and
  clique rows; e2e_ok; solved.
- Wall time (exploratory, unisolated L0): t_ic, t_relations, t_linear_algebra; paired ratios on
  the same instance; IC / expected-steps rho.
- Correctness gates on every run: solved, e2e_ok, pushforward 8P, T(P) ≠ 0, iota pairs constant,
  X_oo = 4c.

## Predictions and falsification (fixed now)

- C0 (control, counted):
  - `lp50_w2` and `dlpf50_w2` at q ≤ 24001 reproduce their `v4` counted fields (rel_candidates,
    la_mulmods, x_recovered).
  - `dlpf50_w1` and `dlpf50_l1` have the same rel_candidates as `dlpf50_w2`, and `lp50_l1` the
    same as `lp50_w2`, on every instance.
  - `dlpx50_l1_aa` has the same counted fields as `dlpx50_l1`.
  - **Failure** of any part invalidates the comparisons it supports.
- S1 (counted): LA mulmods of dlpf50_w1 / dlpf50_w2 lie in [0.45, 0.55] on every instance.
- S2 (counted): LA mulmods of dlpf50_l1 / dlpf50_w1 ≤ 0.50 on every instance. Same for
  lp50_l1 / lp50_w2 ≤ 0.25 (one solve and Lanczos together).
- S3 (safety): every w1 and l1 run has e2e_ok = true and solved = true. **Falsified by** any
  failure.
- S4 (wall time): the median per q of t_linear_algebra(dlpf50_l1) / t_linear_algebra(dlpf50_w2)
  is ≤ 0.35 at every q.
- X1 (counted): dlpx50_l1 LA mulmods < dlpf50_l1 LA mulmods on every instance.
- X2 (wall time): dlpx50_l1 t_ic < dlpf50_l1 t_ic on at least 4 of 5 seeds at q = 24001 and 32003.
- G1 (wall time): with l1, dlpf beats lp (dlpf50_l1 t_ic < lp50_l1 t_ic) on at least 4 of 5
  seeds at every q.
- G2 (wall time): the lowest median IC / expected-steps rho among the l1 arms is < 1 at q = 24001
  and 32003. **Falsified if** ≥ 1 at either.
- F5 (exploratory, no decision): log–log slope of median t_ic over the grid for every arm.

## Limits stated in advance

- Local 4-vCPU shared VM, no isolation (L0). Every wall-time number is exploratory and an
  unverified assertion; isolab-eks is not reachable from this session (no AWS identity). Nothing
  else runs on the host during the campaign.
- C0, S1, S2, S3 and X1 are counted or exact and do not depend on the host.
- §5a: index calculus does no group operations, so arms are compared on paired wall time only. A
  "below rho" result at toy q is the expected crossover of a known attack (the genus-5 cover
  gives about q^1.6 against rho's q^2), measured against this repository's own rho. It is not a
  break of any curve.
- `dlpx` pays for the reducer it does not use (as `dlpf` does) and for 10 % more full relations.
- The w1/l1 repair path (one more solve when e2e fails) is not implemented. Its expected cost is
  bounded from the observed failure rate (rule of three when 0 failures).
- The excess fraction (0.10) and the tree score (total non-zeros) are fixed, not tuned on this grid.

## Addendum (2026-11-16, during data; predictions and analysis unchanged)

The q = 24001 launch hit the session's 2 h background limit after 34 of 40 runs (seed 5, protocol
positions 0–1 done). The remaining six runs (seed 5, positions 2–7) were run with
`run_v5_part.sh`, using the same binary (hash checked), the same per-run command and the same
order, and are marked `"resumed": true`. Their wall times come from a later process. q = 32003
was expected to be about 3× slower per run, so it runs in several launches of `run_v5_part.sh`, in
seed order. Seed 1 ran alone and took 868 s, so seeds 2–3 and then 4–5 run as one launch each. No analysis was run on any data before this addendum.
