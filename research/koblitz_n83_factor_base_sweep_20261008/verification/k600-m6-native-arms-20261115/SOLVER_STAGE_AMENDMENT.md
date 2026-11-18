# K600 m6 native XCNF solver-stage amendment — prepared 2026-11-15 UTC

**Status: proposed execution details, no solver run or execution allowance.**
This adds an explicit native CryptoMiniSat arm to the frozen
[full-base m6 protocol](../k600-full-smooth-m6-20261114/PROTOCOL.md) and
[native-supervisor amendment](../k600-full-smooth-m6-20261114/NATIVE_SUPERVISOR_AMENDMENT_20261115.md).
The primary N83 `a=0` curve, K600/99,600-point read-only S3 object, hash
policy/seed, m6 per-representative pattern-0 XCNF circuit, fixture 0, and
125,000-variable/350,000-domain-clause limits remain fixed. This backend is
one experimental cell; it does not replace the frozen native chain solver
or imply that another solver's measurements are interchangeable.

## Admission and order

1. Require a committed clean source, a content-hashed Linux arm64 exporter,
   replayer and solver binary, an exact source-revision-labelled Linux arm64
   Docker image, a fresh S3 K600 read-only stage, and matching zero-trial
   native preflight lock, outer, config and summary receipts. The preflight
   must report `PREFLIGHT_ONLY`, zero trial/relation/SAT/matrix work, and a
   nonzero cgroup peak no greater than 4 GiB. The stage and receipt gates in
   `examples/n83_k600_m6_native_arms.rs` reject changed inputs before launch.
2. Export and solve `positive-pinned`, then export and solve
   `positive-unpinned` on the same constructed target and circuit. Require a
   full native `primary-k600-m6-replay` PASS for each SAT model. A cap,
   timeout, resource exit, invalid model, or missing receipt stops the arm
   sequence. The pinned arm checks the fixed witness; the unpinned arm tests
   the solver without its 913 pin clauses.
3. Only after both controls pass, export and solve the single `natural`
   fixture-0 query, with the same source, base, solver and limits, then replay
   any SAT assignment against the original XCNF and full group equation.
   Record a censored result if the cap is reached; do not record zero yield
   for a missing or invalid receipt.

## Solver invocation and accounting

Use a Linux arm64 CryptoMiniSat binary whose version and SHA-256 are frozen
in the execution lock before the first arm. The selected command arguments
are `--threads 1 --random 20261115 --maxconfl 10000 --verb 1 --verbstat 1
--printsol 1`, with `--maxtime` no greater than the separately approved
remaining charged seconds and the original XCNF as the final argument.
Capture the complete stdout, stderr, exit code, original XCNF hash, binary
hash, image ID, source commit, wall/CPU and resource fields, and every failed
attempt. Exit 10 with a complete model is `SAT_PENDING_XCNF_REPLAY`, never a
verified relation until native replay passes. Other exits remain distinct
`UNSAT`, `UNKNOWN_solver_cap`, wall-cap, resource-exit, or producer/cleanup
failures with their raw receipts.

Each Docker child uses one pinned CPU, 4 GiB hard memory, zero additional
swap, 64 PIDs, no network, read-only root/inputs, no added capabilities or
privileges, and at most 64 MiB scratch tmpfs. A native outer guard kills on
timeout and verifies container removal. Charge source/image build, staging,
preflight, both constructed exports and solver calls, replay, natural work,
failed stages and cleanup to the **single** proposed 3,600-second envelope.
That envelope has not been approved for this distinct run; no stage may be
launched merely because this amendment or a lock exists.

This one-arm result cannot select the fastest factor base. The eventual
selection still needs ordinary relation yield and independent replay, useful
rank, factor-base logarithms, target descent and verification, all cold setup
and artifact I/O, matched rho, and the full declared combination sweep.
