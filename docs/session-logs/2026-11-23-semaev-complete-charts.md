# 2026-11-23 — Semaev Frobenius complete-chart follow-up

PR: https://github.com/isogeny-labs/crypto/pull/5058
Branch: automation/semaev-complete-charts-20261123
Predecessor: PR #5044 **merged at 2026-11-23 18:49:39 UTC** during this session; its branch was deleted, so new work is on a separate PR, not maintenance of a merged branch.

## What was done (commands, files, features)
- Read AGENTS, current docs/ACTIVE.md task, prior dated task log, core Semaev resultant implementation, sector extractor, and merged findings. Research-KB MCP and local wrapper unavailable. Checked Renes–Costello–Batina 2016 complete addition literature as prior art.
- New summation_split/sector_complete.py: guarded five-case point addition (left/right identity, chord, tangent, inverse) for both Frobenius sectors; shared two-torsion join; per-leaf quadratic Frobenius field lifts for x^p=x and y^p=y. All input x variables unspecialized; total generator degree <=4. No factor-base map touched.
- New summation_split/test_sector_complete.py: exhaustive 514 ordered point pairs across three primes/base+twist; four 50-random small-field cells with every-X resultant root-set comparison and 228 full positive polynomial certificates; six 50-constructed mixed-sector Prime31 positive cells with resultant evaluation, including nonzero 2-torsion.
- Local Python/SymPy: PYTHONPATH=/mnt/data python -m unittest -q summation_split.test_sector_complete => **3 tests passed in 4.625 s** using local standalone *mirrors* of sector.py/core.py matching relevant interfaces; repository-native and independent CI replays remain unverified. Sage, Singular, FGb, msolve not installed on host.
- Documented full derivation, counts, proof obligations and negative controls in summation_split/SECTOR_COMPLETE.md.
- Exact SymPy degree/count diagnostics, Prime31: m=3 => 568 variables/800 equations; m=4 =>715/1020; m=5 =>862/1240; max generator degree 4.

## Decisions made (and why)
- Remain [pcc:unverified]. Producer exact witnesses plus mathematical chart argument support Fp point-set completeness, **not** signed independent validation, a computed elimination ideal equality, F4 degree, or speedup.
- Enforce Fp membership on leaf x/y via quadratic addition-chain gadgets rather than high-degree direct field equations. Inactive inverse/slope variables may be arbitrary since they cannot change a rational output.
- Keep PR #5058 draft until matched unspecialized F4/FGb/MSolve, independent validation and review. No merge.
- Never use generator degree as a proxy for first-fall or Macaulay/F4 solving degree.

## Open threads (unfinished, pick up next time)
- Independent exact-source CI replay of test_sector_complete.py and negative controls; PCC validation.
- Genuine same-slicing unspecialized Gröbner comparison with monolithic Semaev, projective S3 chaining, symmetric and invertible-linear controls after 50 exact projected-set gates per parameter; record actual d_F4, first fall, time, peak memory, and chart overhead.
- Pursue compact complete addition laws to reduce 5 selector overhead, checking curve/twist order exceptions.
- Reconcile existing ACTIVE #5044 line with its merged state.

## Errors (what went wrong, how it was handled)
- First write to #5044 failed 404 because it was concurrently merged and branch deleted; re-fetched live status, stopped maintenance, created separate branch and PR #5058.
- First local Prime31 test timed out because temporary test-only sector.py mirror enumerated square roots; replaced it by exact p congruent 3 mod 4 square root. Checked-in sector.py already uses Tonelli-Shanks.
- Naive SymPy degree counter with every global variable became too slow; replaced with exact per-equation total_degree. Mathematical tests unaffected.

Sources reviewed: Semaev 2004 https://eprint.iacr.org/2004/031 ; Renes–Costello–Batina 2016 https://eprint.iacr.org/2015/1060 ; Kosters–Yeo 2015 https://arxiv.org/abs/1503.08001 ; merged PRs #5023 and #5044. KB unavailable.
