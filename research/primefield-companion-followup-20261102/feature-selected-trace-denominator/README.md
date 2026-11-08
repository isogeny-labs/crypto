# Selected trace denominator

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), [report.pdf](report.pdf).
selected.rs adds an optional scaled recurrence and public recovery adapter.
recurrence.rs preserves the prior per-step-normalized reference byte-for-byte.

From the repository root:

```sh
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-selected-trace-denominator/driver.rs -o /tmp/t1946-replay
timeout 60 /tmp/t1946-replay
```

Compare stdout to validation01/check.stdout. Never overwrite saved evidence.
Only bounded software qualification; no timing/speed ranking or new cloud workload.
