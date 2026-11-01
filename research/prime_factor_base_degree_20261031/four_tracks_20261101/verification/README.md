# Verification receipts and limits

This study runs formal-proof builds and exact native unit correctness checks.
No CAS timing, relation-yield, F4-degree, exponent or rank experiment ran.
The reference proof packages and `tools/lean/AuditAxioms.lean` remain unchanged.

## Accepted command

```sh
env PATH=/workspace/prime-degree-lean/bin:/workspace/n83-rustup/toolchains/1.94.1-x86_64-unknown-linux-gnu/bin:$PATH LEAN_NUM_THREADS=4 RUSTUP_HOME=/workspace/n83-rustup RUSTUP_TOOLCHAIN=1.94.1-x86_64-unknown-linux-gnu CARGO_HOME=/workspace/n83-cargo bash verify.sh verification/final
```

Accepted result: exit0, stage complete; **372 declarations in28 modules,
zero failures and zero trusted/native exceptions**. All71 input hashes match,
ten new modules pass warnings-as-errors, all seven release tests pass,
and the fixed21-system export replay is byte-identical.

The final status and full theorem census are in `final/status.json` and
`final/axioms.log`. `verify.sh` explicitly builds all three local reference
libraries and the new project before the whole transitive auditor. Ten new
modules also compile with warnings-as-errors. The final native receipt covers
release clippy/formatting, seven release correctness tests and byte-identical
21-system export replay, including uncompressed metadata and msolve inputs.
No accepted source contains sorry/admit/custom axioms/native_decide.
The only permitted foundations are propext, Classical.choice and Quot.sound.
`lean4checker` is not installed, so no separate fresh-checker pass is claimed.

`inputs.sha256` binds71 actual sources/inputs including new proofs/native code,
all old local proof modules, the inherited fixture and unchanged axiom auditor.
The Lean/Rust version files, actual Lean binary hash, all nine expected/actual
Git dependency revisions, and source parent commit are separate records. Inputs
were checked before accepting the result; the parent is not misreported as the
commit of then-uncommitted new source. The final Git tree binds the report and
receipts. Final replay JSON and the raw native log are gzip-compressed after checking to meet artifact
limits; decompress it to reproduce the uncompressed comparison.

## Preserved failures

`development-records.tar.gz` preserves recorded build-attempt01–23 logs,
available failed source snapshots, native checks/export logs and rendering logs.
The intermediate `recodedtop16.log` passes before adding the remaining lower
lemmas. Failed build diagnostics may mention sorryAx; those are not accepted
modules. Standard proof development issues included finite Finsupp decisions,
Fin numeral simplification, overly broad polynomial simplifiers, coefficient
functional elaboration, implicit polynomial coefficient types, and associative
rewrites in homogeneous expressions. All final claimed sources pass the actual
whole audit. Some source snapshots are unavailable; no reconstruction of an
unrecorded failed source is claimed.

`attempt01/` preserves the first final-wrapper failure and its exact verifier:
the new import chain did not build two non-imported reference modules, so the
whole audit refused unknown module CanonicalFixture. That attempt completed
all ten warnings-as-errors checks but passed no whole-audit census. The final
verifier explicitly builds the full reference libraries, including
CanonicalFixture and PencilElimination. The auditor was not weakened.

The initial Lake update wrote the pinned manifest but its Mathlib post-hook
could not use the read-only default cache path. Source-built dependencies
already available at the pinned revisions were reused; no successful cache
preflight or fresh remote dependency download is claimed. Conductor was absent
(exit127, fail-open under repository instructions). A local artifact-rendering
venv from an earlier environment was unavailable; a task-specific Typst0.15.0
venv and Ghostscript render the committed three-page PDF. Every page was opened
and visually inspected for legible formulas/labels, identities and citations.

## Broader root check

`root-library-attempt01.log` inside the development archive is the actual
`cargo test --release --lib --offline` attempt at this source parent under
installed Rust1.94.1. After inherited named fixtures/source were materialized,
it exits101 only on the two existing E0658 atomic_try_update uses in
`src/cryptanalysis/ecbench/diagnostic_runtime.rs:139` and
`src/cryptanalysis/ecbench/pencil.rs:79`. This study does not edit those files.
Only Rust1.94.1 is installed locally; a compatible root toolchain/actual CI
must assess that scope. Neither a root-library pass nor a PR-caused CI failure
is inferred. The touched standalone crate passes its own release checks.

## Performance admission

No old local benchmark exception was reused. The default EKS route was
inspected but has no arbitrary standalone CAS adapter or ready identity in this
session. No job was submitted; there is no invented failed-job receipt or
cost/exponent projection. The prospective protocol records missing host/budget
and complete decoder integration. Performance dashboards/benchmarks stay fixed.
The parent study's presentation-comparison table is updated with formal results
and explicit unmeasured fields only.
