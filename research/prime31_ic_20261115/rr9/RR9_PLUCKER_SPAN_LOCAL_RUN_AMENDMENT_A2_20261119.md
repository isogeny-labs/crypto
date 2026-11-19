# Amendment A2: local GNU time path after wrapper failure

Date: 2026-11-19 UTC. PR #4779. Frozen before attempt 03. Every research
statement and future reading in this amendment is `[pcc:unverified]`.

Attempt 02 passed formatting, the original library tests, both focused unit
test suites, and release compilation. It then exited 127 before invoking the
first producer because this container has no `/usr/bin/time`. Its raw files
are retained at `plucker_span_local_20261119/attempt-02/`; its empty
`producer-a.json` is not rank evidence.

Installing the operating-system package was denied by the unprivileged local
environment. GNU time 1.9 was therefore built from its published source into
`/workspace/.prime31-tools/bin/time`. Attempt 03 uses
`run_prime31_rr9_plucker_span_local_a2.sh`, which accepts the executable path
through `TIME_BIN` and records its version. The `-v` fields, 4 GiB
producer/checker cap, 30-minute outer timeout, cases, algorithms, controls,
checker, and decision rule are unchanged.

Exact retry command:

```text
TIME_BIN=/workspace/.prime31-tools/bin/time timeout --signal=TERM 30m bash research/prime31_ic_20261115/rr9/run_prime31_rr9_plucker_span_local_a2.sh research/prime31_ic_20261115/rr9/plucker_span_local_20261119/attempt-03
```

Only a complete attempt 03 may be interpreted under the frozen rank rule.
