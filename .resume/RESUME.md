# Resume: ECC2K-130 open-items sweep (2026-11-14)

## Goal
Chase the owner's seven open items for binary/Koblitz (ECC2K-130-type) index calculus in parallel:
L1 Frobenius/negation-closed non-linear factor base; L2 UNSAT refutation cost and pre-solve
filters; L3 cross-instance SAT amortisation; L4 missing baselines (gcd with subspace polynomial,
FES, polynomial method); L5 function-field Coppersmith on S3; L6 Gröbner-free dissection
splitting; L7 end-to-end normalisation against matched rho.

## Lead / hypothesis
Each lane states the residual question left by prior repository work (or CLOSED-BY-PRIOR) and
a pre-registered prediction in its frozen PROTOCOL.md.

## Spec
research/ecc2k130_open7_20261114/COMMON.md (sha256 479269fe…) and each lane's PROTOCOL.md.
Toy scale, local, counted units decide.

## State
Seven subagent lanes dispatched 2026-11-14. Branch claude/ecc-discrete-log-open-3n28i1.

## Next steps
Collect lane SUMMARY.md/RESULTS.md; consolidate README.md, overview SVG and REPORT.pdf in
research/ecc2k130_open7_20261114/; session log; commit, PR with auto-merge.

## Runs in flight
Local processes started by lanes L1..L7 (1 core, <2 GB each); no remote jobs.

## Stop and success conditions
Stop when every lane has a verdict under its frozen rule (or CLOSED-BY-PRIOR) and the
consolidated report is in a PR. Success = any verified, measured improvement over the current
binary IC baseline in counted units; no lane claims an end-to-end speedup from a stage ratio.
