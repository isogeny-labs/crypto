# CI concurrency: diagnosis and runner-pool plan (2026-10-19)

Status: **plan only. Nothing here has been launched and nothing costs money until the owner approves.**
AWS credentials in the authoring environment have no access key ID, so no price or quota figure
below was read from AWS; every dollar figure is an estimate to be re-checked with the commands in §4.

## 1. Queue observations (aburan28/crypto, 2026-10-19 ~21:30Z)

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

The 18 active **workflow runs are not a count of running jobs**: a run can have multiple jobs,
including jobs that are waiting. The sampled job spent 105 minutes queued and had run for about
five minutes when sampled; its final duration was not measured. These observations establish a
long wait for that job, but do not establish account-wide saturation or its cause. The account's
plan, concurrent running-job count across repositories, runner availability, and applicable
quotas were not collected. Capacity limits are one hypothesis to investigate. The workflow
scan found no self-hosted labels; it did not inventory registered runners. Timing was sampled
for one job and branch counts from the first 100 of 360 queued runs, without a distribution.

Already in place before this PR: 334 of 431 workflows had a `concurrency:` group, 336 mention
`cancel-in-progress`, and 129 of the 136 PR-triggered workflows already filter on `paths:`.
The remaining 7 without `paths:` (`rust-lint`, `rust-all-targets`, `ecbench`, `codeql`,
`llm-review`, `autofix-queue`, `ecc2k130-m10-capacity-once`) guard correctness or are
label/schedule-triggered, so they were left running on every PR.

## 2. Changes in this PR

Added the repo's existing PR-cancelling concurrency block to 12 PR-triggered workflows that had none
(superseded commits on the same PR stop; push, schedule, dispatch and merge-queue runs are never cancelled).
No test was skipped and no `paths:` filter was added or loosened.

Other workflows were **deliberately not edited** because their bytes are
sha256-pinned by `FROZEN.json`/`MANIFEST.json` files of frozen studies, or by
`.github/scripts/check-pr-budgets.py` seals (`weak-curve-lean` and
`weak-curve-conjectures`). Editing them would invalidate preregistered
replays. If the owner wants them cancelled too, that needs a re-freeze decision per study.

Not done: folding trivial workflows together. With 431 files and byte-pinned history it is not
safe to do mechanically, and it would not raise the concurrency cap, only the job count per run.

Effect to expect (not yet measured): fewer superseded runs holding queue slots when a PR is
pushed repeatedly. It does not raise the cap. Before/after queue time must be measured on PRs
opened after merge; this PR does not claim a number.

## 3. What actually raises capacity

1. **Account and plan assessment.** GitHub's [published Actions limits](https://docs.github.com/en/enterprise-cloud%40latest/actions/reference/limits)
   list standard-runner concurrent-job limits of Free 20, Pro 40, Team 60, and Enterprise 500,
   with additional platform limits. This repository's actual account plan and limits have not
   been verified. Confirm those and the running-job census before considering a change; plan
   eligibility, seat count, included minutes, and current pricing determine the cost comparison.
2. **Larger runners** have separate, finite concurrency limits and configured maximums.
   They are [available to organizations and enterprises using Team or Enterprise Cloud](https://docs.github.com/en/actions/concepts/runners/larger-runners).
   Availability for this repository has not been established.
3. **Self-hosted runners** supply separate compute capacity but remain subject to runner,
   workflow, API, and infrastructure limits. The candidate design below needs a verified job
   census, resource measurements, and account-specific provisioning before it is executable.

## 4. Candidate self-hosted ephemeral spot pool (not implemented or launched)

Design: ephemeral, just-in-time (JIT) runners on EC2 spot, one job per instance, scaled by queue depth.
Building on `infra/spot-system/` (`AWS_DEPLOYMENT.md`) and `ecc2k130/aws/` bootstrap scripts.

Constraints found in the repo's own docs:
- Repository documentation records a `us-east-2` Standard Spot quota of **96 vCPUs**, shared with research workers
  (`infra/spot-system/AWS_DEPLOYMENT.md`: 32 vCPU original host + two 32 vCPU managed workers).
  That is a historical value, not a live quota or usage reading. If all three documented workers
  are running, they already use all 96 vCPUs: even eight additional runners would not fit.
  Verify current quota, usage, and reserved research capacity before setting any nonzero pool
  maximum. A quota increase requires a separate request and is not guaranteed.
- Spot interruption kills a job mid-run; GitHub marks it failed and it must be re-run. Acceptable for
  CI that is idempotent; do not put the long-running measurement jobs on this pool.

Illustrative instance candidates; per-job peak memory, disk use, and CPU requirements have not
been measured, so 4 vCPU / 8-16 GiB has not been established as sufficient:

| Type | vCPU/GiB | On-demand $/h (est.) | Spot $/h (est., ~60-70% off) |
|---|---|---|---|
| c7a.xlarge | 4/8 | ~0.164 | ~0.05-0.07 |
| c7i.xlarge | 4/8 | ~0.179 | ~0.05-0.07 |
| c6a.xlarge | 4/8 | ~0.153 | ~0.045-0.06 |
| m7a.xlarge | 4/16 | ~0.232 | ~0.07-0.09 |

Use `price-capacity-optimized` allocation over c6a/c7a/c7i/m7a.xlarge so one pool of types survives
a single type's interruption. Arm jobs would require compatible Arm instances and independently
verified prices; no architecture-specific savings have been measured.

Monthly cost (estimate; busy hours dominate because runners are ephemeral and scale to zero):

    cost/month = runners x busy_hours_per_day x 22 days x spot_$/h

| Scenario | runners | busy h/day | spot $/h | $/month |
|---|---|---|---|---|
| Eight-runner illustration (quota fit unverified) | 8 | 10 | 0.06 | ~106 |
| Medium | 20 | 10 | 0.06 | ~264 |
| Heavy (24 h/day for 22 days) | 20 | 24 | 0.06 | ~634 |

Plus EBS (8-gb gp3 root, ~$0.64/month per 8 GiB while up, negligible if ephemeral), data transfer
(unmeasured), and the controller (a t4g.small, ~$12/month on-demand estimate). These figures omit
taxes and retries and are not quotes. Continuous operation for 30 days at the heavy row's assumed
rate would cost about $864 for instance time alone. Compare verified total costs with eligible
GitHub plans after measuring the workload.

### Provisioning outline (incomplete; needs owner approval; none run)

```bash
# 0. Read real prices and quota first (needs working AWS credentials):
aws ec2 describe-spot-price-history --region us-east-2 --product-descriptions "Linux/UNIX" \
  --instance-types c6a.xlarge c7a.xlarge c7i.xlarge m7a.xlarge --start-time "$(date -u -d '-1 day' +%FT%TZ)"
aws service-quotas get-service-quota --region us-east-2 --service-code ec2 --quota-code L-34B43A08  # Standard Spot vCPUs

# 1. Confirm the account and registration scope. Organization runner groups are not
#    a personal-repository setting. Configure an eligible group if applicable,
#    and restrict repository access and the "crypto-ci-spot" label.

# 2. Launch template user-data (cloud-init) for each ephemeral runner:
#    - fetch a JIT config:  gh api -X POST repos/aburan28/crypto/actions/runners/generate-jitconfig \
#         -f name="spot-$(hostname)" -F runner_group_id=<id> -f 'labels[]=crypto-ci-spot'
#    - ./run.sh --jitconfig "$ENCODED_JIT_CONFIG"     # exits after one job; instance self-terminates
#    - `shutdown -h now` with InstanceInitiatedShutdownBehavior=terminate

# 3. Scaler: a controller (or actions-runner-controller on a small k3s node) polls
#    enumerates jobs within runs, filters eligible labels and trust, and subtracts
#    already assigned/pending runner capacity. Run counts are not queued-job counts.
#    Keep MAX_RUNNERS=0 until live quotas, research reservations, and costs are verified.

# 4. Switch jobs gradually: set repo variable CI_RUNNER, then in a workflow
#      runs-on: ${{ vars.CI_RUNNER || 'ubuntu-24.04' }}
#    Choose initial workflows from measured queued-job minutes and resource requirements.
```

Risks to review before approval: a self-hosted runner executes PR code, so fork PRs must stay on
hosted runners. Same-repository branches also execute arbitrary code and require an explicit trust
policy. Keep controller credentials and runner-registration authority off job instances, and define
instance metadata, network, and artifact isolation before deployment. Runner credentials should be
JIT and single-use; the controller's AWS permissions must be limited to the specific pool resources.
The commands above are an outline, not a complete scaler, security policy, or deployable bootstrap.

## 5. Not done in this pass

- **aburan28/cryptanalysis and aburan28/crypto-autoresearcher** were not inspected: they are outside this
  session's repository scope and `add_repo` was not called. Their queue time and duplicated workflows are unmeasured.
- Before/after queue time. The "before" numbers are in §1; "after" needs PRs opened after this merges.
- Any measurement of which workflows contribute the most queued minutes (run durations were not collected).
