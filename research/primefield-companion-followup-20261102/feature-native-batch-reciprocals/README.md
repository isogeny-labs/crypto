# Checked batch source reciprocals

DERIVATION.md states the arithmetic contract, input validation and counted costs.
lib.rs routes the quartic decoder through routed/source.rs and value.rs.
accounting.rs uses mechanically instrumented copies; generate.py and
verify_instrumentation.py record/check every transformation. report.pdf is inspected.

From the repository root (use fresh output binaries):

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-batch-reciprocals/lib.rs -o /tmp/batch-source-tests
/tmp/batch-source-tests --test-threads=1 --nocapture
python3 research/primefield-companion-followup-20261102/feature-native-batch-reciprocals/verify_instrumentation.py
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-batch-reciprocals/accounting.rs -o /tmp/batch-accounting-tests
/tmp/batch-accounting-tests accounting_tests:: --test-threads=1 --nocapture
```

Each build has a 90-second cap, each test invocation 60 seconds. Clear
QUARTIC_RECOVERY_OUT, BATCH_RECIPROCALS_OUT, COLLAPSE_OUT, ADAPTIVE_PACKET_OUT and
EXTENSION_TRACE_OUT when replaying to preserve frozen artifacts. Saved runners
write logs/receipts and must be rerun only in a scratch copy. counts.json projects
accounting-test.log without cryptographic execution in Python. Inversion counts
refer specifically to the value-division path, not all source operations.

Claims remain unsigned. Native and accounting runs are development validation;
there is no timing comparison or isolated performance panel.
