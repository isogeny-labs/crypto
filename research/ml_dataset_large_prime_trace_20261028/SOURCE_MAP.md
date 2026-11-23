# Native large-prime filter trace: read-only source map

Prepared 2026-10-28 against clean producer source
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83` in
`/home/ec2-user/crypto-ml-binary-root-trace-20261028`. This is software design,
not execution or experiment evidence. Source hashes and exact symbol anchors
are saved alongside this file. No producer source, Cargo state, cloud job,
dataset, budget or deadline was changed.

## The two actual paths

| Path | Exact existing implementation | Present behavior |
|---|---|---|
| General binary, prime, and non-admitted Koblitz/twist models | `src/ml_dataset/relations.rs:387` dispatches failed `model` admission to `relations_generic::solve`; `src/ml_dataset/relations_generic.rs:399` calls `large_prime_filter::filter_large_primes` | Identity automorphism with signed-negation columns; batch filtering of all accepted sources accumulated so far |
| Admitted Koblitz model | `src/ml_dataset/relations.rs:58` model, `:387` solve; `src/cryptanalysis/koblitz_large_prime.rs:881` `WideLargePrimeEliminator` | Signed-Frobenius point-orbit canonicalization; incremental elimination, a different reducer and different limits |

Instrumenting `large_prime_filter.rs` alone does **not** expose admitted Koblitz
Frobenius pivots. Do not reroute those cases to make a coverage claim. Generic
collection already admits arities 2..15 with m-2 small summands and two residual
summands; this design adds no higher-m polynomial elimination.

## Batch filter

All paths below are relative to the producer checkout.

- `src/cryptanalysis/large_prime_filter.rs:65` `PartialRelation` contains stable
  `id`, sparse `(u32, u64)` small columns, sparse `(u64, u64)` large columns and
  `rhs`. Coefficients and RHS are normalized modulo the options modulus.
- `:85` `CompleteRelation` contains small columns, RHS and sparse source-id
  coefficients. It does not contain a scalar recovery or a claim of complete
  dependency enumeration.
- `:96` `LargePrimeFilterOptions` exposes modulus, `max_small_weight` and
  `max_sources` (default4096 each). These fill guards are tested **after** a
  proposed AXPY is computed. Initially complete rows bypass those merge guards.
- `:115` `LargePrimeFilterReport` counts input rows/vertices, duplicate rows,
  peeled/core rows/vertices, pivots, successful merges, rejected fills,
  completed rows and cycles closed. These are coarse algorithm counters.
- `:203` `filter_large_primes(Vec<PartialRelation>, &Options)` returns
  `Result<(Vec<CompleteRelation>, Report), Error>`. Error variants are invalid
  modulus, duplicate source ID and noninvertible pivot. Validation accepts odd
  moduli in [3,2^63); it does not itself prove primality. Dataset public-instance
  validation does require a prime subgroup. Preserve this legacy distinction.
- `:46` `normalise` combines duplicate columns through a BTreeMap and drops
  zero coefficients. `:198` `duplicate_key` excludes source ID; normalized
  identical equations retain the first input row, even when IDs differ.
- `:239` onward constructs incidence, initializes a degree-one queue from
  HashMap iteration, and repeatedly removes the sole active row incident to a
  degree-one large column. All row vertices have their degree decremented.
- `:272` onward emits originally complete rows in input order, constructs the
  surviving core and static core column degrees, then sorts core rows by
  `(large weight, small weight, source id)`.
- `:298` onward chooses an existing pivot using
  `(static_core_degree(column)-1)*(current_large_weight-1)`. Equal scores use
  the first sorted column. Degrees are not recomputed after every merge; this
  is not a full dynamic Markowitz search over the current matrix.
- `:310` onward computes pivot inverse with extended Euclid, then
  `scale = -row[p]/pivot[p] mod r`. `WorkRow::axpy` at `:163` computes
  `row + scale*pivot` for small, large, RHS and source coefficients. A filled
  candidate is dropped when it exceeds either existing fill bound; it must
  not replace an existing pivot (`:330` regression-critical comment).
- Completed reduced rows are emitted, with `cycles_closed` incremented only
  for multi-source rows. Otherwise a raw, **unnormalized** pivot row is stored
  under the selected new large column (`:344`). Its pivot coefficient need
  not equal1; subsequent reductions invert it.
- `:362` `replay_complete_relation` recomputes the source combination and
  returns it only when residual columns cancel. The caller must compare the
  returned row with the claimed row. It shares `WorkRow::axpy` with production
  and assumes valid modulus/unique input IDs; alone it is not a separately
  implemented independent checker or hostile-input validator.

Existing tests at lines 399, 417, 435, 454, 470 cover a single-prime pair, a
double-prime cycle, cascading hypergraph peel, arbitrary coefficients, and
fill rejection preserving existing pivots. Other unchanged callers include
`src/cryptanalysis/ecbench_large_prime.rs:1340,:1491` and orbit tests in
`src/cryptanalysis/ecbench_large_prime/orbit.rs`.

## Dataset binding and current visibility

`src/ml_dataset/relations_generic.rs`:

| Lines/symbol | Binding or visibility already present |
|---|---|
| 23 `Limits`, 34 `Limits::new` | Work capped at 100000; query cap4096; pair cap8192 derived from work/4; residual columns32; x candidates2048; filter every32 accepted sources; small/source fill caps4096 |
| 66 `domain`, 99 `pool`, 178 `pairs` | Exact signed domains, subgroup membership/projection, rejected candidates, all pair sums; identity pairs excluded and duplicate sums keep first |
| 226 `make_row` | Small factor-base coefficients, target column `small.columns.len()` with coefficient `-coef_b`, residual signed columns, RHS `coef_a`, stable source ID equal to query sequence |
| 253 `equation` | Reference group replay: sum of all column-point multiples equals RHS times public generator |
| 299 `check_attempt` | Exact public query `aG+bQ`, m-2 selected small points, remainder, pair lookup and encoded partial |
| 340 `Rank`, 348 `reduce` | RREF, RHS, rank, particular values and per-column determined flags; native dense modular reducer remains internally opaque |
| 392 `Batch`, 399 `batch` | Current filter report, completed rows with source replay and group replay, then rank snapshot |
| 445 `recovered` | Read target value only when target column is determined; require scalar-times-generator equals target; complete factor-base logs only if all columns determined/full rank and group replay passes |
| 474 `Transcript` | Strict v1 record with full public instance, orbit action, m, work/derived limits, domains, pairs, attempts, batches and recovery fields |
| 521 `solve` | Emits query, filter-start and filter-complete events; accumulates all sources and refilters the entire prefix at each scheduled batch |
| 694 `verify` | Reconstructs public domains/pairs, validates every retained attempt, recomputes each scheduled batch with the same native filter, and demands a full query history when unresolved and querying is possible |

Missing: normalized-input decisions, duplicate representative IDs, peel
transitions, before/after degree changes, working rows, selected pivot/score,
pivot inverse/scale, AXPY candidates and fill decisions, stored pivot snapshots,
and the internal provenance of each completed row. Rank snapshots are present;
native Gauss–Jordan pivot operations are a separate missing layer.

The current replayer's complete-attempt-history requirement must remain intact
for v1. A new bounded trace stop requires an explicitly justified v2 prefix.
Do not let an incomplete trace silently pass as the existing v1 completed
batch or rewrite old frozen artifacts.

## Separate Koblitz follow-up

`koblitz_large_prime.rs:743` `SparseRow` has full-width BigUint coefficients,
two RHS scalars, BTreeMap small columns, PointKey residual columns and sources.
`:774` scale and `:788` subtract_scaled mutate all components. `Reducer::feed`
at `:836` chooses the first existing sorted pivot, enforces depth/row/pivot
caps, subtracts a normalized pivot, increments reductions, and normalizes a
new pivot to1 before storing. This differs from the batch filter's raw pivots.

`WideLargePrimeEliminator::feed` at `:1105` validates public point equations,
encodes signed-Frobenius coefficients, feeds the reducer and independently
replays resulting pivot/full rows. Source-ID bookkeeping and accepted/full/
capped counters are updated after the result. `FeedOutcome::UnknownCap`
already expresses incompleteness. Adding trace caps would require a terminal
poisoned/censored state to forbid later feeds after a sink failure; pretending
the feed rolled back while leaving the reducer modified would be unsafe.

Dataset helpers `relations.rs:280` and `:317` replay partial and source
equations; `:370` builds the augmented target-column matrix; `:607` replays
the saved full history. This path should be a later separate implementation
increment with matching native tests, not claimed covered by the batch hook.

## Replay/export integration points

- `src/cryptanalysis/ec_index_calculus.rs:1210`
  `gaussian_eliminate_mod_n_particular`: a free-column particular value is
  arbitrary; only a true `determined` flag makes it a candidate logarithm.
- `src/ml_dataset/mod.rs:1833` `verify_cell`, especially `:1974`: binds the
  large-prime transcript requested work to frozen CaseConfig before replay.
  A v2 schema must be recognized explicitly; the current branch selects
  `requested_work` only for generic v1, otherwise `requested_work_limit`.
- `src/ml_dataset/export.rs:229` `large_prime_feature_projection` and `:340`
  `large_prime_cell_projection`: exact path/role/schema and signed-domain
  binding, scalar-certificate matching, and allowlisted static model inputs.
  These currently recognize only the two v1 schemas. Partial live payloads
  remain restricted provenance when no complete replayable transcript exists.
- `src/ml_dataset/solvers.rs:138` `live_event` and `:144` `live_payload` are
  reusable sinks. The bound capture layer already has outer event/byte budgets
  and durable payload registration. Native trace limits must be aggregate
  across all repeated filter calls and remain independently checkable.

## Primary-source context checked

- Cavallar, *Strategies in Filtering in the Number Field Sieve*, CWI
  MAS-R0012 (2000), sections2.1–2.2, inspected at
  https://ir.cwi.nl/pub/4456/04456D.pdf . It documents duplicate/singleton
  processing and controlled merging before sparse linear algebra. Its NFS
  GF(2) setting does not certify this repository's odd-prime ECDLP arithmetic.
- Official CADO-NFS `filter/merge.cpp`, inspected2026-10-28 at
  https://raw.githubusercontent.com/cado-nfs/cado-nfs/master/filter/merge.cpp .
  The source records merge histories for replay and names its structured
  elimination/Markowitz references. This supports retaining operation history;
  this proposal does not import CADO code or claim equivalent algorithms.
- Repository `docs/ml-dataset/{DESIGN,REMAINING-COVERAGE}.md` and the exact
  source-combination/determined-column contracts above supply the local claim
  boundary. No novelty, speedup, attack or asymptotic result is proposed.

Read-only lookup errors retained: two initially guessed source paths were
absent (`relations/generic.rs`, `koblitz_double_large_prime.rs`); actual imports
resolved them to `relations_generic.rs`, `koblitz_large_prime.rs`. A guessed
upstream `filter/README` URL returned404; official `merge.cpp` and the CWI
paper were available. No source/test/build result was inferred from these
failed lookups.
