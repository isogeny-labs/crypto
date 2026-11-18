# Iso Lab request draft: many-core F4 A/B/A-A for `f4_fp_bench`

Status: **blocked, not submitted** [pcc:unverified]. I dispatched no workflow, submitted no job and spent no money. Everything below is a draft for the main session.

## 1. What the existing routes can and cannot do

| Route | Can it run `f4_fp_bench` with env toggles? | Evidence |
|---|---|---|
| GitHub `isolab-experiment.yml`, default / `spec_path` mode | **No.** It builds only `ecbench` (`build-bundle`: `cargo build --bin ecbench`) and submits an `ecbench.spec/v1`. | `taskq/deploy/eks-karpenter/build-bundle`; workflow step "Prepare the versioned experiment request" |
| ecbench spec (`isolab_k8s_benchmark`, `launch-remote`) | **No.** `ArmSpec` is `{name, role, method}` with `deny_unknown_fields`, so it has no per-arm env. The child env is scrubbed and pins `RAYON_NUM_THREADS=1` (`runner.rs::child_env`). The methods are ECDLP workloads, and no method exposes `f4_fp_bench`. | `src/cryptanalysis/ecbench/spec.rs:39`, `runner.rs:559-576` |
| GitHub `norm_branch_census` mode | Shows the pattern: build an extra static musl payload, publish it as an OCI layer over the commit image (`oci-layer-publish.py`), submit `isolab.eks.command-request/v1` through `remote/launch-remote-command`, then download and verify S3 `SHA256SUMS`. It is hard-wired to the norm-branch study. | workflow lines ~180-470 |
| `isolab.eks.command-request/v1` (`remote/run-command`) | **Yes, if a digest-pinned image in `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks` already contains the binary and a runner.** The request accepts an argv with no shell, `cpu` in {2..28 even}, `isolation_profile` standard or l2, `retries` 0 and `wait_seconds` ≤ 3000. It has no env field, so toggles go in the in-image runner. | `remote/run-command` jq validator |
| MCP `isolab_k8s_run` | Same capability as above, with a digest-pinned image and argv. The MCP is not installed in this session. | `contracts/mcp-tools.json` |
| Resumable command mode | Not suitable. Timing is always L0 and the route is "not yet live-qualified". | `mcp-tools.json` resumable schema |

**Conclusion.** The A/B needs one new, reviewed workflow mode on `main`, modelled on `norm_branch_census`. A draft is in section 3. Once that mode is merged, any later revision that contains the study directory can be dispatched with no further workflow change.

## 2. Facts determined

- **Commit-image support is on `main`.** `origin/main` = `90ea6cb424` has the build, publish-image and OIDC experiment jobs. `HEAD` (`a53cc49711`) and `origin/main` differ only by `checkpoint-source`, its test and one MCP unit test.
- **The repository is `isogeny-labs/crypto`.** The skill and `submission.md` still say `aburan28/crypto`. The OIDC publish works for `isogeny-labs/crypto`: run 43903325505 on 2026-11-16 obtained credentials and pushed `sha256:056f1e5b…`.
- **Live launcher blocker, 2026-11-16 16:28Z.** All four dispatches that day (runs 236-239) failed at StartBuild with `AccountLimitExceededException: Cannot have more than 0 builds in queue for the account`. The CodeBuild account queue/concurrency is currently at 0, so no EKS submission through CodeBuild (GitHub or AWS bridge) can start until an operator restores it. The last successful dispatch was run 231 on 2026-11-14.
- **Instance types.**
  - `cpu-spot` (ordinary, standard profile): c/m/r families, generation > 5, 8/16/32 vCPU, excluding `*-flex`. The NodePool limit is 16 vCPU / 128 GiB and the namespace quota is 16 CPU / 128 GiB / 8 pods.
  - `cpu-l2-spot`: 8-vCPU types only, 8 vCPU / 32 GiB. `c8a.2xlarge` and `m8a.2xlarge` have verified no-SMT L2 evidence.
  - Each worker reserves a housekeeping CPU plus kube/system reservations of 1 CPU and 512 Mi each. A 16-vCPU node should therefore admit a 14-CPU pod. This has not been verified live; `scripts/isolab_pool_check.sh` must confirm it.
  - c8a is AMD Zen 5 and c8i is Intel; both have AVX-512F. `f4_fp.rs` selects AVX-512 or AVX2 at runtime (`is_x86_feature_detected!`), so the workflow's `-C target-cpu=x86-64` build still takes the AVX-512 path.
- **Many-core vs L2.** The L2 pool boots with `isolcpus=domain`. The repository's own NodePool comment says this "would serialize unpinned multithreaded jobs", and rayon threads are unpinned, so a multithread arm on an L2 node measures a serialized pool. Consequences:
  - A many-core A/B must use `cpu-spot` with `isolation_profile: standard`, where load balancing is kept inside the exclusive cpuset.
  - An L2-node arm is meaningful only with `RAYON_NUM_THREADS=1`.
  - Command Jobs get **no native L2 grade, replay or A/A admission**. Those exist only inside ecbench (`run-ecbench`: L2 required, ≥5 rounds, ≥1 warmup, a reference and a matching A/A control). The A/A arm in the payload is our own noise floor, not lab admission. Timing from this route is indicative only, and the request note says so.
- **Allocator confound.** The workflow builds static musl binaries, and musl malloc differs from the local glibc build. Arms share one binary, so ratios are fair, but absolute ms are not comparable to local numbers or to an ecbench row.

## 3. Draft artifacts (lead directory, not in the repository)

| File | SHA-256 |
|---|---|
| `work/research/f4_fp_ab_isolab_20261116/protocol.json`: arms A1, A2 (A/A, defaults) and B (`F4_FP_PAR_OLD=1`); threads {1, 4, 8, 14}; 1 warmup + 15 measured rounds; rotated arm order; output-identity and counter gates | `ed87462541d9138ed6e62ac3c9b2cb74fb2f8c0d9d0d5e6af99fb56cbbb3e85d` |
| `work/research/f4_fp_ab_isolab_20261116/launch/run.sh`: in-image runner. Each run uses `env -i` plus the arm's toggles and `RAYON_NUM_THREADS`. It records host facts and PSI, gates output identity and counter constancy within each arm, writes `SHA256SUMS` and uploads to `ISOLAB_RESULTS_URI`. | `6c4f688af83ec036c5348d8c6877f9c14085c5b37ac6bd7a8fc01ea3110dcfa9` |
| `work/.github/workflows/isolab-experiment.yml`: `origin/main` plus the new `f4_fp_ab` and `f4ab_cpu` inputs, a musl build of `--example f4_fp_bench`, an OCI layer publish, the command request, launch, S3 verification and receipt upload. The other modes are made mutually exclusive with it. | `361a68c70ca6ed11a6c70db1691e34d8afda061fadce0bb9c6bbe3e0412d92f4` |
| `workflow-f4ab.diff`: unified diff of the above against `origin/main` (201 lines) | `cdff612ec8dfb30f769bbd20a1cf202e36eec26a11b2b102e4beeea559a584f2` |

**Local payload smoke test** (not a measurement; HEAD baseline binary; the S3 upload was replaced by `cp`; threads {1, 4}; 1 warmup + 1 round; arm B = `F4_FP_FL=0` as a known counter-changing toggle):
- Exit 0, and `identity.json` has `distinct=1` for all 13 cases.
- `counters.json` has `distinct=1` per (case, arm). It shows quad_n9 `field_ops` of 399,396,830 for A1 and A2 against 331,473,535 for B, which confirms the gate must not require counter equality across arms.
- `SHA256SUMS` verified.
- Negative control: a tampered `basis_fp` makes the identity gate exit 1.

The first version of the gate required counter equality across arms. The smoke test failed it on exactly this FL case, and I fixed it.

## 4. Draft dispatch, valid only after the blockers clear

```bash
# source_revision: 40-hex commit on GitHub that contains (a) the integrated F4 patches with
# their *_OLD toggles, (b) research/f4_fp_ab_isolab_20261116/{protocol.json,launch/run.sh},
# with protocol.json converted to a frozen experiment.protocol/v1 and the B arm's env set to
# the toggle under test.
gh workflow run isolab-experiment.yml --repo isogeny-labs/crypto --ref main \
  -f source_revision=<SHA40> \
  -f f4_fp_ab=true -f f4ab_cpu=14 \
  -f instance_type=c8a.4xlarge \
  -f seconds=1800
```

The workflow generates this request (`isolab.eks.command-request/v1`):

```json
{"schema":"isolab.eks.command-request/v1",
 "job":{"job_name":"gh-<RUN_ID>-<ATTEMPT>-f4ab",
        "image":"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:<f4ab layer digest>",
        "command":["/opt/isolab/f4ab/run","<SHA40>","<protocol sha256>"],
        "cpu":14,"memory":"8Gi","seconds":1800,"isolation_profile":"standard",
        "instance_type":"c8a.4xlarge","retries":0,"wait_seconds":1800},
 "provenance":{"harness":"github-actions","source_revision":"<SHA40>",
        "study":"f4_fp_ab_isolab_20261116","protocol_sha256":"<sha256>",
        "note":"Interleaved F4 A/B/A-A; outputs gated; timing indicative, no native L2 grade."}}
```

**Size.** 3 arms × 4 thread counts × 16 rounds = 192 invocations of `f4_fp_bench 1 large`. In the local smoke test one invocation took 0.35-0.73 s, which is indicative only because the VM was shared. That puts the run at about 1-3 minutes plus provisioning. The 1800 s deadline leaves margin.

**Optional second Job** (a single-thread confirmation on an isolated node): `instance_type=c8a.2xlarge`, `f4ab_cpu=4`, `isolation_profile=l2`, protocol `threads=[1]`. The draft mode hard-codes `standard`, so this needs one more input. It is still not L2-admitted.

**Analysis**, done offline from the S3 bundle; the payload does not do it:
- Per case and thread count, compute the median `f4_ms` of B / A1 and of A2 / A1, with paired bootstrap 95% intervals over rounds.
- Report the B/A effect only where the A/A interval contains 1 and is narrower than the effect.
- Record a `bench.record/v1` with `python3 tools/benchrec.py init --slug f4-fp-ab-isolab --problem kernel:f4_fp_bench`. Then add one `metric` per case and thread count: unit `ratio`, `--reference-kind` set to something other than `matched_rho` (this is a kernel A/B, not a cross-method ECDLP row), and `add-raw --s3-uri s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod_uid>/…`. Finish with `close` and regenerate `benchmarks/progress/` in the same PR. No IC leaderboard row and no `benchmarks/README.md` row follow without a VALIDATION.json, which this route cannot produce.

**Task-card flow.** `experiment-director` compiles `research/f4_fp_ab_isolab_20261116/cards/CARD-f4_fp_ab-20261116-1.yaml` with these steps:
1. `scripts/isolab_pool_check.sh --cpu 14 --memory 8 --pool cpu-spot --save <receipt>/pool.json` (needs AWS/cluster read).
2. The `gh workflow run` above, with `BENCHREC=<record>`.
3. `gh run watch` with a bounded timeout. This step detaches because its timeout exceeds 540 s.
4. Download the `isolab-f4ab-*` artifact.

`executor-mechanical` runs it unchanged, and the director assesses it against `result.json`, `identity.json`, `counters.json` and `SHA256SUMS`.

## 5. Blockers, in order

1. **CodeBuild account queue limit is 0.** All 2026-11-16 launches failed with `AccountLimitExceededException ... more than 0 builds in queue`. Only an operator can clear it (an AWS service quota or account state). It blocks every EKS route, GitHub and AWS bridge alike.
2. **No `f4_fp_bench` image route on `main`.** The `f4_fp_ab` workflow mode (draft above) must be reviewed and merged by PR first, because GitHub registers dispatch inputs only from the default branch. The musl `--example f4_fp_bench --no-default-features` build has not been compiled; the no-builds rule applied to this lead.
3. **The source revision must be pushed.** The integrated F4 patches (for example the `F4_FP_PAR_OLD` toggle, which is absent at HEAD) and the study directory must exist as a commit on GitHub. COST_CONTROLS also requires `check-experiment-git` on the protocol and runner before spending.
4. **No AWS identity in this session.** The keys are rejected by STS (`InvalidClientTokenId`). As a result:
   - `isolab_pool_check.sh` cannot run.
   - The AWS-bridge route (`launch-remote-command` with a self-published layer) is unavailable.
   - Budget headroom cannot be read: the allowance is USD 200/day and USD 1,000/month, and spend is unknown, so the rule "remainder unknown → do not launch" applies.
5. **The resume checkpoint cannot be pushed** (no upload token for `scripts/checkpoint.sh push`). CLAUDE.md requires it before any run.
6. **The protocol is not frozen.** `protocol.json` is a draft. It must become a frozen `experiment.protocol/v1` (controlled-experiment / experiment-protocol skills), with the B-arm toggle named, before data are taken.
7. **Scope limit (not a blocker, but it limits any claim).** A command Job has no native L2, replay or A/A admission, so its wall-clock cannot pass the validation gate. Any ecbench-admitted timing would need a new ecbench method that wraps `f4_fp` with an arm-level toggle passed as a method parameter rather than an env var. That is a larger change.
