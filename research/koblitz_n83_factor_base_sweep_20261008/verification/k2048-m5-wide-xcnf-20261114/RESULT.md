# N83 a=0 K2048 wide-XOR host-gate result

The first `--maxmatrixcols 25000` positive-control attempt retained a
three-sample host preflight and stopped before the solver call. Its recorded
one-minute load was 82.0376, 76.6616 and 74.8535 against the frozen maximum
24; swapouts stayed 164,320. The existing baseline and both exact XCNFs are
unchanged. The wide public-natural arm remains gated on a SAT positive with
independent original-XCNF and group replay.

| Arm | Attempt | Gate status | Load samples | Solver status | Relation |
| --- | ---: | --- | --- | --- | --- |
| K2048 m5 wide positive | 1 | `HOST_REJECTED_LOAD` | 82.0376, 76.6616, 74.8535 | `NOT_STARTED` | not observed |
| K2048 m5 wide public natural | 0 | `AWAITING_POSITIVE` | — | `NOT_STARTED` | not observed |

The [positive lock](NATIVE_POSITIVE_LOCK.json) binds clean source commit
`b4477fd36b70e3fea1a232135fbcabe437329a65`, Rust runner source SHA-256
`5463e06fdb6bc9f015c972963477bd18c1460b51d2aa1ece4ec0ac0ec4ce9cc1`,
runner binary SHA-256
`8068f14749066e0b855fb65946faf7679b6c5b47e814241d5d0af28a5bcad0c6`,
and the exact positive XCNF and CryptoMiniSat binary. The raw
[preflight](native-evidence/positive-01/host-preflight.json) has SHA-256
`e370e8e6779465e5136c4462f723b145191aa210f827f1176af8ef14e9f8f003`;
it is the only file the gate wrote in the output directory. Its local process
returned exit code 1 with `native host load or swapout gate failed; preflight
retained`. The unchanged swapout counts isolate load as the rejecting sample.

The next action is to wait for a local host preflight at load at most 24,
then commit a new lock with a fresh output path and repeat the positive.
The separately running R13 integrated-target process was live at the time of
this gate; this study will not alter it or relax the frozen resource rule.
