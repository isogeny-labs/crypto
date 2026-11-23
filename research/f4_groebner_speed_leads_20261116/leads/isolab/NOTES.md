# Lead isolab: get a many-core F4 A/B ready for Iso Lab on EKS

Scope: preparation only. I dispatched nothing, submitted nothing, spent nothing and built nothing. [pcc:unverified]

## Mechanism

The existing Iso Lab routes measure only `ecbench` specs. Those specs have no per-arm env, pin `RAYON_NUM_THREADS=1` and run ECDLP workloads, so they cannot express "f4_fp_bench under `F4_FP_*_OLD=1` vs default". The one route that can run an arbitrary binary with toggles is the `isolab.eks.command-request/v1` command Job. It needs a digest-pinned image in the lab ECR that already contains the binary and a runner. The GitHub workflow already builds such an image for one study, the `norm_branch_census` mode:
1. build a static musl payload;
2. publish it as an OCI layer with `oci-layer-publish.py`;
3. submit through `launch-remote-command`;
4. verify `SHA256SUMS` from S3.

I cloned that pattern into a draft `f4_fp_ab` mode and wrote an in-image runner. The runner:
- runs `env -i` plus the arm's toggles plus `RAYON_NUM_THREADS=t`;
- interleaves A1, A2 (A/A) and B in rotated order over thread counts {1, 4, 8, 14};
- gates output identity across all invocations;
- gates counter constancy within each arm.

## Prediction (written before the smoke test)

- The runner's output-identity gate passes with the baseline binary when all arms are identical.
- A counter-changing toggle passes the output gate but fails a naive gate that requires counter equality across arms.
- A tampered fingerprint fails the gate.

## Commands run

- `git show HEAD:` and `git show origin/main:` for:
  - `.agents/skills/isolab-eks/{SKILL.md,references/submission.md,references/recovery.md}`
  - `taskq/deploy/eks-karpenter/{HARNESS.md,COST_CONTROLS.md,nodepool.yaml,cost-controls.json,build-bundle,Dockerfile.ecbench,contracts/mcp-tools.json,mcp/src/main.rs,remote/launch-remote-command,remote/run-command}`
  - `.github/workflows/isolab-experiment.yml`
  - `research/prime_leads_chase_20261113/norm-branch-census/launch/run.sh`
  - `examples/f4_fp_bench.rs`
  - `src/cryptanalysis/ecbench/{spec.rs,runner.rs}`
  - `docs/benchmark-records.md`, `tools/benchrec.py`
  - `.claude/agents/experiment-director.md`, `tools/task_card.py`
  - `.claude/hooks/checkpoint-hook.sh`
- `git fetch --depth=1 origin main` gave `origin/main` = `90ea6cb424`. Its diff against HEAD in the isolab paths is only `checkpoint-source`, its test and one MCP test.
- `python3 tools/lessons.py brief --scope isolab-eks`
- `~/.local/bin/aws sts get-caller-identity` returned `InvalidClientTokenId`.
- GitHub MCP:
  - `actions_list list_workflow_runs isolab-experiment.yml` (isogeny-labs/crypto): the last success was run 231 (2026-11-14); runs 236-239 (2026-11-16) failed.
  - `list_workflow_jobs` and `get_job_logs` on job 133440507294: build and OIDC publish succeeded (digest `sha256:056f1e5b…`); `StartBuild` then failed with `AccountLimitExceededException: Cannot have more than 0 builds in queue for the account`.
- Wrote `work/research/f4_fp_ab_isolab_20261116/{protocol.json,launch/run.sh}` and `work/.github/workflows/isolab-experiment.yml`. The diff is in `workflow-f4ab.diff`. The draft parses as YAML; actionlint is not installed, so it was not run.
- Local payload smoke test in `smoke/` with the HEAD baseline `f4_fp_bench`: paths rewritten to `smoke/`, S3 upload replaced by `cp`, threads {1, 4}, 1 warmup + 1 round, B = `F4_FP_FL=0`.

## Results

- **First gate version** (identity over output fields and counters across all arms): exit 1. It flagged 6 large cases, because `F4_FP_FL=0` changes `field_ops` (quad_n9: 399,396,830 under FL vs 331,473,535) while `basis_fp` stays identical (`5a5d7dea3744f69d`). This confirms the second prediction.
- **Revised gate** (output fields across all arms; counters constant within each arm): exit 0. Both `identity.json` and `counters.json` have every `distinct` equal to 1, and all 32 `SHA256SUMS` entries verified. This confirms the first prediction.
- **Tampered `basis_fp`**: the gate exits 1. This confirms the third prediction.
- **Per-invocation wall time**: 0.35-0.73 s, indicative only on a shared VM.

## Determinations

- **Commit-image support is on `main`.** `isogeny-labs/crypto` OIDC publish works.
- **Instance types.**
  - `cpu-spot`: c/m/r families, gen > 5, 8/16/32 vCPU, no flex types; 16 vCPU / 128 GiB pool; quota 16 CPU / 8 pods.
  - `cpu-l2-spot`: 8-vCPU types only; 8 vCPU / 32 GiB.
  - The proposed main arm is `c8a.4xlarge` at cpu 14, standard profile.
- **The L2 pool uses `isolcpus=domain`.** Per the repository's own comment this serializes unpinned multithreaded jobs, so many-core F4 timing must use the standard profile, and L2-node arms are valid only single-threaded.
- **Command Jobs get no native L2, replay or A/A admission.** That exists only inside ecbench.

## Risks and blockers

See REQUEST.md section 5. In order:
1. The CodeBuild account queue limit is 0, which blocks all launches.
2. The workflow mode must be merged on `main` by PR.
3. The integrated F4 toggles and the study directory must be pushed.
4. There is no AWS identity, so I have no pool check, no AWS bridge and no budget readback.
5. The resume checkpoint cannot be pushed.
6. The protocol must be frozen.
7. Results will be indicative timing, not gate-admissible.

Technical risks:
- The musl `--example f4_fp_bench --no-default-features` build is uncompiled.
- musl malloc vs glibc changes absolute times but not arm ratios.
- The cpu 14 fit on a 16-vCPU node is unverified live.
