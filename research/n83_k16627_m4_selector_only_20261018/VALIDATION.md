# Selector-only comparison validation ledger

| Gate | Current state | Receipt or remaining work |
| --- | --- | --- |
| Protocol before code | committed as `3ed109bd24` | [frozen protocol](PROTOCOL.md) |
| Four/28-unit arm separation | compiled, small-field control passed; degree-83 behavior pending | [focused release test](focused-test.log), Rust adapter and CLI |
| Full/selector-only/phase-only small SAT and contradiction fixture | one release test passed; direct/observed complete-model parity, original-field group lift and contradictory UNSAT checked in all modes | [focused release test](focused-test.log) |
| Capped metadata filter | synthetic selector-only and phase-only fixtures accepted; decreasing XOR row scans and wrong-arm identity rejected with exit 5 | [deriver controls](DERIVER_CHECK.md) |
| Five retained panel inputs | SHA-256 checks passed before build | [input manifest](inputs-before-run.sha256), [check](input-check.log) |
| Exact source and Linux worker | pending freeze after tests | source commit, six-file manifest, read-only snapshot, offline binary digest |
| A1/B/A2 and conditional C | not attempted | fresh per-arm output directories and resource receipts pending |
| Separate native relation replay | code updated, unverified | positive and changed-x/row controls pending for returned SAT arms |
| Touched exporter example suite | 20 passed, 0 failed in release mode | [raw log](exporter-example-tests.log) |
| Replay example compile | passed in release mode; the example has no embedded tests | [raw build log](replay-example-build.log) |
| Full release library suite | pending | raw log required before push |

No degree-83 outcome or performance ratio is established by the protocol, synthetic deriver fixtures or unverified code. The predecessor's checked positive run remains separate evidence on its own source revision.
