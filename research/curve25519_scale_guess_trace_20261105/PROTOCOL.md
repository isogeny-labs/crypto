# Scale-up S1: guessing (lead 4), guessing optimum (lead B) and F4 trace replay (lead C) from k = 20 to k = 24 and 28

Frozen 2026-11-05 before any data in this directory. Branch `scale-S1-guess-trace`, base 106141b2b9.
Scope of every statement that will come out of this protocol: **isogeny form, m = 2, mode full, lifts 0,
k in {20 (reference), 24, 28}, t in {6, 8, 10}, G in {4, t-1}, 100 random subgroup targets per cell, the
seeds below**. Nothing is claimed for other k, t, G, m or form, for wall time, or for a distance to Pollard
rho (lead A's unit, not this study's).

## Question

Leads 4 and B measured the hybrid-guessing oracle (`pkm-semaev`, `guess=G`) at k = 20, with a few k = 24/28
cells in lead B; lead C measured the F4 trace replay (`trace=1`) at k = 20 only. How do these three results
change when the same cells are measured at k = 24 and k = 28 on one frozen binary: F4 work per target, the
trace ratio, the per-relation cost at `coverage_exact`, and the uncharged native work as a share of F4 work?

## Instrument (frozen, no code change, no build)

Binary `/home/user/frozen-bin/curve25519_family_scale`, sha256
`d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90`. The driver checks the hash before every
run of the driver (not only once) and refuses to start on a mismatch. CLI:

    ( ulimit -v 3500000; timeout 1000 /home/user/frozen-bin/curve25519_family_scale \
        f4cost <k> isogeny <t> <G> 100 <seed> 900 2 full 0 - <trace> )

i.e. 100 targets, cap 900 s, m = 2, mode full, lifts 0, max_nnz `-`, trace 0 or 1 (argument 13, checked in
`src/bin/curve25519_family.rs`: `args.get(13)`). k = 32 is refused by the tower engine (p < 2^31) and is out
of scope. The code caps G at 12 and refuses G >= t; the grid stays below both (G <= 9).
If a code change looks needed, the study stops and reports; nothing is built.

## Cells

Main grid, 24 cells: k in {24, 28}; t in {6, 8, 10}; G in {4, t-1} (t = 6: 4, 5; t = 8: 4, 7; t = 10: 4, 9);
trace in {0, 1}. Seed `2026112400 + t` (k = 24), `2026112800 + t` (k = 28). Within a (k, t) every G and both
trace arms share the same 100 targets, so the trace pair is paired target for target.
Cap 900 s per cell. A cell that hits the cap reports only the targets it finished and is flagged as a
truncated sample; a cell that fails (nonzero rc, timeout 1000, OOM) is a reported failure with no data and is
not rerun with different parameters.

Reproduction at k = 20, 6 cells, run first:

- R-B (lead B's seeds `2026111600 + t`): t = 6, G = 5 and t = 10, G = 9, trace 0 and 1 (4 cells). The trace 0
  cells must equal lead B's `analysis/cells.jsonl` (stage s1) exactly in `f4_muladds`, `f4_calls`, `hits`,
  `hits_verified`, `solving_degree_max`, `coverage_exact`. The trace 1 cells have no earlier counts at G = t-1;
  they are checked for equal hits/verified hits against their trace 0 partner.
- R-C (lead C's seeds `2026111400 + t`): t = 6, G = 4 and t = 8, G = 4, trace 0 and 1. Added by this study
  (not in the request) because lead C measured trace 1 only at G in {0, 2, 4}, so G = 4 is the only
  point at which trace 1 can be checked against earlier counts. They must equal lead C's
  `analysis/f4trace.jsonl` exactly (`f4_muladds`, `f4_calls`, `trace_learn_muladds`, hits).
  That is 4 cells, so the reproduction set is 8 cells, the whole study 32 cells.

Any mismatch is reported as a failed reproduction and the dependent comparisons are flagged; it is not
repaired.

Relation to lead B's own k = 24/28 cells (stated so cells are not silently duplicated): lead B ran t = 10,
G = 7..9 at k = 24 and 28 with seeds `2026111700 + t` / `2026111800 + t`, trace absent, and t = 12 at k = 24.
This study's t = 10, G = 9 at k = 24 and 28, trace 0, repeat lead B's (t, G) on **new seeds** (new target
sets) because the trace 0 arm is the denominator of the trace ratio and needs the same targets as the trace 1
arm, which lead B's cells do not have; they double as a second sample for the k-independence check
(P1 below). t = 6 and 8, G = 4 and t = 10, G = 4 at k = 24/28, and all trace 1 cells, have no earlier counterpart.
Lead B's k = 24/28 numbers are used only as a secondary comparison and are labelled so.

## Execution rules (shared machine)

Cells run strictly one at a time by `analysis/run_cells.sh`, which reads `analysis/cells.list`, skips any
cell whose `analysis/cells/<id>.json` (or `.fail`) already exists, and so resumes after a container restart.
Each cell: `ulimit -v 3500000`, `timeout 1000`, `uptime` recorded before the cell (field `load_at_start`),
stderr to `analysis/stderr/`, the binary hash recorded in every cell json. Wall times are recorded and
**not compared** (three other agents share the 4 cores). Reference k = 20 values for the cells not rerun
(trace 0 at t = 8, and G = 4 at t = 6, 10, etc.) are lead B's stage 1 cells (seed `2026111600 + t`,
`curve25519_guess_optimum_20261102/analysis/cells.jsonl`), labelled "lead B" in the tables.

## Metrics (per cell, from the cell json)

- F4 multiply-adds per target, raw: `muladds_per_target` (= `f4_muladds` / targets done). For trace 1 this is
  replay work plus fallback work, learning excluded.
- Amortised: `amortised_muladds_per_target` = (`f4_muladds` + `trace_learn_muladds`) / targets done; for
  trace 0 it equals the raw figure.
- `f4_calls` per target, `solving_degree_max`, `hits` / `hits_verified`, `undecided`, `coverage_exact`, `capped`.
- Per relation at `coverage_exact`: PR(cov) = F4 multiply-adds per target / `coverage_exact` (raw for trace 0,
  amortised for trace 1; also raw for trace 1 as a bound). Modelled (cost over exact coverage), not observed.
  Hits (0 to a few in 100 targets) cannot rank cells; `muladds_per_hit` is given only where hits >= 1.
- Uncharged native work: `native.pkm_system_fp_mults_uncharged` + `native.pkm_grid_fp_mults_uncharged`
  divided by `f4_muladds` (lead B's definition). For trace 1 the learning-time uncharged work
  (`setup_native.pkm_trace_learn_spec_fp_mults_uncharged`) and the learning multiply-adds are added to
  numerator and denominator as a second column. Units differ (field multiplications against F4
  multiply-adds); this is a share, not a price. Pricing at 0 / 0.1 / 1 per operation (AGENTS.md section 5a)
  is reported as the share multiplied by 0, 0.1, 1 added to the charged unit; no cross-method comparison is made.
- Trace ratios: trace 1 raw / trace 0 raw per target (learning excluded), and amortised / trace 0.
  Fallbacks / replays from `native` (`pkm_trace_replays`, `pkm_trace_refuted`).

## Scaling report (how, and the limit on what it can show)

Per metric and per (t, G, arm) with all three k points available (k = 20 from the reproduction or lead B
cells, 24 and 28 from this study):

1. The ratio between successive k: y(24)/y(20) and y(28)/y(24) (4 bits per step).
2. A least-squares fitted exponent alpha in y proportional to r^alpha, with r the subgroup order taken as
   2^k nominally (the tool does not print r; an error of a few percent in r changes alpha negligibly over
   4-bit steps), i.e. the slope of log2 y on k, by awk. It is **reported only when 3 points exist and the two
   successive slopes differ by at most 0.5**; otherwise the table says "slopes differ, no single exponent". With three points
   the fit has one degree of freedom, so it is a description of three cells, not an
   estimate with an interval; no exponent is extrapolated to other k, and none is called a law. Where only two points exist
   (e.g. no k = 20 trace 1 cell) only the single ratio is given.
3. No confidence intervals: the tool emits aggregates only, one seed per cell, and k = 24 and 28 differ in
   curve and target set. The differences across k therefore mix the k effect with the curve and target-set
   spread (lead B saw a 2 to 3 percent spread between target sets at fixed k).

## Predictions (frozen before data) with falsifiers

Numbers come from lead B / lead C at k = 20 and lead B's k = 24/28 t = 10 cells.

- **P1 (F4 work per target nearly independent of k).** Trace 0, at each of the 6 (t, G): the ratio
  mul/tgt(k = 28) / mul/tgt(k = 24) lies in [0.90, 1.10] (lead B: 1.01, 1.01, 1.01 at t = 10, G = 7..9); and
  mul/tgt(k = 24) / mul/tgt(lead B, k = 20) lies in [0.85, 1.25] for t = 6, 8 and in [1.00, 1.40] for t = 10
  (lead B: 1.23 at t = 10 where k = 20 stops early on hits). **Falsified** if any of the 6 k = 28/24 ratios is
  outside [0.90, 1.10] (not if only the k = 24/20 band fails: that is reported as a partial miss).
- **P2 (per-relation cost rises with k by the coverage factor).** `coverage_exact` falls strictly with k at
  each t, by a factor in [4, 25] per 4 bits (lead B, t = 10: 11.8 and 16.8; the lambda of `support` at t = 6
  falls 6.6 and 19.8), and PR(cov) rises per step by (that factor) times (the mul/tgt ratio of P1), by
  construction. **Falsified** if any step factor is outside [4, 25] or coverage does not fall. Stated plainly: given P1
  the PR(cov) statement is arithmetic; the empirical content is the coverage factor and P1.
- **P3 (trace ratio stays in the lead C band).** For every one of the 12 (k, t, G) pairs, trace 1 raw / trace 0
  raw per target lies in [0.29, 0.45], and amortised / trace 0 is at most the raw ratio + 0.02 (learning is
  one plain solve per branch system, about the plain per-target cost, spread over 100 targets, so +0.01).
  **Falsified** by any pair outside [0.29, 0.45] (the pair is named). Hits and verified hits equal between arms in every pair, and
  fallbacks / replays at most 10 percent in every trace 1 cell (lead C P3, P4).
- **P4 (trace ratio rises with G; exploratory).** At fixed (k, t) the raw ratio at G = t-1 is not below the
  ratio at G = 4 (lead C: 0.41 to 0.45 rising with G over 0..4). Falsified for a (k, t) where it is lower by
  more than 0.02. No band; labelled exploratory.
- **P5 (uncharged native work stays small and grows with G).** Trace 0: share at G = t-1 is at most 1 percent
  at every k = 24, 28 cell with t <= 10 (lead B: 0.31 percent at t = 10, G = 9, k = 24) and larger than the share at G = 4
  in each (k, t). For trace 1 the share of replay-charged work is larger than for trace 0 in the same pair
  (same specialisation work over less charged work). Falsified by any share above 1 percent at trace 0, any
  (k, t) where the G = t-1 share is not larger than the G = 4 share, or any pair where trace 1's share is lower.
- **P6 (solving degree).** Trace 0: `solving_degree_max` is 5 at G = 4 and 4 at G = t-1 in every cell (lead B).
  Falsified by any other value (reported per cell; trace 1 degree reported, no prediction).
- **P7 (reproduction).** All 8 reproduction cells match the earlier counts exactly (above). Falsified by any mismatch.
- **Failure expectation (not a prediction of a value).** The largest cells (t = 10, G = 4) took 435 s at k = 20
  (lead B); k = 24/28 cells with no early stops may take longer. A cap at 900 s is possible and, if it
  happens, is reported as a truncated sample and excluded from the P1/P3/P5 bands (listed as "not judgeable").

## Reporting rules

Every cell of the 32 appears in the README table, including failures and caps. Prediction outcomes are
stated per prediction as true / false / partial / not judgeable with the numbers. Claims are scoped to the
measured cells. Raw per-cell json over 64 KiB (none expected) goes to the artifact store. No replay certificate,
validator report or matched rho reference is attached, so nothing here is a leaderboard or `benchmarks/` row.
Run locally on the shared agent box, not on EKS: a development scale-up inside the leads, no L2 or
cross-method claim.
