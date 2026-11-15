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

## 2026-11-01 exact boundary-census implementation

Scope: native E1 arithmetic instrument and lead-reconciliation report; no
affected E1 data.

### Native validation

Rust 1.78.0 focused release validation passed:

```text
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml -- --check
cargo test --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release census --lib
cargo check --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release --bin tensor-census
```

The focused suite passed 4/4 tests. Its randomized property control compared
1,152 deterministic rectangular integer matrices with independent rational
Gaussian elimination. Exact rank-one, rank-two and rank-four controls, a
canonical equality-network contraction, pin rejection and cap exhaustion also
passed.

The complete standalone crate suite then passed under Rust 1.78.0: 117 tests,
0 failures, including 67 library tests, every binary/integration suite and
doc-tests. The existing `structured` integration suite took 181.08 seconds;
no timeout or retry was hidden.

Current stable Rust 1.99.1 passed:

```text
cargo clippy --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --all-targets -- -D warnings
```

Rust 1.78 all-target Clippy stopped on the pre-existing `format_collect`
diagnostic at `panel_driver.rs:238`; no suppression or unrelated edit was
made. The current-stable gate is the same toolchain class used for the prior
E0 all-target lint receipt.

Source SHA-256 values before the final commit were:

```text
9d13c2e1d240190a0be0340fbe6b2bbd83588b05f035731f8c6fbe63b2f34a82  crates/hashed-ecdl-tensor/src/census.rs
215f1255bded298c136388fc7fef311a1aa4cbfee6086e50abd8c4f37e3eed68  crates/hashed-ecdl-tensor/src/bin/tensor-census.rs
```

These identify the locally checked source files, not a frozen experiment
binary or image. The protocol's binary/image fields remain unset.

### Report and visual validation

Graphviz rendered `figures/lead-reconciliation.dot` to SVG. Typst 0.15.0
rendered `search-20261101.typ` to a one-page tagged A4 PDF. `pdftotext`
recovered the outcome, evidence table, implementation checks, requirement
status and sources. The page was rendered to PNG and visually inspected at
original detail: the graph, tables, labels and footer are legible and not
clipped.

```text
e199fac3776beff0cc8901bda25f5b2f7ae44522f72977b5b4bc02352a7ac698  figures/lead-reconciliation.svg
5d4afe4aff86d8239f6d40a39c6b92fb079639974519a80d8cd7ec506a558d54  search-20261101.pdf
```

This is a search/implementation report, not a performance result, so no
`benchmarks/` copy or canonical numeric dashboard point applies.

### Repository-level library gate

The required root command was run under stable Rust 1.99.1:

```text
cargo +stable test --locked --release --lib
```

It exited 101 after 1,774.74 seconds: 5,817 passed, 9 failed and 128 were
ignored. None of the failures is in `hashed-ecdl-tensor` or the files changed
by this slice.

Six failures were caused by the sparse checkout omitting tracked fixtures:
five `cryptanalysis::certicom_dlp::device_tests` stopped at
`tracked gpu/certicom source must be materialized`, and
`committed_version_1_bounds_re_derive_under_their_recorded_version` could not
open its tracked session JSON. After materializing exactly `gpu/certicom` and
`research/ecbench_bounds_challenges_20261006`, their targeted reruns passed
6/6 and 1/1 respectively.

Two diagnostic publication tests failed at clock setup because this execution
host lacks both `/proc/self/ns/time` and `/proc/self/timens_offsets`. An exact
four-test namespace rerun passed the two independent controls and reproduced
these two host-limited failures.

The remaining unrelated failure reproduced in isolation:

```text
cryptanalysis::ic_wide::tests::run_f2m4_is_the_one_word_pipeline
assertion failed: left 13130, right 3757
```

Its exact rerun exited 101 after 157.07 seconds with 0 passed and 1 failed.
The broad root gate is therefore reported as failed, not passed. The complete
changed-crate suite and final focused source gates above pass.

### Remaining evidence gates

No production semantic cut, matched natural/null pair, rank ladder or timing
was generated. The E1 protocol remains draft. The production cut generator,
rewired-null constructor, fresh-process A/A runner, disjoint preflight, power
record, final ladder, source commit and image digest remain open.

## 2026-11-01 rigorous boundary-rank bounds

Scope: exact finite lower-bound certificates, architecture-level formal
theorems and a named asymptotic family obstruction. No affected E1 data.

### Formal verification

The standalone Lean project pins Lean 4.35.0, `crypto-lean`
`148f5dc389b341401e06d76abe7783c4ccf42f2e`, and Mathlib
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.

```text
lake build
tools/lean/proof-gate.sh --out /tmp/tensor-bound-proof-receipts \
  research/tensor_boolean_breakthrough_20261031/lean
lake env leanchecker --fresh TensorBoundaryBounds
```

All commands exited 0. The repository proof gate audited eight theorems: eight
passed, zero failed, and zero used trusted native evaluation. The only
transitive axioms were Mathlib's permitted `propext`, `Classical.choice`, and
`Quot.sound`; the three elementary family transitivity theorems require only
`propext`. Fresh `leanchecker` replay exited 0 with no diagnostic. The proof
gate initially returned `FAIL_no_sources` while the new files were untracked;
after staging those exact sources, the same target passed. This was a discovery
rule issue, not a proof retry or theorem change.

The proof map and exposed hypotheses are in
[`RIGOROUS_BOUNDS.md`](RIGOROUS_BOUNDS.md). No `sorry`, project axiom, or
`native_decide` appears in the project.

### Native certificate verification

Rust 1.78.0 passed formatting, the focused release suite, and the census CLI
build. Current stable Rust passed all-target Clippy with warnings denied.

```text
cargo +1.78.0 fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml -- --check
cargo +1.78.0 test --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release census --lib
cargo +1.78.0 check --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release --bin tensor-census
cargo +stable clippy --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --all-targets -- -D warnings
```

The focused suite passed 4/4 tests. Its deterministic property control checks
1,152 integer matrices of every shape from 1-by-1 through 6-by-6: exact rank
agrees with independent rational Gaussian elimination and the rank-profile
minor's independently recomputed determinant is nonzero. The equality-network
test independently verifies its order-two certificate and rejects a changed
determinant. Cap and pin failures still fail closed.

The complete standalone crate release suite passed 117 tests with zero
failures: 67 library tests, 50 binary/integration tests and zero doc-tests. The
pre-existing exact `structured` integration suite took 183.40 seconds; no
timeout or retry was hidden.

Source hashes for the locally checked files are:

```text
00d91c5e7663a11cbfc37fe15778cd2fab37f949c5aeac9b87144c06ae4a0e80  crates/hashed-ecdl-tensor/src/census.rs
08d9f262662167ee8f8d350a750c457167430b6924b2b55f0e00171a381841ad  crates/hashed-ecdl-tensor/src/bin/tensor-census.rs
e2e21c9ab3dd071fa7af952d8ecfdd32de13e0251df8db377bc487cfe804d3e5  research/tensor_boolean_breakthrough_20261031/lean/TensorBoundaryBounds.lean
```

These identify checked source, not a frozen E1 binary or experiment image.

### Bound diagram and PDF

Graphviz rendered `figures/rigorous-bound-chain.dot` to SVG and PNG. Pandoc
with pdfLaTeX rendered `BOUNDS_REPORT.md` to a one-page US-letter PDF.
`pdftotext` recovered the finite bound, family condition, claim boundary and
reproducibility sections. The page was rendered to PNG and visually inspected:
the graph, formulas, caption and footer are legible and unclipped.

```text
4ea554443bc4af3849668c68c6709c72d77d582ef420c7efafd6dd082e34f378  figures/rigorous-bound-chain.svg
f361544edd9af0f184c0e9cc3758ff8a13ee62c438b5f6793e358e89de738ed7  bounds-20261101.pdf
```

### Repository and evidence boundary

`git diff --check` passed. The required pre-edit `conductor check` returned
exit 127 because `conductor` is not installed; the documented warn-and-allow
path was used and no reservation was claimed.

The finite theorem and verifier do not supply the uniform family hypothesis.
No production semantic cut, rank ladder, natural/null comparison, timing, or
benchmark row was generated. The precise open lemma is tracked in
[`problems/hashed-ecdl-boundary-rank-family-lower-bound.md`](../../problems/hashed-ecdl-boundary-rank-family-lower-bound.md).

### Current-head repository-level library gate

The required current-head root command was run under stable Rust:

```text
cargo +stable test --locked --release --lib
```

It exited 101 after 1,856.71 seconds: 5,903 passed, 5 failed and 128 were
ignored. No failure is in `hashed-ecdl-tensor`, the certificate code, or the
formal project. The five failures were:

```text
cryptanalysis::ecbench::optimization::tests::every_live_method_and_parameter_has_an_optimization_policy
cryptanalysis::ecbench::runner::diagnostic_reap_publication_regression_tests::ok_without_observed_reap_cannot_mark_or_read_sidecars
cryptanalysis::ecbench::runner::diagnostic_reap_publication_regression_tests::publication_collision_preserves_previous_bytes_and_pending_failure
cryptanalysis::ic_framework::residual::tests::fused_residual_collector_recovers_and_halves_the_additions_per_step
cryptanalysis::ic_wide::tests::run_f2m4_is_the_one_word_pipeline
```

The two diagnostic publication failures reproduce the documented host limit:
this environment has neither `/proc/self/ns/time` nor
`/proc/self/timens_offsets`. The `ic_wide` mismatch also reproduces the prior
baseline (`13130` versus `3757`). The two additional current-main failures
were rerun exactly and reproduced independently: the optimization registry is
missing `ic.prime_orbit_relations_v1`, and the residual test reports fused
per-step cost `4.900233826968043`. These failures are retained as failures;
the passing standalone and focused gates are not substituted for the required
root result.
