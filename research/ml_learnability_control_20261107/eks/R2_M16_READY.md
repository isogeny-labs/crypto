# R2_M16_READY: mllc size 16 rerun with the PC-D cap (round 2, 2026-11-08, slug r2m16)

Nothing was launched. All numbers `[pcc:unverified]`. research-kb and Conductor are not installed here (not queried). Sources checked: the study files, `eks/R2_READY.md`, `taskq/deploy/eks-karpenter/oci-layer-publish.py`.

## Goal
Make a one-size rerun of m=16 launchable after the PC-D crash of `mllc-all-20261108`, with the minimal fix filed as a protocol amendment.

## What was done
1. Read the harness: the crash is `fpstar_arm` drawing `n = N_curve-1` from a population of `N_f = p-1` (derived; confirmed locally on curve 0: N_curve 63493, N_f 58402). Confirmed rng order: all curve/PC-A/PC-B/null draws precede `fpstar_arm`; PC-D `run_cell` uses null_R 0 and draws nothing; the rng is reseeded per curve.
2. `../AMENDMENT_R2_M16.md` (sha256 1f18ffbc...266a9), `../mllc_harness_r2m16.py` (one-line cap `n = min(n, N-1)`), `../r2m16_harness.diff`.
3. `r2m16_entry.sh`: derived from `r2mllc_entry.sh`; size 16 only, runs `/opt/study/src_m16/mllc_harness_r2m16.py`, same env and upload logic, records source hashes of both harnesses.
4. Image: base = the a43ceee7 image itself (its five layers unchanged) plus one 8 KB layer: `/opt/study/src_m16/{mllc_harness_r2m16.py,mllc_ecgen.py}`, `/usr/local/bin/r2m16_entry.sh`, `/opt/study/build-info-m16.json`. Published with `oci-layer-publish.py`; receipt `image_m16.json`.
5. Request `requests/mllc-m16-20261108.json` (cpu 4, 16Gi, 14400 s), validated.

## Results (evidence class)
- Image digest `sha256:37c6a451e2917d2a2650705719eb9b83daaca6720d8702b9cfae4a00f38eed21`, tag `mllc-m16fix-5f72e0806c` (measured; ECR describe-images matched the manifest digest inside the publisher).
- Development check (not evidence, seconds): the new layer blob was downloaded from ECR, its files' sha256 match the sources, and `fpstar_arm` from the unpacked copy ran for m=16 curve 0 without error (N_f 58402, n 58401). Locally, the committed harness raises on curve 0 and runs on curves 1, 2; the fixed one runs on all three, with curves 1, 2 unchanged (n 58108, 61338). The full size and the entry's S3 upload were not run.
- Validator: `validate_output_m16.txt` -> VALID.

## Decisions made (and why)
- Layered on the published image rather than rebuilding the wheel layer: guarantees identical base and library layers; the original `/opt/study/src` stays in the image unmodified for the record.
- Separate entry script and source dir rather than overwriting the old harness path, so the committed harness is untouched and the run states which harness it used.
- cpu 4 (one process, 2 BLAS threads), seconds 14400 (the user-suggested queue margin; no EKS timing measured for m=16).

## Open threads
- Main session: commit the new files (`AMENDMENT_R2_M16.md`, `mllc_harness_r2m16.py`, `r2m16_harness.diff`, `eks/r2m16_entry.sh`, `eks/image_m16.json`, `eks/requests/mllc-m16-20261108.json`, `eks/validate_output_m16.txt`, this file) byte-identical to the image copies (hashes in the amendment and the request note); record the amendment in `protocol.json` `amendments` if the freeze tooling wants it (not done: frozen file); resume checkpoint, benchmark record, pcc claim ids.
- The mllc-all job's size 12/20/24 outputs and the rerun's `ladder_m16.json` must be merged by the analysis step; the mllc-all request and this one both write under separate job names.
- The git HEAD at build was 5f72e0806c (the base image was built at f13a690df0); the tag uses HEAD's short sha as requested.

## Errors
- First `rm -rf "$(cat ...)"` of my temp dir was refused by the safety check; removed with the literal path afterwards.
- `source.sha256` line in my first entry-script edit would only have logged half the hashes; fixed before building.

## Ready-to-launch
- Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:37c6a451e2917d2a2650705719eb9b83daaca6720d8702b9cfae4a00f38eed21`
- Request: `research/ml_learnability_control_20261107/eks/requests/mllc-m16-20261108.json`
- Validator: `eks/validate_command_request.sh eks/requests/mllc-m16-20261108.json` (VALID)
- Launch: through the isolab-eks skill's `launch-remote-command` with that request (experiment-director/executor tiers), as for the R2 cards.

## Shared files edited
None. New files only (listed above); `mllc_harness.py`, `protocol.json`, PROTOCOL.md untouched.
