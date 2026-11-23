# Validation path for the E-222 / M-221 results (investigation only)

**Status:** unvalidated, read-only investigation. Nothing was submitted to AWS or EKS, no git command was run, no job was started. Facts below come from the files named and from the commands shown; plans are marked as plans.

## A. AWS CLI

- `python3 -m venv` plus `pip install awscli` in the scratchpad works through the proxy: `aws-cli/1.46.3`.
- `aws sts get-caller-identity` with the credentials already in the environment (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`; no session token variable is set) returns `InvalidClientTokenId: The security token included in the request is invalid`. No account id or ARN could be read, so whether the caller is account `104186811093` is unknown. No SSO login was started and no other key was requested.
- Command to re-run once a working identity exists: `<scratchpad>/awsvenv/bin/aws sts get-caller-identity`. The venv is at `/tmp/claude-0/-home-user/ff093acc-b11c-5be3-a67a-345934fd95a9/scratchpad/awsvenv`.

## B. What an isolab-eks submission of the `ic solve-prime-file` panel needs

Sources read: `taskq/deploy/eks-karpenter/{COST_CONTROLS,ACCESS,HARNESS}.md`, `.claude/skills/isolab-eks/SKILL.md`, `references/submission.md`, `references/recovery.md` (headings only), `remote/launch-remote-command`, `remote/run-command`, `contracts/*`, `tools/task_card.py`.

| item | finding |
|---|---|
| Which request type fits | `isolab.eks.request/v1` (`remote/validate-request`, `launch-remote`) allows only `job_name, spec, instance_type, memory, seconds (<=3600), build{image_uri, source_revision, ecbench_sha256}`: it is the `ecbench` contract. A native `ic` panel is not an `ecbench` spec. The fitting path is the custom-command contract `isolab.eks.command-request/v1` (`remote/launch-remote-command`, or MCP `isolab_k8s_run`): `job_name, image (digest-pinned in isolab/tasks), command argv, cpu (2..28 even), memory, seconds, isolation_profile (standard or l2), instance_type, retries (must be 0), wait_seconds (<=3000)`, plus `provenance`. |
| Image and source identity | **Blocker.** `publish-image` bundles only `ecbench` and `isolab-fresh-owner` (`build-bundle` builds just those two bins), so no image in `isolab/tasks` holds `target/release/ic` or this repository. The command needs a digest-pinned image containing the `ic` binary and the checked-out `research/ic_e222_m221_20261106/` files; source SHA, binary SHA-256 and OCI digest must be kept distinct (SKILL.md). No such image exists as far as the files show. |
| Existing frozen `ic` path | `run-ic-sweep` and `specs/ic-size-sweep` (n20/24/28/32) run a different workload (one-target IC against rho on four Spot Jobs, via the GitHub workflow `isolab-experiment.yml size_sweep=true` on `main`). It does not run `solve-prime-file` and does not carry `--project-cofactor`. |
| L2 or plain | The ecbench path requires L2 (>=5 measured rounds, warmup, reference, A/A). Our metric is `trials_total`, an operation count independent of wall time, so isolation does not change it; a plain job (`isolation_profile: standard`, `cpu-spot`) is enough for counts. A wall-time claim would need L2 (`cpu-l2-spot`, 8 vCPU / 32 GiB queue, one worker) and an A/A control, which `solve-prime-file` does not provide. |
| Cost caps | Shared allowance USD 200/day for IsoLab (COST_CONTROLS.md, 2026-11-03), monthly USD 1,000; `cpu-spot` 16 vCPU / 128 GiB, L2 8 vCPU / 32 GiB; `isolab` quota 16 CPU, 8 active pods. The workload (432 runs, ms to seconds, in tests ~16 minutes total on this host) fits one 4-vCPU, 8 GiB job of 1800 s. A finite dollar ceiling, concurrency, original deadline and retry allowance must be written before submission; the remaining budget must be read first (not readable here: no working AWS identity). |
| Git custody | Before any build or trial: commit and push code, inputs (hashed manifest), configuration, seeds and the launch command to a GitHub branch/PR, then `python3 taskq/deploy/eks-karpenter/check-experiment-git --repository OWNER/REPOSITORY --path ... > publication-receipt.json`. The panel inputs are in `main` now; the request file is not (draft only). |
| Task-card relay | Required by CLAUDE.md for an authorized run: the experiment-director writes a `frozen_command` card (`tools/task_card.py`), `lint` accepts it, `executor-mechanical` runs `task_card.py run`. Draft: `draft/card.yaml` (lints clean: `CARD OK ... sha256 b3ac364d...`; not run). It binds `draft/request.json` and `launch-remote-command` by sha256, sets the AWS deployment variables in `env` and no secrets, and lists stop conditions. |
| Resume | `solve-prime-file` has no checkpointing; command jobs are non-resumable (HARNESS.md). The panel is short, so a Spot loss means rerun under the same ledger; resume contract not needed. |

**Draft files (not for execution):** `draft/request.json` (digest and source revision are zero placeholders, deliberately invalid), `draft/card.yaml`. The client-side `jq` shape check of the command request passes; the cluster-side validator was not run.

## C. What `VALIDATION.json` must name, per result in `main`

Applies only when a row is added to `docs/ic/LEADERBOARD.md` / `leaderboard.json` / `ic-leaderboard.html` or `benchmarks/README.md` (docs/VALIDATION_GATE.md). So far none was added; CLAUDE.md asks for a `benchmarks/<date>-<slug>/` row and a leaderboard section for a measured gain, and the H1 trial reduction counts as one [pcc:unverified], so the gate applies once that row is written. Schema: `validation-gate/v1` with `replay_certificate{path,sha256}`, `validator_report{path,sha256}`, `rho_reference{kind, protocol, unit, binary_or_process, matches_protocol, strongest_available, same_unit, same_binary_or_process, gae_band{0,0.1,1}{ic,rho}, order_holds_across_band}`. Toy example: `scripts/fixtures/validation_gate/positive/`. Explain command: `python3 scripts/validation_gate.py evidence DIR`.

| result dir | replay certificate | validator report | matched rho with band |
|---|---|---|---|
| `results/baseline1`, `results/h1` | Rerunning `ic solve-prime-file --seed` reproduces `trials_total` exactly (flag-off arm equals baseline 1), so a local replay is possible now. The gate needs `independent: true`; under `docs/SABOTAGE_RESISTANCE.md` that means a keyring signature (`tools/pcc/pcc.py claim/verdict/sign`), which a producer session does not hold. Not producible by this session. | Raw rows are not in the repository: `rows.jsonl` for baseline1 and h1/on moved to the artifact store (`.artifacts.json`, sha256 `afa76f5f...` for baseline1). The report must cite repository files, so rows must be restored or cited as repository files; the aggregates in `summary.json` alone are not raw data. A separate session already wrote `results/confirm/VALIDATOR_REPORT.md` for a different panel (not checked here). | `results/rho1` is a negation-map walk (A = 2) at 10 to 16 bits on different curves than the IC instances; same binary yes (`ic`), same unit not shown (IC counted in trials, rho in GAE). The 0 / 0.1 / 1 GAE band was not computed for either arm; it needs the unpriced native counters of both arms listed (AGENTS.md section 5a). `ic rho` JSON has per-walk `counters_summed` (empty for plain walks), IC `solve-prime-file` emits none. |
| `results/rho1` | Native deterministic: `ic rho --seed` repeats; independent replay as above. | Recompute from `rho_matched.json` (in repo). | It is the reference. |
| `h2_edwards`, `h3_montgomery` | Python counts; `[r]G = O` and equality of scalar multiples are the checks. Independent replay = another implementation (for example `src/` Rust field code); none written. | Recompute of op counts from `results.json` is possible locally. | Not a rho comparison; the gate's rho reference does not apply unless a row ranks arms. |

Tools present: `scripts/validation_gate.py` (check, freeze, evidence), `tools/pcc/pcc.py` (claim, verify, verdict, ledger, canary; the ledger check runs in CI), `scripts/ic_audit/` (recompute and replay scripts for existing board tables, built around `ecbench`), `tools/benchrec.py` (`bench.record/v1`). Producible locally now: a replay run (non-independent), a recompute script over restored raw rows, the band inputs once counters are emitted. Needs isolation or another party: signed independent verdicts, an L2 wall-time replay.

## D. Knowledge base and Conductor

| requirement | state here | evidence |
|---|---|---|
| `research-kb` (CLAUDE.md "Knowledge base first") | not runnable | `scripts/research-kb status` exits 127: `no executable found; set RESEARCH_KB_BIN or put research-kb on PATH`; `RESEARCH_KB_BIN` unset; the wrapper looks in `../crypto-autoresearcher/research-kb/.venv/bin/research-kb` and `../crypto-autoresearcher` is not checked out (only `crypto` and `cryptanalysis` exist next to the repository). |
| lead atlas (`crypto-autoresearcher/knowledge/leads/ecdlp/`) | not readable | same missing checkout. |
| `ecdb` (curve database) | not available | `ECDB_TOKEN` not set; `scripts/ecdb.sh` exists; secret `ecdb/api` not reachable from here. |
| Conductor (`conductor check`) | not runnable | `which conductor cairn` finds nothing; `.conductor/` absent; `.mcp.json` lists the server but `conductor-mcp` is not on PATH; `docs/CONDUCTOR.md` says hooks warn and let edits through. |

## Blockers and next command

| # | blocker | next command / action |
|---|---|---|
| 1 | AWS identity rejected (`InvalidClientTokenId`), no session token in env | Provide working credentials for account 104186811093 in the executor environment (not initiated here), then `<venv>/bin/aws sts get-caller-identity` and `taskq/deploy/eks-karpenter/remote/check-ready` |
| 2 | no digest-pinned `isolab/tasks` image containing `ic` and the repository | extend `taskq/deploy/eks-karpenter/build-bundle` / `publish-image` (or a new reviewed workflow) to add `--bin ic`; build via `gh workflow run isolab-experiment.yml` on `main` (needs the workflow on `main` and OIDC), then put the digest in `draft/request.json` |
| 3 | request not published to GitHub, no `check-experiment-git` receipt | commit `draft/request.json` and the frozen spec to the branch, then `python3 taskq/deploy/eks-karpenter/check-experiment-git --repository isogeny-labs/crypto --path research/ic_e222_m221_20261106/validation_path/draft/request.json > publication-receipt.json` |
| 4 | no finite dollar ceiling and budget readback | write the ceiling into the card `notes` / request provenance and read `DescribeBudget` for the two daily budgets (needs blocker 1) |
| 5 | raw rows for baseline1 and h1 are in the artifact store, not in the repository | fetch by sha256 from `.artifacts.json` into the evidence directory, or cite as allowed by the gate; then `python3 scripts/validation_gate.py evidence <dir>` |
| 6 | independent replay and validator signatures | `python3 tools/pcc/pcc.py claim ...` by the producer, `verdict`/`sign` by a keyring holder (see `docs/SABOTAGE_RESISTANCE.md`); a session that wrote the data cannot supply it |
| 7 | band at 0 / 0.1 / 1 GAE for IC and rho | emit the unpriced counters in `solve-prime-file` JSON (gap) and price both arms per AGENTS.md section 5a; until then label every IC/rho ratio a lower bound |
| 8 | KB, lead atlas, ecdb, Conductor | check out `crypto-autoresearcher` next to `crypto` (provides `research-kb` and the atlas), set `RESEARCH_KB_BIN`, export `ECDB_TOKEN`, install `conductor`/`conductor-mcp` per `docs/CONDUCTOR.md`; then `scripts/research-kb status` and `conductor check --summary "..." --scope path:research/ic_e222_m221_20261106` |

**Side note, outside this directive:** `research/ic_e222_m221_20261106/` now also holds `RESULTS_extra_relations.md`, `RESULTS_h1_holdout.md`, `RESULTS_sum_table.md`, a `results/confirm/VALIDATOR_REPORT.md` and scripts (`run_confirm.py`, `run_final.py`, `run_table.py`, `run_timed.py`) that another session added; they were not read or checked here.
