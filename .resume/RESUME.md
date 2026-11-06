# Resume: crypto @ claude/tm1514-recheck-20261106 (tm1514 recheck, k = 24 Curve25519-family)

## Goal
Re-measure inc (ic.pipeline wwalk x=0.9, the ledger incumbent wwalkGg-x09-sr-n2) against rho-neg on the five k = 24 Curve25519-family curves, to settle whether tm1514's earlier inc/rho = 1.52 at w = 0 (0.95 at w = 1; research/curve25519_autolab_20261104/multicurve/band-k24.txt, n = 24) is real or a small-sample draw, and measure the price of each arm's uncounted native work.

## Lead / hypothesis
- H: the 1.52 is a draw; tm1514 sits within the sibling ratio (about 1.75-2.1), and the measured native price is well below 1 GAE, so the w = 1 flip does not survive measured pricing.
- Confirm/kill: the frozen decision_rule in research/tm1514_recheck_20261106/PROTOCOL.json (C1 D interval at 98.33 %, C1b, C2 positive control, C3 prices, C4).

## Spec
- Protocol: research/tm1514_recheck_20261106/PROTOCOL.json, frozen 2026-11-06T17:52:16Z, protocol_sha256 8cd5f856...9d645, base 363306c646. Narrative PROTOCOL.md; launch plan PLAN.md.
- Specs: spec-tm1514.json (replicate A, seed 960559345608509) and spec-tm1514-B.json (replicate B, seed 4971825529176011); target_seed 1406057239440944; 96 targets/curve, 6 rounds + 1 warmup, L2, 5 arms (rho-neg, rho-neg-aa, inc, inc-aa, rho-plain).
- Analysis: examples/tm1514_analyze.rs, sha256 59b83670...3b6f2 (pinned in the protocol).

## State
- Owner approved the run and a USD 5 ceiling (2026-11-06).
- Nothing launched at checkpoint time.

## Next steps
1. Dispatch isolab-experiment.yml on main: source_revision = this branch's commit, spec_path = research/tm1514_recheck_20261106/spec-tm1514.json, instance_type c8a.2xlarge, seconds 3600, resumable false. Record run id and time.
2. When A is terminal, dispatch B with spec-tm1514-B.json. Never auto-retry an ambiguous submission; reconcile against existing runs first.
3. Results: the workflow verifies S3 receipts; this session has no AWS read. Someone with lab S3 access syncs runs/<job>/ into sessions/A and sessions/B; then run tm1514_analyze, verify PLAN.md §6 receipts, write report/graph/PDF, benchmarks entry, benchrec close, PR.

## Runs in flight
- None at checkpoint time.

## Stop and success conditions
- Budget: USD 5 ceiling for both replicates and any failed attempt; stop if the remainder is unknown.
- Positive control C2 must pass or the study is invalid.
- L2 admission, replay-all audit and A/A admission must pass; never drop failed records.

## Open questions
- Results retrieval without AWS access (see Next steps 3).
- Whether the c8a L2 node exposes a PMU (cycles); recorded per row if present.
