# Sketches, a structural-rank gate and a mixed solver for the relation-solving step

Frozen 2026-11-15 before any data was taken. Owner request (this session):
"integrate an approach that uses bloom filter, HyperLogLog or count-min sketch
for relation solving step or some algorithmic tricks", then "yes for both
[matching gate; Bloom large-prime sketch null] and try to find novel way here
to integrate these other algorithms into a combined/mixed solver".

Sources checked: `docs/ic/LEADERBOARD.md`; `research/ic_prime_residual_lp_20261026`;
`research/ic_planner_exhaustive_20261026`; `research/n83_bloom_profile_results_20261015`;
`research/ic_tool_program/explorations/blocked-filter-20261112`;
`cryptanalysis:experiments/a1-conjugate-bloom-20261009`;
`src/cryptanalysis/ic_framework/{linalg,residual,large_prime}.rs`. The shared
`research-kb` catalog and the sibling `crypto-autoresearcher` lead atlas are
not present in this container, so "not tried" below means "not found in
`crypto` or `cryptanalysis`" (`git grep` for HyperLogLog / count-min: no hit).

## Why sketches alone cannot move the counted unit

ecbench's counted unit is group operations (GAE). Bloom filters, HyperLogLog
and count-min sketches replace exact membership/count structures; they change
native work (priced at w = 0.1 and 1 GAE in the §5a band) and memory, never
group operations. Every exact structure in the relation-solving step is small
(|F| ≤ a few thousand columns), so an exact array costs less than a sketch.
Bloom prefilters for the decomposition probes were already measured (N83
profile, A1 conjugate filter) or dropped (split-block, 2026-11-12). This study
therefore puts the sketch where it can at least be priced honestly, and tests
the algorithmic devices that can change rows read (hence relation-phase GAE)
or `row_ops`.

## Arms (all in `src/cryptanalysis/ic_framework/{linalg,linalg_mixed}.rs`)

Existing: `incremental-gauss` (target-pinned incumbent), `incremental-gauss-full-rank`,
`incremental-gauss-echelon`, `structured-gauss`, `filter+wiedemann` (= `sparse-cado`),
`filter+lanczos`.

New:
- `filter+wiedemann+match`, `filter+lanczos+match`: the deferred solver behind a
  **structural-rank gate** — an incremental maximum row–column matching
  (Kuhn, one augmenting search per arriving row). No solve while the matching
  is below `cols` (such a solve cannot succeed). Edge visits charged as row_ops.
- `mixed-peel`: the gate decides *when*; the matching decides *which* rows (a
  square system); the square system is **peeled from both sides** as an
  invertible Bloom lookup table is decoded (row with one live column → forward
  substitution; column with one live row → back-substitution stack); the
  2-core is eliminated by **Markowitz** sparse Gauss; non-zero pivots certify
  rank, so **one** solve replaces the deferred solvers' two seeded solves;
  surplus rows are checked against the solution.
- `mixed-peel+bloom`: the same with a **Bloom duplicate screen** (10 bits a row,
  4 probes, canonical row hash up to scalar) in front of an exact check.

## Part A: synthetic planted streams (`examples/ic_mixed_solver_stream.rs`)

|F| ∈ {32, 64, 128, 256, 512}, m ∈ {2, 3} factor-base non-zeros + the target
column, 16 seeds each, modulus 2^61 − 1. Every arm reads the same stream until
its stop rule. Measured: rows read, row_ops, attempts, correctness.

## Part B: ecbench on real prime curves

Spec `specs/partB-p24.json`, `specs/partB-p28.json`: the p24 and p28 curves of
`research/ic_prime_residual_lp_20261026` (`icv1-fp24-tm5099-a6f137c6`,
`icv1-fp28-t28201-65099091`), oracle `mitm-prime-blocked:m=2`, base
`prime-abscissa:size=128`, `targets=walk`, every linalg arm above, plus
`rho.negation` as matched reference; 8 targets × 3 rounds; all solves verified.

## Part C: the Bloom large-prime sketch (documented null)

One ecbench run of `ic.large_prime` (the frozen-large-prime spec of
`research/ecbench_accounting_v2_20261023`, unchanged) to read
`lp_partials_fed`, stored pivots and merges. The upper bound of what a Bloom
"seen-twice" gate could save is the eliminator's work on partials that are never
merged; whatever it is, it is native work, so it cannot move S at w = 0.

## Predictions (falsifiable)

- **P1 (gate).** Part A: `filter+wiedemann+match` reads ≤ the rows of
  `filter+wiedemann` on every stream and has ≥ 1 fewer solve attempt on the median
  stream; its row_ops are lower on the median stream for |F| ≥ 64.
- **P2 (mixed rows).** Part A: `mixed-peel` reads exactly the rows of
  `incremental-gauss-full-rank` on ≥ 99 % of streams (both stop at numeric full
  rank, equal to structural full rank with probability ≥ 1 − cols/r).
- **P3 (mixed work).** Part A: median row_ops of `mixed-peel` < `filter+wiedemann`
  at every (|F|, m), and < `incremental-gauss-full-rank` for |F| ≥ 128.
- **P4 (Bloom null).** Zero duplicate rows on every stream and every Part B run;
  `mixed-peel+bloom` row_ops > `mixed-peel` everywhere (pure overhead).
- **P5 (counted unit).** Part B, w = 0: S(`mixed-peel`) = S(`incremental-gauss-full-rank`)
  within the paired interval; S(`filter+wiedemann+match`) ≤ S(`filter+wiedemann`);
  every full-rank arm costs more than the target-pinned `incremental-gauss`
  (the full-rank rule reads coupon-collector excess rows). No arm is below rho.
- **P6 (large-prime Bloom).** Part C's saving bound is native-only: S at w = 0 is
  unchanged by construction.

## Classification

Any gain is a **component / engineering** result in the linear-algebra stage
(AGENTS.md "Success criterion"; relabelling not counted). A gain at w = 0 counts
only where it comes from fewer rows read (fewer relation trials). Distance to
rho is reported, not gated. Every figure is `[pcc:unverified]` until a
validator report exists; no leaderboard row is claimed (no VALIDATION.json).

## Deviations allowed

None to the predictions. If a spec fails to run (e.g. an arm refused at plan
time), the failure and the fix are recorded in README.md before re-running.
