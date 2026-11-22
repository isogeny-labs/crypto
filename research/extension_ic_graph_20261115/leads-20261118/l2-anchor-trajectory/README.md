# L2: the exact-log class trajectory past rank readiness

Lane key `l2-anchor-trajectory`, 2026-11-19. Every number here is a producer
observation or a model output, labelled [pcc:unverified]. No pcc claim was
registered. No paid compute, Runpod, EKS, git operation or remote side effect
was used. The science is in Rust, built with the l1-steps-2-3 scoped build plus
one patched copy of Run50's collector. This is finite algebra with counted
bitops. It is not a timing result, and it measures no descent saving.

## Question and rule (`../scout-graph-ic/LEADS.md` §L2)

L2 says that collecting past rank readiness enlarges `D_exact`, the set of
exact-log quadratic classes. That raises `f = |D_exact| / |D_universe|` and,
under L1, lowers the expected descent. The question is whether the descent
falls by more than the extra sources cost.

The kill rule, quoted exactly:

> The lead dies if the measured marginal descent saving per added source falls
> below the measured marginal source cost at every checkpoint past rank
> readiness. It depends on L1.

The model, as written in LEADS.md L1/L2:

- `E_desc(f) = 2c / (π·m(f))`
- `m(f) = 1 + 15f + 45f² + 15f³`
- `c = 1.2765e9` bitops per residual, and `π = 9.0e-4`

`|D_universe| = 254,530` canonical classes, measured in l1-steps-2-3 step 2.

**How the rule is applied here.**

- A *checkpoint* is a complete batch of 1,024 B values. That is the collector's
  own checkpoint unit.
- *Marginal* means the difference from the previous checkpoint, divided by the
  sources added in between.
- The **cost side is measured**: the counted relations-phase bitops. No linear
  algebra runs after the stop. The linear-algebra bitops stay at
  443,496,216,906 at every later checkpoint.
- The **saving side cannot be measured** without a descent. It is the
  LEADS.md model evaluated at the measured `|D_exact|`. Every number combined
  this way is labelled **MODEL-ONLY** (critic F6).

## Method

- **`build/patch_l2.py`** copies Run50's 11c9a031
  `jv_extension_deferred_pair_dlp.rs` (sha256 `a3ad4923…`, manifest-checked)
  to `jv_l2_deferred_pair_dlp.rs` and makes three asserted, exact edits:
  1. Two statics. With `L2_EXTEND_PERMILLE = 0` the collector stops as Run50
     does.
  2. A per-batch record: B, sources, graph stats, counts, and the collection
     tally priced by phase. These only read state.
  3. At the first certified batch, a rank-stop snapshot. Collection then
     continues through complete batches until sources ≥ 2.000 × the
     rank-stop sources.

  Nothing else changes. `Tracker::due` is false once the kernel exists, so no
  further rank work runs.
- **`l2_collect.rs`** is l1-steps-2-3's `run50_deferred.rs` driver, with the
  same `JV_*` pinning, one-thread pool and Run50 options.
  `MAX_DESCENT_RESIDUALS = 1`, so the run has no descent.
  - It runs the control first and exits nonzero if any control fails.
  - It replays every anchor witness for integer incidence. The large part
    must equal exactly `+D`, and every other class must cancel.
  - `D_exact` at a checkpoint is the set of anchors whose activation source
    index is below that checkpoint's source count. Anchors are never removed.
  - It cross-checks the per-source activation count against the graph's own
    anchor count at every checkpoint.
- **`l2_model.rs`** combines the measured trajectory with the model.
  - Variant **A** is the model as written.
  - Variant **B** scales `f` by L1's measured class match rate: 482/6,269
    divided by 0.0806, a factor of 0.954.
  - Variant **C** divides `c` by 1.68, which is "with C1 applied" in
    LEADS.md L2.
  - It applies the rule, and as a robustness check it adds windows of 10
    checkpoints and the cumulative net.
  - It cross-checks the trajectory against
    `deferred-graph50/recorded49-graph-replay.json`.

## Commands

```
df -h /                                   # 13G free
S=/tmp/claude-0/-home-user/53beab17-35e6-5dd9-bf91-c8753a71ee81/scratchpad/l2
E=/home/user/crypto/research/extension_ic_graph_20261115
cd build && SCRATCH=$S OUT=$S/bin ./build.sh > $S/build.log   # RUSTUP_TOOLCHAIN=1.98, CARGO_TARGET_DIR=/home/user/cargo-target-round2, CARGO_INCREMENTAL=0, -j 1; ~1.5 min
cd $S
./bin/l2_collect $E/runpod-deferred-pair-20261117-50/public.json 389 2000 \
  $E/runpod-deferred-pair-20261117-50/run-study.json \
  $E/leads-20261118/l1-steps-2-3/runs/run50-control.json \
  $S/../l1-steps-2-3/runs/full.dexact.jsonl $S/runs-x2000          # 104 s wall, one core
./bin/l2_model runs-x2000/trajectory.json $E/deferred-graph50/recorded49-graph-replay.json model-x2000.json
```

L1's `full.dexact.jsonl` can be regenerated with l1-steps-2-3's commands. Its
SHA-256 is in `../l1-steps-2-3/runs/RAW.sha256`.

The large outputs stay outside Git, under `$S`:

| File | Size |
|---|---:|
| `raw.json` | 231 MB |
| `dexact.jsonl` | 60 MB |
| `trajectory.json` | 157 KB |
| `model-x2000.json` | — |

Their SHA-256 values are in `runs/RAW.sha256`. `raw.json` and
`trajectory.json` carry wall-clock fields, so only their scientific content is
reproducible.

## Control: Run50 is reproduced at the rank stop [pcc:unverified]

`runs/collection-control.json` compares the rank-stop snapshot with Run50's
recorded `pair-deferred` outcome in `runpod-deferred-pair-20261117-50/run-study.json`.
**All 14 checks are equal.**

| Check | Value |
|---|---|
| Counts (all fields) | equal |
| Graph stats (all 15) | equal |
| Sources | 90,877 |
| Projections | 518 |
| Screen calls | 251,537 |
| Screen refusals | 167,516 (= Run50's early refusals) |
| Split records | 84,021 (= Run50's slow split records) |
| Stop reason | equal |
| Rank check | B 155,648, 518 rows, rank 508, 443,496,216,906 bitops |
| Relations bitops | 2,042,699,070,612 |
| Factor-base bitops | 719,628,892 |
| Linear-algebra bitops at the stop | 443,496,216,906 (= the rank check) |
| Last checkpoint and cohort hash | `ee0de358…`, equal to l1-steps-2-3's |
| `D_exact` at the stop | 20,510 classes, the same key set as l1-steps-2-3's export |

Further checks:

- **Anchor witnesses.** All 134,830 of 134,830 replay with exact integer
  incidence, up to 2x.
- **Per-checkpoint anchor counts.** The cross-check has 0 mismatches over 303
  checkpoints.
- **Run49 replay.** At all 13 B values that the two trajectories share,
  sources, anchors and vertices equal `recorded49-graph-replay.json`, with 0
  mismatches. One example: 108,058 sources give 32,408 anchors in both.
- **No refusals.** Vertex, edge and witness refusals stay at 0 throughout.
  At the end there are 189,098 vertices, below the 262,144 cap. The largest
  emitted witness has 81 terms, below the cap of 256.

## Primary result: measured `|D_exact|` trajectory [pcc:unverified]

`runs/trajectory.csv` has all 303 checkpoints. The rows below are the first
checkpoints at or after each 0.1x step. Costs are counted relations-phase
bitops. There is no linear algebra after the stop.

| B | sources | × rank stop | **\|D_exact\|** | f | anchors/source since previous row | relations bitops (cum.) | **extra collection bitops since rank stop** |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 155,648 | 90,877 | 1.000 | **20,510** | 0.0806 | — | 2.0427e12 | 0 |
| 172,032 | 100,439 | 1.105 | **26,528** | 0.1042 | 0.629 | 2.2585e12 | 2.158e11 |
| 187,392 | 109,227 | 1.202 | **33,404** | 0.1312 | 0.782 | 2.4553e12 | 4.126e11 |
| 202,752 | 118,381 | 1.303 | **42,385** | 0.1665 | 0.981 | 2.6585e12 | 6.158e11 |
| 218,112 | 127,307 | 1.401 | **53,388** | 0.2098 | 1.233 | 2.8604e12 | 8.177e11 |
| 233,472 | 136,370 | 1.501 | **66,260** | 0.2603 | 1.420 | 3.0621e12 | 1.0194e12 |
| 248,832 | 145,495 | 1.601 | **80,573** | 0.3166 | 1.569 | 3.2655e12 | 1.2228e12 |
| 264,192 | 154,650 | 1.702 | **94,972** | 0.3731 | 1.573 | 3.4685e12 | 1.4258e12 |
| 279,552 | 163,887 | 1.803 | **109,663** | 0.4308 | 1.590 | 3.6748e12 | 1.6321e12 |
| 295,936 | 173,271 | 1.907 | **123,246** | 0.4842 | 1.447 | 3.8874e12 | 1.8447e12 |
| 310,272 | 181,822 | 2.001 | **134,830** | 0.5297 | 1.355 | 4.0792e12 | 2.0365e12 |

What the trajectory shows:

- **`|D_exact|` grows 6.57x while the sources double**, from 20,510 to 134,830.
  `f` goes from 0.081 to 0.530.
- **Marginal anchors per source rise, then fall.** They rise from 0.61 at the
  stop to about 1.6 at 1.6–1.8x, then fall to about 1.3 at 2x as the deferred
  backlog drains. Live deferred edges peak near 75k at about 114k sources and
  are 36k at the end.
- **The cost per source is flat**, about 2.24e7 bitops, with batch values
  between 2.10e7 and 2.43e7.
- **The pre-stop rates match LEADS.md.** Measured on this trajectory: 0.097 at
  598–9,983 sources, 0.428 at 77–87k, 0.556 at 87–96k, and 0.721 at 96–106k.
  LEADS.md gives 0.10, 0.43, 0.56 and 0.72.
- **Post-stop projections are included in the cost.** They grow from 518 to
  10,809 (Full events) and are part of the counted relations cost. A
  collector that does not need them would pay slightly less; this was not
  separated.

## Model-only combination [pcc:unverified, MODEL-ONLY]

Files: `runs/model-checkpoints.csv` (all 151 checkpoints after readiness) and
`runs/model-summary.json`.

| Windows of 10 checkpoints (sources) | anchors/source | measured cost per source | A: model saving per source | B | C |
|---|---:|---:|---:|---:|---:|
| 90,877 → 96,816 | 0.611 | 2.268e7 | 2.220e7 | 2.235e7 | 1.322e7 |
| 96,816 → 102,701 | 0.684 | 2.262e7 | 2.053e7 | 2.077e7 | 1.222e7 |
| 102,701 → 108,659 | 0.820 | 2.216e7 | 2.003e7 | 2.037e7 | 1.192e7 |
| 108,659 → 114,685 | 0.911 | 2.233e7 | 1.780e7 | 1.819e7 | 1.059e7 |
| 114,685 → 120,822 | 1.090 | 2.222e7 | 1.675e7 | 1.720e7 | 9.97e6 |
| 120,822 → 126,763 | 1.221 | 2.257e7 | 1.455e7 | 1.502e7 | 8.66e6 |
| 126,763 → 132,559 | 1.414 | 2.271e7 | 1.303e7 | 1.352e7 | 7.76e6 |
| 132,559 → 138,801 | 1.468 | 2.201e7 | 1.039e7 | 1.083e7 | 6.18e6 |
| 138,801 → 144,806 | 1.571 | 2.233e7 | 8.57e6 | 8.97e6 | 5.10e6 |
| 144,806 → 151,013 | 1.615 | 2.208e7 | 6.86e6 | 7.21e6 | 4.08e6 |
| 151,013 → 157,205 | 1.580 | 2.219e7 | 5.30e6 | 5.59e6 | 3.15e6 |
| 157,205 → 163,298 | 1.593 | 2.237e7 | 4.31e6 | 4.56e6 | 2.56e6 |
| 163,298 → 169,255 | 1.495 | 2.242e7 | 3.33e6 | 3.53e6 | 1.98e6 |
| 169,255 → 175,123 | 1.378 | 2.287e7 | 2.60e6 | 2.76e6 | 1.55e6 |
| 175,123 → 181,822 | 1.36 / 1.24 | 2.24e7 | 2.2e6 / 1.8e6 | 2.3e6 / 2.0e6 | 1.3e6 / 1.1e6 |

The last row joins the final full window (175,123 → 181,241) and the final
partial window (181,241 → 181,822). The values before the slash are for the
full window, the values after it for the partial one.

- **Single checkpoints with model saving ≥ measured cost.** Variants A and B
  have 8 of 151 each, all at or below 112,782 sources (1.24x). Variant C has 0.
- **Windows of 10 with saving ≥ cost.** A, B and C all have 0.
- **Cumulative model net** (descent saving minus extra collection bitops since
  the stop). It is negative at every checkpoint. For A it is
  −1.2e9 at the first checkpoint, −2.3e11 at 1.5x and **−1.02e12 at 2x**. At
  2x, B gives −1.00e12 and C gives −1.43e12.
- **Why the model saving falls.** The anchor rate rises up to about 1.6, but
  `E_desc ∝ 1/m(f)` saturates: it falls from 1.13e12 to 1.19e11. Each extra
  class is therefore worth less and less. At the stop the model's
  `dE/d|D|` is about −4.0e7 per class, and the measured 0.61 anchors per
  source give about 2.2e7 per source. The measured cost is 2.27e7 per source.
  This matches LEADS.md's "about break-even".

## Verdict (rule as written)

- **Variant A, the model as written: NOT KILLED.** At 8 of 151 checkpoints past
  rank readiness the model saving per added source is at least the measured
  cost per added source. The rule kills the lead only if saving < cost at
  *every* checkpoint, so it does not kill it. **This is a narrow,
  noise-driven survival.**
  - All 8 checkpoints lie within 1.24x of the stop.
  - Each comes from one batch of about 600 sources with a high anchor count.
  - Over every window of 10 checkpoints, saving < cost.
  - The cumulative model net is negative at every checkpoint.
  - Under the model, the best stop on this trajectory is therefore rank
    readiness itself, at net 0. Its best later checkpoint is −1.2e9.
- **Variant B** (`f` scaled by L1's match rate): also NOT KILLED, by the same
  8 checkpoints.
- **Variant C** (C1 trace replay applied, `c/1.68`): **KILL**. Saving < cost
  at all 151 checkpoints.
- **Model dependence.** The verdict rests on the model saving side. The rule's
  "measured marginal descent saving" was not measured; that needs a descent at
  each `D_exact`. The model also extrapolates `m(f)` from L1's only measured
  point, `f = 0.08`, up to `f = 0.53`.

In plain terms, the measured trajectory confirms L2's premise strongly:
`|D_exact|` grows superlinearly past the stop (6.6x for 2x sources). Under
L1's model, though, the value of each class falls faster than the class supply
grows. Continuing past rank readiness does not pay for a single target on
p1009, except in single-batch noise near the stop.

## Next step

- **A decisive rule needs no noise-driven passes.** Preregister the window
  size, or the cumulative-net criterion, before any rerun. The current rule's
  single-batch marginals let 8 noisy batches decide the outcome.
- **Measure the saving side at a few `D_exact` points.** Use the exported
  `dexact.jsonl` with its activation indices, at 1.0x, 1.2x and 1.5x. Run
  l1-steps-2-3's `partial_match` on its 30,000-residual stream to measure
  matched rows per solution as a function of `f`. That replaces the `m(f)`
  extrapolation, which matters most at `f > 0.1`, where nothing is measured.
  This costs minutes of local CPU.
- **Not evaluated: amortization over several targets on the same curve.** The
  extra collection is paid once, and the descent saving recurs per target. It
  is outside L2's single-target rule and would need its own preregistered
  protocol. At 2x the model saving is about 1.01e12 per target against
  2.04e12 extra collection. That is model-only and is not a claim.
- **L3 (small p)** is where `f` saturates fastest. This trajectory suggests
  that at p1009 the regime past the stop is saturation, not scarcity.

## Files

- `l2_collect.rs`: the collection driver with the controls.
- `l2_model.rs`: applies the model and the rule, and adds windows, the
  cumulative net and the Run49 cross-check.
- `build/`:
  - `build.sh` reuses `../l1-steps-2-3/build/{fetch.sh,assemble.py,Cargo.lock}`
    read-only.
  - `patch_l2.py` makes the one science change.
  - `Cargo.toml.in`.
  - `build.log` holds the source and binary SHA-256 values: patched module
    `9180edd0…`, `l2_collect` `c94abbb0…`, `l2_model` `9b417e36…`.
- `runs/`:
  - `collection-control.json`: the control, the rank-stop snapshot and the
    final state.
  - `trajectory.csv`: 303 checkpoints.
  - `model-checkpoints.csv`, `model-summary.json`.
  - `RAW.sha256`.

  Each file is under 64 KiB.

## Errors and deviations

- **Rebuild, not Run50's binary.** As in l1-steps-2-3, this is a
  Root41-equivalent rebuild. The exact reproduction of every recorded
  rank-stop number is the evidence that the behaviour matches.
- **Patched collector.** Collection past the stop needs a code change. The
  change is confined to the stop branch and to read-only per-batch records.
  With the default `L2_EXTEND_PERMILLE = 0` the stop is Run50's, and the
  control shows that the pre-stop path is unchanged.
- **One residual of descent.** The descent stage still evaluates 1 residual
  (`MAX_DESCENT_RESIDUALS = 1`). It does not enter the collection tally.
- **Model rebuilt between runs.** `l2_model` was rebuilt twice to add the
  windows and the Run49 cross-check. `l2_collect` was not changed between the
  run and the final build; its hash is `c94abbb0…` in both.
- **Python glue.** The compact CSV and JSON extracts in `runs/` were made with
  `python3 -I` (extraction only).
- **Not done.** No research-kb query was made, because no novelty or
  literature claim is made. There is no visual report or PDF and no
  `bench.record`, because there was no timing or benchmark run; the wall
  times are practicality only. This is left to the lane's PR owner.
