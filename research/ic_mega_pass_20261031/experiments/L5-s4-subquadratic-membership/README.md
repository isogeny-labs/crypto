# L5-s4-subquadratic-membership: is there a sub-n_F^2 structure for S_4 membership? (Lever 2, open problem 4)

**Answer (toy scale plus a counting argument): negative.** Two kinds of structure were tested:
(i) a 2-torsion (FGHR Y(P+T) = -Y(P), D_m) quotient, and (ii) resultant/gcd algebraic third steps.
Neither changes the exponent. Each one sits on the covering line S·D ≈ 3·|X3|.
Classification: the 2-torsion device is a *relabelling* with a constant-factor table saving.
The algebraic arm is *dominated*. No security claims.

Notation: n_F is the base size. r/|Aut| is BRIEF rule 5's N.
X3 = { x(±Pi±Pj±Pk) } (conventions in PROTOCOL.md, which was written before any run; its sha256 is in RESULTS.json).

## (a) Counting argument
**Model L(S,D).** Preprocessing, which depends on F only, stores S x-values (or group elements).
Online, the target R is tested with D non-adaptive equality lookups "x(R + c_j) ∈ table?", where the offsets c_j do not depend on R.

- R is accepted iff x(R) ∈ { x(±t - c_j) }, a set of at most 2SD values.
- A correct decider therefore needs **S·D ≥ |X3|/2**.
- For an unstructured base, |X3| ≈ (2/3)n_F^3 (measured 0.61–0.66 n_F^3 below saturation). So any table with S = o(n_F^2) forces D = ω(n_F).
- Three methods sit on the line up to a constant:
  - the pair table: S ≈ n_F^2, D = 2n_F;
  - the no-table loop: S = n_F, D ≈ n_F^2;
  - the full X3 table: S = |X3|, D = 1.
- Measured S·D/|X3| = 2.9–3.5 for every arm in every unsaturated cell, flat in n_F.

**Algebraic multi-candidate steps** (gcd against a product polynomial) fall outside the lookup model, so they are counted in field operations instead.
- To test whether x_Q ∈ x(F±F) with gcd(∏_c S_3(x_Q,c,Z), f_F(Z)), you need the 2n_F values x_Q = x(R±P3). Each such test costs Õ(n_F).
- That gives Õ(n_F^2) per target.
- Precomputing Φ(X,Z) = ∏_c S_3(X,c,Z) is itself an S = Θ(n_F^2) table.
- Testing roots of G_R(Y) = ∏_c S_3(x_R,c,Y) against f_{F+F} (degree about n_F^2) costs Ω(n_F^2) per gcd.
- So on both routes the polynomial form only moves the n_F^2 between table and online work.

**Scope: where the argument does NOT apply.**
1. Adaptive or iterated lookups (Hellman/Fiat–Naor chains). These are governed by the CGK / 3SUM-indexing trade-offs (BRIEF rule 5), not by this bound.
2. Bases with additive structure, where |X3| ≪ n_F^3. These escape the bound only by losing yield; the 2-torsion arm below is an instance.
3. Non-generic encodings where the third step is a polynomial of degree ≪ n_F whose roots happen to be the valid x3. None is known for an unstructured prime-field base. Any such polynomial would have to vanish on about 3 points out of n_F per member target.

Novelty: the covering bound is the standard 3SUM-indexing counting bound (Demaine–Vadhan; Golovnev–Guo–Horel–Park–Vaikuntanathan 2020 "Data structures meet cryptography: 3SUM with preprocessing") moved to S_4 membership. `novelty_unverified`: no literature search was run in this task.

## (b) Toy results
Setup: C code (`memb.c`), single thread, curve y² = x(x−α)(x−β), p = 2^20, 2^24, 2^28 (p ≡ 3 mod 4), n_F = 32..512, 2 seeds per cell, 2000 random and 200 planted targets per run. Ground truth is a brute-force X3 bitmap kept in RAM. Costs are in M-equivalents with I = 3M.

| p | n_F | arm | X3/RAND | table/RAND | S·D/|X3| | online per relation vs RAND |
|---|---|---|---|---|---|---|
| 2^28 | 128 | TCL (F0 ∪ F0+T) | 0.251 | 0.250 | 3.00 | 4.08× |
| 2^28 | 128 | E2CL (F0+E[2]) | 0.063 | 0.062 | 3.00 | 16.6× |
| 2^28 | 256 | TCL | 0.258 | 0.250 | 3.03 | 4.13× |
| 2^28 | 256 | E2CL | 0.065 | 0.063 | 3.01 | 16.7× |

Full table: `RESULTS_grid.json`, reproduced by `python3 analyze.py`.

**No-table vs pair-table online cost.** The ratio B/A grows linearly in n_F: 57, 124 (p = 2^20, n_F = 32, 64); 115, 238 (p = 2^24, n_F = 64, 128); 230, 473, 997 (p = 2^28, n_F = 128, 256, 512).

**PARI algebraic arm C** (`arm_c_pari.py`, two-summand gcd oracle per x(R±P3)):
- CPU per target was 0.00066 s at n_F = 8 and 0.0435 s at n_F = 64. The fitted exponent is **2.01**.
- S_3 was validated on 300 pairs with 0 bad.
- There were 0 mismatches.

**Correctness.**
- 0 decision mismatches for the A, quotient and B arms across 48 runs.
- 100% of the positive decompositions were re-verified by explicit point addition.
- Arm C v1 had a convention bug: it accepted 2-sums R = ±P3 ±Pk. It was found by the brute-force check (1 mismatch), fixed, and rerun with 0 mismatches. The v1 output is kept in `raw/arm_c_v1_buggy.jsonl.gz`.

**Why the 2-torsion device is a relabelling.**
- If F is <T>- or E[2]-stable, then F±F±F = ∪_t (F0±F0±F0) + t exactly.
- So membership in F±F±F reduces to membership of R+t in a 3-sum set of a base of size n_F/2 or n_F/4. Equivalently, it reduces to the isogenous curve E/<T> (or E/E[2]) with the image base.
- The table shrinks 4× or 16×, but yield shrinks by the same factor. At equal yield the device is constant-factor at best.
- The T-components also vanish in the large prime-order subgroup, which is where the logarithms live. 2-torsion curves are not prime-order curves.

**Prediction scorecard.**

| Prediction | Result | Detail |
|---|---|---|
| P1 | PASS | |
| P2 | PASS below saturation | TCL 0.251–0.267, E2CL 0.063–0.068 |
| P2, near saturation | outside the band | 0.284–0.384 and 0.074–0.109. Cause: RAND's \|X3\| approaches p/2 and collisions saturate. This regime was flagged in PROTOCOL. |
| P3 | PASS | |
| P4 | PASS | |
| P5 | PASS | |
| Falsifier (S·D/\|X3\| decreasing with n_F) | not triggered | |

## Rule-5 certificate
- Pair-table IC with B = n_F and online D ≈ 2n_F per test: per relation, D_rel ≈ 2n_F·(p/2)/|X3| ≈ 1.5p/n_F^2.
- So B·D_rel² ≈ n_F·p²/n_F^4, which is ≥ r for n_F ≤ r^{1/3}.
- Nothing measured here moves this line. The quotient arms keep D per test and divide yield.

## Blockers / caveats
- Online counts include early exit on positives. Cells where RAND is saturated (p = 2^24 with n_F = 256, p = 2^28 with n_F = 512) understate RAND's cost.
- Bitmaps stand in for hash tables, with O(1) lookups that are not counted.
- The PARI arm reports wall time plus formulas, not instrumented operation counts.
- Toy sizes only.

## Files
PROTOCOL.md, memb.c, arm_c_pari.py, analyze.py, RESULTS.json, RESULTS_grid.json, raw/grid.jsonl.gz, raw/arm_c.jsonl, raw/arm_c_v1_buggy.jsonl.gz
