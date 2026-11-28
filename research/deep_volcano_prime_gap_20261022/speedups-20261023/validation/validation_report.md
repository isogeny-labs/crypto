# Validation: deep-volcano speed-ups (2026-10-23)

**Verdict: pass.** The receipts are admissible evidence, and every headline number recomputes from the committed raw files. Two sentences in the README are wrong, and the field-op tally has one small gap. None of the three changes a headline number. The speed-up is in wall time only; in counted cost (GAE) the new method is 2.5% worse. See "What may be claimed".

- Validator: an independent validator subagent. It did not write the evidence, code or sessions, and changed none of them.
- Date: 2026-10-23, 02:01–03:45 UTC.
- Outputs, all in this directory:
  - `replay-certificate.json`
  - `validator-report.json`
  - this file
  - the seven `*.verify.json` replay receipts

## Snapshot and binding

- **Evidence commit.** The evidence is committed in `049a0ac05`. HEAD `b67532210` (a merge of main) does not change the evidence directory.
- **Code under test.** HEAD does not change `src/cryptanalysis/ic_boundary.rs`, `src/cryptanalysis/ecbench/` or `src/cryptanalysis/prime_fast.rs` relative to `d81da47d5`.
- **Raw files.** All 37 raw files listed in `validator-report.json` are tracked, and each equals its blob at HEAD.
  - `MANIFEST.sha256` passes: 12/12.
  - gunzip of every `.gz` equals both the manifest and the `records_sha256` / `plan_sha256` in `session.json`.
- **Early work.** The full rho-lanes replay ran at 02:01Z, before `049a0ac05` existed. It read files whose hashes equal the committed blobs, so it binds to the commit.
- **Recording binaries.** The sessions were recorded by binaries built from a dirty `bec9df9e8` tree, which were not kept:
  - rho-lanes: `d9a6ca93…`;
  - rerun: `4652d200…`.

  The replays below use builds of the committed code instead:
  - `04fde7fe…`, built from `d81da47d5`;
  - `7bbab033…`, built from `b67532210`.

  So exact replays tie the recorded counts to committed code. Wall times cannot be replayed (see "Wall-time provenance").

## 1. Replay

| session | N (validator's choice) | binary | result |
|:--|--:|:--|:--|
| rho-lanes `ECBS1h45f937d2f95a` | all 1152 measured runs (`--replay-all`) | `04fde7fe` (d81da47d5) | 1152/1152 identical, 0 problems, 12/12 checks |
| rho-lanes | 50 | `7bbab033` (b67532210) | 50/50 identical |
| `spec-shard0` `ECBS1h14d98bf2a3ec` | 50 | `7bbab033` | 50/50 |
| `spec-shard1` `ECBS1h846ee50776a9` | 50 | `7bbab033` | 50/50 |
| `spec-shard2` `ECBS1hd4816b5f840b` | 50 | `7bbab033` | 50/50 |
| `spec-aa` `ECBS1h0246a995e87e` | 50 | `7bbab033` | 50/50 |
| `spec-bsgs` `ECBS1hd719e0366969` | 50 | `7bbab033` | 50/50 |

- **What "identical" covers.** It means the same recovered value, `total_gae`, phases, counters and `field_ops`. Each replay re-executes the run in process, with its recorded algorithm seed.
- **Rerun coverage.** The rerun records equal the original `run/sessions` records (§4). The 2026-10-22 validator replayed those records in full with binary `a59c4014`.

## 2. Scalars

- **rho-lanes records.** All 1536 records (1152 measured, 384 warmup) are `verified`, with `matches_target` and `matches_planted` true, `recovered == workload.planted`, and `deterministic`.
- **Rerun records.** The same holds for all 6840 rerun records.
- **Independent bc check.** I wrote my own affine elliptic-curve code in GNU bc and self-tested it on F_97. On all 192 rho-lanes workload targets:
  - G and the target are on the curve;
  - [r]G = O;
  - [planted]G = target.
- **Negative control.** [planted+1]G ≠ target on 192/192, so the bc check can fail.
- **External rows.** All 576 rows of `external_ca.jsonl` have `status ok`, `alg rho`, `threads 1` and `x == planted`, with seeds 1, 2 and 3 for each of the 192 workloads.
- **External inputs.** `external_ca.inputs.tsv` is byte-identical to the distinct (workload, curve, target, planted) rows of all 1536 session records.

## 3. Independent recomputation

Method: jq, awk (mawk) and bc over the raw files. I did not run `summarize.sh` and did not read `summary.json` for values. Medians follow `summarize.sh`: the lower middle of the sorted values for an even count. The wall median follows `ecbench compare`, which interpolates.

| quantity | claimed (`summary.json`) | recomputed |
|:--|--:|--:|
| serial: runs / verified | 576 / 576 | 576 / 576 |
| serial: median ns per GAE | 293.862446685379 | 293.86244668537898 |
| serial: median solve | 362.088438 ms | 362.088438 ms |
| serial: mean S | 0.9303300895824488 | 0.93033008958244878 |
| lanes: runs / verified | 576 / 576 | 576 / 576 |
| lanes: median ns per GAE | 119.4361019557163 | 119.4361019557163 |
| lanes: median solve | 154.595293 ms | 154.595293 ms |
| lanes: mean S | 0.9538025695307257 | 0.95380256953072573 |
| lanes / serial, median ns per GAE | 0.40643540303599734 | 0.40643540303599734 |
| `ecbench compare`: ops ratio | 1.0252302706438448 | 1.0252302706438443 (summation order, 4e-16) |
| `ecbench compare`: wall median ratio | 0.41931135653769125 | 0.41931135653769125 |
| `ca`: median ns per op | 45.43872041472215 | 45.438720414722148 |
| `ca`: median solve | 58.426 ms | 58.426 ms |
| `ca`: mean ops/√r | 0.9763254783434467 | 0.97632547834344674 |
| plan: before / after | 96.644090596 s / 0.308205664 s | the same |
| kernel set-up, 1 CPU: before / after | 0.154 s / 0.047 s | the same |
| map, 1 CPU: before / after | 0.0186 s / 0.0188 s | the same |
| map, 4 CPUs: before / after | 0.0186 s / 0.0063 s | the same |
| construct, whole, 4 CPUs: before / after | 0.884893836 s / 0.282309114 s | the same |
| session span: before / after | 2119.831 s / 733.946 s | the same |

- **Agreement.** Every value is the same double as the claim, except the ops ratio (relative difference 4.3e-16). All rows are within rel_tol 1e-9.
- **Interpolated medians** (the mean of the two middle values) would give:
  - serial 293.8687 and lanes 119.4456 ns/GAE;
  - external 45.4414 ns/op and 58.442 ms;
  - differences of at most 3e-4 relative.
- **Logged timings.** The construct timings are the producer's logged values: `construct_s` to 1 ms and `eval_s_per_point` to 0.1 ms.

Ledger checks on every record:
- `total_gae` equals setup additions + setup doublings + start additions + walk operations + verification operations (1152/1152).
- `walk_operations` equals steps + look-ahead additions + escape doublings (1152/1152).
- For the lanes arm, steps + look-ahead additions equals 32 × `lane_rounds` (768/768, warmups included). Every round therefore charges all 32 additions, including the final round.

§5a band, recomputed. band.json's 96 per-curve rows reproduce to 3.9e-16 relative.

| weight w | S, serial | S, lanes | lanes / serial |
|--:|--:|--:|--:|
| 0 | 0.930330 | 0.953803 | 1.025230 |
| 0.1 | 1.023332 | 1.049151 | 1.025230 |
| 1 | 1.860350 | 1.907284 | 1.025228 |

The order holds at every weight. `band.json` ranks rho-neg cheaper on 48/48 curves.

## 4. Rerun identity

I re-derived this without running `compare.sh`.

- **Key.** I compared the records as multisets under a 17-field key:
  - arm, `method_id`, `method_sha256`;
  - `workload_id`, `workload_sha256`, `target_index`, planted;
  - round, warmup, `algorithm_seed`;
  - status, recovered, `total_gae`, `cost.s`;
  - counters (key-sorted), phases, `field_ops`.
- **Result.** The 6840 original records (gunzip of `run/sessions/{spec,spec-aa,spec-bsgs}/records.jsonl.gz`) and the 6840 rerun records are identical.
- **Per arm.** Both sides have 4560 `rho-neg`, 1520 `rho-neg-aa` and 760 `bsgs-neg` records, all non-warmup.
- **The script's own key.** Using `compare.sh`'s 8-field key, which I computed myself, both digests equal `compare.sha256` (`7bedc5cc…`).
- **Plans.** `spec-aa` and `spec-bsgs` have the original `plan_sha256`.

## 5. Code review: `rho_walk_lanes`

**Walk rules.** Each lane follows `TunedWalker::walk`'s rules.
- **Copied verbatim:** the set-up, the RNG draws, the jump table, the stride and its identity redraw.
- **Same order, same state for each lane:**
  - the start canonicalisation;
  - the look-ahead rule (`flipped && tried < jumps && !identity && index(y) == j`);
  - loss at the identity;
  - window cycle detection, with escape from the minimum-key member by doubling, and abandonment on a repeated escape point;
  - the push to the window;
  - the DP test before the cap test (`len > walk_cap`).
- **State carried:** `recent`, `last_escape`, `len`, `j0` and `tried`, held across rounds.
- **Starts:** `[k]T` is handed out in order by `next_walk`, under the serial loop's guard.

**Differences from the serial walk.**
- The per-step budget test becomes a per-round test. A run can overshoot `max_steps` by fewer than 32 additions; they are charged. No run came near the budget.
- DPs reach the table in a different order. This is intended.

**Charging.**
- `native_lane_walk_adder` adds 1 to `ops.adds` for every request before it computes anything. So every addition of the final round is in `total_gae`, including those after the recovering lane.
- The `steps` counter is topped up to match the charge (`requests.len() - k - 1`).
- A lane with equal abscissae leaves the batch and is charged its own inversion.
- I found no path by which the method reports fewer GAE than the group operations it performed.

**The predicted negative case holds.** The lanes must cost more than the serial walk, by about (lanes − 1) · 2^dp_bits / √r = 31 · 1024 / 1372297.6 = 0.0231 in S. These are the walks still in flight when the collision lands. The measured difference is +0.0235, inside the compare interval [0.0198, 0.0274]. This was tested at one lane count only.

**Unit test.** `cargo test --release --lib lane_rho` passes.
- With 16 lanes, the test checks that the walk recovers, replays and closes its ledger.
- With 1 lane, it checks that the result equals `rho_walk_with` in gae, recovered value and every counter, on 24 seeds.
- It does not compare a multi-lane walk with the serial walks from the same starts. That test is missing.

**Field tally: a minor defect.**
- The doc comment says the tally includes "the conversions to and from Montgomery form". The to-Montgomery conversions are tallied as 2 muls each.
- The from-Montgomery reductions, 2 per lane output (about 2 per GAE), go to `GroupOps.montgomery_reductions`. Neither the method's counters nor the ecbench record exports that field (`field_ops` carries muls, sqrs and invs only). So the recorded lanes tally omits them.
- The batch inversion is charged 3k + 1 multiplications, where 3(k − 1) are needed: a slight over-count.
- GAE is unaffected by any of this.

**Counter semantics.**
- `walks_capped` is 132 for lanes against 216 for serial.
- A capped walk takes 20544 steps, about half of a run's roughly 41k rounds. In the lanes method it is often still in flight when the collision lands, so it is never counted as capped.
- Its steps are charged. Do not compare this counter across the two arms.

**Attribution.**
- The serial arm adds through `PrimeCurve::add`: `CountedFp`, one inversion per addition.
- The lanes arm adds through `FastPrimeCurve` in Montgomery form, with one batch inversion per round.
- The 2.46× per GAE is the two changes together. No arm isolates the batching.

**Fallback.** For more than 16 jumps the method silently falls back to `rho_walk_with`. It cannot trigger, because `rho_jumps_for` never returns more than 16.

## 6. External comparison (`ca solve --alg rho --threads 1`)

**Established:**
- `ca` ran on this machine after the session: `external_ca.jsonl` was written 01:30:04–01:30:44Z, and the session ended at 01:29:02Z.
- The script pins it with `taskset -c 2`.
- Every output reports `threads 1`.
- The repository is clean at `1aa48b5`. The build is Release `-O3`, with `CA_NATIVE` off.
- `ca`'s timer covers the whole solve call (membership check, set-up and walk), which is the same scope as `solve_wall_ns`.

**Not established:**
- That `ca` ran alone on the host, as the run report says: no affinity, load or quiet check was recorded.
- Which binary ran: its sha256 is not in the evidence. The binary there now is `12e69faf…`, built 2026-10-22T08:57Z.

**Whether `ca`'s `ops` count matches GAE.** The convention is the same: additions plus doublings, one each. The differences:
- **What it counts.** `ca` counts 1 per batched walk addition (`ops += W` per round, retries included), and 1 per escape step and doubling.
- **Formula-priced work.** It prices each a·G + b·H at double-and-add counts, although it computes them with fixed-base tables. These are its 512 table multipliers and its 256 walk starts. That is about 93k of its mean 1.34M ops, roughly 7% (an estimate from the source, not measured).
- **Bias.** This biases `ca`'s ns/op low and its ops/√r high. `ca` charges no verification.
- **A different walk.** `ca` runs 256 walks, 512 multipliers and dp_bits 7, against 32 lanes, 16 jumps and dp_bits 10.
- **Consequence.** The per-solve gap (2.65×) does not depend on the op count. The per-op gap (2.63×) is approximate.

## Discrepancies

1. **README §3 and run report `run-20261023T0121Z`.** They say "median wall ratio 0.4193 [0.4173, 0.4214], over 306 admitted pairs".
   - The comparison's required level is L0, with 0 pairs below it, so the median and interval are over all 576 pairs.
   - 306 is the quality block's L2 count.
   - Over the 306 L2–L2 pairs, the median is 0.4179.
2. **README §3.** It says "Both arms run the same walk, so the band does not rank them". `band.json` ranks `rho-neg < rho-neg-lanes` on 48/48 curves, and the order holds at every weight.
3. **`rho_walk_lanes` doc comment.** The field tally does not include the from-Montgomery reductions (§5).

No headline number disagrees with its recomputation.

## Caveats and limitations

- **Wall-time provenance.** The session's wall times come from `d9a6ca93…`, which was not kept. I re-timed with the `b67532210` build: 5 curves × 4 targets × 2 rounds = 40 pairs. This is a sanity check, not evidence. It gave:
  - 294.6 and 116.7 ns/GAE, and a median wall ratio of 0.415;
  - counted costs equal to the session's records on all 80 runs.
- **Isolation.** Pairs were at L1 or L2. There was no in-session A/A, and contention was observed on all 1152 runs. Still, lanes was faster in every one of the 576 pairs (worst ratio 0.973), so noise cannot explain the direction.
- **Timing provenance.**
  - The before and after binaries for `timing.sh` exist only in the session scratchpad under `/tmp`. Their hashes match the jsonl rows.
  - I did not rebuild them from the commits the README names: disk space was too tight.
  - The timing rows carry no timestamps or load records.
  - Spot re-runs of the same binaries agree: plan 100.36 s against 0.295 s, kernel 0.143 s against 0.042 s, with identical outputs.
- **Session span.** The 2.89× is wall-clock span, not faster computation.
  - Total solve time rose, from 2738 s to 2988 s.
  - The gain comes from running the 4560-run main spec as 3 parallel shards: the before span is essentially one 2005 s session.
  - It also includes the old spec session's planning delay of about 115 s (`started_unix_ms` is taken after planning).
- **Whole construct on 1 CPU.** It went from 0.898 s to 0.721 s (1.25×). `summary.json` does not report this. Its 3.13× row is on 4 CPUs, so it includes the map's threading.
- **Scope.**
  - One 4-vCPU KVM VM (Intel Xeon @ 2.10 GHz).
  - One curve class: p of 52 bits, r ≈ 2^40.8, A = 2.
  - 48 curve models, 4 shared planted scalars, 32 lanes.

  Nothing transfers to other sizes, lane counts or hosts without measurement.

## What may be claimed

**May be claimed:**
- On this host, class and configuration, `rho.negation_lanes` solves in 0.419× the paired median wall time of `rho.negation`.
  - That is 2.38× faster per solve, and 2.46× faster per counted operation.
  - The walk law is the same, and the counted cost is 2.5% higher.
- The sharded re-run reproduces all 6840 records exactly and finishes the measurement phase in 734 s instead of 2120 s, by parallelism.
- `ecbench plan` is 314× faster with byte-identical output.
- The N-isogeny kernel set-up is 3.28× faster on 1 CPU, with an identical isogeny.

**May not be claimed:**
- Any improvement in counted cost (GAE). The lanes method is worse on 48/48 curves at every band weight.
- Any index-calculus improvement or IC leaderboard progress. This speeds up the rho comparator, not the IC baseline, so it is at most a component speed-up of rho.
- `rho.negation_lanes` as the GAE rho reference. `rho.negation` stays cheaper in GAE.
- A wall-clock IC-against-rho distance measured against ecbench's rho. The strongest rho at hand is now `ca`, 2.6× faster than the lanes method, so such a distance would flatter IC.
- The sessions-span and 4-CPU construct rows as less work. They come from parallelism.

**For a `VALIDATION.json` rho reference** on the lanes row:
- kind `negation` (A = 2; the only symmetry this class offers);
- reference `rho-neg` (`rho.negation`), in the same session and binary, in GAE;
- band: w = 0 → 0.9303 vs 0.9538; w = 0.1 → 1.0233 vs 1.0492; w = 1 → 1.8604 vs 1.9073;
- `order_holds_across_band`: true, with serial cheaper.

## validation_report

```yaml
validation_report:
  id: VAL-20261023-volcano-gap-speedups
  task_id: coordinator request (no TASK card id given)
  run_ids: [ECBS1h45f937d2f95a, ECBS1h14d98bf2a3ec, ECBS1h846ee50776a9, ECBS1hd4816b5f840b, ECBS1h0246a995e87e, ECBS1hd719e0366969]
  artifact_checks:
    - 37 raw files tracked and equal to HEAD b67532210 blobs; MANIFEST 12/12; gz == manifest == session.json hashes
    - session binaries d9a6ca93 / 4652d200 (dirty bec9df9e8) not preserved; replays with committed builds 04fde7fe / 7bbab033 exact
  metric_recomputations:
    - 36 headline values recomputed with jq/awk; all within 1e-9 (35 bit-identical, ops ratio 4.3e-16)
    - section 5a band recomputed (96 rows, <= 3.9e-16)
  control_checks:
    - bc negative control [planted+1]G != target 192/192
    - predicted GAE excess (lanes-1)*2^10/sqrt(r) = 0.0231 vs measured 0.0235
    - lane-round invariant 768/768; ledger identity 1152/1152
    - rerun identity, 17-field key, 6840/6840
  heuristic_validation_checks: []
  cost_model_checks:
    - unit declared (ns per GAE; ca: own op count, formula-priced fixed-base mults ~7%)
    - field tally omits from-Montgomery reductions (diagnostic; GAE unaffected)
  proof_architecture_checks: []
  verdict: passed
  limitations:
    - wall times from unpreserved dirty-tree binary (validator re-timing consistent)
    - timing before/after binaries only in /tmp; builds not reproduced
    - ca binary hash and host load not recorded
    - README misstates wall pairs (576, not 306) and the band ranking
  artifact_paths:
    - research/deep_volcano_prime_gap_20261022/speedups-20261023/validation/replay-certificate.json
    - research/deep_volcano_prime_gap_20261022/speedups-20261023/validation/validator-report.json
    - research/deep_volcano_prime_gap_20261022/speedups-20261023/validation/*.verify.json
```

**Sources read:**
- `docs/VALIDATION_GATE.md`;
- the prior study's `validation/*.json`;
- every file in `speedups-20261023/`, apart from figures, plot and PDFs;
- `run/sessions/*/session.json` and `records.jsonl.gz`;
- in `src/cryptanalysis/`:
  - `ic_boundary.rs`: `TunedWalker`, `rho_walk_with`, `rho_walk_lanes`, `PrimeCurve::add_raw` and the lane adder, `GroupOps`, the test;
  - `ecbench/methods.rs`;
  - `ecbench/audit.rs`: replay;
  - `ecbench/compare.rs`: ratio and wall median;
  - `ecbench/stats.rs`: quantile;
  - `ecbench/runner.rs`: where `started_unix_ms` is set;
  - `prime_fast.rs`: `add_many_batched`;
- `scripts/validation_gate.py`: the check functions;
- `/home/user/cryptanalysis/src/rho.c` and `ca_internal.h`.

**Not read:** any other reviewer's report.
