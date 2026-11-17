# R2_READY: PKM fall-degree study, round 2 (slug `r2pkm`, 2026-11-08)

Every number below is `[pcc:unverified]`. Evidence classes: **measured** (read from a command or file in this session), **derived**
(arithmetic), **cited** (a repository document says so), **heuristic**, **plan**. Rules followed:
`scratchpad/ROUND2_RULES.md` and `research/open_leads_execution_20261108/DECISIONS.md` (items 1-6). Nothing was launched; no benchmark
record, resume checkpoint, EKS job or CodeBuild run was created.

## Goal

Make the study launchable: build and publish one digest-pinned image from f13a690df016d0a457bd40579fbb9d054f486adf, fill and validate the
requests, draft (not apply) the amendments of decision 6, prepare the large-host change of decision 5, and fix the launch order.

## Sources checked

research-kb and Conductor are **not installed** here; no KB receipt. Read directly: this directory's README, `image/*`, `make_requests.sh`,
`validate_command_request.sh`, `requests/*.json`; `research/prime_fall_degree_20261027/{PREREGISTRATION.md (Amendments 1-8), run.sh}`;
`research/prime_ic_program_20261029/fall_degree_relaunch/{README.md, AMENDMENT_8_DRAFT_m4.md}`; `examples/pkm_tower_pilot.rs` and `Cargo.toml`
at f13a690df0 (`git show`); `taskq/deploy/eks-karpenter/{nodepool.yaml, cost-controls.json, workloads.yaml, apply-cost-controls.py, deploy.sh,
submit-job, COST_CONTROLS.md, ACCESS.md, test_cost_controls.py}`. Read-only AWS: `ecr describe-images`, `ec2 describe-spot-price-history`.

## What was done

1. **Build** from `git archive f13a690df0` of only the paths the build needs (Cargo.toml/lock, src, benches, examples, crates, isogeny_algos, tools,
   the manifest-declared research/experiments paths, `docs/`, `tests/fixtures`, `research/aps_measurement_20261029`, `run.sh`; the first attempt failed
   on `include_str!` of `docs/...`, so `docs/` etc. were added). Target dir inside the scratch tree, deleted afterwards. Disk free 7.4 GB before,
   never below 5.4 GB (measured with `df -h /`), so the 2 GB stop never triggered. `musl-tools` installed with `apt-get update && apt-get install`.
2. **Published** one image (below).
3. **Requests**: `make_requests.sh image/image.json` (17 files), validated; plus 4 pending null requests and 3 negative controls.
4. **Drafts**: `AMENDMENT_R2_A_stageB_m6t6.md`, `AMENDMENT_R2_B_m4_null.md` (not applied).
5. **Capacity**: `CAPACITY_R2.md` (not applied).
6. **Schedule**: below.

## Results (evidence class)

### Image (measured)

| item | value |
|:--|:--|
| repository:tag | `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks:pkm-fd-f13a690df016` |
| **digest-pinned URI** | `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:0d6b2ecd6df3f5f092fe4b6179986b041eb7d6be3b94e52d92809bdab2ff3ae8` |
| source commit | f13a690df016d0a457bd40579fbb9d054f486adf (`git archive`, committed revision) |
| base | `public.ecr.aws/aws-cli/aws-cli@sha256:b94fc761...` (resolved digest `sha256:22e7244e...`) |
| layer | digest `sha256:a0f7e28e...`, 3,791,827 bytes, 14 files; config `sha256:f1adccc4...` |
| ECR readback | `aws ecr describe-images --image-ids imageTag=pkm-fd-f13a690df016` returns the digest above, size 143,956,032 B |
| receipt | `image/image.json` (the publisher's receipt, plus `source_revision` and `study_binary_sha256` that I added because `make_requests.sh` reads `.source_revision`; the untouched receipt is `image/image.publish-receipt.json`) |
| toolchain | rustc 1.97.0 (stable), `--locked --release --no-default-features --target x86_64-unknown-linux-musl`, `RUSTFLAGS=-C target-cpu=x86-64`, `CC=musl-gcc`; all four binaries checked static (no INTERP) |

Layer contents and SHA-256 (`image/binaries.sha256`, `image/build-info.json`):
`pkm_tower_pilot f78f3959...`, `prime_fall_degree_tools 7e8a6703...`, `hky_fp_tower_check c36d324e...`, `pfd_grid_zeros 96283361...`,
`entry.sh 4866aa7e...` (unchanged from the commit), `entry_r2.sh 25b01841...` (new), `/opt/study/run.sh c35ea88e...`. Full 64-hex values are in the files.
The layout is `build_layer.sh`'s: `/usr/local/bin/{binaries,entry.sh}`, `/opt/study/{run.sh,binaries.sha256,build-info.json}`, user 1000:1000, workdir `/work`.

### Requests (measured: validator output in `r2_validate_output.txt`)

Validator: the inline jq of `taskq/deploy/eks-karpenter/remote/run-command` via `validate_command_request.sh` (no standalone validator exists).
All 17 `requests/*.json` **VALID**, all 4 `requests_pending_amendment/*.json` **VALID**, all 3 `requests_negative/*.json` **INVALID** as intended
(`seconds` 700000; `cpu` 16 with a non-ECR image; `retries` 1). All reference the image digest above and source_revision f13a690df0. Largest `seconds`
is **176,400**: none exceeds it. cpu is 14 everywhere. Job names carry the stamp `20261108`.

| order | request | cpu / memory / `seconds` | command (after `/usr/local/bin/entry.sh`) |
|--:|:--|:--|:--|
| 1 | pfd-a | 14 / 56Gi / 172800 | `g1 hky-A g3 d-A cap-A` |
| 2 | pfd-c | 14 / 56Gi / 86400 | `g1 g4 hky-C g3` |
| 3 | pfd-b | 14 / 56Gi / 172800 | `g1 hky-B d-B g3` |
| 4 | pfd-x-t12-k0-c5, pfd-x-t12-k1-c5 | 14 / 56Gi / 90000 | `cap:12:K:5:12800000000` |
| 5 | pfd-x-t12-k0-c6, pfd-x-t12-k1-c6 (conditional) | 14 / 56Gi / 90000 | `cap:12:K:6:12800000000` |
| 6 | pfd-m4-t4-k0-c7, pfd-m4-t4-k1-c7 | 14 / 112Gi / 176400 | `m4:4:K:7:13958643712:4000000` |
| 7 | pfd-m4-t5-k0-c6, pfd-m4-t5-k1-c6 | 14 / 112Gi / 176400 | `m4:5:K:6:13958643712:4000000` |
| 8 | pfd-x-t13-k{0,1}-c5, then -c6 (conditional) | 14 / 112Gi / 90000 | `cap:13:K:C:27900000000` |
| 9 | pfd-x-t14-k0-c6, pfd-x-t14-k1-c6 | 14 / 112Gi / 90000 | `cap:14:K:6:27900000000` |
| (frozen later) | pfd-m4n-t{4,5}-k{0,1}-c{7,6} in `requests_pending_amendment/` | 14 / 112Gi / 176400 | `/usr/local/bin/entry_r2.sh m4n:T:K:C:13958643712:4000000` |

The 4 pending null requests use `entry_r2.sh` (in the same image) and are **REQUIRES AMENDMENT FREEZE** (decision 6); do not move them to `requests/`.

### Amendment drafts (not applied; derived and cited)

- **R2-A** (`AMENDMENT_R2_A_stageB_m6t6.md`): rule for `m >= 3` cells with `lambda >= 0.1` (Amendment 1's threshold): run as registered, written to a separate
  table, excluded from every §6 rule. Derived: the only registered `P1` cell it touches is Kummer `m = 6`, `t = 6` (`lambda = 1.517`); next largest `m = 5`, `t = 6` is 0.0711.
  Depends only on `(m, t, p)`. Freeze before `pfd-b` is submitted.
- **R2-B** (`AMENDMENT_R2_B_m4_null.md`): 4 null cells, flags `--controls null --null-chain` (exist: `pkm_tower_pilot.rs` lines 2008, 2651; `prime_fall_degree_tools`
  only forwards it, line 520, and has no flag of its own). Dev check (seconds, `t = 2`, not a result): runs, exit 0, null and Kummer rows share `x_r` 1102336260.
  `--t-min 4` was not exercised. Freeze before any `M4N` cell is submitted.

### Capacity (cited / measured / derived): `CAPACITY_R2.md`

Needs: `cost-controls.json` `cpu-spot` 16/128Gi to 32/256Gi (512Gi option: 64/512Gi plus `"64"` in `instance-cpu`), the `isolab/jobs` quota 16/128Gi to 28/240Gi
(its three copies `cost-controls.json`, `workloads.yaml`, `nodepool.yaml` header), an owner-restated aggregate ceiling, an operator `kubectl apply` of `nodepool.yaml` and
`workloads.yaml` (`apply-cost-controls.py` can only reduce). Spot, measured today: r6i.8xlarge 0.51-0.61 $/h, r6i.16xlarge 0.82-1.72 $/h. At the study's US$40 per job a
256 GiB host buys about 70 h, a 512 GiB host 25-50 h, short of the ~2 weeks the Amendment 8 draft guesses for `t = 5`, `c = 7`. No dollar figure is a forecast.

### Development checks (measured, dev only, not results)

`hky_fp_tower_check` (musl binary): `G1: 6 cases pass`. `pkm_tower_pilot` Kummer and null-chain `m = 4`, `t = 2`, `N = 8`: both exit 0 (`devcheck_r2/`). Not run: `entry.sh`/`entry_r2.sh`
inside the image (no container runtime and no `ISOLAB_RESULTS_URI` here), any of the 21 requests, any EKS job.

## Decisions made (and why)

- Build with the installed rustc 1.97.0, not `cargo +1.99.1` (deviation, below): the repo pins 1.99.1 only in `backend-leaderboard.yml` for another tool; nothing in `Cargo.toml`,
  the image workflow or the examples requires it, and a second toolchain costs ~1 GB of a shared disk.
- Archive only needed paths (the earlier whole-repo `git archive` filled the disk). `build_layer.sh` is left as committed; the working script is `image/build_layer_r2.sh`.
- New file `image/entry_r2.sh` instead of editing `entry.sh`: the committed `entry.sh` (and the 17 requests) are unchanged; only the null stage is new. The main session commits `entry_r2.sh`.
- Null requests are kept in `requests_pending_amendment/`, not `requests/`, so that a glob cannot launch an unfrozen cell.
- Amendment R2-A runs and excludes the high-lambda cell rather than dropping it, so that `run.sh`, the image and `pfd-b` need no change.
- Capacity: widen `cpu-spot` rather than add a pool, because `submit-job` selects the pool name `cpu-spot` and a new pool would also need a `submit-job` change.

## Open threads

- Freeze R2-A before `pfd-b`; freeze R2-B before any `M4N` cell. Owner/PREREGISTRATION edit is the main session's.
- Scorer patch of Amendment 7 D and the high-lambda exclusion (not done; needed before scoring, not before running).
- Before launch (main session): `scripts/checkpoint.sh init`, `.resume/RESUME.md`, a `bench.record`, Conductor task, director card; commit `entry_r2.sh`, `build_layer_r2.sh`,
  `make_requests_r2.sh`, the new requests and drafts.
- `image/entry.sh` was never run end to end (needs `ISOLAB_RESULTS_URI`, `aws`); the first job (`pfd-a`) is its first real run. Watch its `progress.txt` in S3 early.
- Large host (`CAPACITY_R2.md`) needs the owner's ceiling and a job dollar ceiling; `m = 4`, `t = 5`, `c = 7` and `t = 6` stay unlaunched until then.
- The `seconds` values remain proposals (the registered limits only bound them above). Reclaimed non-resumable cells are reported lost, one relaunch under a new name (decision 3).

## Errors

- `apt-get install musl-tools` failed first ("Unable to locate package"); fixed by `apt-get update`.
- First musl build failed: `include_str!` paths into `docs/` were not in the archive (6 errors). Added `docs/`, `tests/fixtures`, `research/aps_measurement_20261029` and rebuilt (total ~11 min on 4 cores, two cargo runs serialised on the lock).
- `oci-layer-publish.py`'s receipt has no `source_revision`, which `make_requests.sh` requires; added by `jq` (original receipt kept beside it).
- A leftover background `until pgrep` wait of mine reported completion late; harmless.

## Ready-to-launch

Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:0d6b2ecd6df3f5f092fe4b6179986b041eb7d6be3b94e52d92809bdab2ff3ae8` (source f13a690df0).
Requests: `research/pkm_falldegree_unblock_20261107/requests/*.json` (17, validator output `r2_validate_output.txt`). One job at a time (pool 16 vCPU / 128 GiB; derived).
Launch command per request, by the executor under a frozen director card (not run here):

```
ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher AWS_REGION=us-east-2 \
ISOLAB_CLIENT_OUTPUT=<fresh dir> taskq/deploy/eks-karpenter/remote/launch-remote-command \
  research/pkm_falldegree_unblock_20261107/requests/pfd-a.json
```
Status via `isolab.eks.command-status/v1` (README, "Ready-to-run"). The launch commit/image identity must be re-verified against the digest above and `image/binaries.sha256` (the Job's `binary.sha256` upload).

### Serial schedule and expected wall per job

"Ceiling" = the request's `seconds` (hard stop, measured in the file). "Expected" is a **heuristic** from cited data; none is a prediction I measured on EKS. $ = ceiling h x Spot $/h
read today (r6i.4xlarge 0.30-0.32 for 112Gi jobs, m6i.4xlarge 0.22-0.25 for 56Gi jobs; a 14-vCPU pod fits one 16-vCPU node), derived; all under the US$40 per-job decision.

| # | job | ceiling | expected wall (heuristic) | ceiling $ | gate / condition |
|--:|:--|--:|:--|--:|:--|
| 1 | pfd-a | 48 h | many hky closures (1800 s budget each) plus F4 `d-A` (`--budget 1800` each, `t >= 12` censors) plus `cap-A`: tens of hours; the registered worst case is ~3.8e5 s, above the ceiling, so a stop at the ceiling is possible | ~11-15 | must pass G1 and G3 before anything below |
| 2 | pfd-c | 24 h | hours (small `t = 2, 3` cells, `m <= 10`) | ~6 | G1, G4, G3 inside |
| 3 | pfd-b | 48 h | up to the ceiling is possible (`m = 3` `N = 21`, `m = 4` `N = 20` hit `max_entries` and read censored) | ~11-15 | freeze Amendment R2-A first |
| 4 | pfd-x-t12-k0-c5, k1-c5 | 25 h each | cap 5 took 20 s at 4 vCPU in the preflight (cited); minutes | <1 each | after G3 |
| 5 | pfd-x-t12-k{0,1}-c6 | 25 h each | cap 6 stopped at 7,203 s with 8.2 GiB at 4 vCPU (cited): hours, perhaps censored | ~6 each | only if cap 5 did not refute |
| 6 | pfd-m4-t4-k{0,1}-c7 | 49 h each | `N = 16`, F4: 55.5 min / 8 GB (cited); the closure to `c = 7` is unmeasured: hours | ≤15 each | after G3 |
| 7 | pfd-m4-t5-k{0,1}-c6 | 49 h each | 8-67 GB projected (cited): hours to the ceiling | ≤15 each | after row 6 gives an entry count |
| 8 | pfd-x-t13-k{0,1}-c5, then -c6 | 25 h each | uncapped extrapolation ~17 h (heuristic, relaunch README §2.6); a capped step is lower | ≤8 each | c6 only if c5 did not refute |
| 9 | pfd-x-t14-k{0,1}-c6 | 25 h each | extrapolated ~7 days uncapped: expect the 86,400 s budget stop = censored (blocker e) | ≤8 each | last; "undecided" is the likely reading |
| later | pfd-m4n-t{4,5}-k{0,1}-c{7,6} | 49 h each | as their Kummer counterparts | ≤15 each | only after R2-B is frozen; after the counterpart of the same `(t, k)` |

Sum of all 17 ceilings: 2,037,600 s (566 h), about US$130-190 at the rates above if every job ran to its ceiling (derived, upper bound; the US$400 study total is not approached).
Rows 6 and 7 may swap with row 8 and 9 if the director prefers cheaper decisive cells first; rows 1-3 order is fixed by "capped F4 runs only after G3".

## Deviations

1. rustc 1.97.0 instead of `cargo +1.99.1` (`build_layer.sh`), recorded in `image/build-info.json` (`rustc`, `toolchain_note`).
2. `build_layer_r2.sh` (path-limited archive, extra `entry_r2.sh`, tag `pkm-fd-f13a690df016`) instead of `build_layer.sh` (whole-repo archive, tag `pkm-fd-<40 hex>`).
3. `image/image.json` carries two added fields (`source_revision`, `study_binary_sha256`); the untouched receipt is `image/image.publish-receipt.json`.
4. The image has one more file than `build_layer.sh` would make: `/usr/local/bin/entry_r2.sh` and the `run.sh`, `entry*.sh` hashes inside `binaries.sha256`.

## Shared files edited

None outside this directory. New files (all under `research/pkm_falldegree_unblock_20261107/`): `R2_READY.md`, `CAPACITY_R2.md`, `AMENDMENT_R2_A_stageB_m6t6.md`,
`AMENDMENT_R2_B_m4_null.md`, `make_requests_r2.sh`, `r2_validate_output.txt`, `image/{entry_r2.sh, build_layer_r2.sh, image.json, image.publish-receipt.json, binaries.sha256, build-info.json}`,
`requests_pending_amendment/*.json` (4), `requests_negative/*.json` (3), `devcheck_r2/*`. Rewritten by `make_requests.sh`: `requests/*.json` (17; digest and source commit replaced).
