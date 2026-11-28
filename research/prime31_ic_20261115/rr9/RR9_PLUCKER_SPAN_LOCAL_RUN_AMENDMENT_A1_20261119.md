# Amendment A1: cap placement after pre-execution checkout failure

Date: 2026-11-19 UTC. PR #4779. Frozen before attempt 02. Every research
statement and future reading in this amendment is `[pcc:unverified]`.

Local attempt 01 exited 128 during `git worktree add`, before compilation,
tests, producer execution, or any N4/P4/N8/P8 case. Its driver reported that
Git could not memory-map a repository pack under the 4 GiB virtual-memory
limit. The raw attempt is retained at
`plucker_span_local_20261119/attempt-01/`.

The frozen GitHub workflow applies `ulimit -v 4194304` in the producer and
checker steps, after checkout and compilation. Attempt 02 therefore uses
`run_prime31_rr9_plucker_span_local_a1.sh`, which removes the premature
process-wide limit and applies the same 4 GiB limit separately around each
producer, checker, and mutation process. The 30-minute outer timeout remains
unchanged. No case, input, algorithm, control, verifier, or decision threshold
changes.

Exact retry command:

```text
timeout --signal=TERM 30m bash research/prime31_ic_20261115/rr9/run_prime31_rr9_plucker_span_local_a1.sh research/prime31_ic_20261115/rr9/plucker_span_local_20261119/attempt-02
```

Attempt 01 remains a failed setup attempt and supplies no rank evidence. Only
a complete attempt 02 may be interpreted under the original frozen rule.
