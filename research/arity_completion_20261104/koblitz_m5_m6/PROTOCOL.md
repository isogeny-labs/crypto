# Chained-S3 union-base decomposition at m = 5, 6 on Koblitz curves, n = 23..41 and the n = 83 lift (preregistration, draft)

Draft 2026-11-04, `experiment.protocol/v1` id `KOBLITZ-M5-M6-20261104`, version 1. Generated from [`protocol.json`](protocol.json), which is the record the freeze hashes; the two carry the same content. Grid: [`grid.tsv`](grid.tsv); shard runner [`run_shard.sh`](run_shard.sh); analysis [`analyze.py`](analyze.py). Freeze block is `draft` with null fields; nothing here has been run.

## Question

Can chained-S3 SAT decomposition with a Frobenius-invariant (union) factor base decide m = 5 and m = 6 relations on Koblitz curves at n = 23, 29, 31, 37 and 41 within a 600 s cell cap scaled to the m = 4 root-table cost, and at n = 83 with the 180 s cap of n83_msweep_20261024 lifted tenfold to 1800 s; and what does a decided query cost?

Context. The arity review ([`research/factor_base_arity_review_20261024/README.md`](../../factor_base_arity_review_20261024/README.md)) found no measured m >= 5 cell with a Frobenius-invariant base. [`n83_msweep_20261024`](../../n83_msweep_20261024/README.md) capped every m >= 3 chained cell at 180 s (`grid.tsv` column 8, `cell_timeout_s`, applied by `run_shard.sh` with `timeout --kill-after=5`). Its m = 4 root-table complete DLPs took 175.2 s and 188.5 s per target (K1107) against strong rho 1.509 s. Only the plain `chain-s3` engine decided m = 2 there (17.3 s per query); `chain-s3-orbit` capped at 180 s. Both engines are therefore run here.

## Instrument

`examples/n83_msweep_cell.rs`, built with `cargo build --release --example n83_msweep_cell`. Entry `sat_decompose_chained_s3_any_m` (m = 2..9) with the compiled-in `ChainS3Limits`: `max_variables` 4,000,000, `max_domain_clauses` 50,000,000, `max_models` 64, `conflict_budget` 200,000. One fresh process per cell:

```sh
RAYON_NUM_THREADS=1 timeout --kill-after=5 CAP ./target/release/examples/n83_msweep_cell \
  cell M FB ENGINE TRIALS 8609517 10000 public|planted CURVE
```

`FB = union:S` (Frobenius-union base on `span(1, z, ..., z^(S-1))`), `ENGINE` in {`chain-s3-orbit`, `chain-s3`}, `TRIALS` = 4 in Part A and 1 in Part B, `CURVE` = `k<a>n<degree>` or `n83k1`. Chained SAT has `(m-1)(3n^2+4n)+n(2m-1)` variables: 6,923 at (5, 23), 26,486 at (6, 41), 105,908 at (6, 83). Curves: `k0n23`, `k1n29`, `k0n31`, `k0n37`, `k0n41` (aliases in `src/cryptanalysis/curve_aliases.json`; no `k0n29`, so n = 29 is a = 1); Part B uses the pinned `n83k1` (`KoblitzCurve::known_n83_k1`, a = 1), the argument the n83 sweep used. A generic `k1n83` would take the `KoblitzCurve::new` path instead of that fixture.

## Cells (grid.tsv, 148 cells, seed 8609517, per_query_ms 10000 which the chain engines ignore)

| part | family | curves | m | S | probes | engines | trials | cap | cells |
|---|---|---|---|---|---|---|---|---|---|
| A | koblitzA | 5 Part A curves | 5, 6 | 2, 3 | public | both | 4 | 600 s | 40 |
| A | koblitzA | same | 5, 6 | 2, 3 | planted | both | 4 | 600 s | 40 |
| A | aa | same | 5, 6 | 2, 3 | public (A/A duplicate) | both | 4 | 600 s | 40 |
| A | negative | same | 2 | 2 | public | both | 4 | 600 s | 10 |
| B | liftB | n83k1 | 5, 6 | 2, 3 | public | both | 1 | 1800 s | 8 |
| B | liftB | n83k1 | 5, 6 | 2, 3 | planted | both | 1 | 1800 s | 8 |
| B | negative | n83k1 | 2 | 2 | public | both | 1 | 1800 s | 2 |

Caps. 188 s is the parity threshold: one query above the largest m = 4 root-table DLP (188.5 s) already exceeds a whole complete m = 4 solve. 600 s (Part A) and 1800 s (Part B, 10 x the old 180 s) are observation caps, fixed now; the preflight cannot change them.

**Trials and caps.** The cap applies to the cell process and the record is written at its end. Part A keeps TRIALS = 4 under a 600 s process cap, so a decided Part A cell proves at most 150 s per query (factor-base time excluded) and a capped one shows only that four queries together exceed 600 s. Part B runs TRIALS = 1, so its 1800 s cap is a per-query cap and a decided cell can show up to 1800 s per query. The n83 negative uses one trial for the same reason.

## Preflight (development smoke, not evidence)

To be filled by the preflight: one development smoke of the (m = 5, n = 23 = k0n23, S = 2) public cell, engine chain-s3-orbit with the frozen command, output discarded, labelled not evidence, used only to confirm the command runs and to record preflight_runtime. Worst-case total if every cell hit its cap: 110400 CPU-seconds (Part A 78000, Part B 32400); the stopping rule can only lower it.

## Claims and falsifiers

- **K1** (exact_computation). For each engine (chain-s3-orbit and chain-s3), at n = 23 (k0n23) the m = 5 and m = 6 public union:2 cells decide (all four queries SAT or proved UNSAT) inside the 600 s process cap.
  - If supported: The chained-S3 union-base line produces its first measured m >= 5 decided cells; their seconds per query are the first price of the line, to be compared with the m = 4 root-table figure.
  - If not supported: Falsified for an engine if either of its two k0n23 cells caps or is undecided: at the smallest registered curve that engine does not decide m >= 5 inside 600 s single-threaded, and K2/K4 are read accordingly. The two engines are reported separately; the claim holds only if it holds for both.
  - Decided by: C-cells-A, C-positive, C-engine
- **K2** (comparative). For each engine, decided-query seconds grow at least geometrically with n at fixed (m, S), such that every m = 6 public cell at n = 37 and n = 41 (S = 2, 3) hits the 600 s cap.
  - If supported: Cost per query rises past the Part A cap by n = 37; the curve gives the n at which each (m, S) leaves reach.
  - If not supported: Falsified by any decided m = 6 public cell at n = 37 or 41, or by a non-positive log-slope between consecutive decided n at fixed (m, S). A cell skipped by the stopping rule is not measured, and K2 is then supported only on the measured cells (verdict supported_partial).
  - Decided by: C-n-scaling
- **K3** (exact_computation). In every decided planted cell, for both engines, the planted relation is recovered: every query returns a witness that is re-added in the curve group and equals the target (4 of 4 in Part A, 1 of 1 in Part B).
  - If supported: The instrument finds a relation that exists at each decided planted cell; public verdicts there are informative.
  - If not supported: Any decided planted cell with a proved_unsat verdict or an unverified witness invalidates that cell's planted and public records and that engine at that (m, n, S); K4 and K5 are then not evaluable for it.
  - Decided by: C-positive
- **K4** (exact_computation). At n = 83 (n83k1, a = 1 pinned fixture), for each engine, every m = 5 and m = 6 public cell (S = 2, 3) stays undecided at the 1800 s cap, which with one query per cell process is a per-query cap; the tenfold lift does not change the n83_msweep verdict.
  - If supported: The n = 83 m >= 5 chained line is capped at 1800 s as at 180 s; with K5 this is the kill criterion.
  - If not supported: Falsified by any decided n = 83 public cell at m = 5 or 6: the 180 s verdict was a cap artefact and the line stays open at n = 83.
  - Decided by: C-cells-B, C-lift-n83, C-engine
- **K5** (comparative). For each engine, no decided public cell at n >= 31 (Part A) or at n = 83 (Part B) has seconds per decided query below 188 s, the m = 4 root-table parity threshold (n83 K1107 complete DLPs took 175-188 s), so the m >= 5 chained line stays behind m = 4. Stated plainly: Part A keeps 4 trials under a 600 s process cap, so a decided Part A cell proves at most 150 s per query; a decided Part A cell at n >= 31 therefore falsifies K5 by construction, and otherwise K5 stays untested on capped cells (a cap is a censored observation, not a cost above 188 s). Part B runs one query per process under 1800 s, so a Part B cell is the only place a per-query cost above 188 s can be observed.
  - If supported: No decided m >= 5 chained cell undercuts one m = 4 root-table DLP. When no cell at n >= 31 decides, K5 holds vacuously and is reported as such: capped Part A cells show only that four queries together exceed 600 s, which does not test 188 s.
  - If not supported: Falsified by any decided public cell at n >= 31 with seconds per query below 188 s; every decided Part A cell at n >= 31 does so (at most 150 s per query), so Part A decisions at n >= 31 falsify K5 by construction. The line then has a measured improvement over the m = 4 parity point and is reported as the distance remaining, not as a pass.
  - Decided by: C-cells-A, C-cells-B, C-engine
- **K6** (exact_computation). Kill criterion for the high-arity Koblitz line: K4 and K5 both supported for both engines, with K3 not falsified.
  - If supported: The high-arity Koblitz chained-S3 line is closed at these parameters (verdict kill_high_arity_koblitz_line).
  - If not supported: The line stays open (line_stays_open); a falsified K3 gives not_evaluable_instrument_invalid.
  - Decided by: C-cells-B, C-cells-A, C-positive, C-engine
- **K7** (out_of_scope). No complete-DLP, end-to-end IC-versus-rho, leaderboard or benchmarks/README.md claim is in scope: the cells are relation-collection stage diagnostics and no matched rho is run.

## Arms

- `A-public` (candidate): Part A measurement: public probes R = [a]G (4 per cell), engines chain-s3-orbit and chain-s3, m in {5, 6} x union:S for S in {2, 3} x curves k0n23, k1n29, k0n31, k0n37, k0n41 (a = 0 except n = 29, a = 1), 40 cells, 600 s process cap each (4 trials per process, so a decided cell is at most 150 s per query), family koblitzA.
- `A-planted` (control_positive): Positive control for Part A: the same 40 cells (both engines) with planted targets (sum of m random base points), 4 per cell; the planted relation must be recovered.
- `A-aa` (control_aa): A/A for Part A: the 40 public cells (both engines) run a second time with identical arguments (same binary, seed, host class) as separate records of family aa; outcomes must agree exactly, seconds are compared as a noise floor.
- `A-neg-m2` (control_negative): Known-decidable baseline for Part A: m = 2, union:2, public, engines chain-s3-orbit and chain-s3, at each of the five Part A curves (10 cells, 600 s cap).
- `B-public` (candidate): Part B lift: the n83_msweep cells re-run with the cap lifted from 180 s to 1800 s and ONE trial per process (so the cap is per query): n83k1 (KoblitzCurve::known_n83_k1, a = 1), engines chain-s3-orbit and chain-s3, m in {5, 6} x union:S, S in {2, 3}, public, 8 cells, family liftB.
- `B-planted` (control_positive): Positive control for Part B: the same eight cells (both engines) with a planted target, one trial, 1800 s cap.
- `B-neg-m2` (control_negative): Known-decidable baseline at n = 83: m = 2, union:2, public, engines chain-s3-orbit and chain-s3, one trial per process, 1800 s cap (2 cells). The n83 record decided chain-s3 at 17.3 s per query (mean of 4); chain-s3-orbit hit the 180 s cap there.
- `R-n83-180s` (reference): The recorded n83_msweep_20261024 cells (180 s cap, chain-s3 and chain-s3-orbit, union bases, m = 2..9, shards run at commit b654e95b6): not re-run, read from research/n83_msweep_20261024/table.tsv and raw.tar.xz. Used for C-lift-n83 and for the 188 s and 17.3 s figures.

## Controls

- **Positive** (`A-planted`, plus `B-planted`): Every planted cell that completes inside its cap returns four witnesses of four, each re-added in the curve group and equal to the target (verified_relations = trials = 4, no proved_unsat). A planted cell that completes with a proved_unsat or an unverified witness is a miss: it invalidates that (m, n, S) cell's records. Planted cells that cap are unmeasured, not misses.
- **A/A** (`C-aa`): the binary draws its stream from the fixed seed and cannot repeat a draw inside a cell, so the A/A is a duplicated cell with identical arguments: the 40 public Part A cells (both engines) run again as family `aa`. Outcomes, relations, verified relations, `base_points` and `columns` must be identical; the seconds ratio is the noise floor; one capped and one complete is a noise flag. Part B is not duplicated.
- **Negative** (`A-neg-m2`, `B-neg-m2`): The m = 2 union:2 cell on each curve, a query the engine is known to decide (n83 record: chain-s3 decided 4/4 proved UNSAT at 17.3 s per query at n = 83). Same binary, seed 8609517, public probes; m = 2, union:2, engines chain-s3-orbit and chain-s3; arms A-neg-m2 (n = 23..41, 600 s) and B-neg-m2 (n = 83, 1800 s).

## Variable ledger

Independent: m [5, 6]; engine ['chain-s3-orbit', 'chain-s3']; n (curve) ['k0n23', 'k1n29', 'k0n31', 'k0n37', 'k0n41', 'n83k1']; S (union base span(1, ..., z^(S-1))) [2, 3]; mode ['public', 'planted']; part (cell_timeout_s) ['A: 600 s per process, 4 trials', 'B: 1800 s per process, 1 trial'].

- Fixed/measured, **engine family and its domain**: chain-s3 is the explicit-domain encoding the n83 record decided at m = 2; chain-s3-orbit is the signed-Frobenius orbit-domain encoding that capped there. Both read the same limits.
- Fixed/measured, **trials**: TRIALS is the grid trials column. Part B uses 1 so the 1800 s process cap is a per-query cap; Part A keeps 4 under a 600 s process cap, so a decided Part A cell is at most 150 s per query (factor-base time excluded). The n83 planted rows used 2 trials and the public rows 4, so Part B is not like-for-like with the record on trial count.
- Fixed/measured, **seed and probe stream**: StdRng::seed_from_u64(seed ^ (m << 40)); the stream depends on m and the curve, not on S or engine. Same seed in every row, A/A included.
- Fixed/measured, **per_query_ms**: Passed for the record; the chain engines do not read it. A query is bounded by conflict_budget 200000 and the process cap only.
- Fixed/measured, **ChainS3Limits**: Compiled into the binary (examples/n83_msweep_cell.rs); the binary sha256 pins them.
- Fixed/measured, **binary and commit**: One release binary of examples/n83_msweep_cell built once at the frozen commit; sha256 recorded at freeze and in every shard receipt.
- Fixed/measured, **host class and threads**: isolab-eks native-L2 worker class; shards run concurrently on separate cores; host facts recorded per row.
- Fixed/measured, **curve parameter a**: Fixed by the repository's curve aliases (src/cryptanalysis/curve_aliases.json has k0n23, k1n29, k0n31, k0n37, k0n41; no k0n29) and by the n83 record (known_n83_k1). a is confounded with n at n = 29 and n = 83, so C-n-scaling reads n across a mix of a values.
- Fixed/measured, **cell_timeout_s**: Per cell process (timeout --kill-after=5). Part A: four trials share 600 s, so a decided cell has mean seconds per query at most 150 s. Part B: one trial per process, so 1800 s bounds one query.
- Fixed/measured, **instance parity between modes**: public probes are [a]G for a drawn from the stream; planted probes are sums of m stream-chosen base points; the two modes are not matched targets.
- Fixed/measured, **spot preemption and worker**: Worker, node and any interruption are recorded per row; an interrupted cell is discarded and re-run whole on an admitted worker.

Comparisons:

- `C-engine` (one_factor; arms A-public, B-public; differs in ['engine']): At each (m, n, S): outcome class and seconds per decided query of chain-s3 versus chain-s3-orbit, ratio where both decide. Claims K1 to K6 are read per engine; this comparison reports the pair.
- `C-cells-A` (absolute; arms A-public; differs in nothing): Exact outcome per Part A cell, per engine: decided_sat, decided_unsat, decided_mixed, undecided or capped; K1 reads n = 23, K5 reads seconds per query against 188 s at n >= 31 (a decided cell is at most 150 s per query by the cap).
- `C-cells-B` (absolute; arms B-public; differs in nothing): Exact outcome per Part B cell (one query, 1800 s cap), per engine; K4 needs all eight undecided or capped (four per engine), K5 reads seconds per query against 188 s.
- `C-m5-vs-m6` (one_factor; arms A-public; differs in ['m']): Descriptive: seconds per decided query and outcome at m = 5 versus m = 6 at each (n, S); reported as a ratio where both decide, otherwise as the pair of outcomes. No claim is decided by it.
- `C-n-scaling` (one_factor; arms A-public; differs in ['n (curve)']): At fixed (m, S): log2 seconds per decided query against n across decided cells, and the first n at which the cell caps. a is 1 at n = 29 and 0 at the other Part A curves.
- `C-lift-n83` (confounded; arms B-public, R-n83-180s; differs in ['cell_timeout_s', 'binary and commit', 'host class and threads']): The 180 s record was run on another host and commit; only the cap is intended to differ. Read as a descriptive comparison of outcome (capped at 180 s versus decided or capped at 1800 s). The orbit union:2 m = 5, 6 cells have a 180 s record; orbit union:3 has none (the record's union:3 cells are chain-s3), so S = 3 is compared with chain-s3 union:3. Part B uses one trial per process while the record's public cells used four.
- `C-positive` (absolute; arms A-planted, B-planted; differs in nothing): Every planted cell that completes returns only verified witnesses (K3), per engine.
- `C-aa` (aa; arms A-public, A-aa; differs in nothing): Identical arguments; outcome, relations, verified, base_points and columns must be identical; capped-versus-complete disagreement is a noise flag.
- `C-negative` (absolute; arms A-neg-m2, B-neg-m2; differs in nothing): Each m = 2 union:2 cell decides all four queries inside its cap; a cell that does not leaves the capped cells at that n uninformative about arity.

## Metrics

- Primary `decided` (exact, boolean per cell): True iff the cell process exited 0 with a complete record and decided_fraction = 1 (every query of the cell (four in Part A, one in Part B) returned witness or proved_unsat). Classes: decided_sat (all witnesses), decided_unsat (all proved_unsat), decided_mixed, undecided (complete record, some query incomplete, unresolved, unsupported or invalid_model), capped (process_cap: killed at the cell cap), skipped_stopping_rule, process_error.
- Primary `seconds_per_decided_query` (timed, seconds): collection_seconds / trials for a decided cell (mean over its queries: four in Part A, one in Part B, factor-base construction excluded); not defined for capped or undecided cells, whose cap time is never reported as a cost. median_seconds_per_trial is carried as the spread.
- Secondary `conflicts` (counted, conflicts): engine_work: the chained-S3 solver's conflict count summed over the cell's queries.
- Secondary `variables` (derived, variables): Variable count of the chained-S3 encoding at (m, n).
- Secondary `peak_rss_kb` (counted, KiB): GNU time maximum resident set of the cell process tree, from the <record>.time sidecar.
- Secondary `wall_seconds` (timed, seconds): Wall time of the whole cell process including factor-base construction; for capped cells the wall_seconds field of the process_cap record.
- Secondary `models` (counted, models): Not emitted by examples/n83_msweep_cell.rs (the record carries only the conflict total). Declared unavailable; no value is derived or imputed.

## Isolation and execution

The cost per decided query is the question's second half and it is a clock; the chained-S3 conflict count is carried as a counter but is not the price. Seconds are evidence only on isolab-eks with native L2, replay and A/A admission; any run on a lower tier, and the preflight, reports them as diagnostic. The decided/undecided split of a completed cell is a count and needs no isolation, but capped is a wall-clock event and shares the tier.

Backend isolab-k8s (isolab-eks (cluster isolab, namespace isolab, us-east-2): cpu-l2-spot for the timed rows; one frozen_command card per shard (m = 2, 5, 6) via tools/task_card.py). Three single-vCPU shards (m = 2, 5, 6) run concurrently, one `frozen_command` card per shard (`tools/task_card.py`), `raw/shard<m>/` and `receipts/` as in n83_msweep:

```sh
research/arity_completion_20261104/koblitz_m5_m6/run_shard.sh ./target/release/examples/n83_msweep_cell research/arity_completion_20261104/koblitz_m5_m6/raw/shard5 5   # also shard6 (6), shard2 (2)
```

Worst case if every cell caps: 110400 CPU-seconds (Part A 78,000: 120 main-arm cells and 10 negatives at 600 s; Part B 32,400: 16 main-arm cells and 2 negatives at 1800 s). Shard walls at most 9600 s (m = 2), 50400 s (m = 5), 50400 s (m = 6); card timeout 57600 s. The dollar ceiling and deadline are set in the launch request before spend and are null here. No checkpoint: a cell interrupted by Spot is discarded and re-run whole; completed records are never re-run.

## Stopping rules

1. (fixed_n) Fixed grid: 148 cells in grid.tsv (Part A, both engines: 40 public + 40 planted + 40 A/A + 10 negative; Part B, both engines: 8 public + 8 planted + 2 negative), one fresh single-threaded process each, seed 8609517; 4 trials in Part A and 1 trial in Part B; cell caps 600 s (Part A, negatives at n <= 41) and 1800 s (Part B).
2. (budget) Per-cell cap: timeout --kill-after=5 at the cell cap; a killed cell is recorded as capped with its wall time and is never re-run with a longer cap.
3. (preregistered_sequential) Part A stopping rule, per m and per engine: curves run in the order k0n23, k1n29, k0n31, k0n37, k0n41; once all four public cells (S = 2, 3 at the two previous curves, same engine) are capped or skipped, every Part A cell of that m and engine (public, planted, A/A) at the next curve is recorded as skipped_stopping_rule and not run. Part B and the negatives are never skipped.
4. (invalidation) Invalidation stop: a failed positive control, a process_error, a binary sha256 different from the frozen one, or a negative control undecided at its cap stops the affected shard for the director's assessment; no cap is changed.

## Invalidation

- A decided planted cell with a proved_unsat verdict or fewer than four verified witnesses invalidates the planted and public records of that (m, n, S) cell and the engine at it.
- A decided public cell whose verified_relations differs from its relations (a witness that does not re-add to the target) is invalid.
- An invalid_model verdict anywhere invalidates the cell.
- A negative-control cell (m = 2, union:2) that does not decide all four queries inside its cap leaves the capped cells at that n uninformative about arity (reported as not_evaluable for K2 and K4); it does not by itself change any cap.
- An A/A pair whose complete records differ in outcomes, relations, verified_relations, base_points or columns invalidates both records and halts the shard; one capped and one complete is a noise flag reported with the pair.
- A binary whose sha256 differs from the frozen value, a source tree different from the frozen commit, or a worker without the L2 admission for a timed row makes its seconds diagnostic only.
- A process_error (exit other than 0, 124, 137) is an invalid cell, not a cap.

## Decision rule

Each cell receives one exact class (decided_sat, decided_unsat, decided_mixed, undecided, capped, skipped_stopping_rule, invalid_control, process_error). K1 to K5 are read per engine from those classes against the thresholds in the claims (K1: the two k0n23 union:2 public cells decide; K2: all m = 6 public cells at n = 37 and 41 capped and a positive log-slope between decided n; K3: every completed planted cell has only verified witnesses; K4: all four n83k1 public cells undecided or capped; K5: no decided public cell at n >= 31 under 188 s). Plainly on K5: Part A keeps TRIALS = 4 under a 600 s process cap, so a decided Part A cell proves at most 150 s per query and any decided Part A cell at n >= 31 falsifies K5 by construction; otherwise K5 stays untested on capped cells, because a cap is a censored observation and says nothing against 188 s. Only Part B (one query per process, 1800 s) can observe a per-query cost above 188 s. The kill criterion fires when, for both engines, K4 and K5 are supported and K3 is not falsified. A capped cell is never a cost.

Threshold: Per cell: decided = every query of the cell decided inside the cap. Parity threshold 188 s per query (largest m = 4 root-table DLP of the n83 record). Observation caps: 600 s per Part A process (4 queries, so at most 150 s per query) and 1800 s per Part B process (1 query).

Statistic: Exact per-cell outcome class and, for decided cells, mean and median seconds per query; ratio of seconds per decided query to 188 s as the distance from the m = 4 root-table parity point. Interval: No sampling interval; four fixed queries per cell, exact outcomes. Effect size: Seconds per decided query divided by 188 s (m = 4 root-table parity) and the m = 6 over m = 5 ratio; log2-slope of seconds per query in n at fixed (m, S).

Vocabulary: `decided_sat`, `decided_unsat`, `decided_mixed`, `undecided`, `capped`, `skipped_stopping_rule`, `invalid_control`, `process_error`, `supported`, `supported_partial`, `falsified`, `not_evaluable`, `kill_high_arity_koblitz_line`, `line_stays_open`, `not_evaluable_instrument_invalid`.

## Analysis plan

`analyze.py [RAW_DIR] [--json ANALYSIS.json] [--tsv table.tsv]` reads the cell records and `.time` sidecars, prints decided / undecided / capped per cell with seconds, checks the controls, evaluates K1 to K6 per engine plus the C-engine pairing, and exits 0. Blinded: false. Frozen before data: true (its sha256 is entered at freeze).

## Inadmissible

- Raising, lowering or otherwise changing any cell cap after the freeze, or re-running a capped cell with a longer cap.
- Changing the seed, trials, engine, factor-base span or ChainS3Limits after the freeze.
- Reporting a capped cell, or the cap itself, as a cost or as a lower bound used for a projection.
- Using the preflight smoke, its output or its seconds as evidence for any claim.
- Reading seconds from a run without native L2 admission as timing evidence.
- Reading a cell as informative about arity when its planted control missed or its negative control is undecided.
- Presenting these relation-collection cells as a complete-DLP result, a leaderboard row or a speed-up over rho (K7).
- Treating a Part A decided cell at n >= 31 as a pass without stating its distance to the 188 s parity point.
- Reading a capped Part A cell as evidence that a query exceeds 188 s: four queries share the 600 s cap.

## Amendments

None.
