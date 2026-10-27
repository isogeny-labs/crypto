# Tightened Phase 2 native gate

All three phases exit zero; 89 tests pass with no failures. The production
runner is `abd9a046676f8e1da1212899438bd155d47b8f88fc3cfc814209dbf95f0efc1f`.
Per-file source seals, exact commands, compiler versions and full native
test output are copied byte-for-byte from the raw gate metadata.

Raw command receipts intentionally end in a space, and native test stdout
ends in a blank line. The first broad whitespace check reports those four
files. They are preserved unchanged; source whitespace checking excludes
this verbatim metadata directory. This is not a failed native test.

Actual p37 zero/plain phi/doubling records and separate audits remain in
`../../runs/phase2-native-gate-v2/test-artifacts/p37-roundtrip/` for the new
versioned Phase 2 archive. No full 85-configuration result is inferred here.
