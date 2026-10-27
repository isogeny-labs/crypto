# Validation backlog: CI triage and reproducible-benchmark requirements

Scoped 2026-10-11, re-surveyed 2026-10-26 against `main` at `71dd7db5c`. Run
links are to `isogeny-labs/crypto` (formerly `aburan28/crypto`). Each finding is
labelled by how it is known: **log** (quoted from a CI job log), **local** (re-run
in a worktree), or **inferred** (not directly observed). Nothing here edits a
sealed measurement, and no workflow or test was skipped, disabled or quarantined
to make a check pass.

Corrected 2026-10-26 after an independent review of #2588 and the first CI run on
its head (section 6 lists every correction). The Release row and section 3.3 had
called the Windows work "fixed"; the CI run shows it is not confirmed.

Workflow triggers also changed under this document the same day. `02f2bb3a4c`
("Run only the merge set on pull requests", #2578, merged to `main` at 11:56 PDT) made
pull requests run only the gate, size and artifact checks and moved "the research
workflows, Rust builds, ecbench, CodeQL, and the release dry run" to pushes to
`main` and their schedules; `release.yml`, `rust-lint.yml`, `ic-e2e-benchmark.yml`,
`boolean-f5-support-aware.yml` and `ecc2k130-pdp-difficulty-outcome.yml` were
checked here and are push/dispatch-only now. Statements below about what a pull
request triggers describe `main` at `71dd7db5c` unless they say otherwise. After the
change none of these checks runs on a pull request, so the fixes in the follow-up
branch are first exercised on `main` or by `workflow_dispatch`.

## 1. Status of the named classes

| Class | Workflow | On `main` today | First failing step and error | Root cause | In this PR |
|:--|:--|:--|:--|:--|:--|
| Koblitz stage audits | `koblitz-stage*.yml` (119 files) | Not running. All 119 files are `workflow_dispatch`-only (**local**). Four of four sampled (stages 43, 99, 100, 145) are `disabled_manually`, last updated 2026-10-09 22:05-22:07 PDT (**API**) | None found. The recorded stage-100 outcomes are success ([38007448729](https://github.com/isogeny-labs/crypto/actions/runs/38007448729), [38007636058](https://github.com/isogeny-labs/crypto/actions/runs/38007636058), [38007861947](https://github.com/isogeny-labs/crypto/actions/runs/38007861947), 2026-10-10 00:04-00:10Z) and cancelled ([38025808784](https://github.com/isogeny-labs/crypto/actions/runs/38025808784), 04:56Z) | Runner starvation, section 2; no defect in the audits was observed | Nothing. The re-enable step in #1624's test plan has not been done (needs a repo admin) |
| Point-decomposition validation | `ecc2k130-pdp-difficulty-outcome.yml` and `ecc2k130-pdp-difficulty-protocol.yml` | **Red** since 2026-10-08: [37797691034](https://github.com/isogeny-labs/crypto/actions/runs/37797691034); latest [40251433888](https://github.com/isogeny-labs/crypto/actions/runs/40251433888) (replay) and [40251434039](https://github.com/isogeny-labs/crypto/actions/runs/40251434039) (input freeze). Last green [36819153570](https://github.com/isogeny-labs/crypto/actions/runs/36819153570) (2026-10-01) | `verify.py` line 191: `AssertionError: .github/workflows/ecc2k130-pdp-difficulty-protocol.yml`; the freeze job fails identically at `source_preflight.py` line 28 (**log**) | The sealed `SOURCE_LOCK.json` pins that workflow file's bytes (sha256 `2a1aed2f...`). PR #1116 (merge `761f509e8`, 2026-10-08) added an `outcome` job to the file; then the runner-routing script (`c1897c2c7`, 10-20) and the org-runner PRs (`09379b70e`, `b94618d7a`, 10-25) rewrote its `runs-on` (**log + local**) | **Fixed.** File restored to the pinned bytes; the `check_score.py` recompute moved into the unpinned outcome workflow; the file added to the router's `SEALED` list |
| Boolean F5 correctness | `boolean-f5-graded-batch.yml` | Not running: retired to `workflow_dispatch` on 2026-10-11 (comment in the file). Last run [38186739539](https://github.com/isogeny-labs/crypto/actions/runs/38186739539), cancelled, 2026-10-12 | Step "Mathematical and native verifier tests": `error: cannot update the lock file /home/runner/work/crypto/crypto/research/boolean_f5_graded_batch_20261002/Cargo.lock because --locked was passed to prevent this` ([38038835875](https://github.com/isogeny-labs/crypto/actions/runs/38038835875), **log**; the other four failing runs in the brief were not re-read) | The study's lock is stale against the root `crypto` path dependency. Reproduced today: `cargo metadata --locked --manifest-path research/boolean_f5_graded_batch_20261002/Cargo.toml` exits 101 with the same message (**local**). The F5 tests themselves never ran, so this says nothing about F5 correctness | **Not fixed**, section 3.5 |
| Boolean F5 support-aware batch | `boolean-f5-support-aware.yml` (`correctness` ran on push and PR and now runs on push to `main` only; the qualified phases are `workflow_dispatch`-only) | **Red** on 2026-10-26: [40251433848](https://github.com/isogeny-labs/crypto/actions/runs/40251433848) (push, `8bdcd6d0`, 00:52Z) failed on the lock in 3 minutes. The five runs listed from 05:19Z to 10:46Z (for example [40310005070](https://github.com/isogeny-labs/crypto/actions/runs/40310005070)) end in 3-4 s with no step and no retrievable log, so they never reached a runner; the lock cannot be the reason. This matches the "account payment/spending-limit gate" refusals that other sessions recorded in `docs/session-logs/2026-10-26.md` for hosted jobs the same day; it was not confirmed for these runs. The earlier runs 464-468 were not read | Step "Mathematical and native verifier tests": `error: cannot update the lock file /home/runner/work/crypto/crypto/research/boolean_f5_support_aware_20261002/Cargo.lock because --locked was passed to prevent this` (**log**) | The `volcano-cli` entry in the study's lock lacks `libc`, while `tools/volcano-cli/Cargo.toml` has `libc = "0.2"`. Reproduced: `cargo metadata --locked --manifest-path research/boolean_f5_support_aware_20261002/Cargo.toml` exits 101 with the same message (**local**). The lock is not sealed (section 3.5) | **Fixed in the follow-up PR**: lock regenerated by cargo, one line; `--locked` passes (**local**); CI not seen |
| End-to-end benchmark replays | `ic-e2e-benchmark.yml` | Green on every PR head seen: [39197463074](https://github.com/isogeny-labs/crypto/actions/runs/39197463074), [39197392208](https://github.com/isogeny-labs/crypto/actions/runs/39197392208), [39197037341](https://github.com/isogeny-labs/crypto/actions/runs/39197037341), [39195494664](https://github.com/isogeny-labs/crypto/actions/runs/39195494664), [39194714471](https://github.com/isogeny-labs/crypto/actions/runs/39194714471) (2026-10-19); earlier [38047256939](https://github.com/isogeny-labs/crypto/actions/runs/38047256939), [38041236113](https://github.com/isogeny-labs/crypto/actions/runs/38041236113), [38145202544](https://github.com/isogeny-labs/crypto/actions/runs/38145202544). At `71dd7db5c` it **never ran on a push to `main`**: its triggers were `merge_group`, `pull_request` path filters and `workflow_dispatch`. Since `02f2bb3a4c` it is the other way round (push to `main` and `workflow_dispatch` only) | None | n/a | Nothing |
| Solver evidence replay | `ecc2k130-m83-solver-matrix.yml` | Retired to `workflow_dispatch` 2026-10-11. Last success [36514128401](https://github.com/isogeny-labs/crypto/actions/runs/36514128401) (2026-09-29) | None | n/a. Re-run locally today with the workflow's own two steps: `{"verified_files": 131, ... "original_runner_hash_matched": true}`, and `summarize.py` leaves `validated_summary.json` unchanged (**local**) | Nothing |
| Rust lint | `rust-lint.yml` | **Red**: [40251944571](https://github.com/isogeny-labs/crypto/actions/runs/40251944571) (`57d03de2`), [40251434049](https://github.com/isogeny-labs/crypto/actions/runs/40251434049) (`8bdcd6d0`). The run on `71dd7db5c` ([40253272287](https://github.com/isogeny-labs/crypto/actions/runs/40253272287)) was queued at survey time | Step "Clippy" in job `clippy (crypto)`: `error: could not compile `crypto` (lib) due to 1 previous error` and `(lib test) due to 21 previous errors`, e.g. ``manual implementation of `.is_multiple_of()` --> src/cryptanalysis/modular_polynomial.rs:403:28`` (**log**) | New code since #1656 breaks `-D warnings` under the pinned 1.98. The failing lib unit hid every dependent unit from the log, so CI was reporting 21 of at least 62 error sites (**local**, section 3.2) | **Fixed.** 62 sites in 27 Rust files; `cargo clippy --all-targets --keep-going -- -D warnings` now exits 0 locally |
| Release | `release.yml` | **Red**: five of 17 jobs fail in the latest completed non-cancelled runs [40242226163](https://github.com/isogeny-labs/crypto/actions/runs/40242226163) (`8cadd931`), [40240050587](https://github.com/isogeny-labs/crypto/actions/runs/40240050587), [40238021451](https://github.com/isogeny-labs/crypto/actions/runs/40238021451) (the same five job names failed in 40240050587, whose error text was not re-read; 40238021451 was not inspected) | `isogeny` on Linux gnu, musl and macOS, step "Build and test the dedicated executable": `error: cannot update the lock file .../tools/isogeny-cli/Cargo.lock because --locked was passed to prevent this`. `rust (x86_64-pc-windows-msvc)`, step "Build release binaries with source provenance": ``src\bin\ic\sweep\trial.rs:594:39 ... not found in `libc` `` (**log**) | (a) `tools/volcano-cli` gained a `libc` dependency and the `isogeny-cli` lock was not regenerated; the lock diff is exactly one line. (b) `trial.rs` (2026-10-24) calls `libc::getrusage`, which does not exist on Windows. (c) The Windows `isogeny` leg also hit (a) but reported success, section 3.3 | **Linux and macOS confirmed, Windows not.** (a) and (b) are fixed: in [40259290676](https://github.com/isogeny-labs/crypto/actions/runs/40259290676) on the PR head `7f5ca198d` the Linux gnu, musl and macOS `isogeny` legs and `rust` on Linux and macOS pass (**log**). (c) is fixed in the sense that a failing command now fails the step, but both Windows legs were cancelled at their time caps (`isogeny` at 45m15s, `rust` at the 25-minute PR cap), so neither has been seen to pass; `cuda` failed on an unrelated runner-image change. Section 3.3. Follow-up PR: Windows caps raised to 60 (`rust`) and 120 (`isogeny`) minutes for push and dispatch, `cuda` pin restored; the 25-minute PR cap that cancelled `rust (windows)` no longer applies because Release no longer runs on PRs |
| `ic-semantic-metadata` | `ic-semantic-metadata.yml` | **Red**: [40251433841](https://github.com/isogeny-labs/crypto/actions/runs/40251433841) (`8bdcd6d0`); no success found among the 60 runs listed on 2026-10-11 | Step "Fetch authoritative cryptanalysis registry for byte comparison": `curl: (22) The requested URL returned error: 404` (**log**) | **Not confirmed.** The token secret is set (the step passed its empty-token guard) and all five mirrored files exist on `aburan28/cryptanalysis` `main` (the validators landed 2026-10-11 09:30 PDT; **local**). GitHub answers 404 to a token with no access to a private repository, so the leading hypothesis is that `CRYPTANALYSIS_READ_TOKEN` does not cover that repository (an owner move would do it). **Inferred** | **Not fixed**; the step now names the failing file and HTTP status, section 3.4 |

The 2026-10-10 Rust lint failures in the brief (for example run 38047256805: a
`rustfmt` diff in `src/cryptanalysis/prime_fast.rs` and 40 clippy errors) were
fixed by #1656 on 2026-10-11. Today's failures are different, newer code.

## 2. Runner jam and PR #1624

On 2026-10-10 main-branch runs were cancelled in bulk; the `Release` runs for
`9cc38d53` and `845fd323` (created 03:41:53Z and 03:42:07Z) were among them
([38021471054](https://github.com/isogeny-labs/crypto/actions/runs/38021471054),
[38021484816](https://github.com/isogeny-labs/crypto/actions/runs/38021484816)).
The cause, as #1624 records it, was that a path shared by many experiments
(`examples/koblitz_rank_fixture.rs`) matched the path filters of 40+ historical
`koblitz-stage*` workflows on a single PR, which exhausted the account's
concurrent GitHub-hosted runner budget across `crypto`, `cryptanalysis`,
`crypto-autoresearcher`, `cairn` and `dragonfly`. The "about 2,000 cancelled
runs" figure is from #1624's description and was not independently counted.

[#1624](https://github.com/isogeny-labs/crypto/pull/1624) merged 2026-10-10
17:40Z. It made every `koblitz-stage*.yml` `workflow_dispatch`-only, added
`cancel-in-progress` to the ECC2K status and Release PR dry-runs, and pinned the
dispatch-only rule in `scripts/test_workflows_parse.py`. Its description says the
119 stage workflows were disabled by hand on GitHub and must be re-enabled after
the merge; that unchecked item is still open (section 1, row 1). Related changes
on `main` since: #1686 (concurrency groups, no push+PR double trigger, for 185
workflows that no other file references), the 2026-10-11 retirement of the F5 and
m83 gates to `workflow_dispatch`, ARC/Karpenter routing (`c1897c2c7`, 10-20), and
the org-runner switch #2575 (10-25) reverted by #2577 (10-26).

Several `main` runs of `Release` and `Rust lint` on 2026-10-25/26 end `cancelled`
(for example 40251944523, 40251433898). Whether that is the concurrency policy
superseding them or a manual cancel was not determined.

## 3. What changed and how it was checked

### 3.1 PDP replay (fixed)

`SOURCE_LOCK.json` and `SOURCE_CHECK.json` pin
`.github/workflows/ecc2k130-pdp-difficulty-protocol.yml` at
`2a1aed2f352c0cef25635ff64e21da27225d1c12f3300ec1cdcfa26ba87e236f`. That was the
file's content when the passing run 36819153570 happened. The expectation is
right and the file drifted, so the lock is not touched:

- the protocol workflow is restored byte-for-byte (`git show f49b50be2:<path>`;
  the restored file hashes to the pin);
- the `check_score.py` recompute that #1116 had put in that file is now a step of
  `ecc2k130-pdp-difficulty-outcome.yml`, which nothing hashes;
- `taskq/deploy/eks-karpenter/arc/route_workflows.py` gains the workflow in its
  existing `SEALED` set (a data-only edit to a legacy Python script), so a later
  `--apply` does not rewrite it again.

Local replay of every step of both workflows against the restored file, existing
scripts unmodified: `verify.py` PASS (`NO_TRANSFER_WORTHY_PREDICTOR`) and its
`VERIFY.json` field comparison equal; `check_score.py` PASS (`NEGATIVE`);
`check_config.py` output `cmp`-identical to `INPUT_CHECK.json`; `test_source.py`
4 tests OK; `source_preflight.py` output `cmp`-identical to `SOURCE_CHECK.json`.
No file under `research/` is modified.

A repo-wide scan found 83 distinct (workflow path, sha256) pins in sealed files
on `main`. Many are historical receipts or belong to third-party archives, and
only this one was shown to gate a live check. The rest were not classified.

### 3.2 Rust lint (fixed)

CI's log stops at the first failing unit. With `--keep-going` locally the full
set is 62 sites in 27 files, all mechanical:

- the 21 in CI's log: 17 `cloned_ref_to_slice_refs` in `ecbench/bounds.rs`, one
  in `frontier.rs`, one `identity_op` in `isolation.rs`, one `err_expect` in
  `runner.rs`, one `manual_is_multiple_of` in `modular_polynomial.rs`;
- 2 in the new `ic sweep` code (`neg_cmp_op_on_partial_ord` in `objective.rs`,
  written out as `b.is_nan() || *b <= 0.0` so NaN is still refused;
  `large_enum_variant` in `store.rs`, an item-level `#[allow]` with a reason, as
  ten other places in nine linted files already did at the merge base, and eleven
  now that `examples/quartic_ic.rs` has one: `git grep large_enum_variant`, with
  the vendored `research/` snapshots left out);
- 39 in 20 other files hidden behind the lib failure: 12 examples, `ecbench`,
  `icx`, `isogeny_walk` and the `ic` sweep. Single-line rewrites were applied
  from clippy's own suggestions after checking that each source line matched
  the log's quote; the rest by hand: lock files opened with `.truncate(false)`
  (they are flock targets and must keep their contents), unused `drop(closure)`
  removed, two dead `nv` fields removed, a duplicated `#[allow(dead_code)]`
  removed, one doc list given its own paragraph.

Formatting. The 28 changed Rust files (these 27 and `trial.rs`) are all
rustfmt-clean at the PR head, but they were not all clean before it: 21 were, and
seven were not on `main` (`c058bf7cc`, the first parent of the merge, and the
merge base `ac14190f1` agree): `examples/ghs3_vs_rho_analyze.rs`,
`examples/ghs_oakley_isogeny_census.rs`, `examples/ghs_relation_constants.rs`,
`examples/hky_par.rs`, `examples/solinas_order_census.rs`,
`examples/special_prime_curve_constructions.rs` and
`src/cryptanalysis/modular_polynomial.rs` (**local**: `rustfmt --edition 2021
--check` on each file as it was at those commits). The `rustfmt (changed files)`
job in `rust-lint.yml` failed any changed `.rs` file that was not clean (it ran on
pull requests then and no longer does), so touching these seven for the lint fixes
required reformatting them. That churn is rustfmt's
output and nothing else: for each of the seven, formatting the base file with
rustfmt and diffing against the PR head leaves only the lint rewrites. Of the 293
changed lines (removed plus added) in those seven files, rustfmt alone changes
271; the lint rewrites are 28 lines, and the two counts overlap where a rewrite
landed on a reflowed line.

Local toolchain is rustc 1.97.0
and CI pins 1.98: a lint added in 1.98 would not show here, and
`unknown lint: clippy::chunks_exact_to_as_chunks` appears locally for that reason
only.

Tests (debug build in this worktree, rustc 1.97): `cargo test --lib --
ecbench::bounds ecbench::frontier ecbench::isolation ecbench::runner
modular_polynomial` ran 56 tests. Six in `ecbench::bounds` first failed with `No
such file or directory` on committed session data under
`research/ecbench_all_candidates_20261003` and
`research/ecbench_bounds_challenges_20261006`, which this sparse worktree did not
have; with both directories present all 16 `ecbench::bounds` tests pass, so 56 of
56. `cargo test --bin ic -- sweep`: 48 passed, 1 ignored, 0 failed. `--bin icx`: 1
passed. The `isogeny_walk` and `ecbench` bins have no unit tests. The 12 edited
examples were checked by clippy only and not run, and the full `cargo test
--release --lib` that `AGENTS.md` asks for before a push was not run (shared disk
and CPU); CI is that check.


### 3.3 Release (Linux and macOS confirmed; Windows not confirmed)

What #2588 changed:

- `tools/isogeny-cli/Cargo.lock`: regenerated by cargo (`cargo metadata`, no
  hand edit). The diff is one line, `+ "libc"` under `volcano-cli`. `cargo
  metadata --locked` fails before and passes after (**local**). `isogeny_algos`
  already passed.
- `src/bin/ic/sweep/trial.rs`: `peak_rss_mb` followed `experiment.rs`'s gate
  (`cfg(any(target_os = "macos", target_os = "linux"))`, `0` meaning
  unavailable). That gate was too narrow and made the memory net a silent no-op
  where it did not apply; section 3.6.
- Masked failure. The "Build and test the dedicated executable" step has four
  `cargo` commands and no `shell:`, so on `windows-latest` it runs under
  PowerShell, which reports only the last native command's exit status. The
  Windows job log of [run 40242226163](https://github.com/isogeny-labs/crypto/actions/runs/40242226163)
  shows `error: cannot update the lock file ... tools\isogeny-cli\Cargo.lock`
  twice inside that step (the `isogeny-cli` build and test) and the step still
  green after the two `isogeny_algos` commands passed (`Finished release ... in
  19m 02s`, `test result: ok. 5 passed`). The step now has `shell: bash`.

What the first CI run on the PR head showed (**log**, [run
40259290676](https://github.com/isogeny-labs/crypto/actions/runs/40259290676),
head `7f5ca198d`, 2026-10-26 02:28Z):

| Job | Result | Time |
|:--|:--|:--|
| `isogeny` Linux gnu | pass | 36m25s: step 9 (build and test) 23m37s, soft-hash step 10m07s |
| `isogeny` Linux musl | pass | 24m37s |
| `isogeny` macOS | pass | 23m36s |
| `rust` Linux gnu, macOS | pass | 16m33s and 14m25s in "Build release binaries" |
| `isogeny` Windows (job 121369519367) | **cancelled** at the 45-minute cap | 45m15s; inside step 9, 41m42s in |
| `rust` Windows (job 121369519346) | **cancelled** at the 25-minute PR cap | 25m07s; inside "Build release binaries", 23m51s in |
| `cuda` | **failed**, unrelated | "Install nvcc": `E: Unable to locate package cuda-nvcc-12-8` |

So the lock fix works, and the Windows legs are not "fixed": they are not
confirmed either way. The `shell: bash` change did what it was meant to. The
Windows `isogeny` leg used to report green after about 19 minutes because `cargo` gave
up on the lock error before building anything; with the error gone it builds for
real, and a real build does not fit in 45 minutes. The first two cargo commands
(the `isogeny-cli` build and test) took 29m19s on Windows against 14m54s on Linux
gnu, a ratio of 1.97, and the log ends with `isogeny_algos` still compiling
(12m23s in). The step cost 4m09s on Linux gnu on 2026-10-22 ([run
39860097035](https://github.com/isogeny-labs/crypto/actions/runs/39860097035)), so
the work in it grew, not the runner (not investigated). Scaling the Linux gnu
step 9 and soft-hash step by the same 1.97 and adding the 3m04s Windows checkout
gives roughly 70 minutes for the whole leg. That is an estimate: no Windows run of
the real build has finished. The `rust` Windows leg is the same shape: its release
build was past 23m51s, against 16m33s for Linux gnu in that run, so it also
cannot finish inside the PR cap (it took 9m32s in the 2026-10-22 main push run above).

`cuda` is a separate defect from the standard-runner rewrite `b94618d7a`
(2026-10-25): `docs/RELEASES.md` and the job's comment say it pins
`ubuntu-24.04` because the NVIDIA apt repository is addressed by distro, but the
rewrite turned the pin into `ubuntu-latest`, which is now Ubuntu 26.04 (the job
log's runner image). While it fails, the `release` job (which needs it) never runs.

Follow-up PR (`claude/ci-followups-20261026`). Merging current `main` into it
showed that `02f2bb3a4c` had removed the `pull_request` and `merge_group` triggers
from `release.yml`, so there is no PR-time Windows leg left to slim, and the
25-minute PR cap that cancelled `rust (windows)` and the 45-minute `isogeny`
named exception in `check-pr-budgets.py` no longer apply to this workflow (the
exception is now unused; it is left in place). Only push and `workflow_dispatch`
matter:

- `rust` Windows gets 60 minutes and `isogeny` Windows 120 (the others keep 45 and
  50). These are estimates with margin, not measurements: roughly 36 and 70
  minutes by the Linux-scaled figures above.
- `cuda` is pinned to `ubuntu-24.04` again.
- The soft-hash step still rebuilds its dependencies, and a shared target
  directory would not avoid that: the step sets `RUSTFLAGS` to `--cfg
  sha2_256_backend="soft"`, cargo treats different `RUSTFLAGS` as a different
  build of every target-side unit, and none of the step 9 artifacts has that cfg,
  so each unit is compiled again whichever directory it uses. In the Linux gnu log
  of that step the third-party crates take about 2 minutes (03:01:57 to 03:03:51 in
  the log) and the first-party crates (`isogeny_algos`, `isogeny-cli`,
  `volcano-cli`) the other 8; only build scripts and proc-macros are independent of
  `RUSTFLAGS` under `--target`, a small part of the 2 minutes. Not changed.
- `docs/curves/cover-transfer-assessments.json` pins the SHA-256 of `release.yml`,
  so editing the workflow regenerates it (`node
  scripts/build_cover_transfer_assessments.mjs --write`); its other changed hashes
  are files that had already drifted on `main`.

A consequence to watch, not fixed here: with Release only on pushes to `main`, a
Windows leg of about 70 minutes runs while merges land minutes apart. All 100 most
recent completed `Release` push runs on `main` ended `cancelled`, and a run only
publishes if every leg completes, so the rolling release may rarely publish. Whether
a queued run is superseded or an in-progress one is cancelled was not determined.

**Not verified.** No Windows build or workflow run was possible here. Whether the
full legs fit in 60 and 120 minutes and whether `cuda` passes on `ubuntu-24.04`
again are open. To confirm: `gh workflow run release.yml --ref
claude/ci-followups-20261026` runs the full legs and publishes nothing (the
`release` job requires `refs/heads/main`), and its Windows step times replace the
estimates above. Main's push runs cannot be relied on for this (see above).

### 3.4 `ic-semantic-metadata` (not fixed, diagnostic added)

The 2026-10-11 diagnosis (peer repo lacked the mirrored files) is superseded: the
validators landed upstream that day and the token flow was added 2026-10-22
(`71f3dddb1`). Today's 404 happens with a token present, so it is access, not a
missing file, but the log cannot say which of the five requests failed. The loop
now reads the HTTP status and prints the file name and status (`::error::`), which
keeps the check failing and makes the next run self-explanatory. **Owner action**:
confirm `CRYPTANALYSIS_READ_TOKEN` has contents:read on the repository that
`aburan28/cryptanalysis` now resolves to, and update the secret or the URL. The
patch is not verified against the live token.

### 3.5 Boolean F5 (graded batch not fixed; support-aware fixed)

Two F5 studies fail `--locked` because their locks predate the root crate's
current path dependencies (`tools/volcano-cli` among them), and they were handled
differently, because only one lock has a committed bundle that holds a copy of it.

**Graded batch: not fixed, owner decision.** Minimal patch: `cargo metadata
--manifest-path research/boolean_f5_graded_batch_20261002/Cargo.toml` without
`--locked`, which rewrites that lock. It is left unapplied on purpose. `run.sh`
copies the lock into each output bundle, and the committed
`qualified_discovery_01/Cargo.lock` is byte-identical to the study's (`cmp`,
SHA-256 `8ff6242f...`), so a refresh would make the study differ from the bundle
it produced. Whoever owns the study should decide, and should do so before anyone
dispatches the gate (a dispatch fails at `--locked` today; re-checked 2026-10-26:
`cargo metadata --locked` exits 101). Restoring its `push`/`pull_request`
triggers from git history re-arms it.

**Support-aware: fixed.** `research/boolean_f5_support_aware_20261002/Cargo.lock`
lacked `"libc"` in its `volcano-cli` entry; `tools/volcano-cli/Cargo.toml` has
`libc = "0.2"`. Before touching it the lock was checked for a seal and has none:
its SHA-256 appears nowhere on `main` (`git grep` over the whole tree, `research/`
included), the study has no `FROZEN.json`, archive or verify script,
`protocol.json` carries no lock hash, no run bundle is committed, and the original
lock is already preserved byte for byte in `historical-locks/`. The lock had been
refreshed twice before for the same kind of drift (2026-10-11 and 2026-10-19, the
latter with a review note). It was regenerated by cargo (`cargo metadata
--offline`), the diff is the one line `+ "libc"`, and `cargo metadata --locked`
fails before and passes after (**local**). The study's sources are unchanged and
`BUILD_LOCK_REVIEW_20261026.md` records the refresh. The workflow was not edited:
only its `correctness` job runs on push (and ran on PRs until `02f2bb3a4c`), and
the qualified phases stay `workflow_dispatch`-only. The study's tests were not
re-run and CI has not seen the new lock (its `correctness` job runs when this lands
on `main`, because the change touches the study's path).

**Third stale lock, not touched.** `research/full_volcano_classes_20261017/generator/Cargo.lock`
also lacks `libc` under `volcano-cli`. No workflow refers to that directory
(`git grep` over `.github`), so nothing fails because of it. A scan of the 145
`Cargo.lock` files on `main` for a `volcano-cli` entry without `libc` finds exactly
two: the support-aware lock (fixed above) and this one. The graded-batch lock has
no `volcano-cli` entry at all (it predates the dependency), so it is stale in a
different way.

### 3.6 `--max-rss-mb` where `getrusage` does not exist (fixed)

`peak_rss_mb()` returns `0` when it cannot measure, and the watchdog tests
`max_rss_mb > 0 && peak_rss_mb() >= max_rss_mb`, so on such a target the memory
net never fired and nothing said so. #2588 also narrowed the gate to
`any(macos, linux)`, while `glv_gaudry.rs`, `ecbench/record.rs` and
`isogeny_walk/million.rs` use `cfg(unix)`, so FreeBSD and Android lost a guard
that compiled and worked before. Now: `cfg(unix)` (macOS keeps bytes, every other
Unix target KiB, as before #2588); `peak_rss_mb` returns `Option`, with `None`
meaning "cannot measure"; where it cannot, `ic sweep run` and `ic sweep-trial`
each print one warning to stderr and the watchdog no longer polls for a limit it
cannot test. The trial's stderr is captured by `ic sweep run`, which is why the
parent warns too. `docs/ic/SWEEPER.md` and the comments no longer say the net
holds on every host. Checked (**local**): a standalone program built from the
exact lines of `trial.rs` returns the same value as the pre-change function on
Linux (66 and 66 MiB) and prints no warning; built with the Unix cfg switched off
it returns `None`, prints the warning once for two calls, arms nothing, and stays
silent for a limit of 0. On the merged tree `cargo check --release --locked --bin
ic` and `cargo clippy --release --locked --bin ic -- -D warnings` both exit 0 (the
local-1.97 `unknown lint` warning aside) and the touched files are rustfmt-clean;
the `ic` binary was not built or tested, and a Windows build was not run.

### 3.7 PDP outcome workflow (described, one line changed)

The `check_score.py` step moved from the input-freeze workflow's `outcome` job
into the outcome workflow's `replay` job, after `verify.py` with the default
`if: success()`. A failing `verify.py` therefore hides the score check in that
run, which two jobs did not. The step now has `if: ${{ !cancelled() }}` (the workflow runs on push to `main`
only since `02f2bb3a4c`). Two
differences from the old job are described, not changed: the old step was guarded
by `hashFiles(.../OUTCOME.json) != ''` and the moved one is not (`OUTCOME.json`
exists on `main`, so no difference today), and the check name `outcome` from
`ECC2K-130 PDP difficulty input freeze` no longer exists. Nothing in the
repository names that check as required: `git grep` for the workflow's name and
file name outside the two workflow files and this document finds nothing under
`.github`, `docs` or `AGENTS.md`, and `.github/CI.md` (2026-10-11) records that the
default branch had no branch protection or required status contexts. Branch rules
and rulesets live in repository settings, which were not readable here and were not
changed; confirm there that nothing requires `outcome`.

## 4. Reproducible benchmark suite

The program's four requirements map onto mechanisms that already exist. The
exemplars are `research/notes/ecc2k130/m83_solver_matrix_20260927/` and
`research/notes/ecc2k130/compact_swap_quotient_20260929/`. Counts below are on
`main` at `71dd7db5c`: 73 `FROZEN.json`, 3 `SOURCE_FROZEN.json`, 13
`verify_archive.py`, 15 replay-certificate JSON files.

| Requirement | What the repo already does | Where | Gap |
|:--|:--|:--|:--|
| **Exact target fixtures** | Public synthetic known-answer targets only (claim hygiene). Curve named by ICV1 slug / EC1 id. Workload id `W`+12 hex hashes curve, subgroup, generator, target point, law, seed, index, `target_count: 1`, `cache_policy: cold`. Fixture generation command, generator binary sha256, `rustc` version, seed and source hashes recorded in `FROZEN.json`. Solver-visible points kept apart from known-scalar labels ("verifier-only fixture file"). Freeze is two-stage: `SOURCE_FROZEN.json` before any target exists, `FROZEN.json` after | `AGENTS.md`, `docs/ic/boundary_targets.json`; `docs/ecbench/README.md` section 4; swap-quotient `PROTOCOL.md`, `FROZEN.json`, `SOURCE_FROZEN.json`, `fixtures/*.points.jsonl` vs `*.fixture.jsonl` | The point/scalar split and disjointness from earlier panels are stated in one protocol and checked by no shared tool that I found. The swap-quotient `FROZEN.json` source list hashes `Cargo.toml` but not the root `Cargo.lock` (it does pin a sibling study's `research/notes/ecc2k130/compact_shared_log_20260925/Cargo.lock`, as does `SOURCE_FROZEN.json`) |
| **Versioned solver configurations** | Method id `ECM1h`+12 hex hashes the registry id and every parameter with defaults written out; counts are frozen by id (a changed count needs a new id, for example `kangaroo.vow2`). Spec id `ECS1h`, host class `ECBENV3h`, factor base `FB1h`. IC gate configs are files (`docs/ic/params/*.json`) and each reference supersedes the last (`ic-e2e-reference-v1`..`v6`, with `frozen_from` = commit, `ic` binary sha256, host) | `docs/ecbench/README.md` sections 4 and 9; `docs/ic/ci/README.md` | Two parallel systems (ecbench `ECM` ids and `docs/ic/params` + reference vN) with no mapping between them. Toolchain and lock freshness are not part of a configuration's identity. Four lock-drift incidents in this triage (the two F5 study locks, `isogeny-cli`, earlier frozen stage locks), and a stale `volcano-cli` lock no workflow builds (`research/full_volcano_classes_20261017/generator`) |
| **Independent relation verification** | `ecbench verify --replay-all` re-executes runs and requires bit-identical answers, counts, counters and factor base; recomputes derived figures; its receipt hashes every file and its own sha256 is the replay certificate a claim cites; a claim needs a certificate from a different environment and hardware class; CI re-audits every committed session. Per-relation replays exist in single studies: `relations_replayed`, `column_logs_replayed`, `group_replay`, `relation_certified`, `phase_closure` in `f6_ic_geometric_closure_20261003/small_cold/replay-certificate.json`; `verify_panel.py` replays group law, rank transitions, four-sums and scalars; hosted vs second-host receipts compared byte for byte (`second_host_replay_sha256` equals `receipt_sha256` in the manifest); `check_score.py` recomputes without importing the producer | `docs/ecbench/README.md` section 9; swap-quotient `MANIFEST.json`, `verify_panel.py`; m83 `verify_archive.py` | No common relation-level certificate schema across methods. Existing independent verifiers are mostly Python; `AGENTS.md` bans new Python for this, so new verifiers must be Rust, with the Python ones as cross-checks only |
| **Failure accounting** | Every execution writes a sealed record before the next starts; `timeout`, `crashed`, `interrupted` are kept, never dropped; censored pairs bound a ratio (`ops.censored_pairs`, `ops.bound`, verdict `BOUND`); interrupted sessions audited with `--allow-interrupted`; known-incomplete sessions listed with hashes in `docs/ecbench/INCOMPLETE_SESSIONS.tsv`; swap-quotient protocol: "Any missing or failed arm invalidates a complete-result claim; censored work is never silently dropped" and `targets_verified_per_arm` in the manifest | `docs/ecbench/README.md` sections 3, 5, 8; `INCOMPLETE_SESSIONS.tsv`; swap-quotient `PROTOCOL.md`, `MANIFEST.json` | Failure accounting stops at the session. This triage found three places where CI itself lost a failure: the Windows step above; retired replay workflows (F5, m83) that leave sealed evidence un-replayed with no census; and a gate that fails with a bare `curl: (22)` naming no file |

### Checklist a benchmark PR must satisfy

**Targets**
- [ ] Targets are public synthetic known-answer points. The generation command,
  seed, generator binary sha256 and `rustc` version are in a `FROZEN.json` written
  before any evaluation run; a `SOURCE_FROZEN.json` is written before the targets
  exist.
- [ ] The solver sees a points-only file; known scalars are in a separate
  verifier-only file.
- [ ] Disjointness from every earlier panel is checked by a committed command, and
  its output (target-set hashes) is committed.
- [ ] Each target carries its workload id (`W...`) and ICV1 slug.

**Configurations**
- [ ] Every arm is an ecbench method id (`ECM1h...`); changing a counted behaviour
  registers a new id.
- [ ] The freeze hashes `Cargo.toml` **and** `Cargo.lock`, records the `rustc`
  version and binary sha256, and a CI step proves `cargo metadata --locked` for
  every manifest the replay builds.
- [ ] Parameters (K grid, arm order, dp bits, seeds, timeouts, memory caps) are in
  the frozen file, not only in prose.
- [ ] The configuration says which system it lives in (ecbench `ECM` id or
  `docs/ic/params` + reference vN) and cross-references the other.

**Independent verification**
- [ ] A verifier that does not import the producer recomputes every relation, log
  and recovered scalar from public data. It is written in Rust; any Python verifier
  is a cross-check.
- [ ] An `ecbench verify --replay-all` receipt from a second host of a different
  hardware class exists, and its sha256 is recorded as the replay certificate.
- [ ] The CI workflow that runs the verifier is not itself hashed by a seal. If a
  seal must pin workflow bytes, add the file to `SEALED` in `route_workflows.py` and
  to a single check that compares every pinned workflow hash (the PDP incident).

**Failure accounting**
- [ ] The plan lists every cell and the manifest lists every attempted cell with a
  status; the reported denominator is the attempted count.
- [ ] Failures, timeouts and interrupts are kept with their logs; censored arms
  appear as bounds, not as omissions; interrupted sessions are added to
  `INCOMPLETE_SESSIONS.tsv`.
- [ ] No CI step runs several commands under a shell that reports only the last
  exit status (the Windows default); use `shell: bash` or `set -e`.
- [ ] The replay workflow stays triggered on its own paths or by a scheduled
  census. If it is retired, the retirement names an owner and the dispatch command.
- [ ] A cross-repository check reports which file and which HTTP status failed, and
  its secret has a named owner.

## 5. Not verified

- Windows, everywhere. No Windows build or workflow run was possible in this
  worktree. In #2588's CI run both Windows legs were cancelled at their time caps
  (section 3.3), so the `trial.rs` gate and the `shell: bash` change have not been
  seen to build or pass on Windows. The follow-up's 60- and 120-minute estimates
  and the `ubuntu-24.04` pin for `cuda` are likewise untested. A
  `workflow_dispatch` run of `release.yml` on the follow-up branch is the check;
  `main` push runs are mostly cancelled by newer pushes.
- The follow-up's Rust edits (the `--max-rss-mb` warning, a comment in
  `isolation.rs`) were checked with `rustfmt`, `cargo check --release --bin ic` and
  a standalone program; the `ic` tests were not run.
- CI on the regenerated support-aware lock. Its latest runs end in 3 s without starting a job, probably the account payment/spending-limit gate (section 1), which also means hosted CI generally may not start.
- The 1.98 toolchain: lints were checked with 1.97.
- `CRYPTANALYSIS_READ_TOKEN` scope and the `ic-semantic-metadata` fix.
- Re-enabling the 119 `koblitz-stage*` workflows, and the full count of how many
  are disabled (four were sampled).
- Whether any other sealed protocol's pinned workflow bytes gate a live check.
- Repository branch rules and rulesets (section 3.7).
- `docs/ecbench/README.md` section 9 says "The repository does not track
  `Cargo.lock`"; `git ls-files Cargo.lock` lists it. Left as is.

## 6. Corrections to the first version of this document

An independent review of #2588 verified the Rust edits as behaviour-preserving and
found these errors in the text and the workflow (each rechecked before editing):

1. "Fixed (a), (b), (c)" for the Release row. The first CI run on the PR head
   cancelled both Windows legs at their time caps (`isogeny` at 45m15s, `rust` at
   the 25-minute PR cap); Linux and macOS were confirmed. Sections 1 and 3.3.
2. "All 28 changed Rust files were rustfmt-clean on `main`". Seven were not. The
   PR-scoped fmt job forced their reformatting; the churn is rustfmt output.
   Section 3.2.
3. "Five other places" with `large_enum_variant` allows. Ten in nine linted files
   at the merge base. Section 3.2.
4. `ic-e2e-benchmark.yml` also triggered on `merge_group` at `71dd7db5c`
   (it has since moved to push-only). Section 1.
5. The swap-quotient `FROZEN.json` omits the root `Cargo.lock` only; it pins a
   sibling study's. Section 4.
6. The support-aware F5 workflow had the same stale-lock failure and was missing
   from the table, and "three lock-drift incidents" was a miscount. Sections 1,
   3.5 and 4.
7. The Windows time caps. Section 3.3.
8. The `peak_rss_mb` gate, which silently disabled the memory net and narrowed
   `cfg(unix)`. Section 3.6.
9. The PDP check ordering and the vanished `outcome` check name. Section 3.7.
