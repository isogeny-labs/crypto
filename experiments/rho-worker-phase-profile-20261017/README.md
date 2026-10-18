# Collaborative rho phase trace

The instrumented trace measures the existing `pollard_collab` worker's
secp256k1 branch construction, indexed walker starts, and walk steps. Each
walker's terminal record is replayed with the uninstrumented `run_walker`
function. `JobContext::new_profiled` builds the same branch points as
`JobContext::new` and records exclusive validation, branch-table, and
finalization intervals.

## Frozen diagnostic

The command is

```sh
cargo run --offline --release --example rho_worker_phase_profile -- 4 8192 12
```

It fixes `Q=7G`, 32 branches, seed `0x71cc20261017`, four walker indices
`0..3`, no negation map, `dp_bits=12`, and an 8,192-step cap. Fixture
construction, compilation, and independent replay are outside the reported
phase intervals. The program records all four outcomes and a transcript
digest; a mismatch with `run_walker` terminates before it writes a result.
The receipt is a **local diagnostic** from a contended macOS ARM64 host.
It does not qualify a Tier 1 speedup or an allocation-wide worker fraction.

The [source-bound local receipt](local-4x8192-dp12.json) records 32 branch
points in 58.998 s, four starts in 7.125 s, and 10,152 walk steps in
21.270 s. The four terminal records matched the uninstrumented worker.
Branch setup and starts were 75.6% of the profiled interval for this short,
capped trace. This fraction is tied to its four walkers and 12-bit
distinguished-point rule. An earlier same-input local run yielded the same
transcript digest but a 67.9% phase fraction, illustrating host timing noise.

The worker's production job default allows `dp_bits=20`. If the measured
per-start and per-step rates were held fixed, the setup-plus-start fraction
for 256 walkers each taking `2^20` steps would be approximately 0.09%:

`(B + W*S) / (B + W*S + W*L*C)`, where `B` is branch setup, `S` is one
start, `C` is one step, `W=256`, and `L=2^20`. This is a cost-model
extrapolation, not a production timing result. It motivates a qualified
longer trace and a walk-addition profile before selecting a joint-scalar
kernel for the worker.

A full-worker improvement above 1.05x requires the optimized phase to occupy
more than `1 - 1/1.05 = 4.76%` of baseline time even if that phase could be
removed entirely. The 0.09% projection is well below that threshold, but
its phase share still needs a qualified long-worker trace.

## Batched walk-addition candidate

The existing worker keeps affine points and performs a field inversion in
each ordinary walk addition. `walk::step_batch` advances independent walker
states synchronously: it collects each nonzero affine denominator, inverts
their product once, and recovers the individual inverses with prefix products.
The branch selector, coefficients, optional negation map, and exceptional
point additions follow the existing serial step. This reduces the ordinary
inversion count for a batch of `k` walks from `k` to one; it adds field
multiplications and requires a lockstep scheduler to use in the live worker.
Exact-state tests compare multiple rounds with `walk::step`, including
identity, doubling, cancellation, and both negation modes. A full-worker
throughput result requires a frozen host panel and exact trail replay before
changing scheduling or the default path.

`walk::run_walkers_batch` now supplies an opt-in lane-refill scheduler for a
consecutive walker range. It keeps completed outcomes in walker-index order,
checks the distinguished-point rule and step cap at the same point in each
trail as `run_walker`, and assigns the next walker when a lane becomes free.
The replay tests cover widths 1, 2, 7, 32, and 128, both negation modes,
short and longer step caps, and a secp256k1 job. `run_lane` now accepts an
opt-in `batch_width`; its default remains one. Widths above one require the
exact secp256k1 field, coefficients, subgroup order, and generator. Batched
ranges stop at check-in and walker-budget boundaries, and the lane test
compares ordered check-ins and counters with the serial lane. An external
stop request can discard already computed, unpublished walkers in the
current batch.
The [batch walker validation receipt](batch-walker-validation.json) binds its
pre-rebase source hash and complete release-suite log; 2,246 tests passed,
94 were ignored, and none failed.
The [live lane validation receipt](batch-lane-validation.json) binds the
opt-in worker source and both full-suite attempts. The first attempt was
interrupted after 2,247 passes while an unrelated random curve-search test
remained CPU-active; the retry passed 2,248 tests with 94 ignored. Neither
receipt is a throughput measurement.

The [local validation receipt](batch-step-local-validation.json) binds the
pre-rebase batch-step source hash and release test logs. The focused tests passed 2/2.
The first full-library run passed 2,242 tests and hit two loopback
`PermissionDenied` failures in existing network tests under sandboxing.
Those three network tests passed with loopback access, and a full rerun in
that environment passed 2,244 tests with 94 ignored.

## Current-main reconciliation

Current main changed the serial walk to use variable-time point arithmetic,
normalize before escaping a two-cycle, and select the smaller-x cycle member
for the escape. The batched addition now follows that order and uses the
context's precomputed `half_p` for start normalization. The earlier batch
validation receipts above remain tied to their original source hashes.

The [rebase validation receipt](rebased-validation.json) pins the current
rho source and every local test log. On a temporary combination of rho
commit `9069bd688b` and the existing
[main build repair PR #2135](https://github.com/aburan28/crypto/pull/2135)
at `37bcf08a`, all 44 collaborative rho release tests passed, including
the new batch state, outcome, and check-in replays. The temporary repair
merge is excluded from this rho branch's diff. The refreshed
[local profile](local-rebased-4x8192-dp12.json) independently replayed all
four walkers and reproduced the 10,152-step transcript digest. Its observed
setup/start share was 35.6% on the contended host; the previous local
receipt recorded 75.6% on the same inputs.

The [first combined-source full-suite log](rebased-full-censored.log) records
one `jv_cover` F4 replay failure under concurrent load and one long test
still active when the run was interrupted. That `jv_cover` test passed alone
in 36.74 seconds; see its [focused log](rebased-jv-replay.log). A clean
full-suite result on current main still depends on the build repair. A
four-thread retry passed 1,089 tests before it was stopped after the repair
branch advanced; its [partial log](rebased-full-fourthreads-censored.log)
is retained. No throughput result has been measured for the batch lane.

## Next gates

1. Rebuild this worker-profiler source on a qualified Linux allocation and
   freeze its job spec, 32 public Q points, CPU and memory envelope, host
   counters, and A/A controls. The requested source transfer to RunPod was
   rejected by automatic approval review pending explicit authorization.
2. The separate [rho arithmetic closeout in PR #920](https://github.com/aburan28/cryptanalysis/pull/920)
   measured complete target-charged one-restart times for radix-81,
   mixed-76, and generator-only-warmed OpenSSL across 32 Q points. Two full
   panels failed their A/A noise gates. Its separate reuse panel passed the
   Tier 1 gate at 32 and 128 restarts per Q; 1 and 8 failed A/A gates.
3. Run the frozen 32-Q full-worker Tier 1 panel below, preserving every
   pair and A/A control. Compare charged setup-plus-walk time on each Q.
   Require a 95% lower speedup bound above 1.05 and a frozen peak-memory
   cap before changing the default path. The generic C rho path has a
   64-bit group order; the full-size worker is this Rust implementation.

The default worker routing remains serial.

## Frozen full-worker Tier 1 panel

`rho_worker_batch_benchmark` reads the same 32 public secp256k1 Q points as
the [arithmetic closeout manifest](rho-closeout-q-manifest.json), SHA-256
`f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8`.
Each arm runs 32 walkers with 32 branches, `dp_bits=8`, a 2,048-step cap,
and a check-in every eight completed walkers. The serial arm uses batch width
one; the candidate uses width 32. Q decoding and known-scalar fixture replay
precede timing. The charged interval includes Q-dependent job and state
construction plus the complete lane run. Every distinguished-point record
is independently replayed afterward, and a time-normalized digest binds
all emitted check-ins.

On a Linux allocation, build a release example from the frozen source and
use `build_worker_batch_plan.py` to bind the binary, Q manifest, source
bundle, allocation receipt, and selected CPU before any arm runs. The
[Tier 1 runner](run_tier1_worker_batch.py) takes a host-wide lock and pins
one child at a time. Its 32-Q plan has four alternating repetitions of a
serial/batch pair and serial/serial and batch/batch A/A controls: 384 pairs,
768 arms. It retains stdout, stderr, topology, cgroup, CPU, and failure
receipts. The [independent auditor](analyze_tier1_worker_batch.py) requires
equal check-in digests and work counts, no host-noise failures, all A/A
ratios in `[0.90, 1.11]`, and no more than 10% or 64 MiB additional peak
RSS. The integration threshold is a Q-cluster bootstrap 95% lower paired
speedup bound above 1.05. The plan and auditor do not supply a performance
result until the hosted panel passes its gates.

The [local all-Q replay](local-all32-worker-replay/summary.json) completed
64 of 64 arms and 32 of 32 serial/batch exact-output pairs on these frozen
points. Every arm independently checked each emitted distinguished point;
the pair check includes the time-normalized ordered check-in digest, steps,
point count, worker count, dead trails, units, and check-ins. This was a
correctness run on the shared macOS host, so `performance_result` is null.
The [harness validation receipt](worker-batch-harness-validation.json)
records the binary and temporary upstream-repair commit used for this replay,
the 96-case plan expansion, reproducible provisional source archive, and
fail-closed non-Linux preflight. Rebuild and hash the archive on the final
merged source before dispatch; the provisional archive is not a hosted
measurement artifact.
