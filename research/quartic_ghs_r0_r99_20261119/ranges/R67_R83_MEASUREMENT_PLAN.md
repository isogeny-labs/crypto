# R67–R83 execution-readiness and measurement plan

Status: prospective control and measurement contract. No command in a
`timing` block has been executed. The current GHS driver has no parallel-rho
CLI, and checkpoint/isolation admission is unavailable. Therefore no worker
scaling, latency, CPU, cache, or throughput observation is reported
`[pcc:unverified]`.

Range registry: [`../registry/R67_R83.json`](../registry/R67_R83.json).
Deterministic toy-screen receipt:
[`../receipts/r67_83.txt`](../receipts/r67_83.txt).
Plan-validation receipt:
[`../receipts/r67_83_plan_validation.txt`](../receipts/r67_83_plan_validation.txt).

## 1. Question and accounting boundary

The measurement question is deliberately split in two:

1. Does a production implementation of indexed, parallel distinguished-point
   GHS rho preserve the known answer, collision equations, transition
   identity, DP-table semantics, and complete work accounting?
2. On one admitted four-vCPU host, how do one, two, and four workers change
   cold one-target wall latency, total CPU, total/useful steps, and overshoot?

Parallel rho is a latency implementation. It is not credited with reducing
expected rho work. Every table reports wall latency beside total CPU and:

```text
total_steps = useful_steps + overshoot_steps
latency_speedup(p) = process_wall_1 / process_wall_p
parallel_efficiency(p) = process_wall_1 / (p * process_wall_p)
work_inflation(p) = total_steps_p / total_steps_1
overshoot_fraction(p) = overshoot_steps_p / total_steps_p
```

All ratios are derived only within one source/binary, curve/target, host,
placement, DP policy, shard policy, and interleaved measurement block.

The serial counted GHS arm remains the work reference for the matched IC/rho
frontier. A four-worker wall row is a separately labelled practicality row.
It does not replace the serial counted `S` value or change an IC/rho boundary
without a separately frozen protocol amendment.

## 2. Sources and current implementation audit

The plan is grounded in:

- `research/quartic_ghs_strong_rho_20261114/{PROTOCOL.md,ROUND_A.md,main.rs,strong_rho.rs,pod_run.sh}`;
- `research/quartic_ghs_strong_rho_20261114/round-c-integration/RESULTS.md`;
- `research/quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`,
  R6–R8 and M0–M4;
- `docs/POLLARD_COLLAB_DESIGN.md`, indexed walkers and shared-transition DP
  search;
- `docs/ecbench/README.md`, isolation and A/A admission; and
- `problems/ghs-strong-rho-measurement-admission.md`.

Current source inspection establishes the following implementation state only
`[pcc:unverified]`:

| Object | Present behavior | Consequence |
|:--|:--|:--|
| `main.rs` | `selftest` and serial `run` subcommands | there is no executable production parallel cell |
| `StrongOpts` | batch, 32 partitions by default, derived `dp_bits`, serial DP table | no worker/shard/queue/stop-policy identity |
| `StrongOut` / JSON | serial steps, DPs, cycles, field/native counts and `wall_s` | missing total CPU, useful/overshoot, per-worker and shard fields |
| `pod_run.sh` | three seed processes run concurrently | valid for the frozen serial grid, invalid for a within-pod worker-scaling comparison |
| Round A build context | binds the older pre-composition source hashes | must not be relabelled as a parallel or composed-source run |
| R67–R83 toy screen | deterministic scheduler/accounting assertions | useful precondition only; it is not the elliptic-curve walk |

The launch revision must recompute and freeze these hashes after composition:

```sh
git rev-parse HEAD
git status --porcelain=v1
sha256sum \
  research/quartic_ghs_strong_rho_20261114/main.rs \
  research/quartic_ghs_strong_rho_20261114/strong_rho.rs \
  research/quartic_ghs_strong_rho_20261114/pod_run_parallel.sh \
  research/quartic_ghs_r0_r99_20261119/ranges/R67_R83_MEASUREMENT_PLAN.md
```

A nonempty status or any hash different from the reviewed launch manifest is
a pre-launch stop. Inspection-time hashes are not copied into the protocol
because sibling lanes are still composing the source.

## 3. Implementation required before clocks

The following interfaces do not exist yet and are prerequisites, not commands
that may be treated as successful runs.

### 3.1 Non-timing production check

Add this exact CLI contract to the native Rust driver:

```text
ghs-strong-rho check-parallel
  --q Q --instance-seed S --target-index I --target-policy ic-repeat
  --arm ARM --batch-width W --partitions R
  --workers P --worker-start indexed-sha256-v1
  --dp-bits auto --dp-shards H --queue-depth D
  --stop-check-steps C --stop-delay-checks L
  --worker-cpus CPU_LIST --numa-node NODE
  --inject-tail ACTIVE/TOTAL
  --inject-collision-step STEP
  --output PATH
```

`check-parallel` must not read a clock or emit elapsed-time fields. It emits
one canonical JSON record plus a sorted DP transcript. Its exit status is
nonzero on a wrong answer, invalid point/DP, duplicate start index, transition
identity mismatch, dropped queue record, scalar-recovery failure, affinity
readback mismatch, phase-sum mismatch, or accounting-identity failure.

### 3.2 Admitted measurement command

After every deterministic control passes, add this native command:

```text
ghs-strong-rho run-parallel
  --q Q --instance-seed S --target-index I --target-policy sequence
  --arm ARM --batch-width W --partitions R
  --workers P --worker-start indexed-sha256-v1
  --dp-bits DP --dp-shards H --queue-depth D
  --stop-check-steps C --worker-cpus CPU_LIST --numa-node NODE
  --calibration CALIBRATION_JSON --run-label LABEL
  --output PATH
```

The process interval begins before curve/target reconstruction and worker,
jump-table, DP-table, and queue setup. It ends only after the collision is
checked, `k_found*G=Q` is independently checked, workers have stopped, every
queue is drained, final counters are reconciled, and the JSON record has been
flushed. Setup, online walk, stop propagation, recovery, verification, and
cleanup are also emitted separately.

The implementation must read each worker's actual affinity after thread
creation. Merely wrapping the process in `taskset` does not satisfy the
per-worker pinning gate.

### 3.3 Calibration reuse

The existing serial driver recalibrates inside every invocation. Scaling
requires one immutable calibration receipt per `(host,q,instance_seed)` so
all worker counts use identical `r_inv` and `r_lmul`:

```text
ghs-strong-rho calibrate
  --q Q --instance-seed S --triples 600
  --worker-cpus CPU_LIST --numa-node NODE
  --output CALIBRATION_JSON
```

The calibration file records every raw triple, median ratios, source/binary,
host capsule, pinning readback, and SHA-256. It cannot be reconstructed after
the primary calls.

### 3.4 Result-schema minimum

Every parallel record includes at least:

```text
job_id, source_revision, source_sha256, binary_sha256, compiler,
curve_id, q, ell, instance_seed, target_index, target_digest, k, k_found, ok,
arm, batch_width, partitions, dp_bits, workers, worker_cpu_map,
affinity_requested, affinity_observed, numa_requested, numa_observed,
worker_start_scheme, worker_start_digest, transition_digest, jump_digest,
dp_shards, queue_depth, stop_check_steps, stop_policy,
setup_wall_s, walk_wall_s, stop_wall_s, recovery_wall_s, verify_wall_s,
process_wall_s, total_cpu_s, per_worker_cpu_s,
useful_steps, overshoot_steps, total_steps, per_worker_total_steps,
dp_tests, dp_emitted, dp_inserted, dp_duplicates, sterile_collisions,
usable_collisions, rejected_records, queue_submitted, queue_processed,
queue_peak_depth, queue_wait_cpu_s, shard_occupancy, table_bytes, peak_rss,
field_counts_by_phase, native_counts_by_phase, calibration_sha256,
dp_transcript_sha256, recovered_scalar_check, exit_status
```

The validator refuses missing values rather than treating them as zero. PMU
fields may be explicit `null` when unavailable; core accounting fields may
not.

## 4. Deterministic pre-timing matrix

All rows below run through `check-parallel`. They produce no timing sample.
`q=223`, instance seed 1, target index 0 and `ic-repeat` are the existing
public deterministic control unless a row names otherwise.

| Row | Dimension | Frozen cells | Required equality or invariant | Stop condition |
|:--|:--|:--|:--|:--|
| D00 | build/selftest | release build; existing selftest | `SELFTEST OK`; source and binary manifest complete | any compile/selftest/hash failure |
| D01 | one-worker serial bridge | serial `strong-b8` vs parallel P=1, H=1 | target, answer, transition, steps, DPs, cycles and complete scalar counts match under the declared bridge policy | any non-timing field mismatch not preregistered as a schema-only addition |
| D02 | indexed starts | P=1,2,4; first 1,000,000 starts per P | no duplicate within a job; same `(job,P,index)` replays exactly | duplicate or replay mismatch |
| D03 | shared transition | P=1,2,4; 10,000 sealed states | every worker returns the same point/coefficient transition and jump id | any worker-local transition difference |
| D04 | masked tails | W=1,4,8,16,32; active lanes `0..W` | active lanes equal the scalar oracle exactly; inactive lanes are neither read nor charged | value/count mismatch or sanitizer failure |
| D05 | DP sharding | H=1,2,4,8 over one sealed DP transcript | sorted records, collision set and recovered scalar identical; one key has one owner shard | routing or result mismatch |
| D06 | bounded queues | depths 1,2,64; forced producer burst | `submitted=processed+queued`; no silent drop; final queues drain to zero | drop, unreconciled record or deadlock |
| D07 | stop accounting | injected collision step 256; delay checks 0,1,3,8 | answer identical and `total=useful+overshoot`; post-detection work increases only overshoot | missing work or changed answer |
| D08 | collision signs | same-y, opposite-y, sterile denominator and tampered record fixtures | valid equations recover/check k; sterile is counted; tamper rejected | invalid acceptance or valid rejection |
| D09 | known-answer panel | q=223, seeds 1,2,3; P=1,2,4; two exact replays | every run recovers/checks k; canonical non-timing record and DP digest replay for fixed P | wrong answer or replay mismatch |
| D10 | failure preservation | forced queue close, worker panic, invalid affinity, malformed DP | nonzero exit, partial record and exact reason retained; no success JSON | silent success or missing failure artifact |

### 4.1 Exact deterministic command templates

Current source checks:

```sh
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
study=$repo/research/quartic_ghs_strong_rho_20261114
out=$(mktemp -d /tmp/ghs-r67-r83-check.XXXXXX)
test -z "$(git status --porcelain=v1)"
GHS_SOURCE_REVISION=$(git rev-parse HEAD) \
  rustc --edition 2021 -C opt-level=3 "$study/main.rs" \
  -o "$out/ghs-strong-rho"
"$out/ghs-strong-rho" selftest >"$out/selftest.out" 2>&1
grep -qx 'SELFTEST OK' "$out/selftest.out"
sha256sum "$study/main.rs" "$study/strong_rho.rs" \
  "$out/ghs-strong-rho" >"$out/SHA256SUMS"
```

R67–R83 scheduler/accounting precondition:

```sh
rustc --edition 2021 -C opt-level=2 \
  research/quartic_ghs_r0_r99_20261119/prototypes/r67_83.rs \
  -o "$out/r67-r83-screen"
"$out/r67-r83-screen" | tee "$out/r67-r83-screen.out"
grep -qx 'timing_observations=0 performance_claims=0' \
  "$out/r67-r83-screen.out"
```

Prospective production D04 tail calls, after `check-parallel` exists:

```sh
for width in 1 4 8 16 32; do
  active=0
  while test "$active" -le "$width"; do
    "$out/ghs-strong-rho" check-parallel \
      --q 223 --instance-seed 1 --target-index 0 --target-policy ic-repeat \
      --arm strong-b8 --batch-width "$width" --partitions 32 \
      --workers 1 --worker-start indexed-sha256-v1 \
      --dp-bits auto --dp-shards 1 --queue-depth 64 \
      --stop-check-steps 64 --stop-delay-checks 0 \
      --worker-cpus "$GHS_CPUSET_P1" --numa-node "$GHS_NUMA_NODE" \
      --inject-tail "$active/$width" \
      --inject-collision-step 0 \
      --output "$out/tail-w${width}-a${active}.json"
    active=$((active + 1))
  done
done
```

Prospective D05–D07 calls:

```sh
for shards in 1 2 4 8; do
  "$out/ghs-strong-rho" check-parallel \
    --q 223 --instance-seed 1 --target-index 0 --target-policy ic-repeat \
    --arm strong-b8 --batch-width 8 --partitions 32 \
    --workers 4 --worker-start indexed-sha256-v1 \
    --dp-bits auto --dp-shards "$shards" --queue-depth 64 \
    --stop-check-steps 64 --stop-delay-checks 0 \
    --worker-cpus "$GHS_CPUSET_P4" --numa-node "$GHS_NUMA_NODE" \
    --inject-tail 8/8 --inject-collision-step 0 \
    --output "$out/shards-${shards}.json"
done

for delay in 0 1 3 8; do
  "$out/ghs-strong-rho" check-parallel \
    --q 223 --instance-seed 1 --target-index 0 --target-policy ic-repeat \
    --arm strong-b8 --batch-width 8 --partitions 32 \
    --workers 4 --worker-start indexed-sha256-v1 \
    --dp-bits auto --dp-shards 4 --queue-depth 64 \
    --stop-check-steps 64 --stop-delay-checks "$delay" \
    --worker-cpus "$GHS_CPUSET_P4" --numa-node "$GHS_NUMA_NODE" \
    --inject-tail 8/8 --inject-collision-step 256 \
    --output "$out/stop-delay-${delay}.json"
done
```

These templates intentionally fail against the current driver because the
parallel subcommands are prerequisites. They must not be replaced with the
serial `run` command and described as parallel coverage.

## 5. Admitted worker-scaling matrix

### 5.1 Development identities

The scaling-development instances are disjoint from the frozen matched-grid
seeds:

| Region | q | development instance seeds | target policy/index |
|:--|--:|:--|:--|
| low | 4,051 | 101,102,103 | `sequence`, index 0 |
| middle | 16,363 | 101,102,103 | `sequence`, index 0 |
| high development | 65,479 | 101,102,103 | `sequence`, index 0 |

The frozen matched-grid holdout keeps seeds 1,2,3 and `ic-repeat`. No
development result may select a parameter separately for a holdout curve.

### 5.2 Fixed algorithm and DP parameters

Before timing, one composed arm is named in the launch manifest with its
counted and counter-free spellings. The control remains `strong-b8` /
`fast-b8` unless R17–R66 composition has independently passed its promotion
gate. No candidate spelling is inferred from this plan.

For every worker count:

- batch width is 8 and partitions are 32;
- `dp_bits` is computed once by the current formula
  `floor(log2(max(sqrt(ell)/64,4)))`, recorded, then passed explicitly;
- DP shards are 4 and queue depth is 1,024;
- worker starts are `indexed-sha256-v1`;
- stop checks occur every 64 completed scalar steps per worker;
- the curve, target, jump/transition digests and calibration receipt are
  identical; and
- workers are pinned to distinct allowed physical cores on one NUMA node.

Shard count and queue depth are development constants here, not additional
post-result tuning dimensions. A later shard/queue study requires a new
frozen matrix.

### 5.3 Arms, order and sample count

Each `(q,seed)` has four labels:

| Label | Workers | Purpose |
|:--|--:|:--|
| `P1A` | 1 | scaling reference and first A/A arm |
| `P1B` | 1 | byte-identical A/A arm with a distinct position label |
| `P2` | 2 | two-worker latency/work cell |
| `P4` | 4 | four-worker latency/work cell |

Five interleaved blocks are frozen:

```text
block 0: P1A P1B P2  P4
block 1: P4  P2  P1B P1A
block 2: P1B P4  P1A P2
block 3: P2  P1A P4  P1B
block 4: P1A P2  P1B P4
```

This is 20 process invocations per `(q,seed)`, 60 per q, and 180 across the
three development q values. Seeds and configurations execute serially inside
one four-vCPU pod; the existing three-concurrent-seed pattern is forbidden for
this scaling round. The cost estimator must show the full 180-call round fits
the separately approved ceiling before launch. If it does not, the round is
blocked and must be prospectively amended; cells are not silently removed.

The A/A wall gate requires at least five admitted `P1A/P1B` pairs, a 95%
interval containing 1 and wholly inside `[0.9,1.1]`. Every wall cell must be
L2 or higher. Failure leaves wall ratios descriptive and does not trigger
more samples chosen after inspecting variance.

### 5.4 Exact admitted command template

An authorized operator supplies explicit CPU lists from the pod's allowed
cpuset. Example variable names are mandatory; their values are never guessed:

```sh
: "${GHS_CPUSET_P1:?one admitted CPU required}"
: "${GHS_CPUSET_P2:?two admitted same-node CPUs required}"
: "${GHS_CPUSET_P4:?four admitted same-node CPUs required}"
: "${GHS_NUMA_NODE:?NUMA node required}"
: "${GHS_COUNTED_ARM:?reviewed counted arm required}"
: "${GHS_FAST_ARM:?matching counter-free arm required}"
: "${BENCHREC:?open benchmark record required}"
```

For each `(q,seed)`, first create one calibration:

```sh
calibration="$out/calibration-q${q}-s${seed}.json"
"$binary" calibrate \
  --q "$q" --instance-seed "$seed" --triples 600 \
  --worker-cpus "$GHS_CPUSET_P1" --numa-node "$GHS_NUMA_NODE" \
  --output "$calibration"
```

Each frozen label expands to:

```sh
case "$label" in
  P1A|P1B) workers=1; cpus=$GHS_CPUSET_P1 ;;
  P2) workers=2; cpus=$GHS_CPUSET_P2 ;;
  P4) workers=4; cpus=$GHS_CPUSET_P4 ;;
  *) exit 2 ;;
esac

result="$out/q${q}-s${seed}-b${block}-${label}.json"
taskset --cpu-list "$cpus" \
  numactl --physcpubind="$cpus" --membind="$GHS_NUMA_NODE" \
  "$binary" run-parallel \
    --q "$q" --instance-seed "$seed" --target-index 0 \
    --target-policy sequence --arm "$GHS_FAST_ARM" \
    --batch-width 8 --partitions 32 \
    --workers "$workers" --worker-start indexed-sha256-v1 \
    --dp-bits "$dp_bits" --dp-shards 4 --queue-depth 1024 \
    --stop-check-steps 64 --worker-cpus "$cpus" \
    --numa-node "$GHS_NUMA_NODE" --calibration "$calibration" \
    --run-label "$label" --output "$result"
```

The pod runner is itself invoked through the repository benchmark-record and
IsoLab command-job path. The exact record path, immutable image digest,
write-once result URI and job manifest are frozen before submission. No timing
command is run directly from a development shell.

## 6. Pinning, isolation and host admission

All conditions are conjunctive:

1. `scripts/checkpoint.sh gate` passes for the pushed branch.
2. The authorized operator records AWS identity, `us-east-2`, daily/monthly
   budget telemetry, at least the approved unreserved headroom, active jobs,
   namespace quota, and all-tenant pool state.
3. `scripts/isolab_pool_check.sh --cpu 8 --memory 8Gi --pool cpu-spot`
   passes before each wave.
4. The image is selected by digest and contains the exact reviewed Git
   revision; the checkout is clean.
5. One pod requests 4 vCPU and 4 GiB. Scaling calls are serial. No compiler,
   validator, upload compressor, other seed, or other benchmark runs during a
   measured child.
6. The allowed cpuset contains four distinct usable CPUs on one NUMA node.
   The internal thread-to-CPU readback exactly matches the requested map.
7. Memory policy reads back as bound to that NUMA node. Missing NUMA tooling,
   failed binding, SMT conflict, or an affinity mismatch refuses wall
   admission rather than falling back silently.
8. Every measured child meets ECBench L2 or higher. Missing telemetry is
   recorded as missing; it is not converted to zero.
9. The benchmark record is opened before the first call, raw data is uploaded
   by SHA-256, failures receive records, and the progress report is regenerated.

PMU counters are optional separate observations. If available, collect cycles,
instructions, cache misses and branch misses in a distinct admitted pass so
the profiler does not perturb the primary wall interval. If unavailable, set
the fields to `null` and retain the failed probe.

## 7. Replay and known-answer controls

### 7.1 Per-run correctness

Every process must report `ok=true`, `k_found=k`, and an independent
`k_found*G=Q` check. DP records are independently checked as
`R=aP+bQ`. A collision is accepted only after trying the applicable sign
equations and checking the recovered scalar against Q.

### 7.2 Exact replay

For every deterministic pre-timing row and one adversarial row per measured
cell, run the same command twice after removing only these fields:

```text
timestamps, wall intervals, CPU intervals, scheduler counters, host load
```

The canonical comparison retains source/binary, curve/target, worker starts,
transition/jump digests, points, scalars, DP transcript digest, all operation
counts, useful/overshoot/total steps, queue/shard outcomes, answer and exit
status. Any mismatch blocks timing or invalidates the affected measured cell.

### 7.3 Shard independence

Feed one sealed, sorted DP-record stream to H=1,2,4,8 tables. After canonical
sorting, all four must have the same accepted/rejected records, coefficient
pairs per key, collision set and recovered answer. Per-shard occupancy may
differ and is recorded.

### 7.4 Worker-count interpretation

P=1,2,4 are not expected to have equal trajectory length because they start
different sets of trails and stop at different detection points. The required
invariants are common job/transition identity, disjoint deterministic starts,
valid DP records, checked answer, complete work accounting, and exact replay
within a fixed P. Cross-P equality is required only for the final recovered
scalar and mathematical target.

## 8. Stop, censor and continuation rules

| Event | Immediate action | Scientific disposition |
|:--|:--|:--|
| dirty tree, hash/revision/image mismatch | do not build or launch | preflight failure; no sample |
| checkpoint, budget, quota or pool check fails | do not launch | access/infrastructure blocker remains open |
| parallel CLI/schema field missing | do not time | implementation incomplete |
| affinity/NUMA readback mismatch | terminate cell and preserve record | wall observation inadmissible |
| wrong answer or failed `kG=Q` | stop the round | correctness failure; all output retained |
| invalid/tampered DP accepted | stop the round | soundness failure |
| duplicate worker start | stop the round | scheduler correctness failure |
| different transition/jump digest across workers | stop the round | invalid parallel collision domain |
| queue drop or reconciliation failure | stop the round | incomplete work/table accounting |
| `total != useful + overshoot` | stop the round | accounting failure |
| deterministic replay mismatch | stop before timing, or invalidate measured cell | reproducibility failure |
| A/A interval fails the frozen gate | finish already launched block; launch no later wave | wall results descriptive only |
| L2 isolation fails for a pair | retain pair, exclude from wall estimator | insufficient admitted pairs means descriptive only |
| timeout/deadline | preserve partial output; no algorithmic inference | censored observation |
| Spot eviction/upload failure | preserve infrastructure receipt | only the frozen manual infrastructure retry may apply |
| estimated all-attempt cost exceeds ceiling | do not launch | amend prospectively or keep blocked |
| rho-step mean outside `[0.75,1.15]` on matched grid | continue frozen grid and display row | size excluded from IC/rho conclusion |

No failure changes workers, shards, queue depth, DP bits, sample count, target,
or ordering after output has been inspected.

## 9. Analysis and report tables

The worker-scaling report contains, per `(q,seed,P)` and pooled only where the
protocol permits:

- successful, failed, censored, L2-admitted and excluded attempts
  `[pcc:unverified]`;
- process and phase wall intervals with 95% paired intervals;
- total and per-worker CPU;
- useful, overshoot and total steps, with `steps/sqrt(ell)`;
- latency speedup, parallel efficiency, work inflation and overshoot fraction;
- DPs emitted/inserted/duplicated, sterile/usable collisions, table operations;
- per-shard occupancy/skew, table bytes, queue peak/wait CPU and peak RSS;
- A/A ratio and interval, isolation levels and every rejection reason; and
- all individual observations, not only medians.

The counted companion reports complete cold M-eq and the 0, 0.1 and 1 GAE
band. SIMD/vector instruction counts, if any, remain diagnostics; scalar
abstract operations remain the counted authority.

The report must state separately:

1. deterministic implementation status;
2. worker-scaling latency and resource observations;
3. total-work/overshoot behavior;
4. matched serial rho/IC boundary, if the distinct full grid is admitted; and
5. every unmeasured or blocked obligation.

## 10. Readiness ledger

| Requirement | Status at plan freeze | Evidence / unblocker |
|:--|:--|:--|
| R67–R83 registry and toy scheduler/accounting screen | producer diagnostic complete `[pcc:unverified]` | `R67_R83.json`, `r67_83.txt` |
| production indexed parallel walk | not implemented | add and review native scheduler/coordinator |
| `check-parallel`, `run-parallel`, immutable calibration CLI | not implemented | exact contracts in section 3 |
| serial-to-P1 bridge and known-answer corpus | not executed | D00–D10 after implementation |
| masked-tail production sanitizer panel | not executed | D04 |
| DP sharding/queue/stop fault panel | not executed | D05–D08, D10 |
| clean source/binary/image bindings | pending final composition | recompute launch manifest |
| checkpoint, budget, pool and isolation admission | blocked | existing measurement-admission problem |
| 180-call development scaling matrix | not launched | every prior row plus cost approval |
| matched q=4051..4194271 frontier grid | not launched | distinct frozen M3 protocol and validation chain |
| independent replay/validator/PCC standing | not started | sealed raw results required first |

This plan does not mark any unrun cell complete and does not schedule work in
the background. Its purpose is to make the next authorized implementation and
execution mechanically checkable.
