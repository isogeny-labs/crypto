# Bidirectional power columns

Native cubic character construction using forward and reciprocal nonsingular recurrences, integrated into the reflection-odd solver. Read DERIVATION.md for assumptions, bounds and the quartic opportunity. Raw results and receipts are producer assertions awaiting signed PCC coverage.

Replay from repository root:

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-bidirectional-builder/lib.rs -o /tmp/primefield-bidirectional-tests
/tmp/primefield-bidirectional-tests --test-threads=1
```

No comparative timing claim is included.
