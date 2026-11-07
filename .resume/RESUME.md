# Resume: crypto @ claude/serene-davinci-ujoi58

## Goal
User request (2026-11-07): survey the prime-field ECDLP leads, then pursue the assessment's five
priorities in parallel (research/prime_field_lead_assessment_20261107/README.md §4b). This checkpoint
covers the one item that runs now: item 2, the sealed prime medium-tier rho reference bound
(LEAD-RHO-matched-rho-calibration), which every prime-field IC gap is divided by.

## Lead / hypothesis
LEAD-RHO-matched-rho-calibration (crypto-autoresearcher knowledge/leads/ecdlp/generic/). Prediction
registered before data: medium-tier rho.negation about 1.06x the sqrt(pi/4) floor, inside [1.00, 1.10]
(setup_excess); walk_shortfall if >= 1.30 with lower CI > 1.10. PROTOCOL.md §9-§10.

## Spec
research/prime_medium_tier_rho_20261107/: PROTOCOL.md, protocol.json (experiment.protocol/v1, validates
with 0 findings; freeze after the image receipt fills instrument.commit and binary_sha256), curves.json,
seeds.json, specs/J1..J6.json, requests/J1..J6.request.json (validated), finalize_requests.sh, fit.sh,
examples/prime_medium_tier_rho_rfit.rs. Curves registered in docs/curves/registry.json at dbd587a01a.
Publication receipt: check-experiment-git at dbd587a01aefba0634acc459ea30cfb9bbf49a09.

## State
Inputs frozen and published: protocol.json frozen (protocol_sha256 5643e3e7...4cd6) at 529aa8ee4d;
requests/final/J1..J6 bound to image 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:
292bdc3d91ef540bc7719f20902c5acc614dab458676225623275bb3a838cae5 (source dbd587a01a, ecbench sha256
a18af09f...d3b9); publication check at 189b78587b. Benchmark record
benchmarks/records/20261107T2115Z-prime-medium-rho-bound.json initialised. Next: director card, executor
launch of J1..J6 one at a time.

## Next steps
1. Read the image URI (@sha256), source SHA and ecbench sha256 from run 42413370386's receipt.
2. Fill protocol.json arms[].instrument.{commit,binary_sha256}; freeze with
   crypto-autoresearcher tools/experiment_protocol.py freeze --commit auto --filed-before-affected-data.
3. finalize_requests.sh IMAGE_URI SOURCE_SHA ECBENCH_SHA256; commit, push, rerun check-experiment-git.
4. benchrec init --slug prime-medium-rho-bound --problem rho:prime_medium_tier_bound.
5. experiment-director compiles the J1..J6 launch card; executor-mechanical runs it; one job at a time
   (L2 pool holds one c8a.2xlarge); env AWS_REGION=us-east-2,
   ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher.
6. After: verify --replay-all, table/band/compare, fit.sh, export, benchrec add-raw/metric/close.

## Runs in flight
Image build done (run 42413370386, success). No EKS job yet. Jobs will be
named prime-medium-rho-20261107-j1..j6.

## Stop and success conditions
Six jobs, each under 3,600 s; about 3-3.5 c8a.2xlarge instance-hours; proposed ceiling USD 10. Stop on a
failed A/A gate, a failed replay, or a job past its cap; report rather than rerun more than once.
Success: sealed bounds for every arm at w = 0, 0.1, 1 and the R(N) fit, scored by PROTOCOL.md §9.

## Open questions
Whether the default deployed image carries rho.negation_v2 (hence the commit-built image). ecdb returned
HTTP 503 during preparation. Owner decisions pending for item 1 (Mac files, ECR push, spot policy,
ceilings, large hosts).
