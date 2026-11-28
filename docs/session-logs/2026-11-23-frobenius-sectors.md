# 2026-11-23 — Frobenius-sector Semaev decomposition follow-up

Task branch: automation/semaev-frobenius-sectors-20261123; draft PR #5044.
Previous Semaev-only PR #5023 was merged at 2026-11-23 17:26:55 UTC; this is a separate follow-up, not maintenance of a merged PR.

## What was done (commands, files, features)
- Read AGENTS.md, docs/ACTIVE.md, previous merged PR #5023, and summation_split/{FINDINGS.md,core.py,chain.py,solver.py,field.py,projective.py,linear.py,symmetric.py,harness.py}.
- Research-KB MCP unavailable in this runtime; used GitHub repository prior work and published sources instead. Sage/FGb/msolve/Singular not installed; SymPy fallback.
- Created summation_split/sector.py implementing Frobenius +/- sector point arithmetic over Fp plus twist, including O and all rational 2-torsion matches.
- Created summation_split/sector_polynomial.py showing degree-three leaf selector constraints and degree-two guarded addition/doubling equations.
- Added test_sector.py, sector_experiment.py, sector_plot.py, SECTOR_FINDINGS.md, sector_gate.csv and sector_bench_medians.csv.
- Local command PYTHONPATH=/mnt/data python -m unittest -q summation_split.test_sector passed five tests; 700/700 seeded exact root-set gates passed on 14 cells, including six cells over p=2^31-1 and m=3,4,5; four exhaustive toy cells passed 16,440 tuples. Local harness command python -m summation_split.sector_experiment --out /mnt/data/summation_split --samples 50 --repeat 3 generated raw per-sample measurements. Repository test/rerun and independent validation remain pending.
- Discovered minimal incorrect O-only split over F5, E:y²=x³+1, (x1,x2,x3,X)=(0,1,2,3), missing rational 2-torsion (4,0).

## Decisions made (and why)
- Do not label set-theoretic Frobenius-sector reduction a new polynomial identity or elimination-ideal equality.
- Do not claim an F4 speedup: only exact specialized X-root extraction was timed; Macaulay solving degree and first fall remain unmeasured.
- Keep the follow-up PR draft until a globally chart-complete system, paired unspecialized GB controls and independent validation are available.
- Restrict all work to Semaev relation; no factor-base map, ECDLP solver or Pollard rho experiment.

## Open threads (unfinished, pick up next time)
- Implement a complete disjoint chart cover for O, equal-x, inversion, tangent, shared rational 2-torsion and finite-field membership, with auxiliary relations total degree <=4.
- Choose a common unspecialized slicing protocol for monolithic S_(m+1), projective S3 chain, symmetric, linear, Frobenius sectors and a balanced tree. Verify >=50 exact projected sets before each cost comparison.
- Run actual Sage/Singular F4/MSolve/FGb on two or more (p,m) regimes with complete chart overhead, peak memory and maximum Macaulay degree. Search for genuine first-fall stabilization; report falsifiers.
- Independently replay the producer-only Python checks and perform PCC/CI gate.

## Errors (what went wrong, how it was handled)
- The repo's shared docs/session-logs/2026-11-23.md is >2 MiB and simultaneously edited. A per-task dated log was used instead of overwriting its full contents via GitHub contents API.
- A local affine-chart test initially attempted to take a square root on points outside a particular twist. Restricted enumeration to points with correct quadratic character; then five tests passed.
- A 50-case-per-large-cell timing attempt was too expensive because it repeated polynomial factorization inside every timing repetition. Revised to gate 50 first, time only 2 large-field / 10 tiny-field matched examples with cold-setup qualification; no GB speedup claimed.
- Research-KB server and external F4 backends unavailable, documented in findings.

Sources reviewed: Semaev (2004) https://eprint.iacr.org/2004/031 ; Kousidis-Wiemers (2019) https://arxiv.org/abs/1906.05594 ; Kosters-Yeo (2015) https://arxiv.org/abs/1503.08001 ; repository research/pdp_split_dreg_20261011/README.md and research/notes/index-calculus/RESEARCH_SYMMETRIZED_SEMAEV.md.
