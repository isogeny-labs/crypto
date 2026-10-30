# Launch on Iso Lab (harness host, `isolab_k8s_run`)

Not yet run. Needs the frozen commit, an image built from it and pushed to ECR
by digest, and the harness-host MCP. Loading this file launches nothing.

1. Freeze `COMMIT=$(git rev-parse HEAD)` on the PR head.
2. `docker build -f research/prime_fall_degree_20261027/isolab/Dockerfile -t $ISOLAB_IMAGE_REPOSITORY:pfd-$COMMIT .`
   then `docker push`, and read the digest with `aws ecr describe-images`.
   Record source SHA, binary SHA-256 (the job uploads `binary.sha256`) and digest separately.
3. Submit (the CPU cap is 28; the stage A chain and stage C chain are independent jobs):
   - job `pfd-a-20261028-001`: command `["/usr/local/bin/entry.sh","hky-A","d-A","cap-A"]`
   - job `pfd-c-20261028-001`: command `["/usr/local/bin/entry.sh","g4","hky-C"]`
   each with `image` = the digest URI, `cpu` 14, `memory` 56Gi, `seconds` 14400, `retries` 0.
   Stage A restarts from t=1; its t<=10 rows must reproduce the medians in RESTART.md.
4. `isolab_k8s_status`, `isolab_k8s_logs`, `isolab_k8s_results`; pull `runs/*.jsonl` from
   `ISOLAB_RESULTS_URI`, verify the Job/Pod UIDs and the binary hash, then score with
   `prime_fall_degree_tools` and write RESULTS.md.
These runs are not resumable (ordinary fresh jobs). A Spot reclaim restarts the job
from scratch, so partial rows are synced to S3 every 5 minutes for inspection only.
