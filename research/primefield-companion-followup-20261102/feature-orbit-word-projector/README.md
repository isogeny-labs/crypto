# Full orbit-word projector

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md) and [report.pdf](report.pdf).

`word.rs` implements bounded exact image extraction and all-mode scalar phase.
`checks.rs` supplies finite scalar enumeration only for correctness oracles;
`recover_word` receives public curve points and a calibrated source.
The inherited prime-field source and circuit dependencies are pinned in receipts.

Reproduce validation02 from repository root with Rust installed:

```sh
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-orbit-word-projector/driver.rs -o /tmp/t1940-replay
timeout 60 /tmp/t1940-replay
```

Compare stdout with validation02/check.stdout. Do not overwrite saved runs.
This is bounded software qualification, with no cloud workload or timing claim.
Per-call limits:100000 circuit nodes;128 terms,500000 retained coefficient slots,
2000000 monomial pairs (large control:64 terms,100000 slots,same pair cap).
Max calls1+4m, with one-call refusal controls. Output degree cap is unused.
Prime r, certified source compatibility and subgroup order remain preconditions.
