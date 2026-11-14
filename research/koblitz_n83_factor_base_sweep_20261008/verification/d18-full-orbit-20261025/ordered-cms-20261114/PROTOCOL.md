# Bounded external XOR solver on ordered D18 positive control

**Frozen 2026-11-14 before solver execution.** Test CryptoMiniSat 5.14.7
against the original ordered-first-pair D18 constructed-control XCNF whose
329,808-bit native assignment passed independent replay. This is one
solver-backend capacity diagnostic on the primary N83 `a=0` curve. The exact
formula is 50,523,891 bytes, SHA-256
`c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74`;
it contains 2,807,999 CNF clauses and 3,901 XOR rows. The signed PCC
standing for the local replay is still `unverified` [pcc:unverified].

The [export lock](EXPORT_LOCK.json) pins the source commit, worker and solver
binaries, producer and provider input manifests, D18 object, export receipt,
positive replay, formula bytes and shape, flags, and watchdog. The runner is
[run_cms.py](run_cms.py), SHA-256
`7015681c10dc4bc449aecb9b737b7ed3f442ee67e0ba215a456a1dcb1cd60687`.
The solver binary is `/opt/homebrew/bin/cryptominisat5`, version 5.14.7,
SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`.
The source checkout must be clean and its six algorithmic files byte-equal
to export commit `afebef39283466fca4ddff01063d26df9b59c45e`.

Before launch, take three host samples 15 seconds apart. Require one-minute
load at most 24 at each sample and launch, no new swapouts, and at least 2
GiB scratch. Run only the `positive` mode with one thread, fixed random seed
20261114, `--maxtime 600`, `--maxconfl 1000000`, full status/model output,
and a 660-second external process-group watchdog. Save command, UTC bounds,
exit code, stdout/stderr, wall/CPU, sampled peak RSS, swapouts and status.
The solver may return SAT, UNSAT, INDETERMINATE, watchdog or producer failure;
retain each outcome distinctly. Do not impute a successful relation or zero
cost from any incomplete outcome.

If SAT, independently replay the **complete external assignment** against
every original CNF/XOR row before treating it as a solver control pass. A
different model needs its own curve-group and row replay. A natural arm is
outside this protocol and needs its own frozen export, lock, positive-control
gate, and verification. The data here cannot determine an end-to-end IC
factor-base winner on either N83 curve.
