# PR #2134: main reconciliation and final-source validation

This maintenance merges pinned main without changing the completed shape study,
launching another campaign, promoting either draft card, or rewriting evidence.

## Source and resolution

Source commit `01674b60e7268c81eaf08211f7f7b5e239ee0ee0` has two parents:
`381aa0b3a1fbae21e5d23edaeb3bb516005ca3a8` and pinned main
`b7cafe04485eb97d873dffef698e7337cfb38cfe`. Its tested tree is
`b22d66d181d2d078e2f3a5bd6e2b32593841c0fe`.

Copilot's request ended with `copilot_work_finished_failure` at 05:14:41 UTC.
[Takeover was explained before local edits](https://github.com/aburan28/crypto/pull/2134#issuecomment-6246401560).
There was no later pushed resolution or active resolver observed. No rebase or
force-push was used.

The four conflicts were reviewed against both parents. The benchmark index keeps
every result row from each parent byte-for-byte. The methods and Gaussian helper
implementations are exact pinned main. The workload implementation is exact
pinned main plus the feature's unchanged quartic regression module. This retains
upstream constructor bounds, distinct variants, public-target handling and
crate-visible elimination, without duplicate constructors from the independent
earlier compile repairs. No generated metadata conflicted or needed regeneration.

Preservation checks retain 188 protected feature/historical blobs exactly,
including the frozen study, protocols, archives and prior trace/evaluation
payloads. The native CLI, F4 observer and explicit-subspace plug-in blobs are
unchanged. All entries from both parents survive: 2,572 models, 6,143 aliases and
26 store entries. Containers match pinned main. The only changed shared model
adds an alias and source reference; all previous fields are retained. Traits,
covers, Jacobian metadata and measured leaderboard fields are exact pinned main.
Both planning cards remain draft; the older #1963 card is byte-exact.

## Actual validation

All commands used Rust/Cargo 1.98, the existing managed workspace, two build
workers, two Rayon threads, and networking including loopback. These are
functional validation runs, not performance measurements or independent-host
certificates. No new paid host was launched.

| Check | Result |
| --- | --- |
| `cargo test --locked --release --lib -- --test-threads=1` | One complete command: 4,384 passed, 0 failed, 117 existing ignores, 0 filtered |
| Release CLI unit tests | 7 passed |
| Actual CLI subprocess integration tests | 6 passed, no skipped tests or changed assertions |
| Production CLI build and strict release Clippy | Passed, warnings denied |
| Root `cargo check --locked --all-targets` | Passed |
| Standalone independent audit tests / strict all-target Clippy / release build | 8 tests passed; lint/build passed |
| Exact added CI sealed-evidence step | Passed: all 8,546 original member hashes, 64 seals, 1,008 point witnesses; frozen observations, independent coverage and guard coverage match byte-for-byte |
| CI-scope suite | 9 passed |
| Preservation and source-input audits | Passed; all 1,629 frozen native inputs unchanged after validation |
| Relevant workflow YAML and shell syntax | Passed locally, including unique YAML keys; not GitHub Actions validation or CodeQL analysis |

The tested unit binary SHA256 is
`624cae9240b8a86d4002ed72468427d72bbb63bca56bdf7eeeaa265851347831`.
All 731 compiled library dependencies match the inputs frozen before compilation.
The exact embedded trait fixture SHA256 is
`6b3d5bc69a6d4f321455762bce671ee1f5d949582f38ac325c3a88a193f78922`.
No generation overlapped compilation. The documentation child changes no native
inputs. Production and installed historical executable hashes are recorded
separately; no identical-build or new runtime-install claim is made.

Pinned main independently repaired the trait fixture and changed the Jacobian
test's expected classification to `Boundary` for its existing pricing model.
Those upstream bytes are retained exactly. This pass is not an unchanged-assertion
retry of the earlier four-failure snapshot; the earlier receipts and failures
remain untouched. Workflow duplicate-key repairs are also inherited exactly from
main, with no feature-side coverage, limit or ignore workaround.

## Reproducible records and remaining gates

[Raw validation archive](validation/merge-20261018.tar.xz): 173,284 bytes,
SHA256 `7380f9d40ce9d982d63b52973bfc41988ddb289792602d3f2ca8d473f9abbaf8`.
All [84 member hashes](validation/merge-20261018.members.sha256) passed after
extraction; deterministic GNU tar/xz repacking of that extracted tree is
byte-identical. The archive retains exact commands, exits, full library and
targeted logs, source manifests, both-parent diffs, preservation audits, and
setup/packaging failures. Its previously sealed research inputs are referenced,
not reopened or duplicated. Use the archive's `validate.sh`, `ci-replay.sh` and
input-binding helpers with the named source tree and explicit toolchain paths.

Conductor remains unavailable (127), and pinned `.conductor/WORKFLOW.md` returns
404. Neither passed. The three original frozen-protocol EOF advisories remain
historical executed bytes and were not normalized.

At pre-publication inspection, shared head was still `381aa0b3`, open/dirty.
Its repository review check passed a self-test but had no configured provider;
Bugbot was usage-limited and both bot approval results were neutral. No formal
reviews or inline threads were present. Rulesets returned `[]`; administration
protection access returned integration 403. These observations do not bypass any
actual gate or substitute for completed analysis. Fresh applicable CI and review
must be inspected on the published documentation child before an exact-head
guarded merge. Historical failed workflow starts are not security-scan passes.

The existing bounded yield/support leads and their limitations are unchanged.
There is no new regularity, runtime, fastest-tool or end-to-end claim, historical
CPython rerun, historical 384-trace replay, S3 operation, funding or lease.
