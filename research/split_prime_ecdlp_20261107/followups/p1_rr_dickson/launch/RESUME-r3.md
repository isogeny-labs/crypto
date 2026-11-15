# Resume: crypto @ claude/novel-ecdlp-algorithm-iams4j

## Goal
Owner asked (2026-11-08, "Yes run it") to run split_prime P1 cell T24 m5 t4 (RR and T systems,
msolve 0.6.5) to completion on a larger host with about a day's limit; round 2 left it undecided
(RR timeout at 10,800 s, T capped at 13 GiB).

## Lead / hypothesis
Frozen rule 3 of PROTOCOL-R2: does the RR system stay below T on solving degree D and widest
matrix at m = 5 with a Dickson tower? T's lower bounds: D >= 9, widest >= 662,876.

## Spec
research/split_prime_ecdlp_20261107/followups/p1_rr_dickson/PROTOCOL-R3.md; driver
calibration/run_cells_r3.sh; launch/{README.md, job-r3.sh, build_image.sh, make-requests.sh,
image.json, build-info.json, request-*.json}. Source commit ba6c72659b61; image
isolab/tasks@sha256:7f2fa5f7f0b3...

## State
Image published; requests validated. Owner authorized going over the exhausted monthly budget,
ceiling USD 60 for all attempts (launch/README.md).

## Next steps
1. Launch smoke job p1r3-smoke-20261108; verify its S3 output (instance sha check, msolve runs).
2. Launch p1r3-rr-a1-20261108 and p1r3-t-a1-20261108; record receipts in launch/receipts/.
3. Status checks about every 50 min, then every 4 h (status-request.sh + launch-remote-command).
4. When done: aws s3 sync, verify SHA256SUMS and pod UIDs, apply rule 3 with bounds from R2b+R3,
   RESULTS.md section, report PDF, benchrec record, PR.

## Runs in flight
None yet (smoke next).

## Stop and success conditions
Each job stops at completion, cap (VLIMIT) or TLIMIT 86,400 s. Spot loss: at most two
resubmissions per system. Stop launching at USD 60 charged or unknown.

## Open questions
None.
