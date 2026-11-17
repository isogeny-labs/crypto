# Native SAT-trace-only replay for the failed orchestration

Exact native command on the fresh extracted raw archive:

```
raw-bundle/bin/runner-v10 check-sat-trace raw-bundle/runs/k4096-v10/failure-stderr.log
```

[Retained native output](fresh-replay/k4096-v10-sat-trace-only.json) reports
COMPLETE for SAT phase order/cumulative-counter trace, ending at
solve_return/Unknown/400000conflicts. It compares byte-for-byte with the
same exact native command on original retained failure-stderr.log.

This validates that trace, not the original group's full query/custody,
Docker exit/OOM state or complete outer run. The original v10 run remains
LAUNCH_FAILED_NO_AUDIT, unadmitted, and outside the quantitative timing table.
No original final state or audit.json is fabricated from this partial replay.
Both separately admitted runs receive their full downloaded-record native
replays in the other fresh-replay files; preserve these scopes separately.
