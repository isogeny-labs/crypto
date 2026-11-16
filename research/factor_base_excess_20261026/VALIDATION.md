# Native implementation validation — 2026-10-26

**Storage update (2026-10-26):** The owner approved canonical S3 use. The original validation archive is now uploaded and fully read back; the resume checkpoint passed and its contents were verified. [Publication receipt](publication-20261026/README.md) supersedes the historical storage/authorization blockers below. No new research result or broader native-validation pass follows from publication.

Final native source: `7d340f8710e4c22cb8a1a3be356f85f10944467a`, based on
main `4355e5c648b67a679729ac7963f3c054d8362d30`. Later documentation changes
must not be described as new native measurements. **No new research panel
ran, and no useful factor-base improvement is established.**

## Passing affected checks

| Actual local check | Outcome |
| --- | --- |
| `cargo test --locked --release --bin factor-base-sweep` | 17 passed, 0 failed/ignored/filtered |
| Production CLI and backlog build | Passed |
| New actual CLI subprocess tests | 2 passed outside checkout with restricted PATH, no Cargo runtime |
| Existing actual sweep subprocess tests | 6 passed; assertions, caps and resume rules unchanged |
| Touched CLI Clippy driver with existing Cargo lint settings | Passed; separate from the failed Cargo-wide check below |
| Exact touched-file rustfmt | Passed |
| Canonical CI-scope suite | 9 passed |
| Unique-key workflow YAML / exact command delta | Passed; existing test retained, new subprocess suite added, 25-minute cap unchanged |
| New draft card native validation | Passed with the 2 intentional TODO warnings; no dispatch/funding/lease |
| Changed-line naming and registered new slugs | 0 problems; not a full-repository naming pass |
| Initial/final native input and protected-source audits | Passed within the scopes below |

The eight subprocess tests were compiled with `rustc --test`, against the
actual rebuilt executable and existing serde_json rlib. This avoids Cargo
building unrelated binary targets merely to run integration tests. The CI
workflow uses Cargo's normal test targets; local direct compilation is not
misreported as a Cargo integration invocation.

Tests cover independent exhaustive GF(8) product-space answers, basis and
wire-identity invariance, reducible/dependent/truncated inputs, construction
reproducibility, reconstruction of the scale from the emitted XOR selector,
direct Frobenius-sum trace checks, the bounded shape adapter, and 131-bit
field arithmetic without truncation. They are fixed software controls, not
a point-decomposition experiment or evidence of a wide-field solver.

## Incomplete or failed broader checks — not passes

- Full command: `cargo test --locked --release --lib --quiet -- --test-threads=1`,
  with networking including loopback and `RAYON_NUM_THREADS=1`. It registered
  4,940 tests. After compilation it produced 1,920 result marks, including
  11 fixture failures, then remained CPU-active in the serial position of
  `jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
  The test process was stopped with SIGTERM after 3,602.58 seconds of runtime;
  Cargo returned 101. This is an incomplete full-library attempt, not a full
  pass or a proved hang. The named current test is inferred from serial
  order, not a captured native stack. Its source is exact pinned main. No
  assertion, ignore, test budget or coverage was changed to clear it.
- Missing runtime fixtures were restored from exact tracked Git bytes,
  without regeneration or S3. Same-binary diagnostic retries give 21 unique
  passes, including all 11 initially failing cases. One first retry still
  lacked the nested bounds session; its real OS-error-2 log is retained,
  followed by the passing unchanged retry. These filtered checks do not
  substitute for a complete successful library command.
- `cargo clippy --locked --release --bin factor-base-sweep -- -D warnings`
  failed on 7 unchanged library warnings: `jv_bitops.rs:281`,
  `certicom131_symlp/gf2w.rs:481,483,495`, `certicom131_symlp/relgraph.rs:183`,
  `ghs_smooth.rs:45` and `koblitz_groebner.rs:3594`. The touched CLI was also
  linted directly against rebuilt dependencies, with exactly the pre-existing
  Cargo.toml lint configuration. That scoped pass is not a Cargo-wide pass.
- `cargo check --locked --all-targets` failed in unchanged
  `examples/r6_1_direct_s5.rs:1178`: `FactorBase` initializer lacks `known_logs`.
  Both attempts/logs are retained; the inherited example was not edited.
- The production admission gate reports 22 existing benchmark/archive
  problems. Benchmark rows, freezes, verdict trees and gate source are exact
  pinned base; this change adds no admitted result. The local artifact check
  passes for the published source/metadata/report only, not the withheld raw
  archive. Exact outputs and scope proof: [GATE_STATUS.json](validation/GATE_STATUS.json).
- Full repository naming stopped on an off-cone tracked skill file. An
  optional whole-repository metadata inventory was terminated after lazy
  fetches triggered Git auto-repacking; no global naming pass is claimed.
- Conductor returned 127 before editing; the pinned Conductor workflow URL
  returned 404. These were unavailable, not passed. No new Lean/IDC proof is
  claimed for implementing existing field and linear-algebra algorithms.

No unrelated code was patched to conceal these blockers. See the exact
[protected source comparison](validation/protected-source-preservation.json).
No GitHub CI result is inferred from any local check.

## Source and executable binding

[Final binding](validation/final-binding.json) records all binary hashes and
785 actual worktree paths from the compiled library test's dependency file.
Every one matches its hash in the 1,809-file input manifest captured before
that library command. Root library, fixtures and Cargo inputs are unchanged
from pinned main. The initial CLI prototype later changed: the whole initial
native manifest is **not** claimed unchanged. A fresh 1,810-file final native
binding was captured after the last Rust edit and matches after validation:
`92af1411dbb715681d346eb284343a5b68a632b90a3820878fd7e92254294c75`.

Final production executable SHA256:
`d01d756ec3e3969e542ac7bc60f1bc20532a7b86f0e0c290dc77e5078d923010`.
It is retained at `/workspace/fb-excess-validation-20261026/factor-base-sweep-v2`.
The installed oracle-study executable and original byte-bound research
executables remain unchanged; no existing resume is silently moved to this
new binary. The earlier 14-test prototype pass applies only to that earlier
source. Its unit-binary path was reused by the final 17-test build; its old
saved hash does not describe the current file at that path.

Two-page PDF rebuilt; both rendered pages visually inspected after the final
constructor/text changes. It is 64,436 bytes. This is a method diagram/report,
not a new performance graph. No numerical solver result changes a canonical
scoreboard, browser, leaderboard or benchmark admission row.

## Receipts and setup errors

[Published bindings and local raw receipts](validation/README.md) distinguish
the committed source/input metadata from the unpublished command archive.
The archive is 73,608 bytes, SHA256
`fb2b6e1f4c4283f82f2ef7e33cddbb3b9774d6003e6c1892a9f7f02afd7b4982`;
all65 member hashes and an extracted-tree deterministic repack pass locally.
It exceeds the repository's 64KiB archive threshold, while S3 operations
remain forbidden. It was not pushed, split, exempted or given a fabricated
remote manifest. Raw blank-line/trailing-space output is preserved in it,
not silently normalized to pass a text whitespace check. Artifact publication
is a remaining blocker; a metadata receipt is not admitted raw evidence.
SDK, target trees, binaries and inspection PNGs remain external too.

Earlier setup errors remain documented: unsupported `cargo fmt --bin`
(corrected to exact-file rustfmt), missing sparse path dependency/module
indexing (tracked bytes restored), atlas sparse module missing (restored),
PDF initially interpreted as text (study-local binary attribute), Node input
inventory buffer overflow and four nontracked include-like fixture strings,
and a wrong CI-scope test filename (corrected to the existing canonical file).
The first direct Clippy probe lacked libc; the next omitted Cargo's existing
lint configuration. Both failures are retained before the exact-config pass.
The old initial/final whole-manifest comparison fails as expected because the
CLI was subsequently extended; the separate final-source binding is used.
The first inventory/diff tool outputs were truncated, not a full saved raw
transcript. No scientific result, source assertion or budget was changed by
these setup corrections.

## Research execution gate

`scripts/checkpoint.sh gate` returned 2 before upload because no resume file
existed. Current AGENTS requires a successful S3 checkpoint before an
experiment; the user's no-S3 instruction still applies. **No upload, credential
inspection, research launch, old-panel rerun or card execution followed.**
The comparison draft remains unrun. Resolving checkpoint authorization does
not itself establish lower successful solver work, regularity or speed.
