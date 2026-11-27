# Original degree-32 workloads under the source-pinned executable

The source commit is `a5503f4bb66ac3aa4eac3125fba01534d2574096` and the
retained `ic` executable has SHA-256
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`.
Replay the original `degree32` seed `20261128` and the selected
`degree32_replication_extend` seed `20261149` into **separate**, append-only
stores. Each uses 16 repeats, the original ordinary `t=5,seed=5` candidate,
`prime-abscissa:below=32` control, and the same `walk` target stream.

Run size-only subtraction preflights first. Require identical instance and
exactly equal factor-base abscissa counts within each pair. Also compare the
new counts to the original store. A mismatch ends that replay before timing;
retain the preflight JSON and terminal verdict. If sizes match, run the
original m=2 F4 degree-8 chain oracle with 1,000 trials, five seconds per
solver call, one rho context run, and the original order: `degree32` uses
control/candidate; the extension uses control/candidate/candidate/control.
The whole-call cap is 240 seconds. Preserve command, exit code, complete JSON,
external wall, child CPU, and stderr for every invocation. Require successful
verified same-target scalars and report GAE, wall, CPU, relations, trials and
`s_over_rho` without dropping censored attempts.

This replay tests source/build reproducibility of the historical component
stores. A changed base size is a reproducibility result, not a performance
comparison. The earlier `degree32_source_replay.jsonl` remains immutable.
