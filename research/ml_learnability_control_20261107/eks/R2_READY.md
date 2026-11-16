# R2_READY: mllc (ml_learnability_control) launchable on EKS (round 2, 2026-11-08, slug r2mllc)

Nothing was launched. All numbers `[pcc:unverified]`. research-kb and Conductor are not installed here (not queried).

## Goal
Make the drafted learnability-control study launchable through `isolab.eks.command-request/v1`.

## What was done
1. Protocol: wrote `../protocol.json` (experiment.protocol/v1) from PROTOCOL.md; `experiment_protocol.py validate` -> `OK (0 findings)`. Dry-run `freeze` on a temp copy succeeded (hash 674d224c...e6e2, discarded). PROTOCOL.md, harness, ecgen, mllc_job.sh are unchanged.
2. Image: base `python:3.13-slim-bookworm` (index sha256:a5a7017e..., amd64 manifest f3dac913..., Python 3.13.18, glibc 2.36) + one layer: numpy 2.5.3, scipy 1.18.1, scikit-learn 1.9.1, joblib 1.6.0, threadpoolctl 3.8.0, awscli 1.46.3 (botocore 1.43.62, s3transfer 0.17.0) from manylinux wheels into the base's site-packages; study sources via `git show f13a690df0:...` in /opt/study/src; `/usr/local/bin/r2mllc_entry.sh`; `/usr/local/bin/aws`.
3. Entry script `r2mllc_entry.sh SIZE...`: same env and harness call as mllc_job.sh per size, sizes in parallel, uploads outputs + SHA256SUMS to `$ISOLAB_RESULTS_URI` every 600 s and at exit, exit code non-zero if any size failed.
4. Four filled requests, validated.

## Results (evidence class)
- Image published, tag `mllc-f13a690df016`, digest `sha256:a43ceee769b62f514862d219dd447fc2112e30a085b13d90485919687a9ecadd` (measured; `image.json`, `image_manifest.json`). Layer blob re-downloaded from ECR and its sha256 equals the manifest digest ed606365... (verified after the disk-full window). Image config: User 1000:1000, WorkingDir /work, HOME=/work, linux/amd64.
- Development check (not evidence): base blobs (sha256 verified) + layer unpacked, `chroot` into it: `python3 -I` imports numpy/scipy/sklearn at the stated versions, `aws --version` works, harness `--sizes 12 --dev` ran ~100 s and printed accuracies (killed by timeout, no output kept). Risk left: the entry's S3 upload path and awscli credential discovery (pod identity/IRSA) were not exercised (no S3 write allowed); the pkm image used the aws-cli base with the same pod environment. The wheels' RECORD hashes all matched (12158 files).
- Request validator: `validate_output.txt` (4 VALID, negative control INVALID).

## Protocol status and changes
Freezable with changes confined to the study directory: only a new `protocol.json` was needed (the draft had no machine record). Choices written into it: A/A = within-size curve replicates (aa-curve-0/1); positive control PC-A (gate) and PC-B c=1; negative control = relabelling null; seeds = harness frozen rule (`harness_drawn`, master 20261107+1000m+i); arm order fixed with reason; dev-seen m=12 predictions marked calibration/in-sample; execution class `training`, no checkpoint, rerun a lost size whole. `variable_ledger.crosses_map: false` was required by the checker. Analysis script = `mllc_harness.py` (sha256 b01eeb62...bd4e; decisions computed in the harness).

## Decisions made (and why)
- python 3.13 base to match the dev versions exactly; installed into site-packages (not --target + PYTHONPATH) because the job script uses `python3 -I`, which ignores PYTHONPATH.
- Own entry script, mllc_job.sh unused: it runs all four sizes, does not upload, and ignores child exit codes.
- cpu 4 per size job (harness uses 2 BLAS threads per process), 16Gi, per-size jobs because Spot loss kills non-resumable jobs; `mllc-all` (cpu 8, 14400 s) is the single-job alternative: launch either the three per-size requests or `mllc-all`, not both.
- seconds: 7200 for sizes 12+16, 14400 for 20 and 24 (estimate about 2 h each, derived from a loaded-host probe; margin 2x, not measured on EKS).

## Open threads
- The real freeze (`freeze protocol.json --commit auto --filed-before-affected-data`) rewrites `freeze`; the requests' `protocol_sha256` (pre-freeze canonical hash) should be rechecked afterwards. `r2mllc_entry.sh` and `eks/` must be committed; the image contains the entry copy with sha256 93879b32...47a8b (the build-info.json records it) and the committed file must be byte-identical.
- Resume checkpoint, benchmark record decision, research-visuals, pcc claim ids: main session.
- hgb on PC-B in the dev run reads below 0.5 at some cells (e.g. 0.40); not investigated.
- Time estimate for m=24 could be low if the 7e5-sample MLP is slower on Spot nodes; seconds margin is the only protection.

## Errors
- Shared disk hit 0 bytes (another agent) during my first pip install; that tree was deleted and the install redone; wheel RECORD hashes and the published layer blob were verified afterwards.
- pip `--target` skipped botocore/s3transfer (taken as already installed from the system); installed explicitly with --no-deps, s3transfer version guessed (0.17.0 worked with awscli).

## Ready-to-launch
- Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:a43ceee769b62f514862d219dd447fc2112e30a085b13d90485919687a9ecadd`
- Requests: `eks/requests/mllc-m12-m16-20261108.json`, `mllc-m20-20261108.json`, `mllc-m24-20261108.json`, `mllc-all-20261108.json`
- Validator: `eks/validate_command_request.sh eks/requests/*.json` (output `eks/validate_output.txt`)
- Freeze: `python3 ../crypto-autoresearcher/tools/experiment_protocol.py freeze research/ml_learnability_control_20261107/protocol.json --commit auto --filed-before-affected-data` (main session, after committing)
- Launch: through the isolab-eks skill's `launch-remote-command` with a request above (experiment-director/executor tiers); results land under `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod uid>/` with SHA256SUMS.

## Shared files edited
None. New files only: `protocol.json`, `eks/*`.
