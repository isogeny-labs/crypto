# Mixed-relation and explicit Frobenius image validation

This is an append-only follow-up to the existing study. Protocol commit
`6aa9212b4` preceded execution; the synced PR head was
`036c2a1a329276ef94c4e104d447c426181cf26c`. The exact uncommitted implementation
at each recorded attempt is identified by its source or the final input manifest;
the protocol commit is not mislabeled as containing the completed implementation.
Current source/input SHA-256 values are in `INPUTS.sha256` (paths from repo root).
The final implementation and this evidence are delivered together in PR #2381.

## Final checks

| Invocation | Actual outcome | Receipt |
| --- | --- | --- |
| C++ producer, timeout 300 | 942 models, 274 subgroups; complete words 3–6; exit 0 | `native/run-attempt-02.log`, `.exit` |
| Independent Rust tuple/DP replay, timeout 300 | full words, rows, ranks and controls agree; exit 0 | `native/audit-final.log`, `.exit`; `../../native/mixed-20261022T0718Z/audit.json` |
| Exact workflow command `bash research/quadratic_twist_evidence_20261021/run-mixed.sh /tmp/twist-mixed-ci-20261022T0748Z` | full replay and exact bytes; certificate replay; two corruption controls rejected; exit 0 | `ci-replay.log`, `.exit`, `controls/` |
| `cargo test --release --offline --locked --manifest-path research/quadratic_twist_evidence_20261021/native/mixed-audit/Cargo.toml` | 22 tests passed, no failures or ignores | `native/audit-test.log`, `.exit` |
| `cargo test --release --locked --lib`, after current-main refresh | 4,479 passed, 119 existing ignores, zero failed; exit 0 | `cargo-lib.log.xz`, `.exit` |
| `lake build` and repository axiom auditor on all three modules | pass; 36 declarations, zero failing/extra trusted axioms | `lean-build-final.log`, `axiom-audit-final.log`, `.exit` |
| Unmodified repository `tools/lean/proof-gate.sh --out .../lean-gate-final research/quadratic_twist_evidence_20261021/lean` | PASS, 36 declarations, zero trusted; exit 0 | `proof-gate-final.log`, `.exit`, `lean-gate-final/` |
| IDC1 certificate issue/replay | 32 derived + 16 extra points; false plus one rejected; source hash matches final Lean file | `native/certificate-final.log`, `.exit`, `../../native/mixed-20261022T0718Z/certificate/` |
| Shared Conductor scope check | unavailable, exit 127 | `conductor.log`, `.exit` |
| Shell syntax, formatting and diff check | pass | final checks recorded below |
| Report/graph/PDF | final three-page PDF rendered and each page inspected | `pdf-build-final.log`, `pdf-inspection.md`, report directory |

The axiom auditor includes eight generated equation/congruence helpers alongside
the 28 named research theorems; the helper count is not claimed as eight new
mathematical findings. It reports only propext, Classical.choice and Quot.sound.
No new trust manifest entry, skipped theorem, gate patch or native evaluation
axiom is introduced. `lean4checker --fresh` remains unavailable; this is not an
independent external kernel replay claim.

The full library invocation had networking enabled for its existing localhost
socket tests. Its five additional passing tests relative to the earlier 4,474
receipt come from the synced main branch, not from rewriting historical counts.
The main library, registry and old evidence were not edited by this continuation.

## Exact commands and implementation separation

```sh
c++ -std=c++17 -O2 -Wall -Wextra -Werror research/quadratic_twist_evidence_20261021/native/mixed.cpp -lcrypto -o /tmp/twist-mixed
timeout 300 /tmp/twist-mixed docs/curves/registry.json research/quadratic_twist_evidence_20261021/native/mixed-20261022T0718Z/results.json
cargo build --release --locked --offline --manifest-path research/quadratic_twist_evidence_20261021/native/mixed-audit/Cargo.toml
timeout 300 research/quadratic_twist_evidence_20261021/native/mixed-audit/target/release/quadratic-twist-mixed-audit research/quadratic_twist_evidence_20261021/native/mixed-20261022T0718Z/results.json docs/curves/registry.json research/quadratic_twist_evidence_20261021/native/mixed-20261022T0718Z/audit.json
```

The initial audit is retained as `audit-initial.json`; the final audit adds
native-computed breakdowns by word length and subgroup prime. Original per-model
results remain identical. Initial and final IDC1 certificates share the same
identity statement ID, but retain their different exact Lean source pointers;
the original pointer matches `proof-attempt-07/TwistEmbedding.lean.txt`, and the
final pointer matches the active Lean file. Earlier attempts are not represented
as current proof sources. The runnable script checks final result/audit/certificate
bytes, never just a status flag.

Rust reconstructs points from coefficient pairs and computes inversion by
exponent p²-2; it never calls the C++ producer or its arithmetic. Its multiplicity
DP enumerates every coefficient-row/arity state, in contrast to the producer's
ordered-word DFS. Point-sum verification and Gaussian elimination are separately
implemented. JSON and SHA-256/backend IDC1 implementations are shared, on this
same uncontrolled Linux host. No independent-host or performance claim is made.

The false-coefficient and missing-model inputs are reproducible using
`mixed-audit controls INPUT OUTDIR`. Their complete payloads are deterministic
mutations of the committed raw result, and their hashes and rejection logs are
retained in `controls/`. A changed coefficient gives a nonzero actual point;
deletion is caught by the complete model census. Both exit 101, emit no acceptance
receipt and finish before the 300-second cap. The actual P-T control is checked
on every subgroup as part of producer and auditor replay.

The exact two corrupted payloads are preserved as `controls/inputs.tar.xz`,
with the archive digest in `controls/archive.sha256`. It is a deterministic
tar/xz archive (sorted paths, fixed 2026-10-22 timestamp, numeric owner/group 0).
Extract into a new scratch directory with `tar -xJf controls/inputs.tar.xz -C DIR`
from this directory. `controls/payloads.sha256` checks the extracted JSON bytes
from that new directory. The original `controls/output.sha256` is retained
verbatim, including its actual scratch paths.

## Failures retained

- `native/run-attempt-01.log` records `FAIL: stoll`: the first model-name join
  tried to parse unrelated global registry primes exceeding signed 64-bit
  range. No result was emitted. `native/mixed-attempt-01.cpp` retains that source;
  the corrected join selects the declared small-prime population before conversion.
  No arithmetic workload or population was reduced.
- `proof-attempt-01` through `proof-attempt-05` retain failed source/elaboration
  logs and exits. Errors concern missing typeclass arguments, exact polynomial
  multipliers, simplification, constructor use and explicit extension types.
  Provisional axiom printouts containing error-generated placeholders are not
  proof receipts. Attempt 06 checks the initial seven new theorems; attempt 07
  adds finite sums and the polynomial identity. Final cleanup is built/audited
  separately. Failed source uses `.lean.txt` so it remains evidence, not a
  standalone active proof target.
- The first shell invocation used repository-relative paths while already in
  the Lean directory and failed copying its input before Lean ran. It produced
  no proof result; the path was corrected before recorded attempt 01.
- `native/audit-build.log` records the first Rust build's visibility error on
  re-exporting the shared SHA-256 module. That build did not run an audit.
  Subsequent source snapshots and the 22-test receipt are retained.
- `proof-gate.log` exited 3 because Mathlib attempted its default cache path
  under a read-only home. `lean-cache-diagnostic.log` identifies the exact path.
  Setting the supported `MATHLIB_CACHE_DIR=/workspace/twist-tooling/cache/mathlib`
  allows the **same unmodified gate** to pass. This is an environment correction,
  not a relaxed proof check.
- Initial document rendering used temporary writable Matplotlib caches and
  emitted Fontconfig warnings; the first PDF had an overfull provenance sentence.
  The final layout shortens that sentence and separates graph labels. Rendering
  logs are retained; the final PDF is the inspected version.
- `diff-first.log` records the initial staged whitespace check: the renderer's
  SVG paths include trailing spaces and a raw Cargo transcript ends in a blank
  line. Scope-local Git attributes preserve those emitted bytes without
  rewriting evidence. Executable/proof sources retain ordinary whitespace checks.

## Environment and limits

`environment.txt` records compiler/runtime versions, OS and the final capture
time. Rust/Cargo 1.99.1, Lean 4.34.1 and the pinned Mathlib revision were used.
Toolchain paths were `/workspace/twist-tooling/{cargo,rustup,elan}`. Networking
was available through the configured session proxy. No secrets or environment
dump is part of this metadata. Native execution limits were 300 seconds; no
timeout was observed. Process and Cargo test durations appearing in raw logs
are provenance only, not admitted algorithm timings. Python only rendered
already native-recorded graph values; no Python research execution was used.

The report directory contains the editable graph/diagram/PDF sources and final
renders. No benchmark, leaderboard, cover-catalog provenance, registry, old
source, archive or frozen evidence is rewritten. Shared Conductor, remaining
formal bridges, all applicable current-head CI and review gates remain open.

To inspect the full library transcript, run `xz -dc cargo-lib.log.xz`.
