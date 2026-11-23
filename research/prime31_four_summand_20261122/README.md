# Public four-summand extraction and signed-relation integration

This is an additive native-Rust qualification of the existing RR5 monic-quartic
construction on the original ordinary Prime31 curve. The public extractor does
not enumerate factor-base pairs or tuples and refuses native full-field or
finite-domain fallback branches. It recovers signed rows, and a separate consumer
checks the authentic group equation before exact modular rank and target recovery.

**Current result:** local B4 extraction and an integrated F11 target recovery
pass the frozen development controls. The B8 extraction gates are inconclusive.
The original B677 coefficient-lift input is constructed and checked, but no B677
root extraction or fully charged comparison is completed. [pcc:unverified]

- [Results report](REPORT.md) and [PDF](reports/rr5-public-relations.pdf).
- [Frozen development protocol](PROTOCOL.md).
- [Mathematical chart and scope](MATHEMATICS.md).
- [Compact selector derivation](COMPACT_SELECTOR.md).
- [Source and execution-route audit](SOURCE_AUDIT.md).
- [Complete-cost comparison gate](COMPARISON_GATE.md).
- [Native sources](native/) and [immutable raw development receipts](evidence/).
- [Native observation export](reports/development-export/observation.json):
  116 raw-file pins,23 parsed stdout records and2 sampled identity certificates;
  unsigned, with failed and capped attempts retained.

## Reproduction

From this repository's root, with the unchanged production source closure:

```sh
CARGO_BUILD_JOBS=2 cargo test \
  --manifest-path research/prime31_four_summand_20261122/native/Cargo.toml \
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

This release profile is a correctness profile. Its execution time must not be
used as a benchmark result. No paired IC/rho panel, speed ratio, calibrated GAE
or asymptotic exponent is established here.

The metadata-only CLI accepts `export ROOT STUDY_REL OUT_REL` to preserve all
raw evidence pins and parsed records. `register ROOT OBS_REL PRODUCER_COMMIT
UTC_TIMESTAMP` creates an unsigned observation claim against the existing PCC
ledger; it does not issue a verifier verdict. Use a new output path and inspect
any retained state after an error. Registration requires an actual published
producer commit. The metadata tool performs no extraction or experiment.
