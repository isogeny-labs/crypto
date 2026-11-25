# 2026-11-23 — Semaev first-fall artifact

Task branch automation/semaev-firstfall-artifact-20261123, draft PR #5063. Predecessor #5058 merged 2026-11-23 18:56:59 UTC; do not maintain merged branch.

## What was done
- Read current AGENTS, prior firstfall notes in summation_split/SECTOR_COMPLETE.md and predecessor merged PRs #5023/#5044/#5058. Research-KB not available in connector-only runtime; original papers Semaev 2004 and Kosters-Yeo 2015 were consulted in earlier continuation.
- Algebraic identity at each leaf: (plus_x-(1-s)x)+(minus_x-s*x) = plus_x+minus_x-x. Both genuine source generators degree 2, result nonzero degree 1. Same for y. Holds for all m and optional field lifts.
- Committed summation_split/test_sector_firstfall.py, FIRSTFALL_ARTIFACT.md and updated dedicated Semaev GitHub workflow to run both circuit and artifact tests.
- Producer-only Python/SymPy mirror evaluated 8 parameter/field-lift cells, all leaf coordinate identities; 8/8 passed. Exact source CI remains to be checked.
- Source-pinned GitHub CI for merged #5058: workflow Semaev complete-sector polynomial gates run 45243732758/job137934898518 completed SUCCESS. Other repository-global Workflow syntax job failed on unrelated ct330-f103-rho-bsgs-paired.yml at line 99; not from our circuit workflow.

## Decisions made
- Claim only a presentation-dependent degree-two first fall **upper bound**: it is a trivial redundancy and supplies no bound for F4/Macaulay solving degree or GB speedup. Mark [pcc:unverified].
- Stay solely on Semaev constraints, no factor-base L.
- No paid compute and no unspecialized F4 claim.

## Open threads
- Re-run exact source tests in newly triggered PR #5063 workflow; independent replay/PCC pending.
- Redundancy-reduced chart circuit, matched F4 comparison against monolithic, projective chain, symmetric/linear with >=50 exact projected-set gates per parameter.

## Errors
- None in this lane; previously predecessor #5058 auto-merged during its run.
