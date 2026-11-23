# Round-2 image and request (r2img, 2026-11-08): minors_guess_determine_20261107

All numbers and claims are `[pcc:unverified]`. Nothing was launched.

## Goal
Digest-pinned image for the frozen 48-shard minors_gd panel plus a validated command request.

## What was done
- Built `minors_gd` with `cargo build --release --offline --target x86_64-unknown-linux-musl` (no committed Cargo.lock, no dependencies; `--locked` not used). Development check: `minors_gd selftest` printed SELFTEST PASS.
- Image carries /opt/minors/{minors_gd, run_panel.sh, minors_gd_analyze.py, entry.sh, build-info.json}. `entry.sh` (source `eks/r2img_entry.sh`) runs the frozen `run_panel.sh` UNCHANGED with 14 shard processes, syncs `$ISOLAB_OUTPUT_DIR` to S3 every 600 s and at exit (shards, `panel/records.jsonl`, `panel/analysis.json`, `run_panel.log`), then writes a top-level `SHA256SUMS` and uploads.
- Source revision `f13a690df016d0a457bd40579fbb9d054f486adf` (git archive, not the working tree). Binary is static (`readelf -l`: no INTERP, measured).
- Published with `taskq/deploy/eks-karpenter/oci-layer-publish.py` on the lab base (`jq -r .image_uri taskq/deploy/eks-karpenter/image.json`, built FROM the AWS-CLI image; its layers were inspected and have bash, sh, python3, awk, xargs, sha256sum, jq, nproc; `aws` is the base entrypoint binary). Tag `minors-f13a690df016`, user 1000:1000, workdir /work, HOME=/work. Receipt `eks/layer-receipt.json`.
- After the 04:50-05:00 UTC full-disk window: receipts and requests re-checked as valid JSON, ECR digest for each tag re-queried and equal to receipt and request (measured), layers re-read from the registry. R2_READY.md was rewritten (it had been truncated to 0 bytes).

## Results (evidence class)
- Image digest `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:72934ab9ea2d9d93c50088972aca1924cfde23af87755cd8a802a04f6254dd4c` (measured, `aws ecr describe-images` equals receipt and request).
- Binary sha256 (build host, measured): `523c66fffd9a7e49bb2901b1d408824f5cd8096f7c48c03d0f0adeca70c8d686`
- FROZEN_SHA256.txt: 8 src files, Cargo.toml, `minors_gd_analyze.py`, `run_panel.sh`, `PROTOCOL.md`, `sources/NOTES.md` all OK on the git-archive copy (measured). The listed binary hash is the old dev-box glibc build and is expected to differ; the musl binary hash is above.
- Job: 14 vCPU, 32Gi, 43200 s, standard, retries 0, wait_seconds 60. Planned 25-55 core-hours (heuristic), about 2-4 h wall on 14 vCPU.
- Validator: `eks/r2img_validate_output.txt` (VALID; two negative controls INVALID: cpu 15, placeholder digest). It is the inline jq of `remote/run-command`, as in the pkm study.

## Decisions made (and why)
- ONE job, no split: 55 core-h / 14 = about 4 h, far under 86400 s, so no shard mapping or merge step. 43200 s gives about 3x margin on the high estimate plus tail imbalance (b=36/40 shards are cap-bound).
- 14 vCPU / 32Gi fits the 16-vCPU pool (planned under 1 GB per process); no other command job can run concurrently.
- Frozen run_panel.sh kept unchanged; it merges and analyses in-job.

## Open threads
- On preemption or failure, partial shard files are in S3 but run_panel.sh has no skip option and shard files are append-mode (a partial shard must not be reused); a fresh job reruns all 48 shards. A shard-list variant can be added (shards: b in 20,24,28,32,36,40; k in 0..7).
- If run_panel.sh fails (set -e), merge and analysis are skipped; entry still uploads what exists.
- Binary was not run on EKS; only seconds-long development checks locally.
- research-kb and Conductor are not installed here: no `source_id`s, no task attach. Sources checked: the study README/PROTOCOL.md, remote/run-command, submit-job, oci-layer-publish.py.
- Main session still owes: Conductor, `scripts/checkpoint.sh init`, benchrec if hooks classify this as a benchmark, committing these `eks/` files, director card.

## Errors
Shared disk filled 04:50-05:00 UTC (another agent's build) while this session was writing; only the R2_READY.md files were truncated and are rewritten. No publish step was in that window (published 04:44-04:45).

## Ready-to-launch
- Request: `research/minors_guess_determine_20261107/eks/r2img_request.json`
- Validator output: `research/minors_guess_determine_20261107/eks/r2img_validate_output.txt` (re-run: `bash research/minors_guess_determine_20261107/eks/r2img_validate.sh research/minors_guess_determine_20261107/eks/r2img_request.json`)
- Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:72934ab9ea2d9d93c50088972aca1924cfde23af87755cd8a802a04f6254dd4c`
- Launch: `ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher AWS_DEFAULT_REGION=us-east-2 ISOLAB_CLIENT_OUTPUT=<fresh abs dir> taskq/deploy/eks-karpenter/remote/launch-remote-command research/minors_guess_determine_20261107/eks/r2img_request.json`
- Results: `s3://isolab-results-104186811093-us-east-2/runs/<job_name>/<pod-uid>/`; `sha256sum -c SHA256SUMS` after download.

## Shared files edited
None. New files only in `research/minors_guess_determine_20261107/eks/`: `R2_READY.md`, `r2img_entry.sh`, `r2img_request.json`, `r2img_validate.sh`, `r2img_validate_output.txt`, `layer-receipt.json`.
