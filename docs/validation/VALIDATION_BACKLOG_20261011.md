# Validation backlog: CI triage and reproducible-benchmark requirements

Scoped 2026-10-11, re-surveyed 2026-10-26 against `main` at `71dd7db5c`. Run
links are to `isogeny-labs/crypto` (formerly `aburan28/crypto`). Each finding is
labelled by how it is known: **log** (quoted from a CI job log), **local** (re-run
in a worktree), or **inferred** (not directly observed). Nothing here edits a
sealed measurement, and no workflow or test was skipped, disabled or quarantined
to make a check pass.

## 1. Status of the named classes

| Class | Workflow | On `main` today | First failing step and error | Root cause | In this PR |
|:--|:--|:--|:--|:--|:--|
| Koblitz stage audits | `koblitz-stage*.yml` (119 files) | Not running. All 119 files are `workflow_dispatch`-only (**local**). Four of four sampled (stages 43, 99, 100, 145) are `disabled_manually`, last updated 2026-10-09 22:05-22:07 PDT (**API**) | None found. The recorded stage-100 outcomes are success ([38007448729](https://github.com/isogeny-labs/crypto/actions/runs/38007448729), [38007636058](https://github.com/isogeny-labs/crypto/actions/runs/38007636058), [38007861947](https://github.com/isogeny-labs/crypto/actions/runs/38007861947), 2026-10-10 00:04-00:10Z) and cancelled ([38025808784](https://github.com/isogeny-labs/crypto/actions/runs/38025808784), 04:56Z) | Runner starvation, section 2; no defect in the audits was observed | Nothing. The re-enable step in #1624's test plan has not been done (needs a repo admin) |
| Point-decomposition validation | `ecc2k130-pdp-difficulty-outcome.yml` and `ecc2k130-pdp-difficulty-protocol.yml` | **Red** since 2026-10-08: [37797691034](https://github.com/isogeny-labs/crypto/actions/runs/37797691034); latest [40251433888](https://github.com/isogeny-labs/crypto/actions/runs/40251433888) (replay) and [40251434039](https://github.com/isogeny-labs/crypto/actions/runs/40251434039) (input freeze). Last green [36819153570](https://github.com/isogeny-labs/crypto/actions/runs/36819153570) (2026-10-01) | `verify.py` line 191: `AssertionError: .github/workflows/ecc2k130-pdp-difficulty-protocol.yml`; the freeze job fails identically at `source_preflight.py` line 28 (**log**) | The sealed `SOURCE_LOCK.json` pins that workflow file's bytes (sha256 `2a1aed2f...`). PR #1116 (merge `761f509e8`, 2026-10-08) added an `outcome` job to the file; then the runner-routing script (`c1897c2c7`, 10-20) and the org-runner PRs (`09379b70e`, `b94618d7a`, 10-25) rewrote its `runs-on` (**log + local**) | **Fixed.** File restored to the pinned bytes; the `check_score.py` recompute moved into the unpinned outcome workflow; the file added to the router's `SEALED` list |
| Boolean F5 correctness | `boolean-f5-graded-batch.yml` | Not running: retired to `workflow_dispatch` on 2026-10-11 (comment in the file). Last run [38186739539](https://github.com/isogeny-labs/crypto/actions/runs/38186739539), cancelled, 2026-10-12 | Step "Mathematical and native verifier tests": `error: cannot update the lock file /home/runner/work/crypto/crypto/research/boolean_f5_graded_batch_20261002/Cargo.lock because --locked was passed to prevent this` ([38038835875](https://github.com/isogeny-labs/crypto/actions/runs/38038835875), **log**; the other four failing runs in the brief were not re-read) | The study's lock is stale against the root `crypto` path dependency. Reproduced today: `cargo metadata --locked --manifest-path research/boolean_f5_graded_batch_20261002/Cargo.toml` exits 101 with the same message (**local**). The F5 tests themselves never ran, so this says nothing about F5 correctness | **Not fixed**, section 3.5 |
| End-to-end benchmark replays | `ic-e2e-benchmark.yml` | Green on every PR head seen: [39197463074](https://github.com/isogeny-labs/crypto/actions/runs/39197463074), [39197392208](https://github.com/isogeny-labs/crypto/actions/runs/39197392208), [39197037341](https://github.com/isogeny-labs/crypto/actions/runs/39197037341), [39195494664](https://github.com/isogeny-labs/crypto/actions/runs/39195494664), [39194714471](https://github.com/isogeny-labs/crypto/actions/runs/39194714471) (2026-10-19); earlier [38047256939](https://github.com/isogeny-labs/crypto/actions/runs/38047256939), [38041236113](https://github.com/isogeny-labs/crypto/actions/runs/38041236113), [38145202544](https://github.com/isogeny-labs/crypto/actions/runs/38145202544). **It never runs on `main`**: triggers are `pull_request` path filters and `workflow_dispatch` | None | n/a | Nothing |
| Solver evidence replay | `ecc2k130-m83-solver-matrix.yml` | Retired to `workflow_dispatch` 2026-10-11. Last success [36514128401](https://github.com/isogeny-labs/crypto/actions/runs/36514128401) (2026-09-29) | None | n/a. Re-run locally today with the workflow's own two steps: `{"verified_files": 131, ... "original_runner_hash_matched": true}`, and `summarize.py` leaves `validated_summary.json` unchanged (**local**) | Nothing |
| Rust lint | `rust-lint.yml` | **Red**: [40251944571](https://github.com/isogeny-labs/crypto/actions/runs/40251944571) (`57d03de2`), [40251434049](https://github.com/isogeny-labs/crypto/actions/runs/40251434049) (`8bdcd6d0`). The run on `71dd7db5c` ([40253272287](https://github.com/isogeny-labs/crypto/actions/runs/40253272287)) was queued at survey time | Step "Clippy" in job `clippy (crypto)`: `error: could not compile `crypto` (lib) due to 1 previous error` and `(lib test) due to 21 previous errors`, e.g. ``manual implementation of `.is_multiple_of()` --> src/cryptanalysis/modular_polynomial.rs:403:28`` (**log**) | New code since #1656 breaks `-D warnings` under the pinned 1.98. The failing lib unit hid every dependent unit from the log, so CI was reporting 21 of at least 62 error sites (**local**, section 3.2) | **Fixed.** 62 sites in 27 Rust files; `cargo clippy --all-targets --keep-going -- -D warnings` now exits 0 locally |
| Release | `release.yml` | **Red**: five of 17 jobs fail in the latest completed non-cancelled runs [40242226163](https://github.com/isogeny-labs/crypto/actions/runs/40242226163) (`8cadd931`), [40240050587](https://github.com/isogeny-labs/crypto/actions/runs/40240050587), [40238021451](https://github.com/isogeny-labs/crypto/actions/runs/40238021451) (the same five job names failed in 40240050587, whose error text was not re-read; 40238021451 was not inspected) | `isogeny` on Linux gnu, musl and macOS, step "Build and test the dedicated executable": `error: cannot update the lock file .../tools/isogeny-cli/Cargo.lock because --locked was passed to prevent this`. `rust (x86_64-pc-windows-msvc)`, step "Build release binaries with source provenance": ``src\bin\ic\sweep\trial.rs:594:39 ... not found in `libc` `` (**log**) | (a) `tools/volcano-cli` gained a `libc` dependency and the `isogeny-cli` lock was not regenerated; the lock diff is exactly one line. (b) `trial.rs` (2026-10-24) calls `libc::getrusage`, which does not exist on Windows. (c) The Windows `isogeny` leg also hit (a) but reported success, section 3.3 | **Fixed (a), (b), (c)**; Windows not verifiable locally |
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
  five other places in the tree already do);
- 39 in 20 other files hidden behind the lib failure: 12 examples, `ecbench`,
  `icx`, `isogeny_walk` and the `ic` sweep. Single-line rewrites were applied
  from clippy's own suggestions after checking that each source line matched
  the log's quote; the rest by hand: lock files opened with `.truncate(false)`
  (they are flock targets and must keep their contents), unused `drop(closure)`
  removed, two dead `nv` fields removed, a duplicated `#[allow(dead_code)]`
  removed, one doc list given its own paragraph.

All 28 changed Rust files (these 27 and `trial.rs`) were rustfmt-clean on `main`
and still are (the PR-scoped fmt job needs that). Local toolchain is rustc 1.97.0
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


### 3.3 Release (fixed; Windows unverified)

- `tools/isogeny-cli/Cargo.lock`: regenerated by cargo (`cargo metadata`, no
  hand edit). The diff is one line, `+ "libc"` under `volcano-cli`. `cargo
  metadata --locked` fails before and passes after (**local**). `isogeny_algos`
  already passed.
- `src/bin/ic/sweep/trial.rs`: `peak_rss_mb` now follows `experiment.rs`'s
  existing gate (`cfg(any(target_os = "macos", target_os = "linux"))`, `0` means
  unavailable). Linux build and clippy are clean. **A Windows build was not run**;
  only the CI Windows leg can confirm it.
- Masked failure. The "Build and test the dedicated executable" step has four
  `cargo` commands and no `shell:`, so on `windows-latest` it runs under
  PowerShell, which reports only the last native command's exit status. The
  Windows job log of [run 40242226163](https://github.com/isogeny-labs/crypto/actions/runs/40242226163)
  shows `error: cannot update the lock file ... tools\isogeny-cli\Cargo.lock`
  twice inside that step (the `isogeny-cli` build and test) and the step still
  green after the two `isogeny_algos` commands passed (`Finished release ... in
  19m 02s`, `test result: ok. 5 passed`). The step now has `shell: bash`. If the
  Windows leg turns red after this PR, that failure was previously hidden, and the
  PR did not cause it.

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

### 3.5 Boolean F5 (not fixed)

Minimal patch: `cargo metadata --manifest-path
research/boolean_f5_graded_batch_20261002/Cargo.toml` without `--locked`, which
rewrites that lock. It is left unapplied on purpose. `run.sh` copies the lock into
each output bundle, and the committed `qualified_discovery_01/Cargo.lock` is
byte-identical to the study's, so a refresh would make the study differ from the
bundle it produced. Whoever owns the study should decide, and should do so before
anyone dispatches the gate (a dispatch fails at `--locked` today). Restoring its
`push`/`pull_request` triggers from git history re-arms it.

## 4. Reproducible benchmark suite

The program's four requirements map onto mechanisms that already exist. The
exemplars are `research/notes/ecc2k130/m83_solver_matrix_20260927/` and
`research/notes/ecc2k130/compact_swap_quotient_20260929/`. Counts below are on
`main` at `71dd7db5c`: 73 `FROZEN.json`, 3 `SOURCE_FROZEN.json`, 13
`verify_archive.py`, 15 replay-certificate JSON files.

| Requirement | What the repo already does | Where | Gap |
|:--|:--|:--|:--|
| **Exact target fixtures** | Public synthetic known-answer targets only (claim hygiene). Curve named by ICV1 slug / EC1 id. Workload id `W`+12 hex hashes curve, subgroup, generator, target point, law, seed, index, `target_count: 1`, `cache_policy: cold`. Fixture generation command, generator binary sha256, `rustc` version, seed and source hashes recorded in `FROZEN.json`. Solver-visible points kept apart from known-scalar labels ("verifier-only fixture file"). Freeze is two-stage: `SOURCE_FROZEN.json` before any target exists, `FROZEN.json` after | `AGENTS.md`, `docs/ic/boundary_targets.json`; `docs/ecbench/README.md` section 4; swap-quotient `PROTOCOL.md`, `FROZEN.json`, `SOURCE_FROZEN.json`, `fixtures/*.points.jsonl` vs `*.fixture.jsonl` | The point/scalar split and disjointness from earlier panels are stated in one protocol and checked by no shared tool that I found. The swap-quotient `FROZEN.json` source list hashes `Cargo.toml` but not `Cargo.lock` |
| **Versioned solver configurations** | Method id `ECM1h`+12 hex hashes the registry id and every parameter with defaults written out; counts are frozen by id (a changed count needs a new id, for example `kangaroo.vow2`). Spec id `ECS1h`, host class `ECBENV3h`, factor base `FB1h`. IC gate configs are files (`docs/ic/params/*.json`) and each reference supersedes the last (`ic-e2e-reference-v1`..`v6`, with `frozen_from` = commit, `ic` binary sha256, host) | `docs/ecbench/README.md` sections 4 and 9; `docs/ic/ci/README.md` | Two parallel systems (ecbench `ECM` ids and `docs/ic/params` + reference vN) with no mapping between them. Toolchain and lock freshness are not part of a configuration's identity. Three lock-drift incidents in this triage (F5 study, `isogeny-cli`, earlier frozen stage locks) |
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

- Windows: the `trial.rs` gate and the `shell: bash` change were not built or run
  on Windows. The first CI run on this branch is the check.
- The 1.98 toolchain: lints were checked with 1.97.
- `CRYPTANALYSIS_READ_TOKEN` scope and the `ic-semantic-metadata` fix.
- Re-enabling the 119 `koblitz-stage*` workflows, and the full count of how many
  are disabled (four were sampled).
- Whether any other sealed protocol's pinned workflow bytes gate a live check.
- `docs/ecbench/README.md` section 9 says "The repository does not track
  `Cargo.lock`"; `git ls-files Cargo.lock` lists it. Left as is.
