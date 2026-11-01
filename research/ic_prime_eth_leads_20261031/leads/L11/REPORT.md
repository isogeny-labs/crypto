# L11 report: multiplier encodings for pure CDCL on the m = 3 prime-field PDP

Status: partial (b = 6, 9 fully solved; b = 12, 15 cap-saturated). Verdict: constant_only. Label key: [M] measured (files under res/, raw/), [D] derived from those files (analysis.md, analyze.py), [P] plan.

## Encodings (patch: patch_cnf.py applied to a copy of the sweep crate; env MULMODE)
- school: sweep default, quotient-witness mulmod a*b = q*p + r, ripple adders, schoolbook partial products.
- kara: recursive Karatsuba on every non-constant n x n product, leaf 6 bits (schoolbook); middle term via fresh witness z1 with z1 + z0 + z2 = (x0+x1)(y0+y1). Constant-operand products stay schoolbook. Note: at b <= 6 no product exceeds the leaf, so kara == school exactly (identical conflicts) [M].
- pc: every full adder (5 vars) and half adder (4 vars) is encoded by ALL its prime implicates (computed by brute force at start-up), so unit propagation is complete on each adder (GAC per adder); partial products are AND gates compressed by a column tree (full/half adders, carry to next column). This is completeness per adder, NOT per whole partial-product column or whole multiplier (that is the limit of this construction; POS26 itself was not consulted).
- kpc: Karatsuba with pc leaves and pc adders.
- Fourier gadget (iv): omitted (not cheap).

## Instances [M]
bits-grouplaw encoding, small-x factor base, sym = 1, kissat 4.0.4 default, seed 20261031 (field primes p = 1 mod 4 from find_prime; same curve and same targets in every mode because the RNG does not depend on MULMODE). Diagonal p ~ 2^(3 s), l = s: b = 6 (s=2), 9 (3), 12 (4), 15 (5). 15 planted + 15 uniform per cell at b = 6, 9, 12; 5 + 5 at b = 15 (time). Conflict caps (KISSAT_CONFLICTS): 200000 (b=6, 9; never reached), 300000 (b=12), 150000 (b=15). Enumeration arm: T_enum = 4*C(|xs|,2) from the sweep's oracle3, on the same instances. Every SAT answer verified by exact arithmetic, UNSAT checked against the oracle: 0 FAIL verdicts in all 16 result dirs [M].

## Commands
- build: cd sweeper && cargo build --release
- selftest per mode: MULMODE=<mode> sweeper/target/release/prime_pdp_sat_sweep selftest --p 97 --l 3 --m 2 --targets 4 (32 runs, 12 SAT, 0 mismatches, all 4 modes) and --m 3 --targets 3 (30 runs, 4 SAT, 0 mismatches, all 4 modes) [M: raw/selftest-*.txt]. Caveat: p = 97 is 7 bits, so Karatsuba recursion is barely exercised in the selftest.
- cell: MULMODE=<mode> KISSAT_CONFLICTS=<cap> sweeper/target/release/prime_pdp_sat_sweep sweep --m 3 --bits <b> --two-adicity 2 --families small-x --encodings bits-grouplaw --ls <s> --planted 15 --uniform 15 --timeout 60 --out res/b<bb>-<mode>
- analysis: python3 analyze.py > analysis.md (full tables for uniform/planted/all, bootstrap 2000).

## Results (all targets, median per cell) [M/D; full tables incl. decisions, propagations, vars, clauses in analysis.md]
| b (s) | mode | n | solved | censored at cap | median conflicts | median propagations (M) | median nvars | T_enum |
|---|---|---|---|---|---|---|---|---|
| 6 (2) | school/kara | 30 | 30 | 0 | 4429 | 1.1 | 7178 | 12 |
| 6 (2) | pc/kpc | 30 | 30 | 0 | 4559 | 0.7 | 3800 | 12 |
| 9 (3) | school | 30 | 30 | 0 | 22046 | 7.0 | 13891 | 12 |
| 9 (3) | kara | 30 | 30 | 0 | 29237 | 8.6 | 16501 | 12 |
| 9 (3) | pc | 30 | 30 | 0 | 18072 | 3.6 | 7053 | 12 |
| 9 (3) | kpc | 30 | 30 | 0 | 18110 | 4.2 | 8523 | 12 |
| 12 (4) | school | 30 | 4 | 26 | >=300000 | 48.3 | 22706 | 112 |
| 12 (4) | kara / pc / kpc | 30 | 0 / 1 / 0 | 30 / 29 / 30 | >=300000 | 55.7 / 47.0 / 54.8 | 26726 / 11185 / 13585 | 112 |
| 15 (5) | all four | 10 | 0 | 10 | >=150000 | 35 / 46 / 33 / 45 (school/kara/pc/kpc) | 38244 / 47004 / 17636 / 22782 | 480 |

Median conflict ratio mode/school on identical instances [D]: b=9: kara 1.28, pc 0.70, kpc 0.84; b=6: pc 1.15; b=12, 15: 1.00 (cap saturated, uninformative). Instruction counts: not collected (not available through the sweeper); propagations are the proxy. Host load was ~66 (shared), so wall time is descriptive only; conflicts are the unit.

## Slopes of log2(conflicts) vs s [D]
Bootstrap (2000, resample instances within cell). Because b = 12 and 15 are mostly censored at the cap (censored runs enter at the conflicts reached, a lower bound), the four-cell slopes (all targets: school 1.90 [1.86,1.96], kara 1.86 [1.82,1.92], pc 1.92 [1.86,1.94], kpc 1.92 [1.86,1.95]; enumeration log2 T_enum slope 1.92) are saturation-biased: the cap, not the solver, sets the top cells. They are upper-bounded by the cap, hence meaningless as exponents. Only the uncensored cells s = 2 -> 3 give an actual slope (two points, no CI): from the medians above, school 2.3, kara 2.7, pc 2.0, kpc 2.0 bits of conflicts per unit s [D]. Paired difference vs school over four cells: kara -0.04 [-0.06,-0.02] (saturation artefact), pc +0.02 [-0.09,0.07], kpc +0.02 [-0.09,0.07].
Reading: every encoding is at or above ~2 bits per unit s at m = 3; the ladder rule needs e < 0.5. None approaches it, and the enumeration slope (1.92 over s = 2..5; = m-1 = 2 asymptotically) is not beaten.

## Decision rule and verdict
Rule: constant-only differences -> record the best as the sweep default; only a slope change with a CI excluding 0 reopens. Observed: pc/kpc cut propagations ~2x and conflicts 16-30% at b = 9 (0.70 and 0.84), but are 15% worse at b = 6, and unresolved at b >= 12; the only slope CI excluding 0 (kara, -0.04) comes from cap saturation and is not a real slope change. Verdict: constant_only. Best at the one resolved size: pc (fewest variables, conflicts, propagations, wall at b = 9). Recommended sweep default: pc, provisional (one resolved size above the Karatsuba leaf; Karatsuba worse there). Prior from solver_saving_floor (no slope change expected) is not contradicted.

## What was dropped
b = 16..32 (the m = 3 pilot already times out at b >= 12 under 120 s with 2e5-9e5 conflicts: sweep-m3-*bit runs.jsonl); Fourier gadget; Kummer and exp-s3 families; bits-s3 encoding; cadical; larger conflict caps; 30 instances at b = 15 (10 used); instruction counts.

## Skeptic re-run (one cell, b = 9, pc vs school)
cd <this dir> && for m in school pc; do MULMODE=$m KISSAT_CONFLICTS=200000 sweeper/target/release/prime_pdp_sat_sweep sweep --m 3 --bits 9 --two-adicity 2 --families small-x --encodings bits-grouplaw --ls 3 --planted 15 --uniform 15 --timeout 60 --out $TMPDIR/rr-$m; done; compare median conflicts (expect ~22000 vs ~18000).

## Next step
Resolve the slope properly: fresh primes per s on the diagonal b = 9, 10, 11, 12 with a cap >= 3e6 conflicts, 30 targets, pc vs school only (about 1 h per cell on this host), or move to a CDCL(T) arm (L2). Not expected to change the verdict.
