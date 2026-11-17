# N83 a=0 K2048 wide-XOR host-gate result

The second `--maxmatrixcols 25000` positive-control attempt returned SAT at
839 conflicts in 0.3113 seconds of solver process wall. Independent Rust
replay checked all 1,141,221 original CNF clauses, 4,980 XOR rows, 339,968
archived subgroup points, the group sum and the signed-Frobenius relation
row. The first attempt retained a
three-sample host preflight and stopped before the solver call. Its recorded
one-minute load was 82.0376, 76.6616 and 74.8535 against the frozen maximum
24; swapouts stayed 164,320. The existing baseline and both exact XCNFs are
unchanged. The wide public-natural arm admitted three 332-by-20,916 Gaussian
matrices, then reached the frozen 1,000,001-conflict cap with
`INDETERMINATE` status in 151.8622 seconds. It produced no model or verified
relation on this public query.

| Arm | Attempt | Gate status | Load samples | Solver status | Relation |
| --- | ---: | --- | --- | --- | --- |
| K2048 m5 wide positive | 1 | `HOST_REJECTED_LOAD` | 82.0376, 76.6616, 74.8535 | `NOT_STARTED` | not observed |
| K2048 m5 wide positive | 2 | `SAT`, replay passed | 5.6504, 4.9146, 5.2529 | 839 conflicts, 0.3113 s | 5 verified columns and relation row |
| K2048 m5 wide public natural | 1 | `PASS_HOST_GATE` | 3.7930, 3.3335, 3.2031 | `INDETERMINATE`, 1,000,001 conflicts, 151.8622 s | no verified relation |

The [first positive lock](NATIVE_POSITIVE_LOCK.json) binds clean source commit
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

The [second positive lock](NATIVE_POSITIVE_LOCK_02.json) retained the same
formula, source, solver, and flags with a fresh output path. Its
[preflight](native-evidence/positive-02/host-preflight.json),
[solver receipt](native-evidence/positive-02/receipt.json), and
[replay](native-evidence/positive-02/replay.json) have SHA-256 digests
`b772a8e8cac622e721dc65cbb909c1a62acd7a2df58cbaae2d8c586e06b3e8f6`,
`1921ff716185c64d4bba29e17ed4eddb23f56df1020d0cc926e29dacca10d69f`, and
`4b18a7b7ebf5057189aee32bd26260c86ce58e089a2c7c5dbfc394b68406c388`,
respectively. The [natural lock](NATIVE_NATURAL_LOCK.json) binds the replay
hash, exact public-natural XCNF, and unused raw output path before dispatch.
The [natural receipt](native-evidence/natural-01/receipt.json) has SHA-256
`092e28f738549abf4eea12dc5cf3ef6baefe5b34db45c91207b99d17f389f71a`,
and the complete [solver transcript](native-evidence/natural-01/stdout.txt.gz)
decompresses to SHA-256 `d210ed7bc4c6166e7f1f05c3249ead98d65f245159381608210bd8396dfa174b`.
The [positive transcript](native-evidence/positive-02/stdout.txt.gz)
decompresses to SHA-256 `bf7caafb268d9986b8840481d4917aea5ab4db771dfb165937e9f5e760c203cc`.
The solver used 32 Gaussian matrices, including three width-20,916 matrices;
the original 1,000-column limit declined those matrices. This single
natural-query result isolates matrix admission but does not settle the
factor-base selection. The next selector gate is a frozen ordinary-query
panel with verified novel rank, full-width logs, unseen-target descent, and
same-point rho on this `a=0` curve.
