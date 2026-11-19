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

## 2026-11-01 semantic-cut and quotient follow-up

Later work implemented the production semantic cut, exact equality-signal
quotient, symmetry-reduced raw rank, typed functional rewiring, and create-once
development runners. The retained result and artifact hashes are in
[`../prime_ic_elimination_20261101/RANK1_QUOTIENT_RESULT.md`](../prime_ic_elimination_20261101/RANK1_QUOTIENT_RESULT.md).

Focused controls include 48 synthetic quotient tensors checked against every
raw balanced split, direct raw contraction of a fanout fixture, metadata-
absence rejection, semantic-tag mutation, rewired graph replay, draw repeats
and seed mutations. The four held-out natural constructions reproduce exactly.

The full E1 protocol is still not frozen: the arbitrary interior-rewired null,
projected model counter, fresh-process final runner, power record, final ladder,
source commit and image digest remain open. No follow-up timing or speed claim
was admitted.
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
test independently checks its order-two certificate and rejects a changed
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
b441c7407f059ac6ead03992837c389bee3460d70ca2c74d678507e8d6aa8282  figures/rigorous-bound-chain.svg
5e6776ef6ba6616c032285422478a6bb509ca9dbc5032c843201b696543320a5  bounds-20261101.pdf
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

## 2026-11-18 semantic relation-rank follow-up

### Scope and source receipt

This follow-up addresses the encoding-independent hashed factor-base slot
relation from Bochum technical report v0.2, Definition 4, Lemmas 9--13,
Theorem 4 and Remark 2. The pinned PDF has SHA-256
`82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11`.
The repository research-KB wrapper was attempted first, but no KB executable
is installed in this environment. The result therefore cites the pinned
primary text and records the unavailable-KB receipt; it does not claim a KB
update.

### Lean and independent checker

The pinned Lean 4.35.0 project was built after adding the semantic relation
matrix, its decomposition set, and the rank/count obstruction:

```text
ELAN_HOME=/tmp/semantic-rank-elan \
PATH=/tmp/semantic-rank-elan/bin:$PATH lake build
```

Result: exit 0, 1,891 jobs. The repository proof gate reported `PASS 14 thm 0
trusted`. An explicit axiom audit of all five new declarations found only
`propext`, `Classical.choice`, and `Quot.sound`. A focused scan found no
`sorry`, `admit`, user `axiom`, or `native_decide` in
`TensorBoundaryBounds.lean`. The independent fresh checker command
`leanchecker --fresh TensorBoundaryBounds` exited 0.

Two theorem claim records were locally checked through the PCC verifier:

```text
PCC-20261118-4717cb: pass (Lean proof gate)
PCC-20261118-e6722e: pass (Lean proof gate)
```

Their signed PCC standing is still `unverified`; local kernel checks are not
reported as signed verification. [pcc:unverified]

### Rendered evidence

Graphviz rendered the matrix-scope split and updated implication graph. Typst
rendered `semantic-rank-20261118.pdf`; Pandoc with pdfLaTeX rerendered the
one-page `bounds-20261101.pdf`. Text extraction recovered the stated
inequalities and scope exclusions. Both PDFs and the updated implication graph
were rendered to images and visually inspected; they are legible and
unclipped.

```text
5fa58116aeb4b3efca8089ef411f560526cde5d7f3671af54dc449e25aec0d8d  semantic-rank-20261118.pdf
5e6776ef6ba6616c032285422478a6bb509ca9dbc5032c843201b696543320a5  bounds-20261101.pdf
b441c7407f059ac6ead03992837c389bee3460d70ca2c74d678507e8d6aa8282  figures/rigorous-bound-chain.svg
```

### Evidence boundary

The checked finite theorem is `rank_Q(A_S) <= #decompositions`; the source
threshold calculation gives expected rank below two in the specified regime.
This excludes an all-seed or nonnegligible-probability exponential lower bound
for a uniformly sampled source seed, but not an exceptional selected seed
sequence. [pcc:unverified] No equivalence with the frozen low-level BTNIR edge matrix is
assumed, and no conclusion is drawn for nonlinear, randomized, approximate,
or separator-avoiding algorithms. No benchmark or performance result was
generated.

### Repository-wide gates in the replacement sparse checkout

`benchrec.py gate` reported the pre-existing stale generated progress report;
this research-only change did not regenerate or cite benchmark records. The
required `cargo test --release --lib` was attempted with a temporary stable
Rust 1.100.0 toolchain after materializing the root source and path
dependencies. It failed during test compilation with 79 diagnostics: the
sparse checkout omits many `include_str!`/`include_bytes!` fixtures, and the
current branch also contains a duplicate test function named
`the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e`. No diagnostic
points to the changed Lean or report files. These failures remain failures;
the focused proof gates above are not substituted for the repository-wide
gate.

## 2026-11-18 no-subexponential-ECDLP goal follow-up

### Requested scope and formal boundary

The request was to disprove subexponential ECDLP through a Lean goal.  The
implemented goal takes `SolvesECDLP : (Nat -> Nat) -> Prop` as a parameter and
defines subexponential binary work by eventual domination at every positive
rational exponential rate.  Lean checks the sufficient implication from an
ECDLP-wide positive exponential work floor, and checks the registered
rank/message/work endpoint for a named exact-linear backend.  It also supplies
a countermodel showing that the semantic rank/count premises alone cannot
establish the universal goal. [pcc:unverified]

This does not define a complete real-ECDLP machine semantics or establish the
universal lower bound.  That broader requested conclusion remains open; the
missing hypothesis is a theorem giving every correct admitted ECDLP solver a
positive exponential work floor.

### Kernel checks

The final pinned project build completed successfully with 1,891 jobs.  The
repository proof gate reported:

```text
PASS 20 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

The focused hole scan found no `sorry`, `admit`, user `axiom`, or
`native_decide`.  Explicit axiom output for the six new theorems contains only
`propext`, with `Classical.choice` and `Quot.sound` additionally present in the
finite `PUnit` countermodel through standard finite-sum infrastructure.  The
independent command `lake env leanchecker --fresh TensorBoundaryBounds`
exited 0.

Three theorem claims pass the local PCC Lean verifier and retain unverified
signed standing:

```text
PCC-20261118-63eb26  sufficient ECDLP-wide exponential-work bridge
PCC-20261118-ed3d35  registered linear-rank architecture endpoint
PCC-20261118-06e657  semantic-premise countermodel
```

[pcc:unverified]

No performance run, ECDLP solve, benchmark record, progress point, or
leaderboard change was produced.

## 2026-11-18 fixed-arity universal-scheduler follow-up

The instance-gate assembly probe type-checked before the module edit. The final
pinned project build completed successfully with 2,153 jobs. The repository
proof gate reported:

```text
PASS 22 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

The focused source scan found no `sorry`, `admit`, `sorryAx`, custom `axiom`,
or `native_decide`. Explicit axiom output is:

```text
polynomialUniversalSchedulerIsSubexponential: [propext]
geometricSchedulerTailBound: [propext, Classical.choice, Quot.sound]
```

The independent command `lake env leanchecker --fresh TensorBoundaryBounds`
exited 0. The first invocation from the repository root failed because that
directory has no pinned Lake toolchain; the recorded successful command ran
from `research/tensor_boolean_breakthrough_20261031/lean`.

Two theorem claims pass the local PCC Lean verifier and retain unverified
signed standing:

```text
PCC-20261118-546652  polynomial universal-scheduler asymptotic endpoint
PCC-20261118-ed2042  exact geometric scheduler-tail sum
```

[pcc:unverified] Two earlier append-only claim attempts omitted the required
declaration hashes (`PCC-20261118-d64e6f` and `PCC-20261118-5c631f`); their
local verifiers fail closed and they are not cited as evidence.

## 2026-11-18 corrected finite coverage follow-up

The first-moment and event-intersection assembly probe type-checked before the
module edit. The final pinned project build completed successfully with 2,153
jobs, and the repository proof gate reported:

```text
PASS 24 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

Both new theorems have the standard axiom set `[propext, Classical.choice,
Quot.sound]`. The independent command
`lake env leanchecker --fresh TensorBoundaryBounds` exited 0. Claims
`PCC-20261118-07b50f` and `PCC-20261118-aee405` pass their local Lean
verifiers and retain unverified signed standing. [pcc:unverified]

The checked scope is finite counting only. The character-sum estimate and
energy Markov bound needed for the elliptic dilation family remain external
instance obligations.

## 2026-11-19 energy-composition follow-up

The factor-12 Markov and composed coverage assembly probe type-checked before
the module edit. The pinned project build completed successfully with 2,153
jobs, and the repository proof gate reported:

```text
PASS 26 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

Both new declarations have the standard axiom set `[propext,
Classical.choice, Quot.sound]`. The independent fresh checker exited 0.
Claims `PCC-20261119-1fd609` and `PCC-20261119-5bbc92` pass their local Lean
verifiers and retain unverified signed standing. [pcc:unverified]

The composed theorem leaves exactly two supplied finite-sum hypotheses: the
factor-base first moment and the total energy upper bound. Establishing those
for the dilation family remains outside this generic formalization.

## 2026-11-19 planted-diagonal counting follow-up

The polynomial-root, characteristic-polynomial, injective-encoding,
bounded-residue, finite-union and `7/8` survival assembly probes type-checked
before the module edits. The final pinned project build completed successfully
with 2,186 jobs, and the repository proof gate
reported:

```text
PASS 33 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

The root/counting declarations have the standard axiom set `[propext,
Classical.choice, Quot.sound]`; `zmodNatCastInjectiveOnFin` uses only
`[propext, Quot.sound]`. The independent command
`lake env leanchecker --fresh TensorBoundaryBounds` exited 0. Claims
`PCC-20261119-d2e9ce`, `PCC-20261119-b68c62`, `PCC-20261119-698c76`, and
`PCC-20261119-5e863b`, together with residue claims
`PCC-20261119-5adae9`, `PCC-20261119-7817f3`, and
`PCC-20261119-b9e430`, pass their local Lean verifiers and retain unverified
signed standing. [pcc:unverified]

The checked scope is the determinant root count, bounded natural-residue
injection and finite cardinality arithmetic. The paper-specific proof that each
failed planting yields one of the singular residual blocks, and the transcript
independence making those blocks independent of the shift, remain outside this
formalization.

### Pinned upstream v2 planting and round modules

The omitted `v2/lean` tree was materialized from exact source commit
`d1deb5abe008e981206ed58cbd83012b6afc3740`. After fetching the pinned Mathlib
cache, these declared targets built successfully:

```text
lake build ECDLPv2.L114_Plant  # 9,050 jobs
lake build ECDLPv2.L114_Main   # 9,057 jobs
```

An explicit audit imported `ECDLPv2.L114_Main` and printed axioms for
`det_sub_scalar`, `bad_nu_le`, `bad_nu_union`, `block_isUnit`,
`planted_kernel`, `double_count`, `trans_count_indep`, `round_failure_le`, and
`round_output`. Every declaration reports exactly `[propext,
Classical.choice, Quot.sound]`. Source hashes are:

```text
9e98de7e7347505427643dd57dc2be513a58cb1dd11c0a24fbceec3b6c74ac3f  L114_Plant.lean
378057e83147933551ff574be80e485220f867201a1db357f682fccc4b169109  L114_Trans.lean
f247462fc4a23d8e2f4134d768b3e7072e2aba2a263c249c2c9180c173a34b39  L114_Main.lean
```

This closes the previously reported transcript-independence and
failed-planting-to-singular-block formal gaps. The upstream round theorem is
still parameterized by explicit coverage, generation, cap and procedure
interfaces, and the expected sparse Wiedemann--Dixon bit-cost claim is not a
conclusion of these finite theorems. [pcc:unverified]

## 2026-11-19 fixed-numerator dense-route follow-up

An assembly probe first checked the generic conversion from a fixed numerator
divided by arity to the existing vanishing-rate family interface. The final
pinned project build completed successfully with 2,186 jobs, and the repository
proof gate reported:

```text
PASS 34 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

`fixedNumeratorCandidateFamilyHasVanishingRate` depends only on `[propext]`.
The independent command
`lake env leanchecker --fresh TensorBoundaryBounds` exited 0. Claim
`PCC-20261119-76300b` passes its local Lean verifier and retains unverified
signed standing. [pcc:unverified]

The theorem chooses `m = numerator * degree * b + 1` for a requested positive
rational exponent `a / b`. It therefore applies to every fixed numerator,
including the numerator `3` in Kaarelson's unrestricted dense Theorem 5. On the
paper's stated scope, that theorem covers every nonsingular prime-field curve,
arbitrary group order and structure, and every valid `P, Q`, using dense
elimination with expected time `p^(3/m+o(1))`. Consequently sparse
Wiedemann--Dixon is not an obligation for the qualitative subexponential
diagonal; it is needed only to retain the sharper planted exponent
`p^(2/m+o(1))`.

The checked local theorem is a quantifier and natural-arithmetic conversion. It
does not verify the paper-to-machine instantiation of Theorem 5, its concrete
bit costs, the curve-specific coverage estimates, or the universal
interpreter's implementation. Those remain explicit external obligations.

## 2026-11-19 pinned concrete dense-round follow-up

The exact pinned v2 tree contains concrete row, adaptive-cap and generation
modules beyond the previously audited abstract round. With the pinned Mathlib
cache, the following targets built together:

```text
lake build ECDLPv2.L116_Alg ECDLPv2.L116_Chernoff ECDLPv2.L116_Main
# Build completed successfully (9,067 jobs).
```

An explicit audit imported those targets and printed axioms for
`algSpec_row_rel`, `cap_bound`, `gen_two_prod`, `gen_two_explicit`,
`evG_surjective_of_closure`, `hsurj_of_pair`, and `genBound_primePow`. Every
declaration reports exactly `[propext, Classical.choice, Quot.sound]`. The
source hashes are:

```text
6fd6e5267b98ff3c730888835c3020528713b75e3db283e74b063a2f422d6667  L116_Alg.lean
d95c55f402b699f3e2a093d24b9d20190117dc458c6dc5567a457c5dc536ec70  L116_Chernoff.lean
e4d970303f43f75701347e50eef9f042dd9a337df7c95d1c2892fb8709f398bf  L116_Main.lean
```

This checks that the actual first-accepted-row collector has the required row
relations, the history-dependent oracle satisfies the finite Chernoff cap once
a coverage fraction is supplied, a generating pair makes tag evaluation
surjective, and the prime-power lattice generation hypothesis is discharged.
It does not supply the curve-specific coverage fraction, bind the decomposition
machine to `hOrc`, implement the `HowellSpec` procedure with bit costs, or
formalize the expected-time outer restart loop. [pcc:unverified]

### Uniform-generator expectation

The additional exact target `ECDLPv2.L118_Tot` built successfully with 9,065
jobs. Axiom output for `prod_ratio_le`, `card_primeFactors_le_log`,
`totient_ge`, `gen_two_log`, and `gen_two_trials` is exactly `[propext,
Classical.choice, Quot.sound]`. Its source hash is:

```text
f7a1c4817f76a5f7a463a0f6f3e84798c05f6132cef53b904d41f0112cead4de  L118_Tot.lean
```

`gen_two_trials` proves that two uniform samples from a rank-two finite abelian
group require at most `(16/5) * (1 + log₂ |G|)` trials in reciprocal-probability
form. The concrete curve algorithm samples affine points with a stated
near-uniform lower bound rather than sampling all of `G` uniformly. A
constant-factor transfer from that sampler to the uniform theorem is still an
external obligation. [pcc:unverified]

### Near-uniform pair-event transfer

An assembly probe for the division-free weighted pair count type-checked before
the module edit. The pinned project build completed with 2,186 jobs, and the
repository proof gate reported:

```text
PASS 36 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

`weightedPairEventLower` and `nearUniformPairEventTransfer` have the standard
axiom set `[propext, Classical.choice, Quot.sound]`. The independent command
`lake env leanchecker --fresh TensorBoundaryBounds` exited 0. Claims
`PCC-20261119-da3506` and `PCC-20261119-a2fecb` pass their local Lean verifiers
and retain unverified signed standing. [pcc:unverified]

The endpoint proves the exact natural inequality [pcc:unverified]

```text
total^2 * goodPairs <= distortion^2 * |Output|^2 * weightedGoodPairs
```

from a common fibre floor and
`total <= distortion * |Output| * lower`. It makes no division or positivity
assumptions. The curve sampler still needs a concrete finite seed encoding and
a positive fibre for the identity; adding a separate identity branch preserves
the asymptotic cost but is not instantiated by this generic theorem.

[pcc:unverified] Two claim records created without the required declaration
hashes (`PCC-20261119-fa8515` and `PCC-20261119-8212b4`) fail closed, remain in
the append-only ledger, and are not cited as evidence.

## 2026-11-19 dense cubic exponent follow-up

The natural-power assembly probe type-checked before the module edit. The
pinned project build completed with 2,186 jobs, and the proof gate reported:

```text
PASS 37 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

`denseCubicRowBoundAtFixedArity` depends only on `[propext, Quot.sound]`. The
independent fresh checker exited 0. Claim `PCC-20261119-77d40e` passes its local
Lean verifier and retains unverified signed standing. [pcc:unverified]

The theorem proves `rows^(3m) <= 3^(3m) p^3` from `rows <= 3X` and `X^m <= p`.
[pcc:unverified]
This is the exact natural form of the dense `p^(3/m)` exponent. It does not
implement the Howell procedure, count its ring operations, or bound operand
bit lengths.

## 2026-11-19 Bird planted-dense fallback follow-up

The pinned Mathlib dependency is commit
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`. Its Bird sources hash to:

```text
fa690f7f31bdb3108c61c7c4aca62f56182732728a8928e793c2a5b743a4ba54  Bird/Defs.lean
e36d1901d2279de39d4cb702625ac1c6defaae30eaaba372260c8a1551369fc1  Bird/Correctness.lean
```

An assembly probe established the concrete `ZMod` inverse statement before it
was added to the module. The pinned project build completed with 2,188 jobs,
and the proof gate reported:

```text
PASS 44 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

The axiom readback is `[propext, Classical.choice, Quot.sound]` for
`det_eq_birdMatrixDet`, `birdAdjugate_eq_adjugate`, and
`birdZModInverse_eq_inv`, and `[propext, Quot.sound]` for
`birdSixthPowerRowBoundAtFixedArity`. The independent fresh checker exited 0.
Claims `PCC-20261119-7f673b`, `PCC-20261119-de3426`,
`PCC-20261119-cfa7f9`, and `PCC-20261119-598c61` pass their local Lean
verifiers and retain unverified signed standing. [pcc:unverified]

The checked content is determinant value correctness, cofactor/adjugate
correctness, equality of an executable `ZMod` Bird inverse with the canonical
matrix inverse under `IsUnit A.det`, and the exact numerator-6 natural-power
bound. The advertised `O(n^4)` determinant runtime is not kernel-checked by
Mathlib's function-valued implementation. An eager array refinement and
operation counter are still required before multiplying that cost by `n^2`
cofactors and claiming an executable `O(n^6)` planted inverse. Operand bit
costs and the upstream planted-block binding also remain open. [pcc:unverified]

The first inverse claim record, `PCC-20261119-46d8c7`, omitted the mandatory
declaration hash and fails closed. It remains uncited in the append-only
ledger; corrected hash-bound claim `PCC-20261119-598c61` was issued and
verified. [pcc:unverified]

## 2026-11-19 eager Bird operation-count follow-up

The array-backed value-refinement probe and the instrumented scalar-count probe
both type-checked before integration. The pinned project build again completed
with 2,188 jobs, and the proof gate reported:

```text
PASS 77 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

`eagerBirdDet_eq_det` proves that materializing each recurrence state in a
length-`n^2` vector preserves the determinant. `countedBirdStepEntry_value`
connects the instrumented scalar formula to `BirdDet.stepEntry`, and
`countedBirdStepEntry_ops` proves the exact `3(n-i-1)+3` count. Summation gives
`eagerBirdDetRingOps_le_quartic`; multiplying by all `n^2` cofactors gives
`eagerBirdAdjugateRingOps_le_sixth`. All audited declarations use only
`propext`, `Classical.choice`, and `Quot.sound` (some omit
`Classical.choice`). The independent fresh checker exited 0.
[pcc:unverified]

Claims `PCC-20261119-2b2b04`, `PCC-20261119-8d6681`,
`PCC-20261119-fa515d`, and `PCC-20261119-84b10b` pass their local Lean
verifiers and retain unverified signed standing. The model counts scalar-ring
addition, multiplication, and negation. It deliberately excludes array
allocation, integer indexing, operand bit cost, concrete `ZMod` inversion cost,
and the binding of the upstream planted block. [pcc:unverified]

## 2026-11-19 executable planted-kernel follow-up

The upstream audit showed that `L114_Phase.kerMat` is noncomputable precisely
at the use of the canonical matrix inverse in `(-R B⁻¹ | I)`. The local
replacement `executableBirdPlantedKernel` uses `birdZModInverse B` instead.
Under the same hypothesis `IsUnit B.det`, Lean proves equality with the
canonical construction, annihilation of `Matrix.fromRows B R`, and the
identity right-block law consumed by upstream conditional uniformity.
[pcc:unverified]

The pinned project build completed with 2,189 jobs and the proof gate reported:

```text
PASS 82 thm 0 trusted research/tensor_boolean_breakthrough_20261031/lean
```

All four executable-kernel declarations have axiom set `[propext,
Classical.choice, Quot.sound]`; the independent fresh checker exited 0. Claim
`PCC-20261119-d0d20c` passes its local Lean verifier and retains unverified
signed standing. Matrix-multiplication cost, array/indexing overhead, scalar
bit costs, and the direct cross-project import adapter remain outside this
claim. [pcc:unverified]

The follow-up adapter `executableBirdKerMat` uses the exact upstream argument
types: integer `Ttop`, integer `Rint`, integer shift `nu`, and modulus `q`.
`executableBirdKerMat_spec` proves both block laws after the same
`Int.castRingHom (ZMod q)` reduction used in `L114_Phase.kerMat`. The project
gate then reported `PASS 84 thm 0 trusted`; the declaration has standard axiom
output and the fresh checker exited 0. Claim `PCC-20261119-0d4820` passes its
local Lean verifier. Editing and rebuilding the external pinned source tree is
not claimed. [pcc:unverified]

## 2026-11-19 conditional Bird bit-work boundary

The complete inverse budget now includes the determinant used for the scalar
denominator, every adjugate row-update determinant, and the final `n^2` scalar
multiplications. Lean proves the following bound. [pcc:unverified]

```text
eagerBirdZModInverseRingOps n <=
  4 * (n + 1)^6 + 4 * (n + 1)^4 + n * n.
```

`eagerBirdZModInverseBitWork_le` then charges each of those operations a
supplied `C(bits(q)+1)^d` bound and separately charges the single scalar
inverse a supplied `D(bits(q)+1)^e` bound. These are explicit hypotheses, not
claims about evaluator runtime: the pinned `ZMod.inv` reduces to `Nat.gcdA`,
and the pinned library has no proved Turing-machine or other bit-runtime
theorem for that extended-GCD implementation. Array allocation and indexing
also remain outside the model. [pcc:unverified]

The pinned build completed with 2,189 jobs. The repository proof gate reported
`PASS 87 thm 0 trusted`; both new laws use only `propext`,
`Classical.choice`, and `Quot.sound`, and
`lake env leanchecker --fresh TensorBoundaryBounds` exited 0. Hash-bound claim
`PCC-20261119-e1a1a1` passes its local Lean verifier.
[pcc:unverified]

## 2026-11-19 counted rectangular postprocessing

`denseDot` is a structurally recursive finite-vector dot product.
`denseDot_eq_sum` identifies it with the finite sum used by `Matrix.mul`;
`countedDenseDot_value` erases its counter to that value, and
`countedDenseDot_ops` gives exactly `2f` operations for inner dimension `f`.
Summing all entry counters proves the materialized `s` by `f` product costs
exactly `2sf^2` charged operations. Including the `sf` negations gives the
following bound. [pcc:unverified]

```text
eagerBirdPlantedKernelRingOps f s <=
  4(f+1)^6 + 4(f+1)^4 + f^2 + 2sf^2 + sf.
```

The corresponding conditional bit-work theorem charges the same explicit
polynomial scalar costs as the inverse theorem. The pinned build completed
with 2,189 jobs; the proof gate reports `PASS 103 thm 0 trusted`, the seven
audited public laws use only standard Lean axioms, and the fresh checker exits
0. Claim `PCC-20261119-af7e06` passes its local Lean verifier. Memory allocation
and indexing remain outside the model. [pcc:unverified]

## 2026-11-19 identity-inclusive sampler adapter

`identityInclusiveWeight` adds exactly `lower` seeds mapping to a distinguished
identity. From a `lower` fibre floor on every nonidentity output and a base
total bounded by `distortion * (|Output|-1) * lower`, Lean proves a global
fibre floor and total at most `distortion * |Output| * lower`. The composed
endpoint then applies the existing squared-distortion pair-event transfer.
[pcc:unverified]

The pinned build completed with 2,189 jobs; the proof gate reports
`PASS 108 thm 0 trusted`, the four audited laws use only standard Lean axioms,
and the fresh checker exits 0. The concrete affine fibre counts remain an
external curve-specific premise. Claim `PCC-20261119-4e2d4b` passes its local
Lean verifier. [pcc:unverified]

## 2026-11-19 exact finite rejection sampling

The one-trial interface `Seed -> Option Output` now has checked double-counting
and retry laws. `rejectionSamplerWeight_sum` partitions accepted seeds by
output; equal fibres give the exact accepted cardinality and exact pair-event
weight. `uniqueAcceptedSeedWeight_eq_one` reduces singleton fibres to a
successful encoder law and uniqueness of every accepted seed. The geometric
retry tail sums exactly to `total / accepted`, and a `1/cap` success density
bounds this expectation by `cap`. [pcc:unverified]

The pinned build completed with 2,189 jobs; the proof gate reports
`PASS 117 thm 0 trusted`, all six audited public laws use only standard Lean
axioms, and the fresh checker exits 0. The concrete signed-x decoder and Hasse
cardinality specialization are not claimed. Claims `PCC-20261119-d11aae`,
`PCC-20261119-16f188`, and `PCC-20261119-4fe45b` pass their local Lean
verifiers. [pcc:unverified]

## 2026-11-19 concrete full-curve signed-x sampler

The generic rejection interface is now instantiated over Mathlib affine
Weierstrass points. A chosen nonsingular root and its `negY` consume the two
sign seeds; when they coincide, one sign is rejected. Lean proves successful
decoding and uniqueness, hence exactly one accepted seed per affine point.
`Option AffineSamplerCoordinates` is explicitly equivalent to `W.Point`.
Adding one distinguished identity seed therefore gives exactly one accepted
seed per full curve point and exact uniformity for every pair event.
[pcc:unverified]

The full retry series has expectation `(2|K|+1)/|E(K)|`; the checked
four-trial corollary assumes only `2|K|+1 <= 4|E(K)|`. The pinned project now
builds 2,326 jobs. The proof gate reports `PASS 140 thm 0 trusted`, all audited
sampler laws use only standard Lean axioms, and the fresh checker exits 0.
Mathlib supplies no Hasse point-count theorem in this pinned tree, and an
executable square-root bit-cost theorem remains external. Claims
`PCC-20261119-e93da3`, `PCC-20261119-2b8ae7`, and `PCC-20261119-e67bd2` pass
their local Lean verifiers. [pcc:unverified]

## 2026-11-19 Hasse arithmetic and executable three-mod-four roots

The sampler's remaining cardinality premise is now separated from the missing
geometric theorem. `hasseSquareImpliesHalfFieldTrace` proves that, for
`q >= 16`, `t^2 <= 4q` implies `2t <= q`.
`hasseTraceBoundGivesSamplerFour` then combines this with
`q+1 <= #E+t` to obtain `2q+1 <= 4#E`, and the curve-level endpoint composes
that inequality with the exact retry theorem. The pinned dependency audit
found point-count definitions but no Hasse bound, so the curve-specific Hasse
input remains explicit. [pcc:unverified]

For fields with `q % 4 = 3`, `threeModFourSqrt` computes
`a^((q+1)/4)`. Lean proves this squares to every supplied quadratic residue.
The independently defined `countedBinaryPow` erases to ordinary exponentiation
and uses at most two field multiplications per exponent bit; consequently the
root uses at most `2 * Nat.size q` multiplications. Its conditional bit-work
theorem charges only a supplied polynomial cost for one field multiplication.
On elliptic short-form curves with `a1=a3=0`,
`shortThreeModFourRoot_spec` proves the computed root satisfies the exact
nonsingular-root interface used by the sampler. General prime-field square
roots remain separate. [pcc:unverified]

The pinned project builds 2,335 jobs. The repository proof gate reports
`PASS 160 thm 0 trusted`; explicit axiom audits of the Hasse bridge, binary
power, square-root cost, and short-curve root laws contain only `propext`,
`Classical.choice`, and `Quot.sound`. A fresh independent checker replay exits
0. Hash-bound claims `PCC-20261119-c1ad13`, `PCC-20261119-e5e1f9`, and
`PCC-20261119-54ee4b` pass their local Lean verifiers. The first three claim
records (`PCC-20261119-55e0bd`, `PCC-20261119-0b3f70`, and
`PCC-20261119-b73c0b`) omitted mandatory declaration hashes, fail closed, and
remain uncited. [pcc:unverified]

## 2026-11-19 source-pinned Atlas Hasse closure

The project now pins `facebookresearch/atlas-lean` commit
`a4557099d45c620a038e90a9c4b4d9ab08294b81`. The root project keeps its
original Mathlib commit `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`, which
is two days newer than Atlas's v4.35.0 Mathlib tag; no existing dependency was
upgraded. Atlas's `HasseBoundWanted.hasse_bound` builds its Manin degree and
twist proof and states
`((q + 1 - #E : Int)^2 <= 4q)` for every finite-field elliptic Weierstrass
curve. [pcc:unverified]

`integerHasseGivesSamplerFour` converts that integral theorem to
`2q+1 <= 4#E` for `q >= 16`.
`affineFullSignedTrialExpectation_le_four_of_atlasHasse` then proves the exact
full-curve sampler has expected retry count at most four with no curve-count
premise in its statement. The expanded pinned build completes 2,738 jobs; the
proof gate reports `PASS 162 thm 0 trusted`. Axiom audits of the imported Hasse
theorem and both local compositions list only `propext`, `Classical.choice`,
and `Quot.sound`; the seven directly imported Atlas Hasse/Manin source files
contain no `sorry`, `admit`, user `axiom`, or `native_decide`. A fresh checker
replay exits 0. Hash-bound claim `PCC-20261119-c7c015` passes its local Lean
verifier. [pcc:unverified]

## 2026-11-19 fixed-curve character sums and first moment

The concrete dilation analysis now counts the actual quadratic fibres above a
finite set of nonzero abscissas. Lean proves the square-map pushforward
identity, identifies the cubic character sum with an elliptic point-count
trace, and reduces the quartic sum to a second elliptic curve when `B != 0`.
The `B = 0` branch is closed separately by an explicit parametrization of
`Y^2 = X^2 + A`. Atlas Hasse therefore supplies both character-sum bounds
without a generic unproved Weil premise. [pcc:unverified]

`dilationFactorBaseFirstMoment` proves the exact scaled `3/4` mean bound under
integral radius hypotheses. `dilationLargeFactorBaseSeeds_of_card_ge_324`
specializes those hypotheses with radius `2 * (Nat.sqrt q + 1)`: over every
odd finite field with `q >= 324`, at least one sixth of nonzero dilations have
signed factor-base size at least half the abscissa-set size. Hash-bound claim
`PCC-20261119-a5a8e2` passes its local Lean verifier. The earlier radius-form
claim `PCC-20261119-191546` also passes; initial claim
`PCC-20261119-d93617` omitted its declaration hash, fails closed, and remains
uncited. [pcc:unverified]

`dilationAndEnergyGiveOneTwelfthGoodSeeds_of_card_ge_324` composes the clean
first-moment theorem with finite Markov counting. Its only analytic premise is
the displayed total-energy upper bound; corrected Semaev nondegeneracy is
still required to instantiate that premise for the ECDLP collector. This is
not yet an end-to-end subexponential ECDLP theorem. [pcc:unverified]

The pinned project build completes 2,738 jobs. The repository proof gate
reports `PASS 212 thm 0 trusted`; the new clean first-moment theorem, concrete
one-twelfth endpoint, and both Hasse character-sum bounds depend only on
`propext`, `Classical.choice`, and `Quot.sound`. A final-source independent
`lake env leanchecker --fresh TensorBoundaryBounds` replay exits 0.
[pcc:unverified]

## 2026-11-19 corrected Semaev nonvanishing components

`shortCubicPolynomial_scale_eq_iff` expands the scaling identity for
`X^3 + AX + B`, and `shortCubicPolynomial_scale_classification` derives the
two coefficient cases used in Lemma 5.4. In the `A = 0` branch,
`zmodNoFullCubicOrbitInShortInterval` proves that three positive
representatives from one cubic orbit cannot all lie below `X` when `3X < p`.
[pcc:unverified]

`characterProjector_isolates_term` proves the abstract finite character
projector: supplied orthogonality and transformed relations isolate
`|Gamma| * n_i` times the selected point. The corrected final contradiction is
`algebraic_ne_const_mul_transcendental`: an element algebraic over the
constant field cannot equal a nonzero constant multiple of a transcendental
parameter. [pcc:unverified]

`separatingSignCharacters_orthogonality` now proves the finite orthogonality
table from multiplicativity, sign-square-one, and one separating group element
for every distinct character pair. Composing it with the projector gives
`separatingSignCharacterProjector_isolates_term`. Hash-bound claim
`PCC-20261119-1ae928` passes its local Lean verifier. [pcc:unverified]

`unitMulSquare_of_isSquare_algebraMap_fractionRing` proves generically over a
unique-factorization domain that a base element square in its fraction field
is a unit times a square. Its polynomial specialization identifies that unit
as a nonzero constant. `shortCubicPolynomial_squarefree` checks the explicit
Bezout identity for the cubic and its derivative, and
`shortCubicPolynomial_fraction_square_product_classification` composes these
facts with squarefree divisibility and the coefficient classification. Claims
`PCC-20261119-cd776f` and `PCC-20261119-fc17e6` pass their local Lean verifiers.
[pcc:unverified]

`exists_galoisAut_sqrt_ratio_ne_of_not_isSquare_mul` uses the finite-Galois
fixed-field theorem to prove that nonsquare `a*b` forces different Galois
transformation ratios on chosen square roots of `a` and `b`.
`galoisSqrtSign_multiplicative` turns those ratios into integer-valued
multiplicative signs, and `galoisSqrtSigns_separate_of_not_isSquare_mul`
proves pairwise separation. The endpoint
`galoisSignCharacterProjector_isolates_term` composes these facts with the
checked character projector. Hash-bound claim `PCC-20261119-545ba9` records
this endpoint and passes its local Lean verifier. The complete pinned project
build succeeds with 9,086 jobs, and the repository proof gate reports
`PASS 251 thm 0 trusted`. A final-source independent
`lake env leanchecker --fresh TensorBoundaryBounds` replay exits 0.
[pcc:unverified]

## 2026-11-19 canonical quadratic splitting-field projector

`quadraticRadicandPolynomial` is the product of the indexed factors
`X^2 - a_i`. Lean proves each factor and their product nonzero, proves
coprimality for distinct radicands, derives radicand injectivity from the
pairwise nonsquare-product hypothesis, and proves the product separable in
odd characteristic. `quadraticRadicandSplittingField_isGalois` then supplies
the finite-Galois instance for the canonical splitting field. The selected
`quadraticRadicandRoot` is proved to square to its indexed radicand and to be
nonzero whenever that radicand is nonzero. [pcc:unverified]

`affinePointGaloisAddEquiv` defines the additive Galois action on a base-changed
elliptic curve, and `affinePointGaloisAddEquiv_eigenvector` proves that on a
short-form curve each lifted point transforms by its square-root sign.
`transformedRelations_of_addEquiv_eigenvectors` derives every character-twisted
relation from one original zero-sum relation. The endpoint
`quadraticSplittingFieldAffinePointProjector_isolates_term` composes these
facts with the checked Kummer projector. Claim `PCC-20261119-ae2460` binds the
endpoint and passes its local verifier. The complete pinned project build
succeeds with 9,086 jobs, and the proof gate reports
`PASS 268 thm 0 trusted`. A final-source independent
`lake env leanchecker --fresh TensorBoundaryBounds` replay exits 0.
[pcc:unverified]

This does not prove Lemma N' or subexponential ECDLP. The remaining
specialization must construct the Semaev radicands and lifted original point
relation and prove radicand nonvanishing, pairwise nonsquareness and point
nonsingularity. Torsion-to-division-polynomial vanishing, the `A = 0` CM
finite-kernel step, specialized polynomial construction/degree/vanishing, and
collector/runtime assembly remain open. [pcc:unverified]

## 2026-11-19 prime-field interval radicand specialization

`shortCubicPolynomial_scaled_pair_fraction_square_classification` normalizes
the product of `l(left*X)` and `l(right*X)` by the left scale and invokes the
checked rational-function square-product classification. In the `A != 0`
branch a square product forces `right/left = -1`.
`zmodPositivePairRatio_ne_neg_one` proves that positive representatives whose
sum is below the prime modulus cannot have that quotient. [pcc:unverified]

`shortCubicFunctionRadicand` packages the scaled cubic in the rational
function field and its nonzero theorem checks the leading coefficient.
`zmodShortCubicFunctionRadicands_pairwise_not_isSquare_of_A_ne_zero` then
proves the full indexed pairwise-nonsquare premise from positive injective
interval scales. Claim `PCC-20261119-4e1bc6` binds this endpoint and its local
verifier passes. The complete project build succeeds with 9,086 jobs and the
proof gate reports `PASS 277 thm 0 trusted`. A final-source independent
`lake env leanchecker --fresh TensorBoundaryBounds` replay exits 0.
[pcc:unverified]

This closes the radicand nonzero/pairwise-nonsquare premise only for the
`A != 0` branch. The original lifted Semaev point relation, its exact
nonsingularity composition, the `A = 0` collision/CM argument,
torsion-to-division-polynomial vanishing and runtime assembly remain open.
[pcc:unverified]

## 2026-11-19 cubic-CM additive norm endpoint

`shortCM_nonsingular`, `shortCM_slope`, `shortCM_addX`, and `shortCM_addY`
check the coordinate identities for `(x,y) -> (omega*x,y)` on
`Y^2 = X^3 + B`. `shortCMPointMap_add` handles the identity, inverse-point,
secant, and tangent cases and packages the result as `shortCMEndomorphism`.
`shortCM_orbit_sum_eq_zero` then proves the pointwise endomorphism identity
`zeta^2 + zeta + 1 = 0`, including the tangent case at `X = 0`.
[pcc:unverified]

`cubicCMRelation_implies_norm_torsion` carries out the abstract Eisenstein
norm calculation, and `shortCMLinearRelation_implies_norm_torsion`
specializes it: `a*P + b*zeta(P) = 0` forces
`(a^2 - a*b + b^2)*P = 0`. Claim `PCC-20261119-cbf30e` binds that endpoint.
The complete pinned project build succeeds with 9,086 jobs and the proof gate
reports `PASS 293 thm 0 trusted`. [pcc:unverified]

This removes the unproved finite-kernel shortcut from the CM step. It does
not construct the two-coefficient relation from the `A = 0` interval
collision, prove torsion-to-division-polynomial vanishing, construct the
specialized Semaev polynomial, or establish the final runtime theorem.
[pcc:unverified]

The complete pinned project build succeeds with 9,086 jobs. The repository
proof gate reports `PASS 244 thm 0 trusted`. [pcc:unverified]

Hash-bound claims `PCC-20261119-603d9c`, `PCC-20261119-0f3ed3`,
`PCC-20261119-31e2b5`, and `PCC-20261119-752c5b` pass their local Lean
verifiers. These statements do not yet prove Lemma N': specialized finite
Galois square-root data and transformed curve relations,
torsion-to-division-polynomial vanishing, and the CM finite-kernel
specialization remain open.
[pcc:unverified]

The pinned project build completes 2,738 jobs. The repository proof gate
reports `PASS 226 thm 0 trusted`; every new audited declaration uses only
standard Lean axioms. A final-source independent
`lake env leanchecker --fresh TensorBoundaryBounds` replay exits 0.
[pcc:unverified]

## 2026-11-19 inverse-square polynomial root-count bridge

`polynomialPullbackBadSetCardLe` bounds a finite bad set pulled back through a
bounded-fibre parameter map. `inverseSquareFiberCard_le_two` specializes the
fibre bound to `u ↦ u⁻²`, and `inverseSquarePolynomialBadSetCardLe` gives
the exact `2D` root-count conclusion for one polynomial. The uniform
`InverseSquarePolynomialCertificate` interface and
`inverseSquarePolynomialCertificate_card_le` endpoint quantify this over all
tuples. [pcc:unverified]

This verifies the elementary cardinality reduction used by upstream
`DilationNondeg`. It does not construct a Semaev polynomial or prove that its
specialization is nonzero; those remain explicit premises. Hash-bound claim
`PCC-20261119-998f26` passes its local Lean verifier. [pcc:unverified]

The pinned project build completes 2,738 jobs. The repository proof gate
reports `PASS 216 thm 0 trusted`; all four new root-count laws depend only on
`propext`, `Classical.choice`, and `Quot.sound`. A final-source independent
`lake env leanchecker --fresh TensorBoundaryBounds` replay exits 0.
[pcc:unverified]

## 2026-11-19 exact upstream dilation and average-energy bridge

The local project now pins `fast-ecdlp` commit
`d1deb5abe008e981206ed58cbd83012b6afc3740` as the `ECDLP` dependency while
retaining Mathlib commit `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.
Targeted builds of `ECDLP.L49_Dil` and `ECDLP.L49_Main` complete with 9,073
and 9,075 jobs respectively. The package default target is not claimed: it
also reaches unrelated `ECDLP.L35_Challenge`, which fails under this retained
Mathlib revision because its `strings` definition is not marked
noncomputable. [pcc:unverified]

`unitsInverseSquarePolynomialBadSetCardLe` supplies the exact root count over
the upstream unit-group parameter type. `UpstreamDilationPolynomialCertificate`
uses the precise upstream tuple and bad-point predicates.
`upstreamDilationPolynomialFamily_implies_nondeg` proves the actual
`ECDLP.L49.DilationNondeg` proposition from construction, nonvanishing,
degree, and bad-dilation vanishing of one supplied polynomial family.
`upstreamDilationPolynomialCertificate_implies_avg_energy` composes this with
upstream `avg_energy_dil`, preserving its original `Eng` and `Mdil`
definitions. Claims `PCC-20261119-1bad7c` and `PCC-20261119-a2b707` pass their
local Lean verifiers. [pcc:unverified]

`divisionPolynomialRoot_isAlgebraic` uses Mathlib's nonzero leading-term
theorem for division polynomials, and
`divisionPolynomial_ne_zero_at_const_mul_transcendental` proves that such a
polynomial cannot vanish at a nonzero constant multiple of a transcendental
parameter. Claim `PCC-20261119-a69a2d` passes locally. This does not supply the
missing group-law theorem that a nonzero torsion point forces the indexed
division polynomial to vanish at its x-coordinate. [pcc:unverified]

All three claim verifiers pass. The repository proof gate reports
`PASS 233 thm 0 trusted`; a final-source independent
`lake env leanchecker --fresh TensorBoundaryBounds` replay exits 0. The PCC
ledger contains 1,431 entries and verifies after serializing the three new
tail records. The complete local project build succeeds with 9,086 jobs.
[pcc:unverified]

## 2026-11-19 A-zero pair-collision composition

`twoCollisionSignCharacterProjector_isolates_pair` proves that one duplicated
sign-character class contributes exactly its two point terms while all
characters separated at one group element contribute zero.
`shortCMPoints_signedOrbit_of_sq_eq` derives the signed CM partner directly
from cubic `X` scaling and equality of the two `Y` squares.
`shortCubicPolynomial_scaled_pair_CM_ratio_of_isSquare` constructs the
nonidentity cubic ratio and its quadratic relation from the `A = 0`
square-product collision. [pcc:unverified]

`shortCubicPairCollision_implies_CMNormTorsion` composes those facts with the
checked CM endomorphism and norm calculation. Its conclusion exhibits the
cubic ratio and square-root sign and proves that the corresponding explicit
Eisenstein norm annihilates the chosen point. Claim
`PCC-20261119-d842a3` binds this endpoint. The full 9,086-job build succeeds
and the proof gate reports `PASS 301 thm 0 trusted`. A final-source
`lake env leanchecker --fresh TensorBoundaryBounds` replay returns
`FRESH_CHECKER_EXIT=0`. [pcc:unverified]

The remaining `A = 0` work is the Semaev-specific instantiation of the
duplicated-versus-separated Galois sign characters and the transformed
relations. Torsion-to-division-polynomial vanishing, specialized polynomial
construction/degree/vanishing, and runtime assembly also remain open.
[pcc:unverified]

## 2026-11-19 A-zero interval Galois composition

`galoisSqrtSigns_eq_of_isSquare_mul` proves the exact collision criterion:
when two base-field radicands have square product, all automorphisms give the
selected roots the same sign. The complementary nonsquare theorem supplies a
separating automorphism. [pcc:unverified]

`zmodShortCubicFunctionRadicand_mul_not_isSquare_of_A_eq_zero_outside_partner`
proves the missing interval classification. Starting from one square-product
partner, any further square-product index has the conjugate cubic ratio and
therefore completes a full three-element orbit. This contradicts the checked
`zmodNoFullCubicOrbitInShortInterval` theorem under `3X < p`.
[pcc:unverified]

`zmodShortCubicIntervalGaloisPairCollision_implies_CMNormTorsion` composes
that classification with the actual finite-Galois signs and the checked
short-CM endpoint. Its conclusion exhibits the nonidentity cubic ratio, the
point sign, and the explicit Eisenstein norm annihilating the chosen point.
The complete pinned project build succeeds with 9,086 jobs. [pcc:unverified]
The repository proof gate reports `PASS 310 thm 0 trusted`.
[pcc:unverified]
Claim `PCC-20261119-f444b9` binds the endpoint and passes its local Lean
verifier; signed standing remains unverified. [pcc:unverified]
A committed-source `lake env leanchecker --fresh TensorBoundaryBounds` replay
at `21345614` returns `FRESH_CHECKER_EXIT=0`. [pcc:unverified]

This closes the former duplicated-versus-separated character obligation for
the stated interval model. It still assumes the selected square roots,
lifted-point nonsingularity, point identifications, and all Galois-transformed
relations. The Semaev construction, torsion-to-division-polynomial vanishing,
and final algorithm/runtime theorem remain open. [pcc:unverified]
