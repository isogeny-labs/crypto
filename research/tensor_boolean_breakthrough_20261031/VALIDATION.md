# Validation receipt

Date: 2026-10-31
Scope: E0 canonical-IR implementation, experiment design, and rendered report; no affected E1 data

## Protocol and structured records

Command:

```text
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py \
  validate research/tensor_boolean_breakthrough_20261031/protocol.draft.json
```

Result:

```text
OK: experiment.protocol/v1 research/tensor_boolean_breakthrough_20261031/protocol.draft.json (0 finding(s))
```

`jq empty` passed for both `protocol.draft.json` and `experiment-matrix.json`.

The protocol remains intentionally `draft`. Its power record has `passes: false`, and the binary, image, analysis, runtime, memory, final-cut-cap, final-ladder, protocol-hash, commit, and freeze fields remain unset. No production seeds were derived and no affected rows were generated.

The draft now links the canonical implementation and its
[`E0_IMPLEMENTATION.md`](E0_IMPLEMENTATION.md) receipt. This is not a freeze or
an amendment after data: no E1 data exists.

## Canonical-IR and crate checks

The standalone crate was tested with Rust 1.78.0, its documented toolchain.

```text
cargo test --locked --release
```

Result: exit 0. The run passed 63 library tests and 38 binary/integration tests,
101 total, plus zero doctests. The library set includes all six canonical-IR
tests:

- golden bytes/hash, parse/serialize/recapture identity, total and every
  interface-prefix count, least witness, and functional/direct agreement;
- all five registered mutations;
- explicit untagged-fixture policy;
- explicit rejection of an untyped 17-port closure;
- full tiny decomposition capture with a typed 132-port Fourier relation,
  strict parse/rebuild/recapture, corrupt-program rejection, and sampled
  evaluator parity;
- truncated, trailing, and bad-magic parser controls.

The retained test log had SHA-256
`9a4e04ce6835d5e72bceed330bdede95141664fc56f626b1d46a34ef82a13fd8`.
The longest integration suite, `structured`, passed all seven tests in 182.00
seconds.

```text
cargo clippy --locked --all-targets -- -D warnings
cargo fmt -- --check
```

Both exited 0. Cargo 1.78 emitted its pre-existing warning that
`lints.rust.unexpected_cfgs.check-cfg` is an unused manifest key; clippy emitted
no Rust warning. A pre-existing `width.rs` assignment was changed mechanically
to `clone_from` because the required all-target clippy gate otherwise rejected
it.

## Repository-level library gate

The first root attempt under Cargo 1.78 exited 101 before compilation because
locked `blake3 1.8.7` requires Cargo's Edition 2024 support. The exact gate was
rerun under temporary stable Rust 1.99.1:

```text
cargo +stable test --locked --release --lib
```

Result: exit 101 after 1,675.52 seconds: 5,789 passed, 2 failed, 125 ignored.
The two failures were:

```text
cryptanalysis::ecbench::runner::diagnostic_reap_publication_regression_tests::ok_without_observed_reap_cannot_mark_or_read_sidecars
cryptanalysis::ecbench::runner::diagnostic_reap_publication_regression_tests::publication_collision_preserves_previous_bytes_and_pending_failure
```

Both fail at `DiagnosticSessionClock::begin()` before their test assertions with
`No such file or directory (os error 2)`. This execution host does not expose
`/proc/self/ns/time` or `/proc/self/timens_offsets`, which that constructor
requires. Each failure reproduced under an exact one-test rerun; the first also
reproduced outside the filesystem/network sandbox. The retained root log had
SHA-256
`5d6fb8a62d6b3bf0982f58ece26c17af0ece7eacc9b71acf76521418e6ab3b35`.
The broad root gate is therefore reported as failed, not passed. No failure was
in the changed tensor crate or its tests.

## Figures and PDF

- Graphviz rendered `figures/experiment-funnel.dot` to SVG and PNG successfully.
- Pandoc with pdfLaTeX rendered `REPORT.md` to `report.pdf` successfully.
- `pdftotext` recovered the required Recommendation, first-test, SAT-falsifier, claim-boundary, and evidence sections.
- PDF metadata: 2 US-letter pages, 314,858 bytes.
- SHA-256 of `report.pdf`: `1d312fbda0c4963e0c9ed792c10690f0d1ec81bfb9da0bc9386e717ae61aea24`.
- Both PDF pages and the source PNG were opened. Text, table, labels, arrows, caption, and page boundaries were present without clipping. The wide graph is intended for page-width display and remains zoomable in the PDF.

## Other repository checks

`git diff --check`, JSON parsing, protocol-schema validation, PDF text recovery,
and final two-page visual inspection passed. The required pre-work `conductor
check` was attempted for the crate, study, and session-log scopes and returned
exit 127 because `conductor` is absent. The repository's warn-and-allow path was
used; no reservation was claimed.

This was implementation correctness work, not an authorized performance run,
so no EKS benchmark job was submitted. No benchmark row or experiment export
was generated.

## Evidence status

| Item | Status |
|---|---|
| Experiment portfolio and dependencies | Complete at design scope |
| E1 controlled protocol | Draft, schema-valid |
| A/A, positive, negative and sham controls | Specified |
| SAT/BDD triangulation | Experiment card specified; separate protocol not yet frozen |
| Canonical IR core implementation | Implemented |
| Tiny round-trip/count/prefix/witness controls | Passed |
| Representative typed high-arity capture | Passed on the full tiny decomposition construction |
| Final E1 ladder and fresh-process A/A coverage | Open; generator and ladder are not frozen |
| Optional high-arity selector/custom predicates | Open; strict export rejects them without typed programs |
| Runtime/power preflight | Not started |
| E1 measurements | None |
| Breakthrough or speedup claim | Not established |
