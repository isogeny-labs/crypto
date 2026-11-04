# Verification receipts

This continuation ran mathematical proof builds and native unit correctness
checks in the existing worktree. No cryptographic experiment or CAS benchmark
ran. The reference packages and repository auditor remain unchanged.

## Exact accepted command

```sh
env PATH=/workspace/prime-degree-lean/bin:/workspace/n83-rustup/toolchains/1.94.1-x86_64-unknown-linux-gnu/bin:$PATH LEAN_NUM_THREADS=4 RUSTUP_HOME=/workspace/n83-rustup RUSTUP_TOOLCHAIN=1.94.1-x86_64-unknown-linux-gnu CARGO_HOME=/workspace/n83-cargo bash verify.sh verification/final
```

Exit0, stage complete. The script records:

- Forty source/input hashes, all matching.
- Actual Lake build and all five new modules with warnings-as-errors.
- Full unchanged transitive-axiom auditor across eighteen source modules:
  240 declarations, zero failing, zero trusted/native_decide. Only standard
  propext, Classical.choice and Quot.sound are accepted.
- Native rustfmt and release clippy with warnings-as-errors; thirteen release
  unit tests pass, including the existing issuer's tests, full finite fixture
  membership, old FB1 codec identity, fixed new-column relation replay and
  changed-statement rejection.
- A new write-once native fixture export; diff against committed fixture is
  empty, including FB1 data, metadata and evaluated-tier identity certificate.

The final logs and generated comparison files remain under `final/`.
`inputs.sha256` binds the accepted sources, both local reference packages,
original curve metadata, native identity issuer/reference and unchanged
Lean auditor. `dependency-revisions.txt` records all nine Git package
revisions as expected/actual pairs, read directly from their checkouts.
`lean-version.txt`, `rust-version.txt`, the Lean binary hash and source
parent commit are separate records. No current source commit is inferred
from the parent; hashes bind uncommitted-at-check-time accepted bytes.

## Development failures

All recorded build-attempt01–22 logs and available source snapshots are
preserved in `development-records.tar.gz`, together with native tests,
clippy/format/export logs and rendering output. A failing development build
may print diagnostic sorryAx; those failed declarations are not accepted
proof modules. The exact final source builds and passes the full auditor.

Several routine errors are retained: notation-shadowed parameter names,
missing Set lemma, a CharP instance required for polynomial normalization,
a premature degree-tactic application, and one missing X¹ component rewrite.
The first native test needed the original issuer's tracked reference JSON;
sparse materialization fixed it. Initial clippy resolved a read-only default
rustup home; the explicit task toolchain fixes it. Two snapshot-copy commands
were issued in the wrong directory and failed without changing source. The
sparse helper itself was not materialized; native git sparse-checkout added
`problems/`, with no source deletions or edits to the sparse helper.

`lean4checker` is unavailable and no fresh checker pass is claimed. No
cache-download preflight is claimed; existing pinned compiled dependencies
are reused, and all new module sources are checked. Root ecbench was not
changed or benchmarked; the standalone exporter imports the unchanged
shared native identity issuer and runs its tests.

The repository-wide `cargo test --release --lib --offline --locked` was
also attempted. The first exit101 was an unmaterialized tracked local crate
(`root-library.log`). After materializing source/local dependencies, a second
attempt reached compilation and exited101 on missing compile-time fixtures
and two E0658 errors (`root-library-attempt02.log`). All43 named tracked
fixture paths were then materialized, and attempt03 isolates E0658 on
`atomic_try_update` in the unchanged `ecbench/diagnostic_runtime.rs:139` and
`ecbench/pencil.rs:79`, under the installed Rust1.94.1. No root-library pass
is claimed; this continuation does not change those modules or the root
compiler policy. Current-head repository CI remains necessary. These build
receipts are separate from the complete standalone proof/native checks.

## Report inspection and custody

`make check` invokes Typst0.15.0 and PDF/XML checks. The new two-page A4 PDF
is54695 bytes, reports no suspects and was rendered at100dpi with pdftoppm.
Both pages were visually inspected: diagram labels, IDs, limits and sources
are legible without clipping. All body and diagram fonts use DejaVu Sans.
The editable Typst/SVG sources are retained. The failure archive is packed
with fixed ordering, timestamps and owners; an independent repack matches.
The final native log's one empty trailing line is removed for the repository
whitespace check; its exact raw bytes remain in the development archive as
`native-final-raw.log`. No diagnostic or result content is removed.
All deliverables are bound by the directory's SHA256SUMS.

## Remaining work

The prospective panel's native runner/controls and admitted EKS submission
integration are not yet implemented. No launch is claimed. Operation counts,
actual F4 degree, RAM, disk, yield and complete cost remain unknown/null.
See the [tracked successful-work problem](../../../../problems/factor-base-shape-to-successful-work.md).
The inspected sources and known limits are recorded in the report; no failed
literature search establishes novelty. The measured leaderboard/graphs are
unchanged because this continuation has no admitted numerical measurements.
