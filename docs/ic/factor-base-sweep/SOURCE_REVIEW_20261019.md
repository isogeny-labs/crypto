# PR #2134: source819 merge review and native validation

This is software validation of source
`819d6b172c8a58983bf1b2463fe24576f3f58094` (tree
`55b5dc07a850871ff149c22326c1e94ac3f54bdf`), not a new research campaign.
The published source tree matches the local checkout. This receipt is archived
on a separate branch; it does not move or replace the shared PR head.

## Resolution and preservation

Copilot request [6254099148](https://github.com/aburan28/crypto/pull/2134#issuecomment-6254099148)
explicitly ended unsuccessfully at 2026-10-18 19:22:31 UTC. The shared branch
subsequently gained this merge with parents `f84e9a26` and main
`1e2bf071`. This session reviewed that existing merge without editing its
resolution or overwriting it.

Against main, the F4 file adds the existing observer/progress instrumentation
and observer tests. The upstream packed-set/replay test block is byte-exact;
the original observer module is also byte-exact. Both suites pass in the fresh
library run. All 196 protected prior study/evidence/documentation blobs remain
exact. Both parents' 2,572 models, 6,143 aliases and 26 store entries are retained;
those three containers match the main parent. Eleven CLI/plugin/test/release
workflow blobs remain exact. Both planning cards remain unchanged and draft,
unrun, zero-funded and unleased.

## Fresh validation of this source

| Check | Actual result |
| --- | --- |
| One complete `cargo test --locked --release --lib -- --test-threads=1` | 4,400 passed, 0 failed, 118 ignored, 0 filtered; exit0 |
| Release CLI unit tests | 7 passed |
| Actual subprocess CLI integration tests | 6 passed |
| Production CLI build and strict release CLI Clippy | Passed |
| Root locked all-target compilation | Passed |
| Independent native audit tests / strict all-target Clippy / release build | 8 passed; both other commands passed |
| CI-scope tests | 9 passed |
| Sealed archive member hashes | All 8,546 passed |
| Native sealed-record replay | All 64 seals and 1,008 point witnesses verified |
| Observations and independent original/guard coverage | Byte-identical to frozen outputs |
| Native input recapture | All 1,632 pre-compilation inputs unchanged |

The 118 ignored entries are reported as printed. All 117 previous ignores
remain; upstream adds the subprocess-only `inherited_f4_word_xors_probe`.
Its passing parent `inherited_f4_word_xors_ignore_thread_count_and_hash_salt`
explicitly executes it under four thread/hash-salt configurations, checking
24 rows per configuration. That source file is byte-identical to the main
parent. No previous enabled test was newly ignored, and this session changed
no test, assertion, limit, coverage or workflow.

All 1,632 source/configuration/embedded inputs were hashed before compilation.
The actual unit binary SHA256 is
`8ce288f3bfcc1529dc63f91d71bdffa852226582fd3bef110b26e789e8261e1f`;
all 731 compiled tracked dependencies match the pinned input set. The binary
contains the exact traits fixture with SHA256
`6b3d5bc69a6d4f321455762bce671ee1f5d949582f38ac325c3a88a193f78922`
at offset4,484,392. No fixture generation overlapped compilation.
The first metadata classifier did not normalize `..` components and labelled
20 tracked includes as generated; its output/helper remain alongside the
corrected classifier and final 731-dependency result. This metadata correction
changes no native input or test result.

The fresh production CLI SHA256 is
`f7f247ac5ce924d28e8f454043e1999792aedc584787d29f048f9a4c3d5eee71`.
It is a validation artifact, not a replacement of the installed or original
research executables. Old byte-bound resumes still require their original
executable. Tests used Rust1.98.0, two build/Rayon threads and networking
including loopback in the existing managed workspace.

## Raw records

[validation/source819-20261019.tar.xz](validation/source819-20261019.tar.xz)
contains exact commands, native logs/exits, input manifests, unit-binary
binding, both-parent F4 diffs, preservation checks, CI snapshot and all26
then-completed failed/cancelled CI job logs. It is 353,320 bytes, SHA256
`cbc37183775c17e1ad1843702dfc1830627fe5a384adbffc278fc5d8f3a39f16`.
All108 member hashes pass after extraction, and deterministic extracted-tree
GNU tar/xz repacking is byte-identical.

```sh
sha256sum -c docs/ic/factor-base-sweep/validation/source819-20261019.tar.xz.sha256
mkdir source819-validation
tar -xJf docs/ic/factor-base-sweep/validation/source819-20261019.tar.xz -C source819-validation
(cd source819-validation && sha256sum -c MEMBERS.sha256)
```

The extraction includes `runs/package.sh` with the exact deterministic packing
flags. Setup diagnostics are retained: a restricted-network lazy Git fetch
failed before its successful granted-network retry; an initial rustc metadata
probe omitted RUSTUP_HOME and was corrected; direct job-metadata connector
URLs were unsupported before the supported run/jobs collection was used.
Conductor returned127 before validation and again before publication; the pinned
workflow returned404. Neither is a pass. No paid host or S3 operation occurred.

## CI and remaining gates

At the 2026-10-19 00:43 UTC inspection, all50 current-head workflow runs and
all123 registered checks were inspected: 62success,24failure,2cancelled,
11 condition-justified skips,22queued,2neutral. The 24 failures comprise19
unchanged historical/copied-lock failures under `--locked`, two backlog
failures (the unrelated quintic card plus its concurrent-claim test),
two cover-count failures, and one frozen n53 lock-custody mismatch.

Scaled producer117803531472 was cancelled at its25-minute budget.
Framework validate-cli117803530221 was cancelled at its10-minute budget even
though its last diagnostic says `checks_passed`; GitHub's cancelled result
is not a pass. Normal Rust CodeQL, Rust compile/lint, CLI/harness and release
jobs were among the queued checks. Those missing/pending results do not pass
because of this local validation. The eleven skips have unchanged, previously
checked event/input/label conditions; release publishing had not yet registered
its final result in that snapshot.

Repository review117803523581 passed self-test only, warning that no provider
was configured. Neutral Approval Agent/Bugbot results are not completed analysis.
No formal reviews, inline feedback or assigned reviewers were observed.
The PR was still open/mergeable=true/unstable with the same shared head on the
post-validation metadata read. No CI retry, foreground CI polling, weakened
gate or merge was performed. Applicable CI and actual review gates still
control merging, with an exact expected-head-SHA guard.

## Scope limits retained

Replay does not rerun F4 or certify regularity. The original 60-case research,
its three protocols/reports and all historical #1898/#1975 evidence remain
unchanged. No new timing, point logarithm, candidate campaign, speedup,
complete-cost winner or independent-host certificate is claimed. Study graphs
and PDFs remain unchanged because this software validation adds no research
measurement. Both draft cards stay unexecuted; historical registration/replay
blockers are not bypassed.
