# Four-rung single-target IC tournament, 2026-11-15

Owner-authorized execution: up to USD 20 additional spend despite the monthly ceiling, no automatic retries, original 13-hour absolute deadline in `request.json`. All four protocol-default single-target panels remain requested: n=61 (64 evaluation workloads, four tune workloads per K) and n=71/73/83 (16 evaluation workloads, one tune workload per K). Every K candidate is retained.

Algorithm and measurement source is frozen at `a10cb7b4f8e48f094efe70c31d9a24b6d72aee4a`. The published launch-configuration commit is a separate identity. Docker's base is the pinned Linux amd64 Rust image in `Dockerfile`; the final ECR digest and binary hashes will be retained with the launch and result receipts.

Execution is one sequential `cpu-l2-spot` command Job, fixed `m8a.2xlarge`, four requested/limited pod CPUs and 24 GiB, Kubernetes `backoffLimit=0`. Build precedes timing. Each rung gets one attempt capped at three hours and the remaining original deadline. The job records all exit statuses, including unattempted work when the deadline expires, and uploads after each rung and on exit. No `--resume`, retry loop, shortened panel or changed K grid is used.

The command is `/opt/tournament/run.sh` from this directory. Build the image with `docker build --platform linux/amd64 -f research/ic_boundary_tournament_20261115/Dockerfile research/ic_boundary_tournament_20261115`, publish to the existing `isolab/tasks` ECR repository and retain the resolved immutable digest. Submit through the maintained `taskq/deploy/eks-karpenter/submit-job` with `--cpu 4 --memory 24Gi --isolation-profile l2 --instance-type m8a.2xlarge --retries 0`, setting `--seconds` to the original deadline minus current time. Local image preparation consumes the same deadline. Before any cloud build or run, verify exact source/configuration publication with `check-experiment-git` and live capacity with `scripts/isolab_pool_check.sh`.

Budget reservation: at most USD 16 worker cost (13 hours at a conservative USD 1/hour plus terminal/idle margin), USD 4 for image storage, transfer and receipts; image packaging is local. Verify current fixed-instance pricing before submission and do not launch if this bound no longer fits. Never increase pool caps, change providers or create a fresh retry budget.

Accept only paired results on the same point, complete IC online phases, separately reported setup, independent certificate replay, and retained isolation receipts. The existing harness controls determine result standing. Native L2 admission, A/A coverage and independent-host replay must be assessed from receipts, never inferred from Job success or the selected worker pool. No ledger row is promoted by this launch.

Sources checked: autolab README/protocol and `single_target_panel.py`; `tools/isolated_bench.py`; Iso Lab skill, HARNESS, ACCESS, COST_CONTROLS and submit-job. This reproduces registered beats and makes no new mechanism or novelty claim.

Status: authorized preparation; no measurements reported in this document. Raw prelaunch evidence is under `research/sat_factor_base_review_20260908/autolab/runs/tournament-20261115-single-target/`. Terminal reports and durable raw-result references will be added after execution.
