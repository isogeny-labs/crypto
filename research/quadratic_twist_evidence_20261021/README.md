# Quadratic-twist experiment evidence

This imports the completed quadratic-twist study into the requested repository.
It preserves the historical Python source and execution evidence as an immutable
snapshot. The active audit is C++17 with OpenSSL; it never invokes Python.

## Inspect the evidence

- [Historical execution log](historical/run.log)
- [Historical raw results](historical/results.json)
- [Historical provenance and source hashes](historical/run-metadata.json)
- [Source-and-evidence snapshot](historical/source-and-evidence.tar.xz) and
  [archive checksum](historical/source-and-evidence.tar.xz.sha256)
- [Audit protocol](PROTOCOL.md), [native verifier](audit.cpp), and
  [requirements/limitations](REQUIREMENTS.md)
- [Final native audit receipt](audits/run-20261021T064417Z/receipt.json) and
  [transcript](audits/run-20261021T064417Z/run.log)
- [Positive and negative control transcript](audits/controls-20261021T064419Z/run.log)
- [Results report and PDF](reports/README.md)

The snapshot is historical evidence under the repository's native-language
policy. Its Python harness is not an active execution path or a recommended
research implementation in this repository. Extracting it outside the repository
retains the original reproduction commands and report.

## Run the native audit

Requires a C++17 compiler, OpenSSL development headers/library, `tar`, `xz`,
`sha256sum`, and Bash. Negative controls additionally require `jq`.

```sh
bash research/quadratic_twist_evidence_20261021/run-audit.sh
bash research/quadratic_twist_evidence_20261021/test-audit.sh
```

Each command creates a fresh audit directory and refuses to overwrite it.
It verifies the snapshot hash before extraction. The audit replays the saved
pairing/IC certificates and the point-count examples independently in native
code. The CI workflow runs the same checks and uploads the fresh receipts.

## Evidence scope

| Requirement | Status |
| --- | --- |
| Preserve prior source, logs, inputs and results | verified by archive and internal hashes |
| Historical census: 942 models; 4,188,162 S3 triples; 113,342 S4 quadruples | historical Python run; not natively rerun |
| Historical 81 complete IC recoveries | recorded by historical harness; native audit checks all scalar-pairing identities and the three saved IC certificates |
| Native audit of three saved IC certificates | verified |
| Native 81 scalar-pairing and 3,219 bilinearity identities | verified |
| Corruption and false-relation rejection | checked by native controls |
| Full native census/relation-collection parity | not implemented in this import |
| Lean proof of the historical general theorems | not checked; Lean is unavailable in this execution environment |
| General GHS attack or general ECDLP hardness conclusion | not implemented or established |

No performance or security claim is promoted. The historical elapsed times are
uncontrolled process provenance, not admitted algorithm timings. No leaderboard,
benchmark row, curve registry entry, or canonical cost graph changes are made.

See [the validation record](validation/README.md) for repository checks that this
environment could not run. Those checks are not described as passing.
The [full native/formal parity problem](../../problems/quadratic-twist-evidence-native-parity.md)
keeps the remaining implementation and proof obligations explicit.

Native audit records remain loose because the report and review links resolve
their individual paths. The historical source/run tree is preserved as one
deterministic archive with an adjacent checksum.
