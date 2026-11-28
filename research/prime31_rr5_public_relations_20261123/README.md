# Public four-summand extraction and signed-relation integration

This disjoint continuation starts from main
`e51060051c6963b8852f86844f1ec53dc61f895f`. See [LINEAGE.md](LINEAGE.md)
for the preserved producer core 269cd7b, the concurrent same-path publication,
and the interpretation of copied historical evidence. Original main work stays
unchanged. The development01–08 observations below are historical; new-current
qualification is recorded separately.

This is an additive native-Rust qualification of the existing RR5 monic-quartic
construction on the original ordinary Prime31 curve. The public extractor does
not enumerate factor-base pairs or tuples and refuses native full-field or
finite-domain fallback branches. It recovers signed rows, and a separate consumer
checks the authentic group equation before exact modular rank and target recovery.

**Current result:** development09 passes 22 regression controls and three
separately invoked gates: exact Prime31 B8 Grevlex-cofactor positive extraction,
exact empty-query completion, and F11 public recovery through the selected-form
solver. The latter uses 9 actual public queries, 3 independent augmented rows
and a final scalar 3 replay. The Grevlex B8 configuration reproduces concurrent
main's declared fixtures/caps; historical Lex failures remain preserved.
The original B677 coefficient-lift input is constructed and checked, but no B677
root extraction or fully charged comparison is completed. [pcc:unverified]

- [Results report](REPORT.md) and [PDF](reports/rr5-public-relations.pdf).
- [Frozen development protocol](PROTOCOL.md).
- [Mathematical chart and scope](MATHEMATICS.md).
- [Compact selector derivation](COMPACT_SELECTOR.md).
- [Source and execution-route audit](SOURCE_AUDIT.md).
- [Complete-cost comparison gate](COMPARISON_GATE.md).
- [Native sources](native/) and [immutable raw development receipts](evidence/).
- [Current native observation](reports/integration-export/observation.json):
  160 raw-file pins, 41 parsed records and 2 sampled identity certificates,
  including every historical failed/capped attempt and the fresh qualification.
- [Unsigned observation registration](../../pcc/claims/PCC-20261123-688e4c.json)
  and [registration receipt](reports/pcc-registration.json): producer commit
  `9db32aeb03d7f1c1964016efcb23ed9c8241ed9f`, standing `unverified`, no
  signature or verifier verdict. The existing ledger prefix was preserved.
- [Historical native observation export](reports/development-export/observation.json):
  116 raw-file pins, 23 parsed stdout records and 2 sampled identity certificates;
  unsigned, with failed and capped attempts retained. Its paths refer to the
  published 269cd7b namespace; it is not a current-checkout registration input.

## Reproduction

From this repository's root at the qualified source revision:

```sh
CARGO_BUILD_JOBS=2 cargo test \
  --manifest-path research/prime31_rr5_public_relations_20261123/native/Cargo.toml \
  --release --locked --offline --lib --no-run --message-format=json
```

The protocol then copies the resulting test executable to an immutable path,
checks the source hashes, records a new pre-call manifest without replacement,
and executes it with one test worker and the recorded external process limits.
Use a new evidence directory for any replay. The `development-06` record binds
the 21-test qualification to its own source hashes and executable, irrespective
of later source additions. Development08 separately passes the one selector
setup check after development07 exposed an inherited coset-constant error. The
failed attempt remains available. The original shared F4/arithmetic sources are reused;
the scoped closure does not replace the separate full-library gate.

The explicit APIs are `extract_with_form` and `solve_public_with_form`, selecting
`ExtractionForm::ExpandedLex`, `CofactorLex`, or `CofactorGrevlex`. Existing
`extract`, `extract_cofactor`, and `solve_public` retain their historical Lex
behavior. The selected order is recorded for every query. Development09 uses
the actual e5106005 F4 source with optional visitor timing disabled; it has its
own build, source checks, immutable executable and pre/post-call records.

This release profile is a correctness profile. Its execution time must not be
used as a benchmark result. No paired IC/rho panel, speed ratio, calibrated GAE
or asymptotic exponent is established here.

The metadata-only CLI accepts `export ROOT STUDY_REL OUT_REL` to preserve all
raw evidence pins and parsed records. `register ROOT OBS_REL PRODUCER_COMMIT
UTC_TIMESTAMP` creates an unsigned observation claim against the existing PCC
ledger; it does not issue a verifier verdict. Use a new output path and inspect
any retained state after an error. Registration requires an actual published
producer commit. The metadata tool performs no extraction or experiment.
