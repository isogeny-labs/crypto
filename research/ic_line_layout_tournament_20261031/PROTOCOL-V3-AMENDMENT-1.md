# Line-layout tournament, v3 amendment 1 (protocol version 4)

Filed 2026-11-06, before any v3 or v4 data. Machine-readable record:
[`protocol-v4.json`](protocol-v4.json), `protocol_id`
`IC-LINE-LAYOUT-E2E-FP20-V3A1-20261106`. It amends
[`protocol-v3.json`](protocol-v3.json) (frozen 2026-11-01, never dispatched).

## Why

v3 kept the `isolab-experiment.yml` GitHub route. That route failed at the EKS
stage in both earlier versions (v1: L2 gate, exit 42; v2: the qualification job,
remote exit 23). It has not published an image since 2026-11-02. On 2026-11-06
the owner chose to move v3 to the agent-submitted isolab-eks AWS bridge. That
bridge admitted every L2 execution of both XP-20261029 stride-renew sessions on
c8a.2xlarge.

## What changes

| Field | v3 | v4 |
|---|---|---|
| Route | GitHub dispatch of `isolab-experiment.yml` | `taskq/deploy/eks-karpenter/remote/launch-remote` (AWS CodeBuild bridge), submitted from the agent |
| Executable | new build of the v3 merge commit | existing image `isolab/tasks@sha256:3c4ad8bf8ee1bd1fdc327cb4c0e2597a1fc5ef1cdd889aa21e1b1294bdd22914`, built from `ea8e9ef71c3c82027dd9e01535302725519c3fca` by run 41271383665, `ecbench_sha256` `73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01` |
| Arm instruments | commit and binary unset | that commit and binary, on all five arms |
| Qualification spec | `l2-validation.json` at the v3 commit | `l2-validation.json` at `ea8e9ef7`, on the same image |
| Analysis binary | a local build | the ecbench binary extracted from the pinned image, checked against the sha256 above |
| Budget | none recorded | USD 5 ceiling for all three jobs, failed attempts included |

The scientific spec ([`spec-fp20-v3.json`](spec-fp20-v3.json)) is unchanged, so
its unconsumed seeds and its 1,800 s measurement timeout carry over. So do the
900 s job deadline, the arms, the controls, the decision rule and
[`analyze.sh`](analyze.sh). The requests are in [`launch-v3a1/`](launch-v3a1/).

## Scope of the claim

The bridge cannot build an image; it can only run one that already exists.
The only image that contains `layout=lines` is the v2 build of `ea8e9ef7`.
Twenty-five files changed between `ea8e9ef7` and the v3 freeze, among them
`ic_framework` `mod.rs`, `stages.rs` and `residual.rs`, and ecbench. Any
verdict therefore applies to the `ea8e9ef7` executable. It says nothing about
later commits until they are re-measured.

## Concurrency

The two scientific requests are submitted back to back. The L2 NodePool cap
(about 8 vCPUs) can admit them one after the other instead of side by side.
Each job still runs on its own fresh L2 node and passes or fails on its own,
so a serial admission is recorded as an observation, not a deviation from the
decision rule.

## Stopping

The v3 stopping rules hold:

- Exactly one qualification job, then exactly two scientific jobs.
- No retry and no alternate seed.
- Any failure ends this version.
