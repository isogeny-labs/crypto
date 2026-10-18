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
short and longer step caps, and a secp256k1 job. The production `run_lane` still calls the serial
walker; the next integration gate must establish check-in and stop behavior
when batched ranges are used there.
The [batch walker validation receipt](batch-walker-validation.json) binds the
final source hash and complete release-suite log; 2,246 tests passed, 94
were ignored, and none failed.

The [local validation receipt](batch-step-local-validation.json) binds the
batch-step source hash and release test logs. The focused tests passed 2/2.
The first full-library run passed 2,242 tests and hit two loopback
`PermissionDenied` failures in existing network tests under sandboxing.
Those three network tests passed with loopback access, and a full rerun in
that environment passed 2,244 tests with 94 ignored.

## Next gates

1. Rebuild this worker-profiler source on a qualified Linux allocation and
   freeze its job spec, two public Q points, CPU and memory envelope, host
   counters, and A/A controls. The requested source transfer to RunPod was
   rejected by automatic approval review pending explicit authorization.
2. The separate [rho arithmetic closeout in PR #920](https://github.com/aburan28/cryptanalysis/pull/920)
   measured complete target-charged one-restart times for radix-81,
   mixed-76, and generator-only-warmed OpenSSL across 32 Q points. Two full
   panels failed their A/A noise gates. Its separate reuse panel passed the
   Tier 1 gate at 32 and 128 restarts per Q; 1 and 8 failed A/A gates.
3. Connect the opt-in lane-refill scheduler to `run_lane`, preserve check-in
   and stop behavior, and replay each walker's entire affine state sequence
   against the serial worker. Compare paired full-worker throughput on the same Q points.
   Require a 95% lower speedup bound above 1.05 and a frozen peak-memory
   cap before changing the default path. The generic C rho path has a
   64-bit group order; the full-size worker is this Rust implementation.

No worker routing changes are made by this profiler.
