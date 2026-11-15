# Normalized adaptive recovery accounting

The native execution completed on November5; its interrupted turn was reconciled
on November7 without rerunning it. accounting-receipt.json records both exits0.
Read DERIVATION.md for exact counter definitions, stage boundaries and limits.

Replay from the repository root:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-transport-accounting/verify_instrumentation.py
python3 research/primefield-companion-followup-20261102/feature-native-transport-accounting/project_counts.py
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-transport-accounting/accounting.rs -o /tmp/transport-accounting-tests
/tmp/transport-accounting-tests accounting_tests:: --test-threads=1 --nocapture
```

Build cap90seconds; test cap60seconds. Clear inherited artifact-export variables.
run-record.py and generate.py overwrite outputs; run them only in a scratch copy.
The original receipt does not capture a compiler-version string or host inventory;
it does bind source/data bytes, flags, commands and the tested binary SHA-256.
The report uses only recorded helper and allocator counts, with no timing model.

The projection includes every raw count marker, even the first test-prefixed row.
summary.json is a selection of those rows; counts.json retains all1140 records.
render_chart.py draws the saved Q=[25]P stage counts without executing kernels.
The original4563-case cohort is unchanged. Claims remain unsigned.
