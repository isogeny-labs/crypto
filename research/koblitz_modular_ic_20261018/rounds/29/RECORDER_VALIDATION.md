# Cargo recorder validation repair

The existing review identified a fail-open log parser: searching backwards for
`test result: ok.` could skip a failing final Cargo summary and report a nested
subprocess success. The parser now selects the final `test result:` line and
requires success and zero failures. Missing summaries also reject. Both root
and native recorder inputs use that parser before any ledger is saved.

Four focused Rust tests pass: a successful final summary with a nested child,
a final failure after child success, a missing summary, and an inconsistent
success label with a nonzero failure count. The final-failure regression rejects
the unmodified 891e0934 implementation (expected Cargo exit101, 1 test failed).

The complete native crate test build exits101 before tests: independently
inherited root binary_ecc files refer to crate::cryptanalysis and crate::ecc,
which are absent from this standalone study crate. This is not called passed.
RECORDER_REGRESSION.py assembles the exact current parser and test module in a
small Rust crate, with serde_json unchanged. The isolated release test exits0:
4 passed/0 failed, plus 0 doctests. The assembler does no cryptographic math.
Reproduce with `python3 rounds/29/RECORDER_REGRESSION.py /var/tmp/koblitz-recorder-regression`
from the study directory, then `cargo test --offline --release --manifest-path
/var/tmp/koblitz-recorder-regression/Cargo.toml`. Use the retained 4GiB/1200s caps.

This repair does not regenerate historical ledgers, change the sealed R27–R29
scientific sources, or certify the complete root/native build. The earlier
retained integrity checks and proof counts retain their original scope. Raw
software validation logs are retained in the final S3 checkpoint under .resume.
Their SHA256 hashes are:

```json
{
  "r27-r29-recorder-regression.log": "5dea6df6de5c5bfbdd562ce1317c65ba60289c0d9d7deb0f9e67b42eef6f54f4",
  "r27-r29-recorder-isolated.log": "fb80b496118e0afed18319ca91e588a21927e3bcd19e4be676b7dc0ee6f91fec",
  "r27-r29-recorder-baseline.log": "40e9e92e1d557daba225c01388537706f00e54d43e60e0e8fec86b2ae11511f5",
  "r27-r29-recorder-extraction.log": "ecbd40ef55a443ce859da8d7a42263935131ca925027eb8d2d9ccce83f0b5ca7"
}
```
