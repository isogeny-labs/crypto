# Readiness: research/ic-representative-selection_20261105, Stage 0 run and path to the first EKS job

Date 2026-11-14. Track A of the lead-chasing program. Repository
/home/user/crypto, branch `claude/serene-davinci-ujoi58`, HEAD
`0c2cc1b408e33a43e55ee31cb75307e90df9d619` (equal to origin/main when started).
Every number in this file is `[pcc:unverified]`: it was produced by this session
and has no pcc claim id. Estimates are labelled *estimate*. Nothing was
submitted to AWS, EKS, Iso Lab or any cloud; no git commit, push or branch
change was made; no frozen file was edited. The drift evidence is in
[`DRIFT.md`](DRIFT.md); raw logs are in [`logs/`](logs/) with `SHA256SUMS`.

## 1. What ran

Disk before any build: `df -h /` 3.4 GB free (3.3 GB at the build start). Free
space after Stage 0 and the drift checks: 3.1 GB. A watcher was prepared to stop
cargo below 1.7 GB free; it never fired. No worktree was created; the build used
the existing `target/`. Only one cargo build ran at a time.

| step | command (from repo root) | exit | wall | output |
|:--|:--|:--:|:--|:--|
| Stage 0a | `research/ic-representative-selection_20261105/run/00-build.sh` (`cargo build --release --locked --bin volcano-rho --bin ecbench`, native `cargo build`, native `cargo test`, hash record) | 0 | cargo build 8 min 21 s | `logs/00-build.log`; `inputs/tools.sha256` |
| native tests, as asked | `cargo test --release --manifest-path research/ic-representative-selection_20261105/native/Cargo.toml` | 0 | seconds | `logs/cargo-test-native.log` |
| Stage 0b, 24 bit | `run/01-classes.sh 24` | 0 | 9.6 s | `logs/01-classes-24.log`; `inputs/classes/t24*` |
| Stage 0b, 28 bit | `run/01-classes.sh 28` | 0 | 6.4 s | `logs/01-classes-28.log`; `inputs/classes/t28*` |
| Stage 0c | `run/02-specs.sh describe` (the script takes no arguments; `describe` is ignored) | 0 | 0.4 s | `logs/02-specs.log`; `inputs/specs/`, `inputs/jobs.tsv`, `inputs/MANIFEST.sha256` |

`cargo test` result, verbatim from `logs/cargo-test-native.log`:

```
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s   (src/lib.rs)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s    (src/main.rs)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s    (tests/pipeline.rs)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s    (doc-tests)
exit=0
```

That is 16 tests, as the README states. The same tests ran inside `00-build.sh`
(exit 0).

Stage 0 outputs (hashes of what is on disk now, the second run; see 2.2 for the
first):

| file | sha256 |
|:--|:--|
| `inputs/tools.sha256` | `2b2dbd04c45a3030fca1d4759eeba2397fdeba8602d30fbd0d1fa85495c762df` |
| `target/release/volcano-rho` | `bf7c2e149afd01e06df0bf7ade1c5ddbbecf3a11fd466553659640150730fd88` |
| `target/release/ecbench` | `73c4f66bdbd1b7d795ab142fea18dafbb3476d876a8f9f81fde4aeb4f9e581e7` |
| `native/target/release/repsel` | `28f69273899d617308a5fd50b74ffa9becbb41181799a537648761302905f37f` |
| `inputs/jobs.tsv` | `038aaafc19377d9327cd082376d21643c2008f64bbc3cedbf6ff733243b241b8` |
| `inputs/MANIFEST.sha256` (578 lines) | `f5e7e64db409f84ccbe6efff635d6f969637f4a0bbd0d3760e6624a5f36d2e1f` (run 2; run 1 was `9d4d5cbce666fbb583558897b0393a459cd0d6999c6532506ddecff7300f188b`, see 2.2) |
| `inputs/MANIFEST.sha256` without the 96 class-level `spec*.json` lines | `7bbc0fd1a4d6638bb31a7c3f6f76e6d8d8adddb51332c096fa41b2f92ca1ae73` (same in both runs) |
| `inputs/classes/t24.selection.log` | `522c420cb4758db71469c03c8da7a0950edb46eed45bf91f7547a22ce730efa2` |
| `inputs/classes/t28.selection.log` | `5aaaf0e364cdd9cdd2716066dad21701c122941089737b9214d2d89bc351853e` |

`inputs/` (12 MB) is untracked in git; `native/target` is gitignored.

Classes: tier 24, 34 seeds tried (s2401 to s2434), 12 accepted
(2401, 2409, 2414, 2415, 2416, 2418, 2421, 2424, 2426, 2428, 2433, 2434), 22
rejected. Tier 28, 36 seeds tried (s2801 to s2836), 12 accepted (2801, 2803,
2804, 2806, 2811, 2819, 2823, 2830, 2831, 2833, 2834, 2836), 24 rejected. Every
one of the 46 rejections has the same reason, `duplicate workload` raised by
`repsel specs` (see 2.4). No seed was rejected for `volcano-rho` failure, a
reused p, a too-small export or a failed `repsel ascend`. Transport operation
counts from `repsel ascend` on tier 24 class s2401: B 1,080 to 1,087 and B0 650 to
652 native operations over 36 floor curves (one class shown; the others are in
`inputs/classes/t*/s*/ascend.json`).

Checks run beyond the scripts (details in DRIFT.md): `ecbench plan` on all 205
specs, exit 0; `validate-request` on 205 requests built as `03-launch.sh` builds
them with a placeholder image receipt, all pass; exact replay of committed v2
records on the HEAD binary, 600 random and 4 named, 0 differences; a second
run of Stage 0 gave the same `jobs.tsv`, selection logs and `curves.jsonl`.

## 2. Observations on the frozen files (not edited; fixes described)

1. **`run/06-analyse.sh` cross-check reading.** `ecbench band` at HEAD prices the
   IC table's counted inserts by default; `repsel analyse` does not. Fix: add
   `--no-price-table-work` to the `ecbench band` line (keeps the frozen reading), or
   file an amendment that adopts the new default and adds the phase sum of
   `table_insert_attempts` to U in `analyse.rs`. A choice for the protocol owner;
   DRIFT.md section E5.
2. **`run/01-classes.sh` writes a random scratch path into the class spec
   descriptions.** `--out "$scratch/s<seed>"` with `scratch=$(mktemp -d)` ends up in
   the `description` of `inputs/classes/t*/s*/spec*.json` (96 files). Running Stage
   0 twice gave different `MANIFEST.sha256` digests (`9d4d5cbc…` then `f5e7e64d…`)
   while every other file, `jobs.tsv` and both selection logs were byte-identical.
   The A1 digest of the manifest as the protocol defines it is therefore not
   reproducible by a second party. Fix: a fixed scratch path (for example
   `$out/.scratch`), or A1 names the manifest minus those 96 lines
   (`7bbc0fd1…ae73`), which is stable.
3. **`run/00-build.sh` records `git rev-parse HEAD`, not the frozen commit, and
   does not require the checkout to be the frozen commit.** `inputs/tools.sha256`
   line 1 is `0c2cc1b408`. Fix: assert `git rev-parse HEAD` equals the intended
   build revision, or state the revision in A1.
4. **The acceptance rule in `design.json` does not list the `duplicate workload`
   refusal that decided every rejected seed** (46 of 70 seeds, 66%). It is in
   `repsel specs` (`write_jobs`, "refuse ... any duplicate workload") and is logged
   per seed, so nothing is hidden, but the 24 accepted classes are conditioned on
   it. A1 should say so. I did not investigate what in a rejected class
   produces the duplicate.
5. **`02-specs.sh` ordering.** `sort -t s -k2 -n` on a path that contains many `s`
   characters sorts by whole-line byte order, which happens to equal seed order for
   four-digit seeds. Correct here; fragile if a tier needed a fifth digit.
6. **Python in the route.** `run/CARD.md` calls `python3 tools/benchrec.py`; the
   cards come from `tools/task_card.py`; the protocol hash comes from
   `crypto-autoresearcher/tools/experiment_protocol.py`. These are repository-owned
   shared tools, not study code, and this track did not run them. Whether the
   no-Python rule applies to them is for the main session.

## 3. Drift verdict (full evidence in DRIFT.md)

Counted quantities of the methods the study calls show no change from
c6cd50bcc8 to HEAD in the evidence collected: old records replay identically on
the HEAD binary and the HEAD `volcano-rho` regenerates a committed class byte for
byte, so building at HEAD gives the same counted quantities within what this
evidence reaches; a binary at c6cd50bcc8 was not built or compared. The pricing
reading of `ecbench band` and `compare` changed on 2026-11-08 (table inserts
priced by default), which the frozen cross-check does not match, and the Stage 0
binaries were built at HEAD rather than at the frozen commit; both belong in A1.

## 4. Job budget (all figures *estimate*, labelled by model)

### 4.1 Counts

From the specs Stage 0 produced (`inputs/jobs.tsv`; workload = curve x target;
every workload runs each arm for 5 measured rounds + 1 warmup = 6 executions):

| tier | jobs | aa | ab | b | cal | ctl | workloads | IC executions | rho executions | BSGS executions |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 24 | 100 | 12 | 24 | 12 | 40 | 12 | 35,728 | 224,256 | 428,736 | 9,888 |
| 28 | 105 | 12 | 24 | 12 | 45 | 12 | 24,352 | 155,904 | 292,224 | 9,792 |
| pre-selection total | 205 | | | | | | 60,080 | 380,160 | 720,960 | 19,680 |

("IC executions" counts the `ic-mitm` arm; the ctl stage adds a second IC arm,
`ic-mitm-plus1`, counted in the IC column. rho executions are `rho-neg` plus
`rho-neg-aa`. Counted rounds are 5/6 of these.)

Hold jobs do not exist until `run/05-hold.sh` runs on each class's selection. From
`specs.rs` `hold_jobs` and `design.json` (hold shard 16 at tier 24, 8 at tier 28;
holdaa shard 16; at most 16 crater vertices per class; 12 classes at tier 24 and 6 at
tier 28; each pick runs models {0, ic_model, rho_model}, one to three distinct):

| | jobs | workloads | IC executions | rho executions |
|:--|--:|--:|--:|--:|
| hold + holdaa, tier 24 | 24 | 3,072 to 6,144 | 18,432 to 36,864 | 36,864 to 73,728 |
| hold + holdaa, tier 28 | 18 | 1,536 to 3,072 | 9,216 to 18,432 | 18,432 to 36,864 |
| hold total (low to high) | 42 | 4,608 to 9,216 | 27,648 to 55,296 | 55,296 to 110,592 |

Study total, *estimate*: about 247 jobs (PROTOCOL.md says about 234), IC
executions 408,000 to 435,000, rho executions 776,000 to 832,000.

### 4.2 Time on the stated host (c8a.2xlarge, 8 vCPU, one L2 worker, one job at a time)

Two models, because they disagree by about 2x and neither is a measurement of this
study:

- **Frozen cost model** (`design.json` `est_seconds_per_run`: tier 24 IC 0.15 s,
  rho 0.1 s; tier 28 IC 0.6 s, rho 0.12 s; per workload-round `2 rho + IC`, plus
  `bsgs + IC` in ctl). Pre-selection measurement 207,572 s (57.7 h; tier 24 21.5 h,
  tier 28 36.1 h; largest job 1,374 s against the 1,400 s budget). Hold 14,194 to
  28,386 s. Measurement total 61.6 to 65.5 h. The protocol says the harness
  doubles it with `--replay-all` (assumed: replay costs the same as the run):
  123.2 to 131.1 h. Plus per-job provisioning and settle, assumed 300 s + 60 s
  from `research/prime_medium_rho_p4_confirm_20261108/analysis/cost.json`
  (`provisioning_s_per_job` 300, `settle_s` 60), 247 jobs: 24.7 h. **Job-hours
  147.9 to 155.8; serial wall 6.2 to 6.5 days.**
- **Sweep-implied** (`taskq/deploy/eks-karpenter/validation/ic-size-sweep-20261026`,
  `n24/session/session.json`, `n28/session/session.json`: whole-session wall of 18
  records = 6 rounds of `rho-neg`, `rho-neg-aa`, `ic.pipeline` on one curve and one
  target on c8a.2xlarge Spot, L2: 0.483 s at 24 bits, 2.565 s at 28 bits; so 0.0805 s
  and 0.4275 s per workload-round). This is n = 1 curve and the sweep README says
  the 28-bit A/A interval is wide. Measurement 23.7 to 25.2 h, doubled 47.3 to
  50.3 h, plus the same 24.7 h overhead. **Job-hours 72.0 to 75.0; serial wall 3.0
  to 3.1 days.** The frozen model is 4.3x slower at 24 bits and 2.0x slower at 28
  bits than this sample.

Both fit inside the protocol's 10-day deadline from the first launch (6.5 days
worst case, with no retries); a `-r2` retry per failed job is allowed once and is
not in these numbers. The per-job pod cap is 3,600 s (`03-launch.sh`); the largest
job under the frozen model is 1,374 s of measurement, about 2,750 s with replay,
a 1.3x margin, and about 700 s under the sweep-implied model.

### 4.3 Dollars

A Spot price is recorded in the repository: **USD 0.0988 to 0.1577 per hour for
c8a.2xlarge**, "measured Spot prices", in
`research/prime_medium_rho_p4_confirm_20261108/analysis/cost.json`
(`assumptions.spot_usd_per_hour`) and `.../README.md` line 111; a planning figure
of "c8a.2xlarge Spot about 0.16 USD/h" is in
`research/arity_completion_20261104/launch/README.md` line 79. Neither is a price
read today; prices move.

| model | job-hours | USD at 0.0988/h | USD at 0.1577/h | USD at 0.16/h |
|:--|--:|--:|--:|--:|
| frozen cost model | 147.9 to 155.8 | 14.6 to 15.4 | 23.3 to 24.6 | 23.7 to 24.9 |
| sweep-implied | 72.0 to 75.0 | 7.1 to 7.4 | 11.2 to 11.8 | 11.5 to 12.0 |

*Estimate*: about USD 7 to 25 of instance time, against the protocol's ceiling of
USD 150 and the daily allowance of USD 200 (`COST_CONTROLS.md`). Not included:
CodeBuild launcher minutes, the image publish build, S3, EBS, data transfer,
retries, the A/A and replay overhead if replay costs more than the run, and
Karpenter idle time between jobs.

## 5. Remaining steps to the first EKS job, in order

The first job is the first line of `inputs/jobs.tsv`:
`repsel-t24-s2401-ab-0` (256 workloads, estimated 538 s).

| # | step | who / what it needs | state now |
|--:|:--|:--|:--|
| 1 | Decide D1: the table-insert pricing reading (frozen reading with `--no-price-table-work` in the cross-check, or adopt the 2026-11-08 default and amend `analyse.rs`) | protocol owner / core or frontier tier; DRIFT.md E5 | open |
| 2 | Decide D2: the revision the image is built from (the protocol and `CARD.md` say the frozen commit `c6cd50bcc8fa382ab89c9cf6fadb0145fef5d1a1`; Stage 0 used HEAD `0c2cc1b408`) and state it in A1. If the image is built at c6cd50bcc8, the counted-quantity statement in DRIFT.md rests on replay evidence only; building a c6cd50bcc8 binary and comparing records on one spec would close it (needs about 1.5 GB and a worktree or a checkout swap, which this host cannot afford beside `target/`) | core tier; needs disk or another host | open |
| 3 | Run the launch scripts from a checkout that has the `run-request` PATH fix (HEAD has it, c6cd50bcc8 does not): `launch-remote` uploads the local `remote/run-request` to S3 and the launcher runs that copy, so a launch from the frozen checkout would risk the exit 127 the fix addresses (reading of `launch-remote` lines 14-32; not tested) | executor's checkout | open |
| 4 | Get a fresh checkpoint upload token. The one in this environment expired `2026-11-12T04:12:09Z` (`scripts/checkpoint.sh gate` fails with that message). The owner's Mac job re-mints `~/.config/checkpoint/upload-token` every 15 minutes, or the session is handed `CHECKPOINT_UPLOAD_TOKEN`; `~/.config/checkpoint` does not exist here | owner / a session on the owner's side | blocked here |
| 5 | Rewrite `.resume/RESUME.md` for this effort. The present file belongs to a different effort ("Yes to all open leads", 2026-11-08); fill goal, lead, spec path, state, next commands, runs in flight, stop conditions; `scripts/checkpoint.sh init`, then `push`, then `gate` must pass before any run launcher (hooks refuse launchers otherwise) | main session | open |
| 6 | Conductor: `conductor check --summary "…" --scope path:research/ic-representative-selection_20261105`; Conductor is not installed here (hooks warn only) | main session / owner | not available here |
| 7 | Draw the mask nonce after the inputs freeze: `od -An -N8 -tx8 /dev/urandom`, recorded as the line `mask nonce: <16 hex>` in INPUTS.md | frozen-analysis owner (not this track, to keep the key unseen by the analyst) | open |
| 8 | File amendment A1 in `INPUTS.md`: the build revision (D2) and `inputs/tools.sha256`; `inputs/classes/t24.selection.log` and `t28.selection.log` with their sha256 above; the manifest digest in the form chosen (full manifest from the run kept, or minus the class-level `spec*.json`, 2.2) and `jobs.tsv` (sha256 in section 1); the Iso Lab publish receipt `inputs/image.json`; the nonce line; the D1 reading; the `duplicate workload` conditioning (2.4); and a statement that no IC cost of any study curve has been seen. This track ran no `ecbench run` on any Stage 0 spec (only `ecbench plan`), and the class search ran with `--controls 0`; whoever files A1 makes the statement for their own session | main session | open |
| 9 | Amend `protocol.json` with the `A1` entry (`filed_before_affected_data: true`) naming INPUTS.md and its digests; recompute the protocol hash with `crypto-autoresearcher/tools/experiment_protocol.py` | main session; the tool is Python, see 2.6 | open |
| 10 | Commit, push, open the PR ready for review (not draft), obtain the `check-experiment-git` receipt for the pushed commit (`taskq/deploy/eks-karpenter/check-experiment-git`) | main session (Git write access; this track did none) | open |
| 11 | Publish the image: `gh workflow run isolab-experiment.yml --repo aburan28/crypto --ref main -f source_revision=<D2 sha> -f publish_only=true`, then write `inputs/image.json` `{"image_uri": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:<64 hex>", "source_revision": "<sha>", "ecbench_sha256": "<64 hex>"}`. The `ecbench_sha256` is the binary inside the image, not the local `73c4f66b…` | a session with repository Actions access; not available in this sandbox (GitHub access is bound to the session repository; not tested) | open |
| 12 | AWS identity for the executor: `aws sts get-caller-identity` must show account 104186811093. Not probed here (no cloud call was made); AWS credential variables are present in the environment by name only, and their validity is unknown | executor session | unchecked |
| 13 | Budget and daily-allowance check (`COST_CONTROLS.md`, `cost-controls.json`; ceiling USD 150 for the study, stop when exhausted or unknown) | main session | open |
| 14 | Director compiles the first card from `run/CARD.md` (`tools/task_card.py`, Sonnet 5.5 `experiment-director`, binds SHA-256 of `03-launch.sh`, `04-collect.sh`, `common.sh`, the job spec, `inputs/image.json`, `validate-request`, `launch-remote`); main session relays it verbatim to `executor-mechanical` (Haiku 5.5); `benchrec init`, `run`, `add-raw`, `close` per job | main session, director, executor | after 1 to 13 |
| 15 | Launch `repsel-t24-s2401-ab-0` under `tools/benchrec.py run <record> -- run/03-launch.sh repsel-t24-s2401-ab-0`, then `run/04-collect.sh`; one job at a time in `jobs.tsv` order, tier 24 first; after each class's `cal` jobs run `run/05-hold.sh <tier> <class>`, commit and push `selection.json`, then the hold jobs | executor | after 14 |

Disk note for any follow-up on this host: 3.1 GB free now; `target/` is 1.5 GB;
`inputs/` 12 MB; `native/target` 27 MB. A second full build of the main crate cost
about 8 minutes and net about 0.3 GB.
