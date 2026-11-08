# Round-2 image and request (r2img, 2026-11-08): split_graph_secant_cost_20261107

All numbers and claims are `[pcc:unverified]`. Nothing was launched.

## Goal
Digest-pinned image for the frozen sgsecant panel plus a validated command request.

## What was done
- Built static musl `sgsecant` from the archived `native/` (`cargo build --release --offline --target x86_64-unknown-linux-musl`, cargo 1.97.0). `eks/build_and_publish.sh` was not used verbatim: it passes `--locked` and `native/` has no committed Cargo.lock (no dependencies), so cargo would refuse. Same layer layout (/opt/sgsecant/sgsecant).
- Filled `eks/request.template.json` with jq into `eks/r2img_request.json`; the command string is unchanged from the template. Development check: `sgsecant panel --describe` printed the frozen config.
- Source revision `f13a690df016d0a457bd40579fbb9d054f486adf` (git archive, not the working tree). Binary is static (`readelf -l`: no INTERP, measured).
- Published with `taskq/deploy/eks-karpenter/oci-layer-publish.py` on the lab base (`jq -r .image_uri taskq/deploy/eks-karpenter/image.json`, built FROM the AWS-CLI image; its layers were inspected and have bash, sh, python3, awk, xargs, sha256sum, jq, nproc; `aws` is the base entrypoint binary). Tag `sgsecant-f13a690df016`, user 1000:1000, workdir /work, HOME=/work. Receipt `eks/layer-receipt.json`.
- After the 04:50-05:00 UTC full-disk window: receipts and requests re-checked as valid JSON, ECR digest for each tag re-queried and equal to receipt and request (measured), layers re-read from the registry. R2_READY.md was rewritten (it had been truncated to 0 bytes).

## Results (evidence class)
- Image digest `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:d1ab2c5c44bc83931d5d402e4ac58f58f5ea92af19d951b1441cc32068c2f07b` (measured, `aws ecr describe-images` equals receipt and request).
- Binary sha256 (build host, measured): `dc7c013bc2ac19aecc69f6cd5c9a253563ef02e49d72c5590b0d9e951ef88ee9`
- `native/SHA256SUMS` all OK; PROTOCOL.md sha256 `20acbf94d87b87f47fa20b4866c79143e9df6154e64cc1a07de9e56e10a84ed1` matches `eks/SHA256SUMS`; template matches recorded 7baabdb1... (all measured).
- Job: 4 vCPU, 8Gi, 3600 s, standard, retries 0, wait_seconds 60. Planned 5-15 min (heuristic, README).
- Validator: `eks/r2img_validate_output.txt` (VALID; two negative controls INVALID: cpu 15, placeholder digest). It is the inline jq of `remote/run-command`, as in the pkm study.

## Decisions made (and why)
- Template command and resources kept unchanged (frozen with the study). Upload is the template's `aws s3 cp --recursive $ISOLAB_OUTPUT_DIR $ISOLAB_RESULTS_URI` after writing SHA256SUMS. Weakness: with `set -eu` a failed panel exits before any upload, leaving only pod/launcher logs.

## Open threads
- After the run: download, `sha256sum -c SHA256SUMS`, `python3 -I research/split_graph_secant_cost_20261107/sgsecant_recheck.py out/panel`.
- Spot interruption means a full rerun under a new job name (README).
- Binary was not run on EKS; only seconds-long development checks locally.
- research-kb and Conductor are not installed here: no `source_id`s, no task attach. Sources checked: the study README/PROTOCOL.md, remote/run-command, submit-job, oci-layer-publish.py.
- Main session still owes: Conductor, `scripts/checkpoint.sh init`, benchrec if hooks classify this as a benchmark, committing these `eks/` files, director card.

## Errors
Shared disk filled 04:50-05:00 UTC (another agent's build) while this session was writing; only the R2_READY.md files were truncated and are rewritten. No publish step was in that window (published 04:44-04:45).

## Ready-to-launch
- Request: `research/split_graph_secant_cost_20261107/eks/r2img_request.json`
- Validator output: `research/split_graph_secant_cost_20261107/eks/r2img_validate_output.txt` (re-run: `bash research/split_graph_secant_cost_20261107/eks/r2img_validate.sh research/split_graph_secant_cost_20261107/eks/r2img_request.json`)
- Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:d1ab2c5c44bc83931d5d402e4ac58f58f5ea92af19d951b1441cc32068c2f07b`
- Launch: `ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher AWS_DEFAULT_REGION=us-east-2 ISOLAB_CLIENT_OUTPUT=<fresh abs dir> taskq/deploy/eks-karpenter/remote/launch-remote-command research/split_graph_secant_cost_20261107/eks/r2img_request.json`
- Results: `s3://isolab-results-104186811093-us-east-2/runs/<job_name>/<pod-uid>/`; `sha256sum -c SHA256SUMS` after download.

## Shared files edited
None. New files only in `research/split_graph_secant_cost_20261107/eks/`: `R2_READY.md`, `r2img_request.json`, `r2img_validate.sh`, `r2img_validate_output.txt`, `layer-receipt.json`.
