# PR #2924: combined-main development validation

The checked hash-prime feature is unchanged in scope. This receipt validates
its integration with main, rather than replacing the original frozen report.
No scientific computation, profiling or benchmark was launched.

## Source identity and conflict review

- Original feature head: `edfadbab5ac71471be7fd2971a9e50825307363f`.
- Merged main: `e534a6d05a8885c6ffc2a22aba35281350ddc0d7`.
- Resolved index tree before this additive receipt/log entry:
  `2551a1c462e592793ee3297d0c90e073d774d6ce`.
- [Current native source manifest](native-sources.sha256) covers all 40
  tracked package files, including manifests, lockfile, examples and tests.
  It identifies the exact tested source; the final merge head is recorded on
  [the PR](https://github.com/isogeny-labs/crypto/pull/2924).

Copilot received the merge-main request at 03:59 UTC and remained unanswered
for more than 2.5 hours. The author used AGENTS.md's reasonable-wait fallback,
announced it on the PR, merged main without rebase/force, and kept auto-merge
disabled during review. Conductor is absent (exit 127); no shared reservation
is claimed. The last recorded T-828 lease expired at 05:07:17 UTC.

Four conflicts were resolved: the curve imports combine `SquareRootContext`
with the checked-prime `overflow` helper; the dated journal keeps both complete
ordered parent histories; the live problem history keeps both additions; its
index row names both implemented features and retains the open audit. The
[journal check](log-preservation.txt) matches all 596 old-head lines and all
2,416 main-parent lines in order. No frozen evidence conflict was present.

Main's reusable roots, canonical points, bounded reads, strict JSON, functional
pruning, finite backends and Hexagon additions remain intact. Compared with
main, only the original four native feature source files, the fixture,
preparer and six-test integration file change in this package. The three new
fixture/preparer/test files equal the already landed cryptanalysis copies.
The original study's 19 files are byte-identical to the original feature head;
main's frozen speed and finite-size studies are unchanged.

## Completed checks

All commands below exited 0 against the combined source:

```sh
cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml
cargo clippy --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --locked --all-targets -- -D warnings
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --check
# Rust 1.78.0
cargo check --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --locked --all-targets
```

The full suite passes **82 tests**: 51 library, 5 runner, 3 source-panel,
6 hash-prime, 1 factoring CLI, 9 finite-extension, 3 strict-JSON
and 4 width-extension tests. Both examples compile during the suite/all-target
checks. The seeded whole-certificate comparison and hash candidate/RNG control
pass. These are development checks; their elapsed times are not performance
results.

Raw output is retained losslessly in `tests.log.gz`, `clippy.log.gz`,
`msrv.log.gz` and `fmt.log.gz`. [Artifact hashes](ARTIFACTS.sha256) and
[uncompressed hashes](uncompressed-logs.sha256) preserve their exact bytes.

`git diff --cached --check` against the old feature head reports inherited
main whitespace (exit 2, 4,670 diagnostic lines); its complete output is in
`inherited-whitespace.log.gz`. Checking the actual PR diff against the merged
main parent passes (exit 0). Unrelated frozen main artifacts were preserved.

## Remaining gates and requirements

Check the final uploaded head's `gate`, `size`, `artifacts`, every applicable
check and blocking review before a guarded MERGE. The active main ruleset
requires those three checks and resolved review threads, allows MERGE only,
and requires zero approving reviews. The branch-protection endpoint returned
HTTP 403; no protection or review bypass is used. Codex's earlier service
review failed because its quota was exhausted; it is not represented as a
completed independent review. No inline review threads or formal blocking
reviews were present at refresh.

Profiling, the original frozen matched A/B panel, authenticated current-source
execution/image admission, the binary challenge pipeline, both challenge
scalars and the independent general audit remain open. A fresh source S3
checkpoint/readback is required before any new scientific run. This task does
not edit or launch the separate new ECBench integration study.
