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

## Next gates

1. Rebuild the exact source on the isolated Linux runner. Freeze source,
   binary, job spec, workload, CPU, host counters, A/A controls, and a
   second independently verified `Q` point before Tier 1 timing.
2. Measure complete target-charged one-restart times for fixed radix-81,
   mixed-76, and warmed OpenSSL. Exclude only Q-independent initialization.
3. Measure 1, 8, 32, and 128 starts per Q separately, including Q-table
   preparation, peak memory, and total time.
4. Replay identical branch points and walk states after any secp256k1
   specialization. The generic C rho path has a 64-bit group order and is
   not the integration target for this full-size curve.

No worker routing changes are made by this profiler.
