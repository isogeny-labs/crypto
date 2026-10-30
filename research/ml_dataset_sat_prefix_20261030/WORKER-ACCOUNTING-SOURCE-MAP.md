# Read-only source map

Repository: `/home/ec2-user/crypto-ml-sat-prefix-20261030`.

| Location | Accounting significance |
|---|---|
| `src/ml_dataset/telemetry.rs:10,34` | RUSAGE_SELF, durable phase start and legacy PMU wrapper; user-space/thread scope and explicit exclusions. |
| `src/ml_dataset/telemetry/checked/policy.json` | Fixed calling-thread policy, bounded collection, training/whole-worker/platform flags false. |
| `src/ml_dataset/telemetry/checked/mod.rs:238,476,515` | Strict replay forbids whole-worker total; checked collection dispatch and terminal metrics. |
| `src/ml_dataset/telemetry/checked/native.rs:220` | Native syscall/request/readback assessments; reuse discipline, not scope assumptions. |
| `src/ml_dataset/telemetry/checked/tests.rs` | Existing fake-counter lifecycle/error/identity/multiplex/overflow tests provide reusable controls. |
| `src/cryptanalysis/ecbench/isolation.rs:1526,1617,1679` | Legacy inheritance/group counters, scaled estimates, calling-task perf open. Separate validation strength from checked policy. |
| `src/ml_dataset/mod.rs:1153` | SupervisedChild owns stop/poll/reap; integrate resource reap here to avoid double wait. |
| `src/ml_dataset/mod.rs:1258` | Worker phase boundaries, out-of-phase artifact writes, result/registry serialization. |
| `src/ml_dataset/mod.rs:1630` | Spawn, process-group and parent-death setup, RLIMIT_AS, diagnostic timing and polling. |
| `src/ml_dataset/mod.rs:1734` | Explicit null whole-worker instruction count, output cap scope, no mid-call checkpoint. |
| `src/ml_dataset/mod.rs:1748` | Manifest seal, independent verify, publication and budget finalization outside worker. |
| `src/ml_dataset/mod.rs:2083` | Live-process/group reconciliation before interrupted-attempt handling. |
| `src/ml_dataset/capture.rs:139,333,751,970` | Bounded emitter, immutable acknowledged-prefix deltas and verification; no process-checkpoint claim. |
| `src/ml_dataset/solvers.rs:326` | Current method/domain/trace capability matrix. |
| `src/ml_dataset/solvers.rs:943,1168,1522,2074,2434,2642` | SAT, Boolean F4/F5/Weil, PKM, prime chain, F6 and prime-root scope/counter gaps. |
| `docs/ml-dataset/DESIGN.md:237` | Existing typed counter, concurrency, overlap, instruction and instrumented-arm contract. |

Inspected SHA-256:

```text
85b2b35da6fe4d22a705a657ae5213399e1c63434982b7095ac19963437d0b84  src/ml_dataset/telemetry.rs
ec61340c821d3e9d711d8b1ce2bf79eadc60dce9b132c20ea3e395d3e6119d5b  src/ml_dataset/telemetry/checked/mod.rs
45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50  src/ml_dataset/telemetry/checked/policy.json
1128d5716835d83cd9c2cf1216c56bf1ccf182119e57af503aad84e43f07598c  src/ml_dataset/mod.rs
a9ddde7ccc79ad90bf489e7e601cb92dc499d2fd898f0125201f518a4504fe11  src/ml_dataset/solvers.rs
2edf35218ace7f51e4c37a16b9e6eefca315533be31a553c0fadfcd9d321023a  src/ml_dataset/capture.rs
```

Only scratch design artifacts were written. Source/read-only outputs are not
native execution, hardware admission, cloud custody or completed coverage evidence.
