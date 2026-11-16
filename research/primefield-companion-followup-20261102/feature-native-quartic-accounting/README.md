# Quartic pipeline arithmetic-event instrument

Read DERIVATION-CORRECTED.md for scope, omissions and stage boundaries. This is development
validation on saved fixtures, not a performance panel. counts.json projects raw
rows from test.log; report.pdf shows one fixture's event distribution. The claim
in claims.json is unsigned. Original source modules remain unchanged.

From the repository root:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-quartic-accounting/verify_instrumentation.py
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-quartic-accounting/lib.rs -o /tmp/quartic-accounting-tests
/tmp/quartic-accounting-tests accounting_tests:: --test-threads=1 --nocapture
```

The recorded build limit is 90 seconds and test limit 60 seconds. Three accounting
tests ran; 120 imported tests were filtered. Clear QUARTIC_RECOVERY_OUT,
ADAPTIVE_PACKET_OUT, EXTENSION_TRACE_OUT and COLLAPSE_OUT when replaying. Do not
rerun the packaging runner in the evidence directory: use a scratch copy because
it writes logs and receipts. generate.py recreates the routed source map without
cryptographic execution; verify_instrumentation.py is read-only.

The allocator records requested bytes, not peak or net memory. The point verifier
is an extra harness stage, not part of the frozen recovery API. Input labels are
used only by assertions and selection of known-answer fixtures.
