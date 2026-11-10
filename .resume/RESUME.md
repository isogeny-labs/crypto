# Resume: crypto @ claude/tm1514-recheck-20261106 (tm1514 recheck, k = 24 Curve25519-family)

## Goal
Re-measure inc (ic.pipeline wwalk x=0.9, the ledger incumbent wwalkGg-x09-sr-n2) against rho-neg on the five k = 24 Curve25519-family curves, to settle whether tm1514's earlier inc/rho = 1.52 at w = 0 (0.95 at w = 1; research/curve25519_autolab_20261104/multicurve/band-k24.txt, n = 24) is real or a small-sample draw, and measure the price of each arm's uncounted native work.

## Lead / hypothesis
- H: the 1.52 is a draw; tm1514 sits within the sibling ratio (about 1.75-2.1), and the measured native price is well below 1 GAE, so the w = 1 flip does not survive measured pricing.
- Confirm/kill: the frozen decision_rule in research/tm1514_recheck_20261106/PROTOCOL.json (C1 D interval at 98.33 %, C1b, C2 positive control, C3 prices, C4).

## Spec
- Protocol: research/tm1514_recheck_20261106/PROTOCOL.json v2 + PROTOCOL-AMENDMENT-1.md, frozen 2026-11-10T17:17:31Z, protocol_sha256 d348123a…81d6 (v1: 8cd5f856…, 2026-11-06). Narrative PROTOCOL.md; launch plan PLAN.md.
- Specs: spec-tm1514.json (replicate A, seed 960559345608509) and spec-tm1514-B.json (replicate B, seed 4971825529176011); target_seed 1406057239440944; 96 targets/curve, 6 rounds + 1 warmup, L2, 5 arms (rho-neg, rho-neg-aa, inc, inc-aa, rho-plain).
- Analysis: examples/tm1514_analyze.rs, sha256 59b83670...3b6f2 (pinned in the protocol).

## State
- 2026-11-10: AMENDMENT-1 frozen (PROTOCOL.json v2, protocol_sha256 d348123a…81d6, commit ff23963210): counted-only, local L0 sessions, replicates A3/B3 (specs spec-tm1514-A3/B3.json). Replicate A (EKS run 42269476702, exit 23) is a dry run; A2/B2 withdrawn.
- Binaries: scratchpad tm1514/bin (ecbench b8570e7f…, tm1514_analyze d0412039…).
- Owner approved the run and a USD 5 ceiling (2026-11-06).
- Nothing launched at checkpoint time.

## Next steps
1. Run A3 and B3 locally (binaries in the session scratchpad tm1514/bin, sha256 in its SHA256SUMS) into research/tm1514_recheck_20261106/sessions/A3 and sessions/B3, each under `tools/benchrec.py run`.
2. `ecbench verify --dir sessions/<X> --replay-all` for both; every replay must reproduce.
3. `tm1514_analyze --a sessions/A3 --b sessions/B3 --out analysis` (frozen analysis, sha256 6534cfe1…f197).
4. Push records.jsonl/plan.json to the artifact store (tools/artifacts), report/graph/PDF, benchmarks entry, benchrec close, PR; update treatise §12.7.

## Runs in flight
- None at checkpoint time.

## Stop and success conditions
- Budget: USD 5 ceiling for both replicates and any failed attempt; stop if the remainder is unknown.
- Positive control C2 must pass or the study is invalid.
- L2 admission, replay-all audit and A/A admission must pass; never drop failed records.

## Open questions
- Results retrieval without AWS access (see Next steps 3).
- Whether the c8a L2 node exposes a PMU (cycles); recorded per row if present.
