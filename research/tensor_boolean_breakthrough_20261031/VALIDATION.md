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
