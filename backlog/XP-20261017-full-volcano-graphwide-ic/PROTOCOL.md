# Full graph-wide direct elliptic IC matrix: protocol

**Status: DRAFT, not frozen.** Written 2026-10-18 after the sizing probes of
§10 and before any full-mode vertex has run. It may still change. It becomes
frozen only once the code gaps of §11 have landed, the open decisions D1 and D2
of §9 are settled, and `protocol.blob` and a merged `code.commit` are recorded in
`card.json`. Card: [`README.md`](README.md). Parent requirement: the study
protocol [`research/full_volcano_graphwide_ic_20261017/PROTOCOL.md`](../../research/full_volcano_graphwide_ic_20261017/PROTOCOL.md)
(the card's earlier `PROTOCOL.md` was a byte copy of it, blob `0df38659`).
This protocol makes that requirement executable. It does not weaken it.

Every statement carries one label: *recorded* (a committed file),
*measured* (the §10 probes, L0, unqualified wall time), *derived* (a
calculation shown here) or *predicted* (stated before the runs).

## 1. Question

How do complete cold index-calculus costs and relation yields vary across
every vertex and every conductor stratum of the three completed isogeny
classes? The targets are exact matched largest-prime-subgroup targets.

This is a measurement of coverage and cost. It does not test whether IC is
faster than rho. The improvement question of §8.3 is secondary.

## 2. The matrix

| | value | source |
|:--|:--|:--|
| graphs | `two` = `icv1-fp57-t536870914-6cabb0c9`, `three` = `icv1-fp57-tm536870941-e6d0c766`, `mixed` = `icv1-fp59-t1073741954-4dd0b1ef` | *recorded*: `evidence/inventory.json` |
| vertices | 8,192 + 6,561 + 10,070 = **24,823**, every one | same |
| strata | 14 + 9 + 28 = **51** conductor strata, with populations as in `per_conductor_vertices` (e.g. `three`: 1, 2, 6, 18, 54, 162, 486, 1,458, 4,374) | same |
| subgroup `r` | 61,751 / 19,768,889,568,671 / 4,253, the certified largest prime per class; `G = [h]P` per vertex | same; `queue.jsonl` `input.curve` |
| inputs | `{two,three,mixed}-workloads.jsonl`, SHA-256 `37d9e0ad…`, `a6ac094f…`, `d28679bb…` | same |
| targets | 8 public one-target workloads per vertex, `target_seed = 20261017`, `target_law = hash_to_subgroup_v1`, `target_index` 0–7 | `driver.rs:112` |
| rounds | 5 measured rounds and 1 warmup, `order = alternate`, `measurement.seed = 1`. Each round draws its own algorithm seed (*measured*: rho counts differ across rounds, §10) | `driver.rs:112` |
| child timeout | 600 s per run | `driver.rs:112` |

**Arms (12 per workload),** exactly as `spec(row, "full")` builds them
(`driver.rs:83–113`):

| arm | role | method |
|:--|:--|:--|
| `rho` | reference (matched rho) | `rho.negation` |
| `bsgs` | reference | `bsgs.textbook` |
| `rho-aa` | A/A control | `rho.negation`, same seeds as `rho` |
| `projected-{16,32,64,128}-{mitm,subtract}` | candidates (8 arms) | `ic.pipeline_counted`, `prime-projected-abscissa:size=B,scan_cap=20000`, `targets=walk`, `linalg=incremental-gauss-full-rank-checked`, `max_trials=1000000` |
| `raw-control` | negative control (unprojected base) | `ic.pipeline_counted`, `prime-abscissa:size=64`, `mitm`, 1,000,000 trials |

**Counts** (*derived*): one vertex has 12 × 8 × 5 = 480 measured records and
96 warmups, so 576 child runs. The matrix has 24,823 × 480 =
**11,915,040 measured records** and 14,297,808 child runs.

**Finite-field arm.** Finite-field IC is not implemented for the actual
embedding degrees k = 30,875, 3,953,777,913,734 and 2,126 (*recorded*,
queue rows `finite_field_applicability`). Every vertex carries one
`unsupported` row with its exact k. That row is not run. It is not counted as
IC and not dropped. That requirement of the parent protocol stays open.

## 3. Sharding and checkpoints

- **Shard key.** The global queue order is the existing one: one
  representative of each of the 51 strata first, then every other vertex,
  round-robin over the graphs (`driver.rs:165–199`). Shard `k`
  (k = 0…255) holds the vertices whose global ordinal ≡ k (mod 256). That is
  96 or 97 vertices per shard. Each shard holds about 32 `two`, 25–26 `three`
  and 39 `mixed` vertices, because 256 and 3 are coprime. So shard costs are
  balanced (*derived*, ≈ 57.5 core-h each, §10.3). Each of shards 0–50 starts
  with its stratum representative.
- **One shard is one campaign directory.** Each shard has its own frozen
  queue, `ledger.sqlite` (WAL, `synchronous=FULL`), writer lock and copy of
  the frozen `ecbench`. A vertex is the atomic unit: one sealed ecbench
  session, standard audit (`verify --replay-all`) and exhausted replay
  receipt (`driver.rs:409–447`).
- **Slots.** One shard runs per CPU slot, single-threaded
  (`RAYON_NUM_THREADS=1`). It is pinned with `taskset -c <slot>` and runs
  `ecbench run --cpus none --lock <shard>/bench.lock`. This needs code gap G3.
  Shards are taken in order 0, 1, 2, … as slots free up. The first wave
  therefore covers 32 of the 51 strata representatives, and the second wave
  covers the rest.
- **Spot interruption.**
  - *Process loss, volume kept.* On restart the driver marks the in-flight
    job `interrupted` (`driver.rs:352–358`). It retries the job in a new
    numbered attempt directory. The interrupted directory is kept and
    excluded from accepted summaries. Loss is at most one vertex per running
    shard: ≤ 2.2 core-h on `three` and ≤ 95 s on `two`/`mixed` (*derived*).
  - *Volume loss.* When a shard finishes, its campaign directory is packed
    per AGENTS.md §13a (deterministic `tar | xz -6 -T1`, `.sha256`). It is
    pushed off-host (`crypto store push`, or the study's Drive folder) before
    its slot takes the next shard. A lost volume therefore loses at most the
    one in-flight shard per slot. Nothing that was already archived is lost.
- **Resume** means invoking the identical `run` argv again on the same
  campaign with the same driver binary. A changed driver, queue or `ecbench`
  is refused (`driver.rs:342–350`).

## 4. Outcomes: nothing is dropped

| status | meaning | in summaries |
|:--|:--|:--|
| `verified` | scalar recovered, `[x]G = Q` checked by runner and auditor, deterministic replay identical | means of cost and yield, with n |
| `exhausted` | the 1,000,000-trial (or rho cap) budget ran out. The final report has complete counters, phases and relation counts. Replayed by the driver's exhaustion receipt | its own row. Its cost is a **censored lower bound** on that arm's solve cost. Never averaged with `verified` |
| `timeout` | the 600 s watchdog fired. Counters are **missing, not zero**. A late child message goes into the sealed diagnostic and is not accepted (watchdog repair) | counted per cell. Means in that cell are labelled "conditional on completion" |
| `error` / wrong answer / failed audit | instrument failure | the driver halts (`driver.rs:453–497`). Retained. Investigated before resume. Never retried for a different answer |
| `interrupted` attempt | infrastructure loss | retained and counted in an interruption table. Its retry is a new attempt |
| `not_run` | vertex never reached | reported with its count. Nothing is inferred for it |
| `unsupported` | finite-field arm (§2) | counted per vertex with k |

Every cell of every table states n per status. The total over statuses must
equal the planned count (§2). A gap is a defect, never a rounding.

## 5. Controls

1. **Gate controls, before shard 0.** The frozen full binary repeats the
   existing `control` (p = 113, r = 7) and `pilot` (three root vertices)
   campaigns. `match_profiles.rs` matches its completed records against the
   repaired-gate receipts (`execution/repaired-qa-summary-v2`). Any mismatch
   of a completed pair halts launch. Censored pairs are listed, not called
   equal.
2. **Matched rho.** `rho` solves the same `(curve, r, G, Q)` as every IC arm
   in the same session. Every IC row reports its gap to rho as
   `S_IC / S_rho` on the same `(vertex, target, round)`.
3. **A/A.** `rho-aa` must equal `rho` exactly on every completed pair (same
   algorithm seed). Any difference is an instrument failure.
4. **Negative control.** `raw-control` uses unprojected base points, whose
   logs to `G` are not qualified. It must never produce a `verified` row
   whose relation logs fail the base-log check. *Recorded*: exhausted in
   every survey row. *Measured*: exhausted 40/40 on `two-v0` and `mixed-v0`
   at 1M trials.
5. **Positive control.** `two` and `mixed` projected arms verify (*measured*:
   40/40 per arm on `two-v0` and `mixed-v0`). A projected arm that verifies on
   neither graph halts the run as an instrument failure.
6. **Replay.** `ecbench verify --replay-all` covers every verified record.
   The driver's exhaustion receipt covers every exhausted record, and both
   must be identical. These are same-implementation replays. Independent
   replay on another host class is a separate later step (§8.4).

## 6. Cost accounting

- Unit: `S = cold total GAE / sqrt(r)` per run, every phase included (§5
  of AGENTS.md). The IC phases come from ecbench: factor-base build,
  decomposition setup, relation collection, linear algebra, target descent
  and recovery, and scalar verification. Cold setup is rebuilt and charged
  in every one-target run. No reusable logs are free.
- Yield: `relations_found / decomposition_calls` (actual oracle calls) and
  `relations_found / targets_tried` (source trials). Numerators and
  denominators are both retained (summary v2 definitions).
- Unpriced native counters of every arm, references included, are listed
  with counts. `ecbench band` prices them at **0, 0.1 and 1 GAE**, applied
  the same way to all arms. Two arms are ranked only where the order holds at
  all three weights (AGENTS.md §5a).
- Wall time is recorded but **not qualified**: runs are L0 (`--cpus none`,
  shared host). No runtime or speedup claim follows from this matrix.

## 7. Predictions (before any full-mode vertex)

1. *Derived.* **Every IC arm on `three` exhausts with zero relations.** For a
   base of B points (2B with signs), a trial hits a two-term relation with
   probability ≈ 2B²/r. At B = 128 and r = 1.98·10¹³ that is 1.7·10⁻⁹. One
   million trials give ≈ 1.7·10⁻³ expected relations, and a solve needs
   ≈ B + 1. *Recorded*: 0 relations in all survey `three` IC rows at 4,096
   trials. *Measured*: 0 at 10⁶ trials on `three-v0`, target 0.
2. *Measured, predicted to hold.* On `two` and `mixed` every projected arm
   verifies in ≥ 99 % of rows. `raw-control` exhausts in 100 %.
3. *Derived from §10.* **No timeouts.** The slowest run (`projected-128-subtract`
   on `three`) took 44 s, against a 600 s limit.
4. *Predicted.* `rho` and `bsgs` costs show no conductor-stratum effect.
   Every vertex of a class has the same group order and r. This is the
   stratum analysis's internal null control.
5. *Predicted.* Every IC row stays above matched rho by at least 10×
   (*recorded* survey: `two` rho ≈ 460 GAE against projected IC ≥ 7,000 GAE).

## 8. Decision rule (fixed before the runs)

1. **Coverage.** Complete iff the union of shard ledgers has all 24,823
   vertices `audited` and the record statuses sum to 11,915,040 measured
   records. Otherwise the result is "partial", with exact denominators. No
   sample is called graph-wide.
2. **Stratum effect.** For each (graph, arm), take the per-vertex means of
   cold total GAE over verified rows. Permute stratum labels within the
   graph, 10,000 times, seed 20261017, and use as statistic the variance of
   the stratum means. A conductor effect is claimed only if p < 0.01 **and**
   the max/min stratum-mean ratio is ≥ 1.05 **and** both hold at all three
   band weights. Otherwise the result is "no detectable stratum effect at
   this resolution". If prediction 4 fails for `rho` or `bsgs`, the analysis
   is suspect and is reported as such.
3. **Improvement over the verified IC baseline.** The baseline is
   `projected-64-mitm` (decision D2). Each other projected arm is paired
   with it on (vertex, target), using 5-round means, and only where both
   are verified in all 5 rounds. Unpaired rows are counted. The statistic
   is the geometric-mean ratio candidate/baseline, with a vertex-cluster
   bootstrap 95 % CI (10,000 resamples, seed 20261017).
   - **Improvement** iff the CI upper bound < 1 at all three weights.
   - **Regression** iff the CI lower bound > 1 at all three weights.
   - Otherwise **unresolved**.

   A candidate that verifies where the baseline exhausts is a coverage
   difference, reported separately and not as a cost ratio. On `three` no
   pair is expected (prediction 1), so the question is reported as
   undecidable there.
4. **Promotion.** No ledger or leaderboard row is promoted until a
   cross-host-class replay passes (`ecbench verify` on another host class).
5. **Instrument first.** These void a reading until explained: an A/A
   mismatch, a gate-control mismatch, a failed audit or replay, a wrong
   answer, or a planned-vs-recorded count mismatch.

## 9. Open decisions (must be settled before freezing)

- **D1: rounds on `three` IC arms.** *Derived* (§10): `three` is 97 % of
  the cost. Most of that is 1M-trial subtraction arms that prediction 1 says
  exhaust with certainty. They run 40 times per vertex and are then replayed.
  - **Plan A** (registered default) keeps 5 + 1 rounds for every arm:
    ≈ 14,700 core-h.
  - **Plan B** runs the 9 IC arms on `three` with 1 round and 0 warmups (8
    targets kept), and everything else as Plan A: ≈ 3,600 core-h. Plan B
    amends the parent protocol's "5 interleaved rounds" for `three` IC rows
    only. It needs code gap G5.

  Recommended: Plan B, because the 4 extra rounds re-measure a fixed budget
  whose outcome is predicted. The owner decides.
- **D2: baseline arm.** `projected-64-mitm` is the arm the survey reports
  per graph. If the owner names another verified recipe, it replaces the
  baseline before freeze.
- **D3: exact representation registration** (gap G4). The parent protocol
  requires exact per-vertex subgroup/generator registration. Either
  register all 24,823 curves, or freeze the native workload-input digest
  (`input_id`, `VWI1…`, bound per vertex in `queue.jsonl`) as the
  representation identity.

## 10. Sizing probes run for this draft (disclosure)

Run 2026-10-18 20:51–20:56 UTC on the shared spot host (Xeon 6975P-C, 16
cores / 32 threads, 124 GB, load ≈ 12–14 from other experiments). The binary
was `ecbench` built `--release` from `abee07ee7` (sha256 `35a9017d…a8490`),
pinned `taskset -c 24-27`, `--cpus none` (L0), private lock, `ulimit -v 16 GiB`.
Specs are the exact `spec(row,"full")` arms for vertex 0 of each graph.
Sessions were kept outside the repository (`/tmp/xp-volcano-probe`) and are
not evidence for any result.

| probe | spec | wall | outcome |
|:--|:--|--:|:--|
| P1 | `three-v0`, 1 target × 1 round, 12 arms | 88.5 s (sum of child process walls 88.3 s); `verify --replay-all` 3.4 s | rho/bsgs/rho-aa verified; all 9 IC arms exhausted at 10⁶ trials, 0 relations |
| P2a | `two-v0`, full spec (8 × (5+1) × 12 = 576 runs) | 51.5 s; audit 19.9 s | 528 verified, 48 exhausted (all `raw-control`) |
| P2b | `mixed-v0`, full spec | 51.0 s; audit 27.4 s | 528 verified, 48 exhausted (all `raw-control`) |

### 10.1 P1 per-arm solve time on `three` (one target, one round)

| arm | s | GAE |
|:--|--:|--:|
| rho / rho-aa | 0.44 / 0.44 | 2,005,511 |
| bsgs | 2.15 | 6,391,777 (peak RSS 171 MB) |
| projected-16/32/64/128-mitm | 0.32 / 0.32 / 0.32 / 0.33 | ≈ 1.0·10⁶ each |
| projected-16/32/64/128-subtract | 5.78 / 11.31 / 22.16 / 43.94 | 3.3·10⁷ / 6.5·10⁷ / 1.29·10⁸ / 2.57·10⁸ |
| raw-control | 0.32 | 1,010,542 |
| **sum of the 9 IC arms** | **84.79** | |

### 10.2 Per-vertex cost (*derived* from §10 and §10.1)

- `two`: 51.5 run + 19.9 audit + 40 × 0.375 exhausted replay = **86.4 s**.
- `mixed`: 51.0 + 27.4 + 40 × 0.391 = **94.1 s**.
- `three`, Plan A, **7,829 s = 2.17 core-h**:
  - run: 48 × 89.1 s = 4,277 s (P1's 88.3 s, plus 0.8 s for mean rho cost
    3.5·10⁶ GAE, *recorded* in the release QA);
  - audit: 40 × 3.75 s = 150 s, plus ≈ 10 s parsing;
  - exhausted replay: 40 × 84.79 s = 3,392 s. The driver's replay
    re-executes each exhausted record (`driver.rs:244–321`) at solve cost.
- `three`, Plan B, **1,712 s = 0.48 core-h**:
  - IC arms: 8 × 85.1 s run + 8 × 84.8 s replay = 1,358 s;
  - generic arms: 48 × 4.05 s = 194 s;
  - audit: 160 s.

### 10.3 Totals (*derived*)

| | `two` | `three` | `mixed` | **total core-h** |
|:--|--:|--:|--:|--:|
| Plan A | 8,192 × 86.4 s = 197 | 6,561 × 7,829 s = 14,268 | 10,070 × 94.1 s = 263 | **14,728** |
| Plan B | 197 | 6,561 × 1,712 s = 3,120 | 263 | **3,580** |

Per shard (1/256): ≈ 57.5 core-h (Plan A) or 14.0 core-h (Plan B).

Peak RAM per slot is ≤ 0.35 GB: the largest child is 171 MB (`bsgs` on
`three`), the ecbench parent ≤ 81 MB and the auditor ≤ 172 MB, all
*measured*. 32 slots need ≤ 11 GB.

**Disk** (*derived*):

- One vertex's session is 3.93 MB on `two`/`mixed` (*measured*, 3,928,401 B
  and 3,931,201 B for 576 records). On `three` it is ≈ 3.98 MB (576 ×
  6,684 B/record from P1, plus plan/host/spec).
- Each vertex adds an audit (56 KB) and an exhaustion receipt (≈ 8 KB, or
  68 KB on `three`): ≈ 4.0 MB per vertex on `two`/`mixed` and 4.11 MB on
  `three`.
- Raw sessions: 18,262 × 4.00 + 6,561 × 4.11 = **100 GB (93 GiB)**.
- 256 copies of the 25.2 MB `ecbench`: 6.5 GB.
- Packed shard archives: ≈ 12 GB, at the *recorded* 2,935-vertex archive
  ratio of ≈ 0.81 KB per record.
- Total ≈ 119 GB (110 GiB). Each shard needs ≈ 0.43 GB, so the guard is set
  at 1 GiB.

## 11. Code that must land before freeze

- **G1** `driver.rs:339–341`: `run` refuses `full` unconditionally. Replace
  this with a check that the gates of §5.1 passed and that D3's registration
  manifest hash is bound in `campaign.json`.
- **G2** `driver.rs:131–222` (`plan`): there is no shard selection. Add a
  `K/N` selector that filters the global queue order (lines 165–199) by
  ordinal mod N before `emit`. Record `shard`, `N` and the global queue's
  SHA-256 in `campaign.json` (lines 216–219).
- **G3** `driver.rs:409–425`: the driver hard-codes `--cpus auto --allow-busy
  --wait --settle 0.1` and the default lock `/tmp/crypto-bench.lock`.
  - `runner.rs:383` holds that lock exclusively for a whole session, so
    parallel shards would serialize.
  - `--cpus auto` evicts other processes from the reserved core
    (`runner.rs:398`, `isolation.rs:766`), which would disturb other
    experiments on a shared host.
  - Fix: pass `--cpus` and `--lock` through from the driver argv.
- **G4** (D3) `src/cryptanalysis/ecbench/workload.rs:505–540`: the registry
  is compiled in with `include_str!` and parsed per workload.
  - *Measured*: the three root slugs resolve `registered: true` but
    `ec1: null`, because no representation has these (r, G).
  - The other vertices' slugs are unregistered.
  - At ≈ 1.85 KB per entry (*recorded* 4.75 MB / 2,572 curves), 24,823
    entries add ≈ 46 MB. That calls for a runtime-loaded registry shard, or
    for D3's digest route. `inventory.rs:144` writes
    `PENDING_EXACT_SUBGROUP_GENERATOR_REGISTRATION`.
- **G5** (only if Plan B) `driver.rs:83–113`: `spec()` takes one
  `rounds`/`warmup` for all arms. Plan B needs two sessions per `three` vertex
  (generic arms 5+1, IC arms 1+0), both sealed, audited and replayed under
  the same job.
- **G6** `summarize.rs` emits min/max/mean only (lines 14–26, 238–250). The
  §8.2 permutation test and §8.3 paired bootstrap need a native analysis
  command.
- **G7** `Cargo.toml:130–137`: `checkpoint.rs` (the native receipt/seal
  inventory used for the 2,935-vertex checkpoint) is not a Cargo bin. The
  card's `outputs.verify` needs it, or an equivalent, as a per-shard
  verifier.
- **G8** The card format wants a `resume_flag`. The driver resumes by plain
  re-invocation. Either add an explicit `resume` alias (`driver.rs:516–524`)
  or record that convention.

## 12. Inadmissible after freeze

- Changing the base grid, oracles, `scan_cap`, trial budget, targets,
  seeds, rounds/warmup (beyond the D1 choice made before freeze), timeout or
  arm list.
- Dropping a vertex, stratum, arm, target, round or status class.
- Dropping or zero-filling timeout counters.
- Pooling across binaries, campaigns or the earlier survey, dev99 or
  release70 datasets.
- Re-running a vertex for a different answer. A retry is admissible only
  after an `interrupted` or infrastructure attempt, and both attempts are
  retained.
- Changing GAE accounting or the pricing band.
- Counting `raw-control`, `rho`, `bsgs` or any generic fallback as IC.
- Calling a sample or a partial ledger "graph-wide".
- Making any wall-time, speedup or faster-than-rho claim from this L0
  matrix.
