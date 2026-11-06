---
name: ecbench-independent-runner
description: Reproduce or cross-check an ecbench session on another machine with exact replays and an independent replay certificate. Route default x86 lab measurements to isolab-eks; retain the legacy runner for explicitly requested NATS or bare-metal placement. Use for a second host or independent validation before a claim.
---

# Independent runs and reproduction

Read [`docs/ecbench/README.md`](../../../docs/ecbench/README.md) §6 and §9.
For default x86 lab experiments, use [isolab-eks](../isolab-eks/SKILL.md) and
its EKS Spot request, authentication and S3 verification contract. An independent
replay still needs the distinct host class required below; another pod alone
does not establish independence. EKS L2 does not establish L3 or bare metal.
If those are explicitly required, retain that requirement and use the specified
qualified runner or report the missing capability.

## Explicit legacy NATS or bare-metal submission

Only for an explicitly selected legacy backend, read `isolab/README.md` and use
the following commands. These are a different API from `isolab_k8s_*` and the
EKS portable launcher. Every run is graded from ecbench's own observations.

For client installation and project MCP registration, start with
[`docs/ISOLAB_AGENTS.md`](../../../docs/ISOLAB_AGENTS.md). The `isolab` connection
below uses the existing NATS hub. The separate `isolab-eks` connection exposes
`isolab_k8s_*` for the existing EKS deployment: its ecbench L2 workflow does not
accept legacy `isolab.job/v1` documents or establish the NATS strict tier.

1. Build `ecbench` for the worker's architecture (Linux x86-64 or arm64), from a
   committed, clean tree. The binary's SHA-256 is its identity.
2. Write the job:

   ```bash
   ./target/release/ecbench isolab-job --spec SPEC.json --binary path/to/linux/ecbench --cpus 2 --policy strict > job.json
   ```

   - `--cpus 2` places one core and its SMT sibling. The measured child runs on
     one CPU and the runner sleeps on the other. Use `--cpus 4` for two cores.
   - `--policy strict` (the default) expands in isolab to bare metal, perf
     counters and isolation tier A, so it places only on a bare-metal worker. On
     a VM worker use `--policy standard` and expect the records to show steal or
     frequency blockers.
   - `--git-url URL --commit SHA` builds on the worker instead. `Cargo.lock` is
     untracked, so dependencies resolve there and the job needs network. Prefer
     `--binary`.
3. Submit with the isolab MCP tools (`isolab_submit`, then `isolab_wait`) or the
   CLI (`isolab submit job.json`). Do not edit the generated `fidelity` or
   `resources` sections to make a job place.
4. Fetch the artifacts (`isolab fetch JOB_ID`, or `isolab_result` over MCP).
   `session/` is an ordinary ecbench session. The job's
   `verify` step already ran `ecbench verify --replay 3` on the worker. Run the
   audit again locally:

   ```bash
   ./target/release/ecbench verify --dir artifacts/session --replay 12 --out artifacts/session.audit.json
   ```

5. Report the levels earned and the top blockers from the records, not from the
   job's policy. A strict placement that still shows steal or foreign ticks earned
   less, and the record says so.

If `meow34.pem` or lab credentials are not available in your environment, report
that blocker. Do not create replacement keys or accounts (AGENTS.md §9).

## Reproduce a session on another machine

Operation counts depend only on code, workload and seed, never on the host. Any
machine can therefore check them:

```bash
./target/release/ecbench verify --dir research/TOPIC/sessions/SESSION --replay 24 --exit-code --out receipt.json
```

- Use a binary built from the session's `git_commit` (in `session.json`). A
  different commit is a different method unless the replays still match.
- `identical` on every replay means the figure is reproduced. Cite the receipt's
  SHA-256 as the independent replay certificate.
- For a `vs_rho` claim, run it with `--replay-all` on a host whose class
  (`ecbench host`, `env_class_id`) differs from the session's, keep the receipt
  somewhere durable, and hand `ecbench claim build` the file and its location
  (`--independent-receipt`, `--pointer`). The receipt records the auditor's host
  class and the hash of every session file; the claim builder refuses one from
  the session's own class, for other bytes, or that did not reproduce both runs.
- Wall time does not reproduce across hosts, and nothing in this check claims it
  does. To confirm a wall-clock result, rerun the spec as a new session on a host
  of the same class (`env_class_id`) at the required level and compare the two
  intervals.

## Under Claude Code

For EKS, use the task-card procedure in the `isolab-eks` submission reference.
For explicitly selected legacy runs, the `isolab-job`, fetch and
`ecbench verify --replay` commands above may run as a command card:
`experiment-director` writes it from the frozen spec or
session path, the main session relays it verbatim to `executor-mechanical`,
and `experiment-director` assesses the returned text against the receipt on
disk (AGENTS.md "Model tiers"). Comparing levels, blockers and
disagreements stays with the reasoning tier.

## When results disagree

Do not average them away. Record both sessions. Compare `host.json` stable facts
(the class id says whether the hosts match), the levels and their blockers, and
`binary_sha256`. Report the disagreement as a finding.
