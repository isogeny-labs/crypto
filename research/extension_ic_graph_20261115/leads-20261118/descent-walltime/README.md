# Descent wall time: round-1 trace replay checked against the critic

Lane key `descent-walltime`, 2026-11-18. Every number here is a producer
observation or a model, labelled [pcc:unverified]. No pcc claim was
registered. No paid compute, Runpod, EKS or protocol dispatch was used. All
timings are **indicative, non-isolated local timing**: a shared 4-vCPU host
with other agents running, 1 rayon thread, no CPU pinning, N = 6 paired rounds.

## Questions

- **F2.** Amendment 13 found that certified F4 trace reuse lost 2.981–16.086x
  against fresh F4 in all 33 controlled cases. How does round-1's
  `traced_solver` (descent-finder C1) differ, and does that negative apply to
  it?
- **F3.** Round 1 called 1.62–1.71x "conservative", but the linear stage was
  priced from multiplications only. What is the ratio once the counted
  additions are included, and what does wall time say?
- **F4.** Run50 ranked its arms on observed descent counts: 1,728 residuals
  for original IC and 3,415 for the graph arms. What is the ranking when
  descent is normalised to the expected residual count? This is a model only.

## Short answers

1. **F2.** The Amendment 13 negative does not transfer as a loss.
   - **What differs.** Round-1's path differs in trace scope, certification,
     fallback and system family; see the table below.
   - **What was measured.** On the identical stream, the traced arm's descent
     is 1.25–1.29x faster in wall time (geometric mean over 6 paired rounds,
     both streams, with and without hooks).
   - **What does transfer.** Counted savings overstate wall savings. Replay
     still pays symbolic preprocessing, allocation and pair bookkeeping.
   - **Not equivalent to Amendment 13.** Round 1 is not "certified" in
     Amendment 13's sense: there is no Buchberger pair check. [pcc:unverified]
2. **F3.** The 1.62–1.71x label was **wrong**. It is relabelled below:
   - **Counted ratio.** With the linear stage counted exactly (adds, macs,
     words, probes, RNG draws) and the learning-RREF bound charged, the
     descent counted ratio is **1.68x (seed 389) / 1.60x (seed 401)**.
   - **Still incomplete.** Replay bookkeeping is unpriced, so this is still not
     conservative.
   - **Wall time.** The measured descent wall ratio is **1.29x / 1.26x**
     (hooks off).
   - **Critic partly wrong.** The critic's point that Weil and post
     multiplications were unpriced does not hold: they are inside the hooked
     subtotal (reconciliation below). [pcc:unverified]
3. **F4 (MODEL ONLY).** With an arm-independent expected residual count,
   descent is a common term, so the ranking equals the non-descent ranking.
   - **Ranking.** Quadratic table < deferred < original IC < grounded pairs.
   - **§5a band.** At w = 1 GAE, deferred and original IC swap. Only
     "table cheapest" and "grounded most expensive" hold across the band.
   - **Run50's observed ranking.** Run50's observed "original IC cheapest"
     rests on its luckier descent stream. [pcc:unverified]

## F2: how round-1's traced solver differs from Amendment 13

Sources: `AMENDMENT_13_CERTIFIED_TRACE.md`;
`local-certified-s4-trace-20261116-18/{README.md,summary.json}`;
`f4_fp/reuse.rs` at `fcda1e22` (`TracePool::run`, `certify`, `reduces_to_zero`);
`descent-finder/traced_descent_check.rs` (`traced_solver`, `certified`).

| | Amendment 13 (`f4-trace-delayed`, `TracePool`) | Round-1 `traced_solver` (C1) |
|---|---|---|
| System family | Synthetic `F_p^3` S4 coordinate-recovery systems; p ∈ {67, 127, 1039} | Nagao's 6 quadrics in 6 unknowns over `F_1009` (genus-3 Jacobian over `F_{p^2}`); the exact Run50 / nagao-e2e descent streams |
| Trace scope | A **new pool for each batch of 8** distinct residual systems. Capacity 4, 10^6 retained words, keyed on input leading monomials. Learning happened up to 3 times per batch (p67 first batch: learned 3, 5 accepted, 1 rejected). | **One trace per process for the whole stream.** 1 learning run amortised over 3,414 (or 1,505) replays. |
| What is recorded | Only **complete** runs (`!trace.stopped`): a full Gröbner basis | Only a **staircase-stopped** run (`trace.stopped` required): the trace ends at the 64-monomial staircase |
| Acceptance / certification | Replay accepted only with no `trace_mismatch` **and** a full Buchberger certificate. Every input reduces to zero and **every S-pair of the basis reduces to zero**, using a `BTreeMap` term-by-term reducer: 680 pairs and 3.08M certification multiplications in p67's first batch. | Accept if not timed out, no pair above the degree bound, and either `1 ∈ J` or a staircase ≤ 64. Argument: J ⊆ I by construction, and dim R/J ≤ 64 = dim R/I. This assumes **no solutions at infinity**, the same assumption the production staircase stop makes. **There is no pair check**, so the certification costs about nothing (the staircase count is already in F4). A replay that mismatched mid-run but finished at the staircase is accepted (33 + 11 cases). |
| Fallback | On rejection, the trace is **evicted and relearned** (`f4_record`, including the uncounted tracked RREF). | On rejection, plain staircase F4. The trace is kept, and the rejected work is charged. Observed rejections: **0**. |
| Cost scope of the result | Wall time of cold batches. Learning, certification, rejection, fallback and pool lifecycle are all inside the timer. | Round 1: counted operations only. This lane adds wall time (below). |

**Why Amendment 13 lost.** The certification was a full Buchberger check
with a slow sparse reducer. Learning was amortised over at most 7 replays,
and every rejection cost an eviction plus a relearn. The round-1 path has
none of these three costs.

**Does the negative apply?** Not as a loss. On identical inputs, the traced
arm's descent wall is lower in every one of the 24 paired rounds (table
below). Two parts of Amendment 13's lesson do apply:

- **Counted savings shrink in wall time.** The 2.29x counted F4 saving
  becomes 1.32–1.36x in F4 wall time, because replay still runs symbolic
  preprocessing (`Packed::symbolic` interning), pair maintenance and
  allocation (callgrind, below).
- **The certificate is weaker.** Round 1's acceptance rests on dim R/I = 64,
  which is assumed and not checked per system. On these two streams the
  oracle arm (a certified basis with no stop) gave identical solution sets on
  all 4,921 residuals. Holding replay to Amendment 13's certification standard
  would add the S-pair check's cost, which is **unmeasured** here.
  [pcc:unverified]

## F3: counted work, relabelled

Source: `runs/counted/*.json` and `runs/counted-compare-seed*.json`.

- **Build.** These runs use the `descent_walltime_counted` build. It links the
  complete-accounting lane's counted `zero_dim_la.rs`, SHA-256 `f0c50f5f…`,
  pinned in `build/assemble.py`.
- **Arms.** One process per arm, jv_bitops hooks on, 1 thread,
  `foreign_hook_calls = 0`.
- **Prices.** bitop/v1 at a 10-bit field: `fp_mul` 1230, `fp_add` 130.

| Stream | Residuals | Production descent | Traced descent | Ratio | Traced + learning-RREF bound | Ratio |
|---|---:|---:|---:|---:|---:|---:|
| Run50 deferred, seed 389 | 3,415 | 4,358,996,976,408 | 2,483,262,669,096 | 1.755 | 2,596,037,940,936 | **1.679** |
| nagao-e2e, seed 401 | 1,506 | 1,919,259,976,764 | 1,088,291,394,172 | 1.764 | 1,201,066,666,012 | **1.598** |

Checks:

- **Linear stage identical in both arms.** The counted `LinOps` are identical
  in both arms on both streams: add, mac, mul, word and probe counts
  (`identical_linear_stage_ops_both_arms: true`). This is now measured; round
  1 had only assumed it.
- **Outputs.** Output digests (solutions and decompositions) are identical in
  every run of both arms and both builds. Decompositions occur at the same
  indices as in round 1 (263 and 3414; 997 and 1505).
- **Production total cross-check.** The production total equals the
  complete-accounting lane's independent recount (4,358,996,976,408) exactly.
- **Weil and post multiplications are hooked.** Hooked `field_mul` minus
  (`f4_ops` + `weil_muls` + `post_muls` + linear-stage mul + mac) is
  **+3,258,618** (seed 389) and +480,983 (seed 401), identical in both arms.
  If Weil and post multiplications (14.8M and 6.4M) were outside the hooks,
  this difference would be negative. The code agrees: `jv_cover`'s field
  arithmetic goes through `gaudry_cubic::mm`/`am`, which call `hook_mul` and
  `hook_add`.

**Relabel.** Round-1's "conservative 1.62–1.71x" should read:

> counted descent ratio 1.68x / 1.60x (bitop/v1, linear stage fully counted,
> learning-run RREF charged at its upper bound). **Not conservative.** F4
> replay bookkeeping, symbolic preprocessing, hashing and allocation are
> unpriced, and they shrink the saving in wall time to 1.29x / 1.26x.

The descent-finder README is outside this lane's write scope, so the erratum
is recorded here for the orchestrator. Original files are untouched.
[pcc:unverified]

## Wall time: interleaved A/B (indicative, non-isolated local timing)

Source: `runs/ab/` (48 per-run JSONs plus `order.log` with timestamps and
load average) and `runs/ab-summary-seed{389,401}-hooks{0,1}.json`.

- **Setup.** `run_ab.sh`, frozen linear-stage build (`zero_dim_la.rs`
  byte-identical to Run50's `5ae85b34…`). Order: production, traced,
  production, traced… for 6 rounds per stream and hook setting.
- **What is timed.** Only the `nagao_decompose_using` calls. Learning,
  rejection and fallback are inside the timer. The stream advance is outside.
- **Statistics.** The ratio is production/traced. The 95% interval is a
  percentile bootstrap of the geometric mean over whole paired rounds
  (10,000 deterministic draws). A/A is the ratio of consecutive same-arm runs.
- **Load.** The 1-minute load average was 0.7–2.4 during the runs.

| Stream | Hooks | Median descent wall, production / traced (ms) | Descent wall ratio [95%] | F4 wall ratio [95%] | Linear-stage wall ratio [95%] | A/A range |
|---|---|---|---|---|---|---|
| seed 389, n=3415 | off | 10,489 / 8,219 | **1.285** [1.270, 1.304] | 1.361 [1.344, 1.381] | 1.009 [0.994, 1.027] | 0.950–1.053 |
| seed 389, n=3415 | on | 10,798 / 8,616 | 1.247 [1.226, 1.271] | 1.322 [1.298, 1.348] | 0.993 [0.979, 1.008] | 0.950–1.051 |
| seed 401, n=1506 | off | 4,633 / 3,661 | **1.263** [1.206, 1.323] | 1.337 [1.274, 1.403] | 0.992 [0.955, 1.030] | 0.887–1.107 |
| seed 401, n=1506 | on | 4,672 / 3,689 | 1.273 [1.246, 1.304] | 1.352 [1.322, 1.386] | 0.999 [0.982, 1.017] | 0.929–1.040 |

- **Built-in control.** The linear stage does identical work in both arms. Its
  wall ratio sits at 1.00 on every stream, so the per-arm timers are not biased.
- **Every paired round favours traced.** All 24 descent-wall paired ratios lie
  in 1.18–1.36, outside the A/A range on seed 389. On seed 401 with hooks off,
  the A/A range (0.887–1.107) is wider, but every pair is still ≥ 1.18.
- **Per counted F4 operation, replay costs 1.69–1.72x more wall time:** about
  6.1 ns against 3.6 ns.
- **Scope.** This is one host and two exposed p1009 streams. It is not
  isolated, not a pipeline result and not a rho comparison. [pcc:unverified]

### Where replay time goes (callgrind, instruction counts, not time)

Source: `runs/callgrind/annotate-seed389-n300-*.txt`. Valgrind callgrind,
seed 389, first 300 residuals, hooks off. Total instructions:

- production: 7.624e9
- traced: 5.874e9 (ratio 1.30, matching the wall ratio)

Function buckets come from a regex over the annotate output, so they are
approximate:

| Bucket | Production | Traced | Ratio |
|---|---:|---:|---:|
| F4 row arithmetic and reduction (`rref32_deferred`, `add_lazy_avx2`, row closure, interreduce) | 2.838e9 | 1.461e9 | 1.94 |
| malloc/free/memcpy | 2.022e9 | 1.833e9 | 1.10 |
| symbolic preprocessing and hashing (`Packed::symbolic::Interned::id`, …) | 0.895e9 | 0.770e9 | 1.16 |
| F4 driver body (pairs, bookkeeping) | 0.469e9 | 0.422e9 | 1.11 |
| linear stage (`zero_dim_la`) | 0.629e9 | 0.629e9 | 1.00 |
| other | 0.764e9 | 0.753e9 | 1.01 |

Replay halves the arithmetic, which is what bitop/v1 prices. It leaves
allocation, symbolic interning and pair bookkeeping almost unchanged. These
are about 44% of the production arm's instructions and none of them is in the
bitop model. [pcc:unverified]

## F4: Run50 arm ranking with descent normalised (MODEL ONLY)

Source: `walltime_analyze run50` → `runs/run50-normalised-model.jsonl`.

- **Line 1** is the §5a band ranking. It uses U (priced but excluded uniform
  sampler work), B (unpriced native op counts) and GAE = 331,250 bitops from
  `complete-accounting/runs/run50-recount.json`, SHA-256 `19276779…cb267`.
- **Line 2** is the model.
- **This is not a measurement and not a re-accounting of Run50.**

**Expected residual count.**

- Every Run50 arm stopped at 2 accepted descents.
- Pooled success rate over three independent streams: 6 / 6,649 =
  9.02e-4. The streams are Run50 original IC (2 / 1,728), Run50 graph shared
  (2 / 3,415, counted once) and nagao-e2e (2 / 1,506).
- E[N] = 2/p = **2,216** residuals. Garwood 95% on the rate gives 1,018 to
  6,039.

**Assumptions.**

- The success probability is the same in every arm: the same `nagao_decompose`
  and a 509-element base.
- The descent cost per residual is each arm's recorded mean: 1.0037e9 or
  1.0033e9 bitops.

| Arm | Observed total | Observed residuals | Non-descent | Normalised total (MLE, recorded scope) | Non-descent + U + w·B·GAE, w = 0 / 0.1 / 1 |
|---|---:|---:|---:|---:|---|
| original IC | 4,525,556,132,414 | 1,728 | 2,791,131,206,510 | 5,015,705,151,791 | 2.7912e12 / 2.7916e12 / 2.7947e12 |
| quadratic table | 5,230,830,441,710 | 3,415 | 1,804,549,822,274 | 4,028,204,272,116 | 1.8046e12 / 1.8051e12 / 1.8093e12 |
| grounded pairs | 6,235,988,188,572 | 3,415 | 2,809,707,569,136 | 5,033,362,018,978 | 2.8293e12 / 2.9336e12 / 3.8716e12 |
| deferred pairs | 5,913,875,255,822 | 3,415 | 2,487,594,636,386 | 4,711,249,086,228 | 2.5041e12 / 2.5918e12 / 3.3811e12 |

**Rankings.**

- Observed: original IC < table < deferred < grounded.
- Model, at every E[N]: table < deferred < original IC < grounded, because
  the descent term is common.
- Across the §5a band: table is cheapest and grounded most expensive at every
  w. **Deferred and original IC cannot be ranked**: deferred is lower by
  287e9 at w = 0 but higher by 586e9 at w = 1, because the graph arms carry
  2.6M unpriced sampler ops.

The JSON also gives the normalised totals with the complete linear stage, and
with the traced descent (C1 plus the learning bound). Both add the same term
to every arm, so neither changes the order. Rho distance and parity remain
null; this model does not touch them. [pcc:unverified]

## Commands run

1. `df -h /`: 14 GB free before the build.
2. `SCRATCH=<scratchpad>/dw OUT=<scratchpad>/dw/bin bash build/build.sh > build/build.log`
   (`CARGO_TARGET_DIR=/home/user/cargo-target-round2`; refuses to run under
   5 GB free).
   - It assembles the frozen `fcda1e22` descent closure as in
     descent-finder's mini-build, with the `nagao-e2e` `jv_cover` overlay and
     byte-copied shims, plus the counted linear stage behind
     `--features counted`.
   - It builds `descent_walltime` twice, about 14 s each.
   - Hashes are in `build/build.log` and `build/BINARIES.sha256`, and
     `build/Cargo.lock` is kept. The `Cargo.lock` hash equals
     complete-accounting's.
3. Smoke test, n = 20, both builds, hooks on and off. `f4_ops` and hooked
   bitops equal round-1's smoke summary exactly (14,257,995 / 6,547,928 and
   19,979,875,420 / 9,421,065,460).
4. `./run_counted.sh <bin>` → `runs/counted/` (27 s).
5. `./run_ab.sh <bin> 6 0 runs/ab` and `./run_ab.sh <bin> 6 1 runs/ab`
   (about 4.5 min total).
6. `rustc -O --edition=2021 walltime_analyze.rs`. It is std-only Rust; the
   JSON parser is copied from `descent_breakdown.rs`. Then
   `walltime_analyze ab|counted|run50 …` → `runs/*.json*`.
7. `valgrind --tool=callgrind … 389 300 {production,traced} 0`, then
   `callgrind_annotate` → `runs/callgrind/`.
8. `runs/RAW.sha256` holds hashes of every file under `runs/`.

## Errors and deviations

- **Bug fixed during analysis.** The first `walltime_analyze ab` printed a
  Rust set as `{…}`, which is not JSON. It was fixed before any summary was
  kept. A first `run50` output without the band was deleted after an edit to
  the analyzer failed partway; it was regenerated in full from the final
  analyzer.
- **Changes to the copied solver code** (both marked in
  `descent_walltime.rs`):
  - The hooked-linear-stage diagnostic now runs only with hooks on, so a
    timing run carries no extra snapshot cost in the traced arm alone.
  - A rejected replay's milliseconds are added to its fallback's F4 time.
    There were no rejections, so this has no effect here.
- **Missing build inputs.** The descent-finder `mini-build/build.sh` points at
  a worktree that no longer exists and at a `Cargo.lock` it never committed.
  This lane's `build/` repoints at the main checkout and keeps its lock file.
- **research-kb unavailable.** `scripts/research-kb search …` printed "no
  executable found", and no research-kb MCP tools are loaded. Fallback sources
  read: Amendment 13 and its local result README/summary, `f4_fp.rs` (trace
  record/replay and symbolic preprocessing), `f4_fp/reuse.rs`, the
  descent-finder README/CANDIDATES/sources, complete-accounting
  README/recount, and Run50's README, `run-study.json` and raw `work` fields.
  The primary paper (Joux–Vitse CT-RSA 2011) was not re-read this session.
- **Non-isolated host.** Other agents shared the host. Load is logged per run
  in `runs/ab/order.log`.

## Leads and next steps

1. **Replay that skips symbolic preprocessing (engineering, testable
   locally).** Record each step's full row layout, including reducer rows and
   column order, in the trace. Then replay would not intern monomials or
   maintain the pair set.
   - Correctness condition: unchanged (staircase certificate plus fallback).
   - Upper bound from callgrind: symbolic, hashing and driver work are about
     20% of traced instructions.
2. **Allocation in `f4_fp` (both arms).** malloc/free/memcpy are 26–31% of
   descent instructions. Buffer or arena reuse would cut wall time in
   production and traced alike without changing any counted bitop. It must be
   reported as wall time, never as a counted saving.
3. **bitop/v1 vs wall for F4 variants.** Any comparison of F4 variants by
   counted ops misstates wall time by up to 1.7x per operation. Proposal:
   ledger symbolic, hash and allocation events, or require wall A/B next to
   any counted F4 claim.
4. **Certification gap (owner decision).** If replay must meet Amendment 13's
   certificate (a pair check), measure `pair_certificate` /
   `certify` on this stream before adoption. Alternatively, accept the
   staircase certificate already used by production, with the oracle
   cross-check.
5. **Protocol fix for pipeline comparisons.** Run50's arms used different
   descent streams (original IC derives its stream after the factor-base
   draws), so descent was unpaired and decided the observed ranking. Future
   comparisons should share one descent stream across arms, or report descent
   normalised as above.
6. **Isolated confirmation.** `PROTOCOL_DRAFT.md` (draft only, not
   dispatched) is an isolated A/B on an isolab-eks host.
7. **Next-largest descent term.** C4, the linear-stage normal-form replay, is
   now about 15% of traced descent wall (1.23 s of 8.22 s).
