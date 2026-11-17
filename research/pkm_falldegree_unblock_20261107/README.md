# PKM prime fall-degree study: unblock check (2026-11-07)

Slug `pkm_falldegree_unblock`. Preparation only: nothing was launched on EKS, no image was built or pushed,
the preregistration was not edited. Every number quoted from the repositories or produced here is
`[pcc:unverified]`. Evidence classes: **measured** (read from a recorded file or command output),
**derived** (arithmetic on recorded values), **cited** (a repository document says so), **plan**.

## Goal

Get the preregistered prime fall-degree study (`research/prime_fall_degree_20261027`, Amendments 1-8) to a
launchable state, or document exactly what still blocks it. Lead: `LEAD-ICP-pkm-tower-algebraic-oracle`
(`open_blocked`). Blockers recorded earlier: (a) route, (b) Mac-only raw runs, (c) 130-380 GB hosts, (d) 3,600 s cap.

## Sources checked

- research-kb: **not available** (`scripts/research-kb` does not exist in this checkout). Instead:
  `python3 -I tools/lead_atlas.py --match ...` and `build_frontier_map.py --match ...` in
  `crypto-autoresearcher` **both fail** with `ModuleNotFoundError: orchestration` (sparse checkout lacks the
  package), so no match hits are recorded. The lead file
  `knowledge/leads/ecdlp/ic-prime/LEAD-ICP-pkm-tower-algebraic-oracle.yaml` was read directly.
- Conductor not installed (not installed here, per rules).
- Route source (read in full): `taskq/deploy/eks-karpenter/remote/{launch-remote-command,run-command,delete-command,status-command,current-run-request-buildspec.yml,test-launch-remote-command.sh}`,
  `ACCESS.md` ("Command Jobs through the bridge"), `submit-job`, `nodepool.yaml`, `cost-controls.json`,
  `resume/CONTROLLER.md` ("Command mode"), `oci-layer-publish.py`; skill `.claude/skills/isolab-eks/{SKILL.md,references/submission.md}`.
- Study: `PREREGISTRATION.md` (Amendments 1-8), `DERIVED.md` (not needed), `RESTART.md`, `run.sh`, `isolab/{LAUNCH.md,Dockerfile,entry.sh}`;
  `prime_fall_degree_t14_20261101/{README,HANDOFF,ISOLAB_CARD}.md`, `runs/`, `preflight/PEAKS.txt`;
  `prime_ic_program_20261029/{DECISIONS.md,fall_degree_relaunch/*}`; `arity_completion_20261104/launch/` (the only prior user of the command route).
- Live read-only AWS: `aws sts get-caller-identity` (account 104186811093, user `coreuser`) and
  `aws ecr describe-images --repository-name isolab/tasks`.

## What was done

1. Read the command launcher and its validator (answer to question 1 below).
2. Read the preregistration clauses on re-running stage A (question 2).
3. Wrote `make_requests.sh` -> `requests/*.json` (17 `isolab.eks.command-request/v1` documents) and
   `validate_command_request.sh`, which runs the repository's own validator (the inline `jq` program of `remote/run-command`; there is no standalone validator for this kind, `test-launch-remote-command.sh` extracts it the same way). Output: `validate_output.txt`.
4. Wrote `image/entry.sh` (entry script with a one-capped-run stage) and `image/build_layer.sh` (daemon-free image recipe). Neither was run against ECR.
5. Development check of the binary (section "Development check").

## Results

### 1. Does the command route satisfy D-ROUTE for the frozen jobs? **Yes for submission; three limits remain** (measured from source)

D-ROUTE (DECISIONS.md) approved "Option 1, an `isolab.eks.command/v1` request kind plus an ECR push route". What exists on `main` is the kind
**`isolab.eks.command-request/v1`** (not `command/v1`), with `command-status/v1` and `command-delete/v1` companions, dispatched by
`remote/launch-remote-command` through the existing CodeBuild launcher. It is already in use: `research/arity_completion_20261104/launch/` submitted real Jobs with it
(requests and receipts there). Its contract, read from `remote/run-command`:

| item | command route | frozen need | verdict |
|:--|:--|:--|:--|
| accepted kinds | `command-request/v1`, `command-status/v1`, `command-delete/v1` | argv + image | ok |
| image | digest-pinned `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:<64 hex>` only; the validator checks the pattern, not that it was built from a given commit | needs `pkm_tower_pilot` etc. | **an image must exist; none does** (below) |
| source pin | `provenance.source_revision` 40 hex, required; unchecked against the image | HEAD | ok, but binding image to commit is on us |
| cpu | even, 2..28 | 14 | ok on paper; **real ceiling 16 vCPU / 128 GiB** (nodepool `cpu-spot` limits and `isolab` ResourceQuota, `cost-controls.json`; owner cost ceiling 2026-11-01). LAUNCH.md's "CPU cap is 28" and the proposed jobs' "two 14-vCPU pods fit" are stale: **one 14-vCPU job at a time** (derived) |
| memory | `^[1-9][0-9]*(Mi|Gi)$`, no upper bound in the validator | 56Gi, 112Gi | ok up to the 128 GiB pool; a 112Gi pod needs a 128 GiB node (r-family, 16 vCPU), which consumes the whole pool limit. Pods get `ephemeral-storage` 10Gi, `/work` emptyDir 8Gi (`submit-job`) |
| instance type | optional, regex only | unpinned | ok; entry.sh records the host model |
| max duration | `seconds` 1..604800 (becomes `activeDeadlineSeconds`); `wait_seconds` <= 3000 bounds only the launcher | 86,400 s per system | **lifts the 3,600 s cap** (blocker d resolved). The 3,600 s limit remains only on `isolab.eks.request/v1` |
| retries | must be 0 | 0 | ok |
| isolation | `standard` or `l2` | standard | ok; **no L2/replay/A/A admission for command Jobs** (ACCESS.md). This study reports degrees and multiply-add counts, not timings (relaunch README section 1), so none is claimed |
| S3 result path | Job's own `ISOLAB_RESULTS_URI=s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod uid>/`; the command uploads its own files; launcher receipts under `runs/<job>/launcher/<id>/` | rows + hashes | ok; `image/entry.sh` syncs every 300 s and at exit |
| checkpoint/resume | none. Spot reclaim kills the Job (`retries` 0, no restart). CRIU command mode: "implemented and unit-tested only ... no live CRIU qualification" (`resume/CONTROLLER.md`) | 24 h non-resumable cells | **open, see blocker (e)** |

New blockers/corrections found in source (not in the earlier list):

- **(e) Spot reclaim vs 86,400 s non-resumable cells.** The arity run on this route lost 4 of its pods to Spot reclaim within 4-21 minutes at 8 vCPU
  (`arity_completion_20261104/launch/receipts/attempts/SPOT-LOSSES.md`, cited). The pool is Spot-only. A 24 h, 112 GiB job with no checkpoint will probably be reclaimed (heuristic, from that sample).
  `pkm_tower_pilot` has no internal checkpoint. The owner chose non-resumable on 2026-11-01 (ISOLAB_CARD.md); expect censored or lost rows.
- **(f) cpu 16 in `ISOLAB_CARD.md` is unschedulable** on a 16-vCPU node: the node config reserves the housekeeping core(s) (`nodepool.yaml`, `reservedSystemCPUs`). Requests here use 14 (derived; arity ran 4 CPU on an 8-vCPU node, so 14 on 16 is by analogy, not measured).
- **(g) `run.sh` does not implement Amendment 7.** `d-A` and `cap-A` use `--budget 1800 --max-nnz 2e9` and `D-1` caps only; the per-system `--max-dense`/86,400 s bracket exists only in the t = 14 directory's `run_cells.sh` and in the unrun `proposed/entry.sh`. `image/entry.sh` adds a `cap:T:K:C:DENSE` stage. The scorer patch of Amendment 7 D (read `CAP-*`/`X-*` rows, C-step line) is **not done** (relaunch README section 5, item 7); it is local work, not an image dependency.
- **(h) `LAUNCH.md` is stale** versus Amendments 7/8 and the corrected image/entry (relaunch README section 5 items 1-6, 13-15): do not use its `entry.sh`/Dockerfile.

### Image: does one exist for HEAD?

**No** (measured, `aws ecr describe-images`, 2026-11-07). `isolab/tasks` holds `gh-*` ecbench images, `n83-*`, `arity-20261104-*` (built from frozen commit `4fa25efc0e`, which is not in this checkout, so whether it has the `hky` engine and `--max-dense` is unverified), `recovery-*`, `rule-v9-*`. None is tagged `pfd`/`pkm`/`fall`/HEAD. The commit-image workflow `isolab-experiment.yml` builds **ecbench only** (`build-bundle`; no Dockerfile input), so it cannot produce this image.
Route: `image/build_layer.sh COMMIT` (static musl build, one layer on the aws-cli base by digest via `oci-layer-publish.py`, no daemon). That needs ECR push rights for `coreuser`; the arity session pushed with this identity on 2026-11-04 (cited), but push permission was **not tested here**. Build from a **committed** revision: the working tree is shared and dirty, and HEAD moves (it was `8bb2629d...` at the start and `716f6dec...` later in this session).

### 2. Cells launchable without the Mac files

Reading of the preregistration (quoted):

- Amendment 2: "A restarts from `t = 1`. Its `t <= 10` rows are recomputed, and the repeats must agree. Disagreement is a reproducibility failure and stops the study."
- Amendment 5: "Stage A is rerun from `t = 1` (repeats must agree) so that every A row carries [`muladds`]."
- Amendment 7 header: "before any Kummer `m = 2` row at `t >= 12` on the pushed branch; the `t <= 10` rows are being recomputed". Amendment 7 D.4: "The Amendment 2 repeat check, extended to rows imported from the first run **if they are recovered**." Amendment 7 E: recovered rows are optional, listed separately, and "never enter an Amendment-7 median".
- `RESTART.md` step 3: "Stage A restarts from t = 1, and its t <= 10 rows must agree with the medians above."

**Conclusion (reading, not reinterpretation):** the protocol *requires* a fresh re-run of stage A from `t = 1`; it does not require the Mac rows. A fresh re-run on EKS is therefore the registered procedure, and the Mac files are an optional upgrade of the repeat check from median level to row level.

**Ambiguity, owner decision needed:** Amendment 2 says "the repeats must agree" with "its `t <= 10` rows"; the first copies are lost, so only a median-level comparison against 3,4,4,4,4,5,5,5,5,6 is possible. That reading is written in `fall_degree_relaunch/README.md` section 2.5 and DECISIONS.md D-MAC ("the `t <= 10` reproduction check stays at median level"), **not in the preregistration**. D-MAC also says the owner "chose to recover them": that is the owner's step, unfinished. Amendment 7 E and the t = 11 disclosure (round-2 rows already gave `D = 6`) stand: only `t = 12, 13, 14` are blind tests of C-step.
Also: gate records (G3's 70 runs, G4) are lost; the relaunch reruns G1/G3/G4 (README section 2.2), which the entry script does.

Launchable now (given an image), by cell group:

| group | request(s) | resources | status |
|:--|:--|:--|:--|
| stage A + F4 + cap-A (t = 1..14 under `run.sh` limits), gates G1, G3 | `pfd-a` | 14 cpu / 56Gi / 172800 s (proposal) | launchable. t = 12..14 F4 inside `d-A` will be budget-censored at 1800 s; real t >= 12 data come from the `cap` jobs |
| stage C (gates G1, G4, G3) | `pfd-c` | 14 / 56Gi / 86400 s | launchable |
| stage B (+ `d-B`, G3) | `pfd-b` | 14 / 56Gi / 172800 s | launchable; m = 3 N = 21 and m = 4 N = 20 hit the program's own `max_entries` and read censored (Amendment 2 limits), they are not run at the 130-380 GB the earlier note says (blocker c remains for resolving them) |
| Amendment 7 B, t = 12, cap 5 and cap 6, both draws | `pfd-x-t12-k{0,1}-c{5,6}` | 14 / 56Gi / 90000 s; `--max-dense 12800000000` | launchable; cap 6 only if cap 5 does not refute (the cap-6 job is the conditional step). Preflight at 4 vCPU on a dev host (not a result): cap 5 20 s, cap 6 stopped at 7,203 s with 8.2 GiB |
| t = 13, cap 5 and cap 6 | `pfd-x-t13-k{0,1}-c{5,6}` | 14 / 112Gi / 90000 s; `--max-dense 27900000000` | launchable; sequential, one 112Gi job at a time |
| t = 14, cap 6 (cap 5 already run off-route, no refute) | `pfd-x-t14-k{0,1}-c6` | 14 / 112Gi / 90000 s | **launchable at request level; completion unlikely** (blocker e; expected censoring per ISOLAB_CARD.md) |
| Amendment 8 m = 4: t = 4 to c = 7, t = 5 to c = 6 | `pfd-m4-t{4,5}-k{0,1}-c{7,6}` | 14 / 112Gi / 176400 s; `max_entries 13958643712`, `max_cols 4e6` | launchable after G3 (projected 8-33 GB and 8-67 GB, derived, from Amendment 8 draft) |
| m = 4 t = 5 c = 7; m = 4 t = 6 | none | projected 80-510 GB | **not launchable**: exceeds the 128 GiB pool; Amendment 8 says not to force it. Needs a 256-512 GB host or a disk-backed echelon (new capacity decision) |
| shape-matched m = 4 null at t = 4, 5 (Amendment 8) | none | -- | no request generated: needs `--null-chain` flags that I did not validate |
| scorer patch (Amendment 7 D), decomposability scoring | local | -- | not done; `cap:` stage also runs `pfd_grid_zeros` so the C certificate is produced per row |

Ordering (from the amendments, plan): `pfd-a` (contains G1, G3) before any `pfd-x-*` or `pfd-m4-*` ("capped F4 runs only after G3"); `pfd-c`, `pfd-b` any time after; the pool admits one 14-vCPU job at a time (derived), so the schedule is serial.

### 3. Request documents

`requests/*.json` (17 files), exact kind `isolab.eks.command-request/v1`. `source_revision` is HEAD at generation time; the **image digest is a placeholder (64 zeros)** because no image exists. Validation (full file `validate_output.txt`): all 17 `VALID` under `remote/run-command`'s validator; two deliberately broken requests (`seconds` 700000; `cpu` 16 with a non-ECR image) are `INVALID`.
Replace the placeholder with `./make_requests.sh image.json` after `image/build_layer.sh <committed sha>`; that rewrites every request with the image's own digest and source commit and reruns validation.

### Development check (not a result)

See the section at the end of this file.

## Decisions made (and why)

- Treat `isolab.eks.command-request/v1` as the D-ROUTE route: it is deployed, used by the arity run, and tested offline (`test-launch-remote-command.sh`).
- One capped F4 run per job (`cap:` stage), not the draft's multi-step `x:` bracket: Amendment 7 B steps depend on each result, and a three-step job would need 3 x 86,400 s inside one non-resumable Job.
- cpu 14 not 16 or 28 (pool limit and reserved cores). Memory 56Gi/112Gi from Amendment 7 A; `--max-dense` recomputed (largest multiple of 1e8 with `4*C <= request - 8 GiB`: 12.8e9 at 56Gi, 27.9e9 at 112Gi).
- `seconds` values are proposals, not derived: the registered limits only bound them from above (worst case of chain A is ~3.8e5 s).
- Did not touch PREREGISTRATION.md, `run.sh`, LAUNCH.md or any shared file.

## Open threads

- Build and push the image from a committed revision (needs ECR push, not tested) and rewrite requests with its digest.
- Run `scripts/checkpoint.sh init`, fill `.resume/RESUME.md`, create `bench.record`/budget ceiling, Conductor task, director card, before launch (not done here).
- Scorer patch of Amendment 7 D; `hky_fp_tower` tests (G1 is an example, not `cargo test`).
- Spot-reclaim policy for the 24 h cells (blocker e).
- Verify that `4fa25efc0e`-built arity image is irrelevant (it likely lacks the study flags; unverified).

## Errors

- `lead_atlas.py --match` and `build_frontier_map.py --match` failed (`ModuleNotFoundError: orchestration`); no hits recorded.
- A first `grep -rIl` over the repo exceeded 120 s and was moved to the background; no effect.

## Ready-to-run

Nothing to run until the image exists. Sequence for the main session: (1) commit; (2) `image/build_layer.sh <sha>` (ECR push); (3) `./make_requests.sh image.json && ./validate_command_request.sh requests/*.json`; (4) checkpoint, record, budget; (5) per request:
`ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher AWS_REGION=us-east-2 ISOLAB_CLIENT_OUTPUT=<fresh dir> taskq/deploy/eks-karpenter/remote/launch-remote-command requests/pfd-a.json`, then status via `isolab.eks.command-status/v1`.

## Owner decisions needed

1. Confirm that a fresh stage A re-run with a median-level `t <= 10` check (3,4,4,4,4,5,5,5,5,6) satisfies Amendment 2 ("repeats must agree"), or recover the Mac files first (`MAC_RECOVERY.md`; D-MAC says recover).
2. Authorize ECR push by the launching identity (or a build route) for one image of the study binaries; the commit-image workflow cannot build it.
3. Spot reclaim: accept that 24 h non-resumable cells may be lost and report them as lost, or qualify CRIU command mode for these jobs, or allow capacity that is not Spot.
4. `seconds` and dollar ceiling per job (proposals above; Amendment 7 fixes 86,400 s budgets, not Job deadlines).
5. Whether m = 4 `t = 5, c = 7` and `t = 6` (80-510 GB) may use a larger host; otherwise they stay unlaunched.
6. Stage B `m = 6, t = 6` (lambda = 1.52) rule, and Amendment 8 null cells (open from the relaunch README).

## Shared files edited

None.

## Development check (labelled: not a result, not stage A)

Built with `CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --example pkm_tower_pilot --example prime_fall_degree_tools` (24 min cold; later `--example pfd_grid_zeros --example hky_fp_tower_check`), rustc 1.97.0, glibc, **dev host binaries (not the musl lab build)**; SHA-256s in `devcheck/binaries.sha256`. Disk was 9-11 GB free throughout. Working tree was shared and uncommitted; source state = HEAD plus other agents' edits. Run with `RAYON_NUM_THREADS=2`, seconds each, outputs in `devcheck/`:

- `hky_fp_tower_check`: prints `G1: 6 cases pass` (measured, dev).
- Stage-A-style closure, Kummer m = 2, P1, `t = 1..2`, 2 draws each (`dev-A-m2-t1-2.jsonl`): rows carry the fields the scorer and Amendment 5 need: `first_fall_degree`, `last_fall_degree`, `falls`, `censored_at`, `muladds`, `timed_out`, `x_r`, `tower`, `curve`. Values: `last_fall_degree` 3, 3 (t = 1) and 4, 4 (t = 2); `muladds` 76 and 3,485. These are the first two registered medians (3, 4), but 2 draws at 2 tiny cells on a dev build prove nothing about reproduction; stage A proper is still to be run on EKS.
- F4 engine (`d-A` options) same cells: `solving_degree_max` 3 and 4, `inconsistent: true`, `muladds` 17 and 1,415.
- The `cap:` stage's exact flags (`--engine tower --budget 86400 --max-dense 12800000000 --trace --cap 4 --targets 0`) at `t = 6` start and finish (`degree_reached` 4, `inconsistent: false`, `muladds` 18,609,796); `pfd_grid_zeros` on that row gives `grid_zeros 0`, `decomposable false`, `x_r 464833983`. That `x_r` equals the round-2 committed value for `t = 6`, target 0 (`fall_degree_relaunch/README.md` section 2.5 (3), cited), so the pilot's rng consumption is unchanged since round 2 for that draw.
- The `m4:` stage's flags (`--engine hky --m 4 --hky-cmax 5 --hky-max-entries 13958643712 --hky-max-cols 4000000 --t-min 2 --max-t-m4 2 --max-t 2 --targets 0`) start and finish at `t = 2` (`N = 8`, `first_fall_degree` 4). `--t-min 4` was **not** exercised.
- Not tested: `image/entry.sh` end to end (needs `ISOLAB_RESULTS_URI`, `aws`, the image paths), `image/build_layer.sh`, the musl build at this HEAD, any Docker/ECR step.

Request source_revision (HEAD of /home/user/crypto when last generated, 2026-11-07): 0bd1d78cbf722fe996f2814304ef2f3fa8f52013. HEAD moves as the main session commits; regenerate with make_requests.sh.
