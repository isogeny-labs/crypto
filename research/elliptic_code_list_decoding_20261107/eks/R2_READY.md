# Round-2 image and request (r2img, 2026-11-08): elliptic_code_list_decoding_20261107

All numbers and claims are `[pcc:unverified]`. Nothing was launched.

## Goal
Digest-pinned image for the 64 frozen ecld_jobs.txt lines plus a validated command request.

## What was done
- Built `ecld_toy` with `rustc -O --target x86_64-unknown-linux-musl ecld_toy.rs` (rustc 1.97.0) from the archived file.
- Wrote `eks/r2img_entry.sh` (image path /opt/ecld/entry.sh): checks the in-image `ecld_toy.rs` sha256 against the frozen value (exit 3 on mismatch), runs each ecld_jobs.txt line sequentially with argv0 replaced by /opt/ecld/ecld_toy, one stdout/stderr file per line (`lines/line_NN.jsonl|.err`), logs per-line exit and wall to `progress.txt`, syncs to `$ISOLAB_RESULTS_URI` after each line, writes `panel.jsonl` and `SHA256SUMS`. Exit non-zero if any line fails or fewer than 64 ran.
- Development test of the entry on the build host with a fake `aws` and 2 lines: both exit 0, 24 JSONL rows, SHA256SUMS produced (development only).
- Source revision `f13a690df016d0a457bd40579fbb9d054f486adf` (git archive, not the working tree). Binary is static (`readelf -l`: no INTERP, measured).
- Published with `taskq/deploy/eks-karpenter/oci-layer-publish.py` on the lab base (`jq -r .image_uri taskq/deploy/eks-karpenter/image.json`, built FROM the AWS-CLI image; its layers were inspected and have bash, sh, python3, awk, xargs, sha256sum, jq, nproc; `aws` is the base entrypoint binary). Tag `ecld-f13a690df016`, user 1000:1000, workdir /work, HOME=/work. Receipt `eks/layer-receipt.json`.
- After the 04:50-05:00 UTC full-disk window: receipts and requests re-checked as valid JSON, ECR digest for each tag re-queried and equal to receipt and request (measured), layers re-read from the registry. R2_READY.md was rewritten (it had been truncated to 0 bytes).

## Results (evidence class)
- Image digest `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:a361afc0755649e32453e3140260bf58ed0a85975d5998c3d30d056bde5bc568` (measured, `aws ecr describe-images` equals receipt and request).
- Binary sha256 (build host, measured): `087cc8e3e3deecf87b186e9a8fd0a903cd79516429e3918ac7abccc54eb6d024`
- `ecld_toy.rs` sha256 `de901e230d2558cb488dee653e8bdfe44ff44b928a611322a014670c5fa8769d` equals the PROTOCOL.md value (measured on the git-archive copy). `ecld_jobs.txt` sha256 `d3bef30ad87ec1148b2503a55be581ee4a4fb60431c4a26283ac82cc99d7a2b5` (no frozen value to compare). PROTOCOL.md sha256 `e8481f7c5df55ad8d91358c0b8519c5c63e3852dc3865f79b4f05a0465d4ac43`.
- Job: 2 vCPU (validator minimum), 2Gi, 21600 s, standard, retries 0, wait_seconds 60. Planned total about 70 single-core minutes (heuristic), so about 5x margin.
- Validator: `eks/r2img_validate_output.txt` (VALID; two negative controls INVALID: cpu 15, placeholder digest). It is the inline jq of `remote/run-command`, as in the pkm study.

## Decisions made (and why)
- ONE job, sequential: PROTOCOL.md section 5 freezes "one thread, no co-running panel jobs on the same pod". Parallelism would shorten wall time but deviates from the protocol; counts are deterministic and the cost is about an hour of Spot.
- Per-line upload and continue-on-failure so finished lines survive preemption; a rerun is a new job name and stale rows are discarded per PROTOCOL.md.

## Open threads
- A preempted job reruns all 64 lines (about an hour); the entry has no skip list. Add one if wanted.
- Director checks P1-P5 of PROTOCOL.md on `panel.jsonl`.
- Binary was not run on EKS; only seconds-long development checks locally.
- research-kb and Conductor are not installed here: no `source_id`s, no task attach. Sources checked: the study README/PROTOCOL.md, remote/run-command, submit-job, oci-layer-publish.py.
- Main session still owes: Conductor, `scripts/checkpoint.sh init`, benchrec if hooks classify this as a benchmark, committing these `eks/` files, director card.

## Errors
Shared disk filled 04:50-05:00 UTC (another agent's build) while this session was writing; only the R2_READY.md files were truncated and are rewritten. No publish step was in that window (published 04:44-04:45).

## Ready-to-launch
- Request: `research/elliptic_code_list_decoding_20261107/eks/r2img_request.json`
- Validator output: `research/elliptic_code_list_decoding_20261107/eks/r2img_validate_output.txt` (re-run: `bash research/elliptic_code_list_decoding_20261107/eks/r2img_validate.sh research/elliptic_code_list_decoding_20261107/eks/r2img_request.json`)
- Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:a361afc0755649e32453e3140260bf58ed0a85975d5998c3d30d056bde5bc568`
- Launch: `ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher AWS_DEFAULT_REGION=us-east-2 ISOLAB_CLIENT_OUTPUT=<fresh abs dir> taskq/deploy/eks-karpenter/remote/launch-remote-command research/elliptic_code_list_decoding_20261107/eks/r2img_request.json`
- Results: `s3://isolab-results-104186811093-us-east-2/runs/<job_name>/<pod-uid>/`; `sha256sum -c SHA256SUMS` after download.

## Shared files edited
None. New files only in `research/elliptic_code_list_decoding_20261107/eks/`: `R2_READY.md`, `r2img_entry.sh`, `r2img_request.json`, `r2img_validate.sh`, `r2img_validate_output.txt`, `layer-receipt.json`.
