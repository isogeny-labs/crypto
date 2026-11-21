# Validator report: j=0 symmetric 3-summand holdout (independent validator agent, 2026-11-11)

Disposition: **validated as a measurement, with qualifiers**; no flaw voids the claim; the headline range "1.9x-2.9x" is overstated at the top; not eligible for the leaderboard. Built from `b7192dc025` (only whitespace differs from the data commit `5ecde54676`; `src/` and `Cargo.*` unchanged). 4 vCPU VM, load ~0.2-0.5.

## Freeze before data
Verified locally: freeze commit `de2f4a328b` is the parent of `5ecde54676`; `hold_*` mtimes follow the freeze JSON; prediction (>=1.5x) and abandon rule (<1.2x) are in the frozen file. Not verified: remote saw the freeze first. Gap: the freeze commit pins no source/binary/command hash (`sym_solve(summands=3)` was first committed after the data). Mitigation: the committed code reproduces the deterministic counts (`sym_targets`, `sym_rels`, `sym_inv`) exactly in 54/54 tuning rows and 408/408 holdout row pairs.

## Code reading
Relations are group-verified (2-term `want==r`, 3-term `acc==r`); elimination sound, final `kG==Q` inside the timer; same curve, target, extra=8, seed, 1 thread for both arms; timer scope comparable (curve/lambda/beta setup outside the sym timer costs ~2.6 ms at 26 bit incl. point-order search, beta_of 0.0015 ms: negligible). `sym_solve` never sees k. Observation: only ~33% of the orbit pair-table entries have stored x equal to the key; the rest hit but always fail the group check, so the measured gain is conservative and "3B^2 entries cover an equal x-set" overstates effective coverage.

## Independent certificates and reruns
Pure-Python checker (no shared code): 490/490 certificates pass (curve, p=1 mod 3, n prime, Hasse, nG=O, lambda/beta, planted k, both arms' x). 540/540 solves recovered the planted log.
Ratios plain3/sym, same run, original binary: 26b c0.5 2.78/2.38, c1.0 2.62/2.51; 28b c0.5 2.15/2.01, c1.0 2.53/2.69. Wider grid (12 targets/curve, pooled): 26b 2.07-2.50, 28b 2.10-2.64, 30b 2.08-2.68; lowest per-curve cell 1.98. Fresh models CSKIP=5 (b50,b60, 26b): 2.27-2.82 vs c in {0.5,0.7,1.0}. Different primes (CSKIP=1000; p=67108729 at 26b, p=268434949 at 28b): 2.27-2.60 and 2.09-2.11.

## Findings (none voids the claim)
1. The printed "1.9x-2.9x" mixes pairings (sym3 column from the c=0.5 run, ratio from the c=1.0 run); identical-work sym medians differ 2%-41% between runs. Better wording: **about 2.0x-2.4x faster than the best 3-summand `ic_solve` arm at 26-30 bits on j=0 prime-order curves** (all 72 rows: median 2.35, 10th-90th pct 1.84-2.84, min 1.64, sym never slower).
2. The baseline c grid missed its optimum (~0.7; at 26b c=0.8 gave 19.1 ms vs 23.1 ms at c=0.5): best-arm ratio overstated by up to ~16%; vs c=0.7 it is ~2.1x at 28 and 30 bits. Sym B was tuned only at 26 bit.
3. Holdout curves at one bit size share p and n and differ by sixth-power twists: "two curves" is one group; fresh-prime reruns above address this.
4. Rows lack commit, binary hash, argv, env (`C3`,`CSKIP`,`NOPLAIN2`), host; no bench.record, pcc id or `VALIDATION.json`.
5. Wall clock, no isolation, no A/A interval; 41% rerun disagreement is the noise scale.
6. Reporting: wall ms only (no S column); gap to negation-rho 2.8-3.8x (26b), 7-9x (28b), 16-23x (30b), growing with size, consistent with engineering, not an exponent change; no §5a band.

Gate: cannot enter `docs/ic/leaderboard.json` or `benchmarks/README.md` (no replay bundle, matched rho with §5a band, `VALIDATION.json`, isolated L2 run, bench record).
