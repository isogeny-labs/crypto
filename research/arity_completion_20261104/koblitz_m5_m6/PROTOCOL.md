# Chained-S3 union-base decomposition at m = 5, 6 on Koblitz curves, n = 23..41 and the n = 83 lift (preregistration, draft)

Draft 2026-11-04, `experiment.protocol/v1` id `KOBLITZ-M5-M6-20261104`, version 1.
Machine-readable twin: [`protocol.json`](protocol.json) (same content; the JSON
is the record the freeze hashes). Frozen grid: [`grid.tsv`](grid.tsv); shard
runner [`run_shard.sh`](run_shard.sh); analysis [`analyze.py`](analyze.py).
Freeze block is `draft` with null fields; nothing here has been run.

## Question

Can chained-S3 SAT decomposition with a Frobenius-invariant (union) factor base
decide m = 5 and m = 6 relations on Koblitz curves at n = 23, 29, 31, 37, 41
within a 600 s cell cap scaled to the m = 4 root-table cost, and at n = 83 with
the 180 s cap of the n83 sweep lifted to 1800 s; and what does a decided query
cost?

Context. The arity review
([`research/factor_base_arity_review_20261024/README.md`](../../factor_base_arity_review_20261024/README.md))
found no measured m >= 5 cell with a Frobenius-invariant base.
[`n83_msweep_20261024`](../../n83_msweep_20261024/README.md) capped every
m >= 3 chained cell at 180 s (`grid.tsv` column 8, `cell_timeout_s`, applied by
`run_shard.sh` with `timeout --kill-after=5`); its m = 4 root-table complete
DLPs took 175.2 s and 188.5 s per target (K1107) against strong rho 1.509 s,
and chain-s3 decided m = 2 at 17.3 s per query.

## Instrument

`examples/n83_msweep_cell.rs`, built with
`cargo build --release --example n83_msweep_cell`. Entry
`sat_decompose_chained_s3_any_m` (m = 2..9) with the compiled-in
`ChainS3Limits`: `max_variables` 4,000,000, `max_domain_clauses` 50,000,000,
`max_models` 64, `conflict_budget` 200,000. Command shape, one fresh process
per cell:

```sh
RAYON_NUM_THREADS=1 timeout --kill-after=5 CAP ./target/release/examples/n83_msweep_cell \
  cell M FB ENGINE 4 8609517 10000 public|planted CURVE
```

`FB = union:S` is the Frobenius-union base on `span(1, z, ..., z^(S-1))` in the
ambient basis; `ENGINE = chain-s3-orbit`; `CURVE` is `k<a>n<degree>`
(`KoblitzCurve::new`) or `n83k1`. Chained SAT has `(m-1)(3n^2+4n)+n(2m-1)`
variables: 6,923 at (m = 5, n = 23), 26,486 at (6, 41), 105,908 at (6, 83).

Curves and `a`: `k0n23`, `k1n29`, `k0n31`, `k0n37`, `k0n41` (aliases in
`src/cryptanalysis/curve_aliases.json`; there is no `k0n29`, so n = 29 is
a = 1). The n = 83 lift uses the pinned `n83k1` (`KoblitzCurve::known_n83_k1`,
a = 1, 53-bit subgroup), the curve argument the n83 sweep used; a generic `k1n83` spec would take the
`KoblitzCurve::new` path instead of the pinned C082 fixture, so it is not used.

## Cells (grid.tsv, 80 cells, 4 trials, seed 8609517, per_query_ms 10000)

| part | family | curves | m | S | probes | engines | cap | cells |
|---|---|---|---|---|---|---|---|---|
| A | koblitzA | 5 Part A curves | 5, 6 | 2, 3 | public | chain-s3-orbit | 600 s | 20 |
| A | koblitzA | same | 5, 6 | 2, 3 | planted (positive control) | chain-s3-orbit | 600 s | 20 |
| A | aa | same | 5, 6 | 2, 3 | public, duplicated cell (A/A) | chain-s3-orbit | 600 s | 20 |
| A | negative | same | 2 | 2 | public | chain-s3-orbit, chain-s3 | 600 s | 10 |
| B | liftB | n83k1 | 5, 6 | 2, 3 | public | chain-s3-orbit | 1800 s | 4 |
| B | liftB | n83k1 | 5, 6 | 2, 3 | planted | chain-s3-orbit | 1800 s | 4 |
| B | negative | n83k1 | 2 | 2 | public | chain-s3-orbit, chain-s3 | 1800 s | 2 |

Caps. 188 s is the **parity threshold**: a single query above the largest
m = 4 root-table DLP (188.5 s) already exceeds one whole complete m = 4 solve.
600 s (Part A, 3.2 x parity) and 1800 s (Part B, 10 x the old 180 s, 9.6 x
parity) are **observation caps**. The Part A cap is fixed at 600 s now; the
preflight below cannot change it.

Cap arithmetic that the claims inherit. The cap applies to the cell process,
which runs all four queries, and the record is written only at the end. A
decided cell therefore has mean seconds per query at most cap/4 minus
factor-base time: 150 s in Part A, 450 s in Part B. A capped cell shows only
that the four queries together exceed the cap, not a per-query cost.

## Preflight (development smoke, not evidence)

One run of the (m = 5, n = 23 = `k0n23`, S = 2) public cell with the frozen
command, output to a scratch directory outside `raw/`, discarded. It is
labelled not evidence and is used only to confirm that the command runs and to
record `sample_size.preflight_runtime` (fixed seconds, per-sample seconds,
estimated total, peak RSS for the memory request). Its records, seconds and
outcome may not support any claim.

## Predictions (claims with falsifiers)

- **K1.** At n = 23, the m = 5 and m = 6 public union:2 cells decide (all four
  queries SAT or proved UNSAT) inside 600 s. Falsified if either caps or is
  undecided.
- **K2.** Decided-query seconds grow at least geometrically in n at fixed
  (m, S), such that every m = 6 public cell at n = 37 and n = 41 (S = 2, 3)
  hits the 600 s cap. Falsified by a decided m = 6 cell at n = 37 or 41, or a
  non-positive log2-slope between consecutive decided n. A cell skipped by the
  stopping rule is unmeasured: K2 is then `supported_partial`.
- **K3.** Every decided planted cell recovers the planted relation (4/4
  witnesses, each re-added in the group). Positive control; a miss invalidates
  that cell.
- **K4.** At n = 83 every m = 5 and m = 6 public cell (S = 2, 3) stays
  undecided at 1800 s. Falsified by any decided cell.
- **K5.** No decided public query at n >= 31 costs less than 188 s. Falsified
  by any decided public cell at n >= 31 under 188 s. Holds vacuously when
  nothing decides; the report says so. Note that in Part A any decided cell has
  mean seconds per query <= 150 s, so a decided Part A cell at n >= 31
  falsifies K5 by construction of the cap; K5 can only be supported there
  because no such cell decides, and at n = 83 because decided cells, if any,
  exceed 188 s (possible only in Part B, up to 450 s per query).
- **K6, kill criterion for the high-arity Koblitz line.** K4 and K5 both
  supported, K3 not falsified: verdict `kill_high_arity_koblitz_line`;
  otherwise `line_stays_open`.
- **K7 (out of scope).** No complete-DLP, IC-versus-rho, leaderboard or
  `benchmarks/README.md` claim. These are relation-collection stage
  diagnostics; no matched rho is run, so no gap to rho is computed. Where a
  cell decides, analysis reports its distance to the 188 s m = 4 parity point.

## Controls

- **Positive.** Planted mode on every cell (a sum of m random base points).
  A decided planted cell must return 4 verified witnesses; a proved_unsat or an
  unverified witness invalidates the cell and its public twin.
- **A/A.** The binary draws its stream from the fixed seed and cannot repeat a
  draw inside a cell, so the A/A is a **duplicated cell with identical
  arguments**: the 20 public Part A cells run a second time as family `aa`.
  Outcomes, relations, verified relations, `base_points` and `columns` must be
  identical; seconds ratio is the noise floor. One capped and one complete is a
  noise flag. Part B is not duplicated (4 x 1800 s more for cells expected to cap).
- **Negative (known-decidable baseline).** m = 2, union:2, public at each n
  (n83 record: chain-s3 4/4 proved UNSAT at 17.3 s per query at n = 83).
  Run with `chain-s3-orbit` (the arm engine) and `chain-s3` (the record's
  engine). Caution: in the n83 table `chain-s3-orbit` union:2 m = 2 is a
  `process_cap` at 180 s, only `chain-s3` decided it. The orbit negative at
  n = 83 is therefore not known-decidable; it is measured here at 1800 s, and
  if it stays undecided K4 is read as supported but weak.

## Variable ledger

Independent: m {5, 6}; n (curve); S {2, 3}; mode {public, planted}; part/cap
{600, 1800}. Fixed: engine, trials 4, seed 8609517 (stream
`seed XOR (m << 40)`), per_query_ms 10000 (inert for the chain engines, which
are bounded by `conflict_budget` and the process cap), `ChainS3Limits`, one
binary (sha256 at freeze), host class, `RAYON_NUM_THREADS=1`. Curve parameter
a is confounded with n at n = 29 and n = 83 (a = 1). Comparisons:
`C-m5-vs-m6` at each n (one factor, descriptive), `C-n-scaling` at each m
(one factor), `C-lift-n83` against the 180 s record (confounded by host and
commit, descriptive; orbit union:3 has no 180 s record, so S = 3 is read
against chain-s3 union:3), `C-cells-A`, `C-cells-B`, `C-positive`, `C-aa`,
`C-negative`.

## Metrics

Primary: `decided` (exact, per cell) with classes decided_sat, decided_unsat,
decided_mixed, undecided, capped, skipped_stopping_rule, process_error; and
`seconds_per_decided_query` (timed, `collection_seconds / trials`, defined only
for decided cells; a cap is never a cost). Secondary: conflicts (`engine_work`),
variables (formula above, derived), peak RSS (`/usr/bin/time` sidecar), process
wall seconds. **Models** are not emitted by the cell driver and are declared
unavailable.

## Isolation and execution

Counts need no isolation. Seconds are diagnostic at L0 and admissible as timing
only on isolab-eks with native L2, replay and A/A admission (AGENTS.md
section 9a); `capped` is itself a wall-clock event and shares that tier.
Backend isolab-eks (Spot, cluster `isolab`, namespace `isolab`, `us-east-2`),
three single-vCPU shards (m = 2, 5, 6) run concurrently, one `frozen_command`
card per shard from `tools/task_card.py`, `raw/shard<m>/` and `receipts/` as in
n83_msweep:

```sh
research/arity_completion_20261104/koblitz_m5_m6/run_shard.sh ./target/release/examples/n83_msweep_cell research/arity_completion_20261104/koblitz_m5_m6/raw/shard5 5   # also shard6 (6) and shard2 (2)
```

Worst case 60,000 CPU-seconds (Part A 42,000; Part B 18,000); shard walls at
most 9,600 s (m = 2), 25,200 s (m = 5), 25,200 s (m = 6); card timeout 28,800 s.
The dollar ceiling and deadline are set in the launch request before spend and
are null in this draft. Restart policy: no checkpoint; a cell interrupted by
Spot is discarded and re-run whole, completed records are never re-run.

## Stopping rules

1. Fixed grid, 80 cells.
2. Per-cell cap (`timeout --kill-after=5`); a capped cell is final.
3. Part A, per m: curves run in the order 23, 29, 31, 37, 41; once all four
   public cells (S = 2, 3) at the two previous curves are capped or skipped, the
   Part A cells of that m at the next curve are written as
   `skipped_stopping_rule` and not run (`run_shard.sh`, function `stopped`).
   Part B and the negatives are never skipped.
4. Invalidation stop: failed positive control, process_error, binary sha256
   mismatch, or an undecided negative control halts the shard for assessment.

## Decision rule

Exact per-cell outcomes, vocabulary `decided_sat`, `decided_unsat`,
`decided_mixed`, `undecided`, `capped`, `skipped_stopping_rule`, plus claim
verdicts `supported`, `supported_partial`, `falsified`, `not_evaluable`. No
sampling interval. Effect size: seconds per decided query over 188 s; m = 6 over
m = 5 ratio; log2-slope in n.

## Analysis plan

`analyze.py [RAW_DIR] [--json ANALYSIS.json] [--tsv table.tsv]` reads the cell
records and sidecars, prints decided / undecided / capped per cell with seconds,
checks the controls, evaluates K1 to K6, and exits 0. Blinded: false.
Frozen before data: true (its sha256 is entered at freeze).

## Inadmissible

Raising or lowering a cap after the freeze, or re-running a capped cell longer;
changing the seed, trials, engine, span or limits; reporting a capped cell as a
cost; using the preflight as evidence; reading seconds from a run without L2
admission as timing; reading a cell whose planted or negative control failed as
informative about arity; presenting these cells as a complete DLP, a leaderboard
row or a speed-up over rho.

## Amendments

None.
