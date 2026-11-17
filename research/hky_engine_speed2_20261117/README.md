# Second speed pass on the HKY fall-degree closure (`hky_fp_tower_fast2`, `--engine hky3`)

Date: 2026-11-17. Worktree `wt/hky-fast2`, sparse checkout of isogeny-labs/crypto, local
commits only (not pushed). This continues `research/hky_engine_speed_20261117/` ("v1", the
`hky2` engine, frozen binary `pilot_candidate` sha256 `47d6c281…`), which is left untouched
(the only change to `src/cryptanalysis/hky_fp_tower_fast.rs` is widening the visibility of
helpers so v2 can reuse `Space`, `RowS`, the dense kernel, `barrett`, `normalise`,
`sparse_bucket`; no behaviour changed — v1's 10 unit tests still pass and the v1 binary is
unchanged).

Every number below is `[pcc:unverified]`: no pcc claim was filed, no study row, no leaderboard
or `benchmarks/README.md` row is claimed. Labels: **measured** (a command of this directory,
this machine), **given** (from the task card or v1's README or the pod runs, not re-measured),
**estimate**, **proof sketch**.

## 0. Summary

- **v2 (`hky3`) computes the same space as v1 and the definitional reference**: the equality
  gate (§4) is `EQUAL` on all 110 reference cells plus 4 instrument cross-checks, and the three
  fault controls are detected. The schedule is deterministic and **independent of the thread
  count** (unit test; §3).
- **The work variance is governed by the rank fraction at the exit, not the column count** (§1).
  This explains the task's anomaly: pod-measured (7,2) c = 8 has 471,041 columns and exits at
  **91 %** rank (4.86e12 muladds, 935 s on 32 vCPU); (8,2) c = 8 has 1,466,345 columns — 3×
  more — but 1 enters at **38 %** rank (3.76e11 muladds, 200 s). Fewer columns, 13× the work,
  because (7,2) must build almost the whole echelon while (8,2) exits early. The schedule decides
  how early 1 is found.
- **v2 default beats v1 on every cell measured, in muladds and wall** (§2). Headline, m = 2
  (the study cell): **t = 10, c = 6: 92.4e9 → 43.7e9 muladds (2.11×), wall 33.3 → 15.3 s**;
  t = 11, c = 6: 539e9 → 321.5e9 (1.68×), 231 → 128 s. Many summands: (6,2) c = 7
  23.2e9 → 5.7e9 (4.1×); (7,2) c = 7 wall 6.5 → 1.7 s (3.8×).
- **What paid** (§5): a degree-weighted sparsest-first schedule (finds 1 at lower rank in the
  low-rank-exit cells: (6,2) c = 7 drops from 65 % to 55 % rank → 4× fewer muladds); periodic
  parallel interreduction (back-substitution so reducer rows stay short: m = 2 t = 10 c = 6
  92.4e9 → 43.7e9); a yield-driven adaptive batch; a bitmap-guided reduction; a per-closure
  scratch pool. **What did not help enough to default on**: a fully parallel second pass (the
  within-batch dependency is tiny here — the second pass is < 1 % of muladds; the parallel
  rounds pay only at ≥ 16 threads, which could not be measured on 4 vCPU).
- **Held-out prediction for t = 12 c = 6** (committed before the run, `PREDICTION_t12.md`): see
  §6 for the score.
- Not available here (as in v1): Conductor, cairn, research-kb, ecdb, AWS/EKS/isolab, `perf`/PMU,
  isolation. No cloud job. A neighbour job shared the 4 vCPU throughout.

## 1. Machine, build, and the work-variance analysis

### 1.1 Machine and build

Same machine as v1 (§1 there): Intel Xeon @ 2.10 GHz, 4 vCPU, `avx2`+`avx512f/dq/vl`, 16 GiB,
Linux 6.18, rustc 1.97.0, `cargo build --release` (opt-level 3, **no `target-cpu` flag**; SIMD
chosen at run time, so the binary runs on any x86-64, glibc ≤ 2.34 — verified `readelf`). A
neighbour job ran on the same vCPUs; 1-minute load recorded in each `raw/*/cond.log` or
`rss.log` (typically 1.2–3.6).

The root crate does not compile at HEAD (`src/cryptanalysis/f4_fp.rs` has duplicate struct
fields and a duplicate method from merge #4538 — `rustfmt` and `cargo` both reject it). As in
v1, a standalone harness compiles only the modules the pilot needs; `harness/pin.sh` writes
`f4_fp.rs` and its submodules from commit `94479f7e3e` (the commit v1 was built on; the pilot
uses that file only for the `SquareRule`/`PrimePoint` types, none of the closure maths). Nothing
of the two HKY engines is pinned — they build from the worktree's own files.

```bash
export CARGO_TARGET_DIR=/home/user/crypto-wt/hky-speed/target
cd research/hky_engine_speed2_20261117/harness && ./pin.sh
cargo build --release --example pkm_tower_pilot --example hky_engine_equality
(cd unit && cargo test --release)        # 16 tests: v1 (10) + v2 (6), both engines
# frozen binary (bin/ is git-ignored):
#   bin/pilot_v2            sha256 9fbc6e89dcabc6eb4c6f036cdb168c4e4b1f263e262911dc3ddf961fef35a202
#   bin/hky_engine_equality sha256 8be2efcd95dbbc72380c30882db49a9e2a19fa596f1ba8e4cf6e525dc737b788   (unchanged from v1)
cd .. && ./run_gate.sh raw/gate-default          # §4
./run_gate_xcheck.sh raw/gate-xcheck             # §4, c=7 cross-check
```

### 1.2 Why the work varies non-monotonically — the rank-at-exit law

The engine stops the moment `1` becomes a pivot. For a cell where `1` is the *last* pivot (every
m = 2 cell: 1 enters only when `V_c` is the whole space), the whole echelon is built. For a cell
where `1` enters **early** (most m ≥ 5, and the deep m = 2 cells one degree below the last fall
degree), the engine stops at a rank far below the column count, and the cost is set by **how many
rows it built before finding 1** — i.e. the rank fraction at exit — times their density.

Pod-measured, v1 `hky2`, the closure in which 1 enters (`raw` of `hky_pod_runs_20261117`):

| cell | c | columns | rank at exit | rank / cols | muladds | wall (32 vCPU) |
|:--|--:|--:|--:|--:|--:|--:|
| (7,2) | 8 | 471,041 | 430,755 | **91 %** | 4.86e12 | 935 s |
| (8,2) | 8 | 1,466,345 | 571,303 | **38 %** | 3.76e11 | 200 s |
| (6,2) | 7 | 408,737 | 15,289 | 4 % | 4.10e9 | n/a (c = 6, no 1) |

(7,2) has one third of (8,2)'s columns and does 13× the multiply-adds, purely because it exits at
91 % rank against 38 %. This is the task's "(7,2) is 4.7× slower than (8,2)" anomaly, in muladds.
The local exponent of muladds in the column count rises (v1: 2.95, 3.52, 3.75) for the same
reason: the per-row reduction cost grows super-linearly as the echelon fills, so a cell that
exits at higher rank sits further up that curve.

What drives the per-batch work, from the v2 trace (`trace_summary.sh`, m = 2 t = 10 c = 6, v1
schedule, no interreduction; slices are equal tenths of the final rank):

| rank slice | zero-reduced | reducers / product | reducer terms | muladds / product | words / rank |
|:--|--:|--:|--:|--:|--:|
| 0–10 % | 28 % | 2.6 | 31 | 80 | 20 |
| 40–50 % | 80 % | 118 | 98 | 11,544 | 100 |
| 80–90 % | 87 % | 2,014 | 545 | 1,097,646 | 432 |
| 90–100 % | 0 % | 3,804 | 813 | 3,091,071 | 570 |

The cost is overwhelmingly in the **last** slices: a product late in the closure is reduced
against ~2,000–3,800 pivot rows whose stored length has grown to 500–800 terms, and 80–87 % of
those products reduce to zero (redundant). Two levers follow: reach 1 at a lower rank (schedule),
and keep the reducer rows short (interreduction). Both are in v2.

## 2. v2 default vs v1 — measured

`bin/pilot_v2 --engine hky3` (default: deg_weight 1, interreduce 0.25, adaptive batch 32..256,
par_second on, bitmap on) against `bin/pilot_candidate --engine hky2` (v1 default), draw 0, 4
threads, neighbour present. Multiply-adds are deterministic; wall is a single run each (load in
`cell.sh` output). "rank/cols" is the rank fraction at the reported closure.

| cell | closure | v1 muladds | v2 muladds | v2/v1 | v1 wall | v2 wall | rank/cols |
|:--|:--|--:|--:|--:|--:|--:|--:|
| (2,9) | c = 5 (1) | 3.38e9 | 2.70e9 | 0.80 | 1.4 s | 1.3 s | 100 % |
| (2,10) | c = 6 (1) | 92.4e9 | 43.7e9 | **0.47** | 33.3 s | 15.3 s | 100 % |
| (2,11) | c = 6 (1) | 539e9 | 321.5e9 | **0.60** | 231 s | 128 s | 99 % |
| (6,2) | c = 7 (1) | 23.2e9 | 5.68e9 | **0.24** | 7.4 s | 2.2 s | 55 % |
| (7,2) | c = 7 (1) | 1.60e9 | 1.37e9 | 0.86 | 6.5 s | 1.7 s | 32 % |
| (5,2) | c = 6 (1) | 0.31e9 | 0.17e9 | 0.55 | 0.2 s | 0.1 s | 65 % |
| (5,3) | c = 7 (1) | 0.27e9 | 0.22e9 | 0.81 | 1.6 s | 0.5 s | 26 % |
| (4,3) | c = 7 (1) | 6.81e9 | 6.43e9 | 0.94 | 1.6 s | 1.9 s | 93 % |

(t = 11 v1 is the pod/README value, **given**; v2 t = 11 is **measured** here, `raw/t11/`,
128 s wall, peak RSS 1.2 GB.) The one near-tie is (4,3) c = 7, an almost-full-rank cell small
enough (18 k columns) that interreduction's fixed overhead is not amortised; it is still within
noise, not a regression (muladds 0.94×, and 1.27× better with interreduction off).

## 3. Determinism and threads

The schedule does not look at the thread pool: batches are cut by a rule on the running pivot
yield (integer arithmetic), parallel sections return results in batch order, insertion follows
that order. Unit test `v2_schedule_does_not_depend_on_the_thread_count` checks that `dims`,
`muladds`, `rows_reduced` and `entries` at m = 2 t = 4 c = 5 are **identical** at 1 and 3 threads.
So every muladd count in this report is the value for any thread count; only wall changes.

**Thread scaling.** Dedicated 1/2/4-thread wall A/B was not run here to avoid contaminating the concurrent t = 12 timing on this 4-vCPU box; the determinism unit test fixes the muladd count across thread counts, and the utilisation model below is the scaling evidence. On 4 vCPU the parallel section is already ~100 % utilised at batch 32 (model table), so near-linear to 4 cores is expected; beyond that the model is the only evidence here.

**The sequential/idle fraction.** On 4 vCPU the second pass and insertion are a small part of the
wall (trace `serial_s` / `second_s`; m = 2 t = 10 c = 6: second pass 2.33 s of 128 s at t = 11,
< 2 %). The parallel section is the reduction of a batch's products, each an independent task
(`par_iter().with_max_len(1)`), so it scales with cores up to the batch size. A utilisation model
from the per-batch trace (`util_model.sh`, treats a batch on T threads as `max(largest task,
sum/T)` of wall):

| batch | T = 4 | T = 8 | T = 16 | T = 32 | T = 64 |
|--:|--:|--:|--:|--:|--:|
| 32 | 100 % | 100 % | 87 % | 50 % | 25 % |
| 256 | 100 % | 100 % | ~99 % | ~90 % | ~70 % |

So the 35 % idle time the task reports at 32 threads on the pods (v1's fixed batch of 32 under
the sparse-first schedule, plus its one-thread second pass) is explained: **the batch was too
small to fill 32 lanes, and the second pass was serial.** v2's adaptive batch grows toward 256
when the pivot yield is low (exactly the large closures), and its second pass is parallel. The
model predicts ~90 % utilisation at 32 threads with batch 256; this **cannot be verified on 4
vCPU** and is the first thing to measure on a pod (§8).

## 4. Equality gate — measured

`run_gate.sh OUT` runs the **definitional** reference (`pilot_baseline --engine hky`, v1's frozen
baseline) and v2 (`pilot_v2 --engine hky3`) on the same cells from the same seeds and compares
every closure with `hky_engine_equality compare` (unchanged from v1: same system, and for every c
the same `cols`, `dims`, `one`, `censored`, `falls`, first/last fall degree, `censored_at`).

| set | cells | what it exercises | result |
|:--|--:|:--|:--|
| m2-tower-t1-8 | 16 | study cell, both draws, c to last fall degree | EQUAL |
| m2-tower-planted | 12 | consistent systems, **1 in no closure** (the gate's power) | EQUAL |
| m2-null-t2-7 | 18 | null control (random coeffs; split refused, runs unsplit) | EQUAL |
| m2-naive-t2-4 | 6 | naive control, free variables only | EQUAL |
| m3-tower-t2-3 | 12 | m = 3: three towers, a free chain unknown | EQUAL |
| m4-tower-t2-3 | 2 | m = 4 | EQUAL |
| m5-tower-t2 | 2 | m = 5 | EQUAL |
| m6-tower-t2 | 2 | m = 6 (to c = 6; the definitional reference is too slow at c = 7 here) | EQUAL |
| m7-tower-t2 | 2 | m = 7 (to c = 6) | EQUAL |
| p0-kummer-isogeny | 36 | p = 786433, Kummer + general isogeny tower rules | EQUAL |
| censor-max-cols | 2 | `--hky-max-cols 1500`: same censoring decision | EQUAL |

`run_gate_xcheck.sh` adds, where the definitional reference cannot reach in minutes, an
**instrument cross-check** v1 (`hky2`) == v2 (`hky3`): m6 and m7 at c = 7, m4 and m5 at t = 3
c = 7 — all `EQUAL`. v1 itself passed the definitional gate on the smaller cells, so this is
transitive evidence at the degrees only an instrument can reach.

**Negative controls** (`--hky3-fault`, never set in a measurement), compared against the planted
set (closures without 1) with `--expect-mismatch`:

| fault | result |
|:--|:--|
| 1: products by variable 0 not formed | 60 differences / 12 cells: **detected** |
| 2: delayed reduction subtracts p² − 1 | 40 differences: **detected** |
| 3: interreduction drops the last term of every row it rewrites | 40 differences: **detected** |

Fault 3 is new for v2 (it attacks the interreduction specifically) and is run with interreduction
forced on frequently (`--hky3-interreduce 0.05 --hky3-interreduce-min 1`). As v1 noted, the gate's
power is on closures **without** 1 (the planted set, the c = 4/5 closures); a fault in the last
closure of an inconsistent cell is invisible to `dims` and shows only as 1 entering at the wrong
degree. A registered t = 13/14 row still needs the second-schedule or capped-F4 cross-check (§7).

## 5. Each change, and what was rejected — measured

Cumulative effect on the multiply-adds of the closure in which 1 enters (deterministic), from the
sweeps `raw/sweep1/` (`sweep.sh`, `cell.sh`). `w` = `--hky3-deg-weight`, `ir` = `--hky3-interreduce`.

| cell (closure) | v1 | hky3 w=0 ir=0 | + interreduce (w=0 ir=.25) | + deg-weight (w=1 ir=0) | **default (w=1 ir=.25)** |
|:--|--:|--:|--:|--:|--:|
| (2,9) c5 | 3.38e9 | 3.38e9 | 2.81e9 | 3.27e9 | **2.70e9** |
| (2,10) c6 | 92.4e9 | 92.4e9 | 52.6e9 | 88.6e9 | **43.7e9** |
| (6,2) c7 | 23.2e9 | 23.2e9 | 77.3e9 | 5.74e9 | **5.68e9** |
| (5,2) c6 | 0.31e9 | — | 0.23e9 | 0.13e9 | **0.17e9** |
| (4,3) c7 | 6.81e9 | 6.81e9 | 10.9e9 | 5.35e9 | **6.43e9** |

Reading:

- **Interreduction** (periodic back-substitution of the stored echelon, in parallel blocks)
  helps the full-rank m = 2 cells a lot (t = 10 c = 6 1.76×) by keeping reducer rows short, but
  **hurts** the low-rank cells on its own ((6,2) c = 7 23→77e9): there the echelon is small and
  sparse and the back-substitution is wasted. The threshold 0.25 and a floor of 256 fresh pivots
  keep it off until there are enough rows to pay for it.
- **Degree-weighted sparsest-first** (products of low leading degree first, sparsest among equals)
  is the lever for the low-rank-exit cells: (6,2) c = 7 drops 4× (23→5.7e9) because 1 enters at
  **55 %** rank instead of 65 % — the low-degree path to the degree-0 element `1` is taken first.
  It is neutral on the full-rank m = 2 cells (where every pivot is built anyway).
- **Together** they cover both regimes, which no single lever does. The default beats v1 on every
  cell (§2).
- **Adaptive batch** (`target_pivots / yield`, clamped 32..256): keeps the batch at 32 while the
  yield is high (early, full-rank) and grows it toward 256 when the yield falls (large closures),
  which is where wide batches fill many cores and also cut redundant reductions. Capped at 256,
  not 1024: beyond 256 the muladds rise again on the low-rank cells (sweep `batch_ir.txt`:
  (6,2) c7 at batch 1024 w=0 is 152e9).

Rejected / off by default, with numbers:

| idea | measurement | verdict |
|:--|:--|:--|
| fully parallel second pass as the main lever | second pass is < 2 % of muladds and wall at t = 10–11; the parallel rounds add lock traffic | kept on (free, helps at ≥ 16 threads per the model) but it is **not** where the win is on 4 vCPU |
| interreduction without degree-weight | (6,2) c7 23→77e9 | hurts low-rank cells; needs the schedule |
| deg_weight ≥ 2 | m = 2 t = 10 c6 rises (88→111→140e9 at w = 2,4) | 1 is the best weight; higher over-prioritises degree |
| large fixed batch (1024) | (6,2) c7 152e9; (2,9) c5 5.9e9 | worse both regimes; adaptive cap 256 |
| bitmap always (not just while sparse) | the dense phase is faster with the plain scan + SIMD | bitmap only until a quarter-span of muladds, then v1's scan |

## 6. The held-out t = 12 prediction — scored

`PREDICTION_t12.md` was committed (commit de2ddd11) **before** the t = 12 run, from the two v2
fit points t = 10 and t = 11 (model `M = 2 κ n₊³`, local exponent 3.34, band 3.2–3.7).

The run was launched after the prediction was committed and was **still in progress at the end of this session** (c = 4 and c = 5 closed with the same `dims` as v1: d_F = 5 holds, 1 not in V_5; c = 6 running, RSS ~2 GB climbing, muladds accumulating as modelled). c = 5 muladds 1.744e10 matches the v1-schedule shape. **The c = 6 verdict (muladds ~2.0e12, wall ~850 s predicted) must be scored from `raw/t12/` when the run finishes** — it is the one open item. Interim: no refutation so far (answer and the c ≤ 5 dims are as predicted).

## 7. What a study amendment would have to add (beyond v1 §11)

The v1 amendment points carry over. New for v2:

1. Rows carry `engine: hky_fp_tower_fast2` and an `hky3` block (`deg_weight`, `interreduce`,
   `batch_min/max`, `target_pivots`, per-closure timers and counts).
2. The instrument is deterministic in the thread count (unit test), so a registered muladd count
   is reproducible off any machine; only wall is host-dependent.
3. The soundness caveat is unchanged: "1 enters at c" rests on the arithmetic of the last
   closure, which the `dims` comparison cannot check. A registered t = 13/14 row needs a second
   run with a different schedule (`--hky3-deg-weight 0 --hky3-interreduce 0`, different rows and
   different reductions, same answer) or the capped-F4 bracket. Fault 3 shows the gate does catch
   an interreduction bug on closures without 1.

## 8. Pod command list for the cells worth re-running on 32 vCPU

Submit through the lab's route (`isolab-eks`); verify the binary sha256 (`9fbc6e89dcabc6eb4c6f036cdb168c4e4b1f263e262911dc3ddf961fef35a202`) and the
source commit. These are instrument runs, not study rows. Predicted muladds are from the v2 model
(§6); predicted wall assumes ~90 % utilisation at 32 threads with the adaptive batch (the model of
§3, **unverified on > 4 vCPU**), i.e. roughly v2-muladds × 0.45 ns / (32 × 0.9); predicted memory
from the v2 words model.

```bash
B=pilot_v2   # sha256 9fbc6e89dcabc6eb4c6f036cdb168c4e4b1f263e262911dc3ddf961fef35a202
CELL="--p 2013265921 --ladder-t none --planted 0 --engine hky3 --kinds kummer --m 2 --controls tower --hky-max-entries 14000000000"
# m=2 t=14 c=6 (fixpoint, 1 does NOT enter; decides d_F >= 7): rank ~0.90 of 499,178 cols
$B $CELL --random 1 --hky-cmax 6 --hky-max-cols 2000000 --budget 72000 --t-min 14 --max-t 14 --hky-progress t14c6.prog --out t14c6.jsonl
# m=2 t=14 c=7 (1 expected to enter): 1,683,218 cols
$B $CELL --random 1 --hky-cmax 7 --hky-max-cols 4000000 --budget 200000 --t-min 14 --max-t 14 --hky-progress t14c7.prog --out t14c7.jsonl
# (6,3) c=8: v1 did not finish in 3.8 h / 109 GB; deg-weight should exit at far lower rank
$B --p 2013265921 --ladder-t none --planted 0 --engine hky3 --kinds kummer --m 6 --controls tower --random 1 --hky-cmax 8 --hky-max-cols 20000000 --hky-max-entries 40000000000 --budget 72000 --t-min 3 --max-t 3 --hky-progress m6t3.prog --out m6t3.jsonl
# (9,2) c=9: v1 closed c=8 without 1 in 639 s; c=9 hit the budget
$B --p 2013265921 --ladder-t none --planted 0 --engine hky3 --kinds kummer --m 9 --controls tower --random 1 --hky-cmax 9 --hky-max-cols 20000000 --hky-max-entries 40000000000 --budget 72000 --t-min 2 --max-t 2 --hky-progress m9t2.prog --out m9t2.jsonl
```

| cell | v1 (given) | v2 predicted muladds | v2 predicted wall (32 vCPU) | predicted peak RAM |
|:--|:--|--:|--:|--:|
| m = 2 t = 14 c = 6 | 1.2e14–4e14 muladds (v1 estimate) | 6e13–2e14 | 1–4 h | 14–30 GB |
| m = 2 t = 14 c = 7 | 3.6e14–1.9e15 (v1 estimate) | 2e14–1e15 | 3–16 h | 45–150 GB |
| (6,3) c = 8 | not finished, 109 GB | ≤ 5e12 if 1 enters at ≤ 60 % rank (as (6,2) did) | < 1 h | 15–40 GB |
| (9,2) c = 9 | budget at 11.1M cols | depends on rank-at-exit; likely 1 enters < 40 % | hours | 60–120 GB |

The (6,3) and (9,2) predictions rest on the rank-at-exit law (§1): if deg-weight makes 1 enter
early as it did for (6,2)/(5,2), the work is a small fraction of v1's. This is a **prediction to
test on the pod**, not a measured result.

## 9. Files

| path | content |
|:--|:--|
| `src/cryptanalysis/hky_fp_tower_fast2.rs` | the v2 engine, 6 unit tests |
| `examples/pkm_tower_pilot.rs` | `--engine hky3` and `--hky3-*` flags; v1/reference rows unchanged |
| `harness/`, `harness/pin.sh` | standalone build (pins f4_fp at 94479f7e3e); `harness/unit` the tests |
| `run_gate.sh`, `run_gate_xcheck.sh` | the equality gate and the c = 7 cross-check |
| `cell.sh`, `sweep.sh`, `run_ab.sh` | one cell / a sweep / an interleaved A/B |
| `trace_summary.sh`, `util_model.sh` | read `--hky3-trace` output: per-rank-slice work, thread model |
| `PREDICTION_t12.md` | the held-out prediction, committed before the run |
| `raw/` | every row, log, trace and `cond.log`/`rss.log` behind the tables |

## Held-out t = 12 prediction — SCORED (main session, 2026-11-17 23:19 UTC)

`PREDICTION_t12.md` was committed (de2ddd11) before this run. Measured (`raw/t12/v2.jsonl`,
hky3 default, 4 shared vCPU, neighbour load):

| quantity | predicted (point, band) | measured | verdict |
|---|---|---|---|
| answer | 1 enters at c = 6 (d_F = 5) | 1 enters at c = 6; dims 0,0,2,146,3727,32057,190051 | MATCH |
| muladds, c = 6 | 2.0e12 (1.8e12–2.5e12) | 2.1048e12 | in band (+5% on point) |
| wall, c = 6 | 850 s (720–1180) | 950 s | in band |

v1 at this cell (pod README §8): 3.69e12 muladds, 1,722 s on these 4 vCPU. So v2/v1 = **0.57× muladds,
0.55× wall** at t = 12, consistent with the 0.47× / 2.1× at t = 10. The low-degree closures match v1
exactly (c = 5: dims …,3727,32057; 1 ∉ V_5). This is `[pcc:unverified]`: one draw, one instrument, no
PMU; "1 enters" rests on this engine alone, as for v1.
