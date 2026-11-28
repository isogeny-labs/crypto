# Local execution addendum: Prime31 RR9 Pluecker-span gate

Date: 2026-11-19 UTC. PR #4779. Status: frozen before the local study run.
Every research statement and future reading in this addendum is
`[pcc:unverified]`.

## Owner request and unchanged acceptance rule

The owner authorized one local execution after the required S3 resume
checkpoint failed for lack of an upload token and AWS credentials. The owner
requested that durable raw artifacts be committed to Git. This is a one-run
waiver, not removal of the repository checkpoint rule.

The mathematical question, N4/P4/N8/P8 cases, inputs, controls, and decision
rule remain exactly those frozen in
`RR9_PLUCKER_SPAN_PROTOCOL_20261119.md` with SHA-256
`ab67f98a2f1e58f4e5f6c610d5b4ca6ab3fe7eabc8050e94f2103200322ca62a`.
The execution venue and artifact transport are the only changes.

| Requested item | Evidence target |
|---|---|
| Run the exact gate locally | `run_prime31_rr9_plucker_span_local.sh` exit receipt |
| Preserve artifacts in Git | `plucker_span_local_20261119/attempt-01/` |
| Keep the frozen cases and verdict | producer JSON plus protocol digest check |
| Use an independent verifier | Rust permutation-minor checker output twice |
| Preserve controls and failures | unit-test logs, mutations JSON, exit-code files |
| Bind source and environment | candidate commit, source hashes, toolchain and host receipt |

## Explicit deviations and claim boundary

- No S3 resume checkpoint exists for this run. The failed gate output remains
  recorded in the session history and the waiver is appended to the PCC
  ledger before execution.
- The run is local rather than on the frozen GitHub runner. Therefore it does
  not satisfy the original runner-attestation condition and cannot be called
  source-verified CI. `[pcc:unverified]`
- Git is the requested durable artifact store. Raw JSON, stderr, `/usr/bin/time
  -v` records, exact exit codes, SHA-256 bindings, and the driver log are
  committed without deleting failed attempts.
- The run establishes no benchmark, ECDLP speedup, solver, or novelty claim.
  Elapsed time is operational metadata only.

## Local command and caps

From the repository root, after committing this addendum, the runner, and the
PCC bypass entry:

```text
timeout --signal=TERM 30m bash research/prime31_ic_20261115/rr9/run_prime31_rr9_plucker_span_local.sh research/prime31_ic_20261115/rr9/plucker_span_local_20261119/attempt-01
```

The runner applies a 4 GiB virtual-memory limit before compilation or
execution. It checks out original arithmetic commit
`1df714645aa683cf975bc76fcacb9842cb4780fd` in a fresh temporary worktree,
materializes its shared Rust modules, overlays only the committed producer and
checker, and uses Rust 1.100.0. It runs original library tests, focused unit
tests, immutable release builds, two producers, two independent checker
replays, byte comparisons, and mutation controls.

Timeout, OOM, compile failure, failed test, output mismatch, rejected source
binding, checker disagreement, or mutation acceptance is retained as a failed
attempt. Only complete exit-zero execution permits applying the frozen rank
decision rule.
