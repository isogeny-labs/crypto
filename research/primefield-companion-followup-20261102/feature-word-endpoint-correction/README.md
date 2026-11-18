# Endpoint-corrected orbit-word traces

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), [report.pdf](report.pdf).
The optional corrected.rs path uses linear sign factors with sparse endpoint-image
corrections. The inherited word.rs remains the exact zero-safe reference.

From the repository root:

```sh
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-word-endpoint-correction/driver.rs -o /tmp/t1943-replay
timeout 60 /tmp/t1943-replay
```

Compare stdout with validation01/check.stdout; never overwrite saved evidence.
The receipt pins source dependencies, copied files, binary and build/check logs.
Software qualification only, no timing or total-work superiority claim.
