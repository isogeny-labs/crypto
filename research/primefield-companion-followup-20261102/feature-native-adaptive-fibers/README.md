# Adaptive sparse-fiber decoder

Native pair-mean feature synthesis and lazy extra-sign recovery. DERIVATION.md gives the exact identity, scope and costs. The fixed r479 route uses15 initial signs and asks for offset15 only on its two complete exceptional fibers. The runtime has no scalar-answer table. [pcc:unverified]

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-adaptive-fibers/lib.rs -o /tmp/adaptive-tests
/tmp/adaptive-tests --test-threads=1
rustc --edition=2021 -O -C overflow-checks=yes research/primefield-companion-followup-20261102/feature-native-adaptive-fibers/check.rs -o /tmp/adaptive-check
/tmp/adaptive-check research/primefield-companion-followup-20261102/feature-native-adaptive-fibers/program.tsv
```

Receipts are local correctness records with unsigned PCC standing; no timing comparison is included.
