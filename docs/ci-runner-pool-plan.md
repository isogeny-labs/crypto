# CI concurrency: diagnosis and runner-pool plan (2026-10-19)

Status: **plan only. Nothing here has been launched and nothing costs money until the owner approves.**
AWS credentials in the authoring environment have no access key ID, so no price or quota figure
below was read from AWS; every dollar figure is an estimate to be re-checked with the commands in §4.

## 1. Diagnosis (measured, aburan28/crypto, 2026-10-19 ~21:30Z)

| Measurement | Value | Source |
|---|---|---|
| Runs `queued` | 360 | `actions_list` status=queued (total_count) |
| Runs `in_progress` | 18 | `actions_list` status=in_progress (total_count) |
| Queue time, example job | created 19:41:36Z, started 21:26:37Z = **105 min** | job 118267496391, run 39313092276 |
| Same job, run time so far | ~5 min (checkout alone took 115 s) | same job's step timestamps |
| Runner for that job | `GitHub Actions` group, label `ubuntu-latest` (GitHub-hosted) | job `runner_group_name`, `labels` |
| Labels in 431 workflow files | `ubuntu-24.04` ×522, `ubuntu-latest` ×60, `ubuntu-24.04-arm` ×9, matrix/macos/windows ×7 | grep `runs-on` |
| Self-hosted runners | none referenced | grep |
| PR-triggered workflows | 136 files, 226 jobs | parsed with PyYAML |
| Queued PR runs per branch | 13 branches held 95 queued `pull_request` runs (up to 13 runs for one PR head) | first 100 queued runs |

Reading: about 18 jobs run at once while hundreds wait, with every job on GitHub-hosted runners.
That is the signature of the account-level concurrent-job cap, not of offline self-hosted runners
(there are none). Queue time (105 min) is ~20x the run time (~5 min) for the sampled job, so
throughput is capacity-bound. Caveat: I sampled one job's timing and the first 100 of 360 queued
runs; I did not compute a distribution. The 18-versus-cap comparison assumes the plan cap is 20
(GitHub Free) — the owner should confirm the plan under Settings → Billing.

Already in place before this PR: 334 of 431 workflows had a `concurrency:` group, 336 mention
`cancel-in-progress`, and 129 of the 136 PR-triggered workflows already filter on `paths:`.
The remaining 7 without `paths:` (`rust-lint`, `rust-all-targets`, `ecbench`, `codeql`,
`llm-review`, `autofix-queue`, `ecc2k130-m10-capacity-once`) guard correctness or are
label/schedule-triggered, so they were left running on every PR.

## 2. Changes in this PR

Added the repo's existing PR-cancelling concurrency block to 12 PR-triggered workflows that had none
(superseded commits on the same PR stop; push, schedule, dispatch and merge-queue runs are never cancelled).
No test was skipped and no `paths:` filter was added or loosened.

27 further workflows lack the block but were **deliberately not edited**: 25 have their bytes
sha256-pinned by `FROZEN.json`/`MANIFEST.json` files of frozen studies, and 2
(`weak-curve-lean`, `weak-curve-conjectures`) by `.github/scripts/check-pr-budgets.py` seals. Editing them would invalidate preregistered
replays. If the owner wants them cancelled too, that needs a re-freeze decision per study.

Not done: folding trivial workflows together. With 431 files and byte-pinned history it is not
safe to do mechanically, and it would not raise the concurrency cap, only the job count per run.

Effect to expect (not yet measured): fewer superseded runs holding queue slots when a PR is
pushed repeatedly. It does not raise the cap. Before/after queue time must be measured on PRs
opened after merge; this PR does not claim a number.

## 3. What actually raises capacity

1. **GitHub plan (owner only).** Published concurrent-job limits: Free 20, Pro 40, Team 60,
   Enterprise 500 (standard Linux runners). Team is listed at about $4 per user per month;
   Enterprise about $21 per user per month. Verify current pricing and limits on github.com/pricing
   before deciding. Team (3x today's cap, if today is Free) is the cheapest first step.
2. **Larger/self-hosted runners** are not subject to the hosted-runner cap (self-hosted runners are
   limited only by how many you register), which is the plan below.

## 4. Self-hosted ephemeral spot pool (ready to run, not launched)

Design: ephemeral, just-in-time (JIT) runners on EC2 spot, one job per instance, scaled by queue depth.
Building on `infra/spot-system/` (`AWS_DEPLOYMENT.md`) and `ecc2k130/aws/` bootstrap scripts.

Constraints found in the repo's own docs:
- The `us-east-2` Standard Spot quota is **96 vCPUs**, shared with the research workers
  (`infra/spot-system/AWS_DEPLOYMENT.md`: 32 vCPU original host + two 32 vCPU managed workers).
  A CI pool of 20 four-vCPU runners (80 vCPUs) would **starve the research workers**. Either request a
  quota increase (free, needs owner) or cap the pool at ~8 runners (32 vCPUs) until it lands.
- Spot interruption kills a job mid-run; GitHub marks it failed and it must be re-run. Acceptable for
  CI that is idempotent; do not put the long-running measurement jobs on this pool.

Instance choice (4 vCPU / 8-16 GiB is enough for these jobs; the 105-min wait is the problem, not job size):

| Type | vCPU/GiB | On-demand $/h (est.) | Spot $/h (est., ~60-70% off) |
|---|---|---|---|
| c7a.xlarge | 4/8 | ~0.164 | ~0.05-0.07 |
| c7i.xlarge | 4/8 | ~0.179 | ~0.05-0.07 |
| c6a.xlarge | 4/8 | ~0.153 | ~0.045-0.06 |
| m7a.xlarge | 4/16 | ~0.232 | ~0.07-0.09 |

Use `price-capacity-optimized` allocation over c6a/c7a/c7i/m7a.xlarge so one pool of types survives
a single type's interruption. Arm (`ubuntu-24.04-arm`, 9 uses) would use c7g/c8g.xlarge, ~20% cheaper.

Monthly cost (estimate; busy hours dominate because runners are ephemeral and scale to zero):

    cost/month = runners x busy_hours_per_day x 22 days x spot_$/h

| Scenario | runners | busy h/day | spot $/h | $/month |
|---|---|---|---|---|
| Minimal (fits quota) | 8 | 10 | 0.06 | ~106 |
| Medium | 20 | 10 | 0.06 | ~264 |
| Heavy (24 h/day) | 20 | 24 | 0.06 | ~634 |

Plus EBS (8-gb gp3 root, ~$0.64/month per 8 GiB while up, negligible if ephemeral), data transfer
(small), and the controller (a t4g.small, ~$12/month on-demand). Compare with the GitHub Team upgrade,
which for a small number of seats costs less than the cheapest row.

### Launch sequence (needs owner approval; none run)

```bash
# 0. Read real prices and quota first (needs working AWS credentials):
aws ec2 describe-spot-price-history --region us-east-2 --product-descriptions "Linux/UNIX" \
  --instance-types c6a.xlarge c7a.xlarge c7i.xlarge m7a.xlarge --start-time "$(date -u -d '-1 day' +%FT%TZ)"
aws service-quotas get-service-quota --region us-east-2 --service-code ec2 --quota-code L-34B43A08  # Standard Spot vCPUs

# 1. Runner group + label (once, owner):  Settings > Actions > Runner groups > "ci-spot", label "crypto-ci-spot"

# 2. Launch template user-data (cloud-init) for each ephemeral runner:
#    - fetch a JIT config:  gh api -X POST repos/aburan28/crypto/actions/runners/generate-jitconfig \
#         -f name="spot-$(hostname)" -F runner_group_id=<id> -f 'labels[]=crypto-ci-spot'
#    - ./run.sh --jitconfig "$ENCODED_JIT_CONFIG"     # exits after one job; instance self-terminates
#    - `shutdown -h now` with InstanceInitiatedShutdownBehavior=terminate

# 3. Scaler: a controller (or actions-runner-controller on a small k3s node) polls
#    `actions/runs?status=queued` and launches min(queued_jobs, MAX_RUNNERS) instances.
#    Keep MAX_RUNNERS=8 until the vCPU quota is raised.

# 4. Switch jobs gradually: set repo variable CI_RUNNER, then in a workflow
#      runs-on: ${{ vars.CI_RUNNER || 'ubuntu-24.04' }}
#    Start with rust-lint and rust-all-targets (largest queue contributors), measure, then widen.
```

Risks to review before approval: a self-hosted runner executes PR code, so fork PRs must stay on
hosted runners (the pool is for same-repo branches only, as `autofix-queue` already assumes); runner
credentials are JIT and single-use; the AWS role for the controller needs `ec2:RunInstances`,
`ec2:TerminateInstances` and `iam:PassRole` on the runner role only.

## 5. Not done in this pass

- **aburan28/cryptanalysis and aburan28/crypto-autoresearcher** were not inspected: they are outside this
  session's repository scope and `add_repo` was not called. Their queue time and duplicated workflows are unmeasured.
- Before/after queue time. The "before" numbers are in §1; "after" needs PRs opened after this merges.
- Any measurement of which workflows contribute the most queued minutes (run durations were not collected).
