# Resume: crypto @ main (2026-11-04) — arity completion runs on isolab-eks

## Goal
Execute the two frozen, merged protocols of `research/arity_completion_20261104/`
(prime towers m = 5 at t = 2,3,4; Koblitz chained-S3 m = 5,6 on n = 23..41 and
n = 83 with the 1800 s cap) on Iso Lab EKS, with the owner's AWS identity
(`coreuser`, account 104186811093), and report P1–P5 and K1–K7 through the
frozen `analyze.py` scripts. Owner (2026-11-04 07:26Z): "Run it on isolab now".

## Lead / hypothesis
No new hypothesis: the preregistered claims in
`research/arity_completion_20261104/{prime_tower_m5,koblitz_m5_m6}/protocol.json`
(frozen at 1fd6ece55f and 4fa25efc0e). Preflight expectation (AM1): the orbit
engine caps at m >= 5; chain-s3 is the engine whose negative control decides.

## Spec
- Protocols, cards, grid, run_shard.sh: `research/arity_completion_20261104/`.
- Route (selected this session): CodeBuild bridge
  `taskq/deploy/eks-karpenter/remote/launch-remote` cannot run arbitrary
  commands (ecbench only), and the EKS API admits only two CIDRs, so a
  command-job launcher (`remote/run-command`, `remote/launch-remote-command`,
  `remote/current-run-command-buildspec.yml`) is being added on a branch, plus a
  study image (ubuntu 24.04 + the two example binaries built at main, `bc`,
  `time`, aws CLI v2) pushed to `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks`
  by digest with a daemon-less OCI pusher.
- Jobs: Koblitz shards m2/m5/m6 on `cpu-l2-spot` (submit-job --isolation-profile l2,
  cpu 4, c8a.2xlarge), seconds 9900/50700/50700; prime-tower controls, t2-t3
  (8 GiB class, standard), t4 (120 GiB, only if t3 decided). Results upload to
  `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod-uid>/` every 60 s
  and at exit; no L2 admission exists for command jobs, so seconds are
  diagnostic only (Koblitz invalidation rule 6).
- Budget: daily budgets show USD 93.9 (isolab-daily-cost-100, limit 200) and
  37.5 (spot-experiments-daily-25, limit 200) actual; monthly 379 of 1000.
  Ceiling for this campaign: USD 40 (c8a.2xlarge spot ≈ 0.16 USD/h × 3 shards ×
  14 h ≈ 7; r-class 128 GiB host 24 h ≈ 15; CodeBuild minutes ≈ 1).

## State
- 07:36Z checkpoint pushed (crypto/main/20261104T073616Z-91a3524dfd6a); identity verified.
- Launcher extension and study image in progress on branch claude/arity-run-20261104;
  no Job submitted yet.

## Runs in flight
None yet (07:40Z). Builds: `/dev/shm/crypto-target/release/examples/{pkm_tower_pilot,n83_msweep_cell}`.

## Stop and success conditions
- Success: every submitted shard/card job uploads `job-exit.json` (exit 0) and
  its records; `analyze.py` of each study runs on the downloaded `raw/`;
  verdicts reported per claim with caps as censored.
- Stop: a card gate fails (controls mismatch, m2 process_error), a job is lost
  to Spot twice (max 3 infrastructure attempts per protocol), or the USD 40
  ceiling is reached. Never raise a cap or change a frozen file.

## Next steps
1. Finish launcher extension + tests; commit/push branch `claude/arity-run-20261104`.
2. Build and push the study image; record digest and binary sha256s.
3. Director compiles launch cards (one per job) binding request JSON, launcher
   scripts and image digest; executor runs them; collect via S3 + status runs.
