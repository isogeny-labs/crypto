# Resume: crypto @ claude/elliptic-curves-graph-weaknesses-fw42m4

## Goal
Owner asked (2026-11-08, "Please run it") to run the open n = 37 test: does msolve 0.6.5 (F4,
grevlex) reach maximal step degree 5 on a fresh m = 2 descended S3 instance at n = 37, as
Kousidis-Wiemers report for Magma (corpus MEAS-GB-KW-TABLE1)? Locally it was OOM-killed at 15 GB.

## Lead / hypothesis
Petit-Quisquater Assumption 2 at m = 2 (HEUR-GB-PQ-ASSUMPTION-2). msolve gives 4 on n = 17..34 and
n = 36 (merged in PRs #3658, #3682). Magma reports 5 from n = 37.

## Spec
research/ecdlp_weakness_census_20261104/pq2_rank/f4/eks_n37/{protocol.json, run.sh, input.ms},
committed at e63bf8e55d (publication receipt in the run's receipt directory). Input dump
pq2_rank/f4/step_n36_37/dumps/n37_i0.json.

## State
Image: isolab tasks base (aws-cli) + one layer /opt/msolve (msolve + its shared libs + loader +
run.sh + input.ms), published with taskq/deploy/eks-karpenter/oci-layer-publish.py. Command job
via taskq/deploy/eks-karpenter/remote/launch-remote-command (isolab.eks.command-request/v1),
8 vCPU, 100Gi, 21600 s, retries 0, cpu-spot. Results land in
s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod-uid>/ (wrapper uploads every 10 min).

## Next steps
1. Publish image, write request.json, validate, launch; record Job/Pod UIDs.
2. Read status with an isolab.eks.command-status/v1 request; download S3 outputs, verify SHA256SUMS.
3. Parse max step degree, check linear = k = 33; update pq2_rank README, corpus MEAS-PQ2-MSOLVE-F4,
   session log; PR.

## Runs in flight
(filled at launch)

## Stop and success conditions
One attempt, no retries; stop at exit, OOM or 6 h. Budget ceiling USD 5 (monthly account
headroom was USD 23.67 at submission).

## Open questions
None.
