# Hand-off: build the image and submit the cap-6 cards (owner: "1+2", 2026-11-01)

The session that wrote this could not do either step: its AWS identity (IAM user, account `104186811093`) is denied
`ecr:DescribeRepositories`, `ecr:GetAuthorizationToken`, `eks:DescribeCluster` and `codebuild:BatchGetProjects`; Docker has no
daemon; `isolab_k8s_run` is not in its tool list. Everything that could be checked without those was.

## Verified here

- The static musl build recipe works at the tree of this PR: `cargo +1.99.1 build --locked --release --no-default-features
  --example pkm_tower_pilot --target x86_64-unknown-linux-musl` with `musl-tools` and `CC_x86_64_unknown_linux_musl=musl-gcc`
  gives an x86-64, statically linked (no `INTERP`) 7.4 MB binary.
- That binary, run on the `t = 9` regression cell, gives `D = 5`, 8,644 columns and 45,957,046,778 mul-adds, equal to the recorded
  row (`runs/R-t9-t10.jsonl`). Its SHA-256 on this host was `4b917a4b…db0f` (a cross-check only: a rebuild at another commit or path
  need not match it bit for bit).
- **`main` did not build for musl**: `src/ml_dataset/telemetry/whole_worker.rs:258` passed a `c_ulong` as the `ioctl` request,
  which is `i32` on musl. The one-line fix (`PERF_EVENT_IOC_ID as _`) is in this PR. It is outside the research directory and
  changes no behaviour on glibc. The ecbench static build in `build-bundle` links the same library, so the Iso Lab
  commit-building workflow should have failed at any `main` commit before this fix; worth a look by whoever owns it.

## Not verified (needs the lab host)

`docker build`, `docker push`, the ECR digest, `run_cell.sh` inside the image (`aws s3 cp` upload), the `isolab_k8s_run` call.

## 1. A session on the lab host

Give it this prompt:

> Read `.claude/skills/isolab-eks/SKILL.md` and `research/prime_fall_degree_t14_20261101/{HANDOFF.md,ISOLAB_CARD.md}` at the commit
> of PR #3183 or later on `main`. Run `image/build_image.sh <that commit>` with `ISOLAB_IMAGE_REPOSITORY` and `AWS_REGION` set.
> Then call `isolab_k8s_run` twice with the card's frozen fields (`job_name` `pfd-t14-k0-c6-20261101` and `…k1…`, `command`
> `["/usr/local/bin/run_cell","0"]` and `["/usr/local/bin/run_cell","1"]`, `memory` `112Gi`, `cpu` 16, `seconds` 90000, `retries`
> 0, the digest it printed). Non-resumable; never resubmit under a new name. Retain Job and Pod UIDs, the image digest and the
> results URI; download the objects and check `T-t14-kK-c6.sha256`. Put the rows in `runs/` and update `REPORT.md`.

## 2. Giving the session that wrote this the access

It then needs, in account `104186811093` / `us-east-2`: ECR push to the `isolab/tasks` repository (`ecr:GetAuthorizationToken`,
`ecr:BatchCheckLayerAvailability`, `ecr:InitiateLayerUpload`, `ecr:UploadLayerPart`, `ecr:CompleteLayerUpload`, `ecr:PutImage`,
`ecr:BatchGetImage`), a Docker daemon in its container, and a custom-command submission route: the `isolab_k8s_run` MCP (installed
on the lab host) or an EKS access entry for `isolab` plus a network route to its private endpoint. The CodeBuild bridge
(`launch-remote`) does not carry this workload (ecbench specs, `seconds` ≤ 3,600).

## Reminder

Expect censoring at the 86,400 s budget (card, "Reading"). Raising the budget or making the run resumable changes Amendment 7 A
and is the owner's call.
