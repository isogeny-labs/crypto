# Item 7 RESULTS: exceptional loci of the two-summand mu_{2^k} chain closure (2026-11-14)

All results below are [pcc:unverified] (computed by this agent, no pcc claim ids, no replay certificate, no independent run). No leaderboard or benchmarks/README rows.
Engines: /home/user/ml-cryptanalysis/scripts/ic_bounds_growth_lib.py (git ml-cryptanalysis 27a67b9, sha256 6d7e2711...), unit test /home/user/crypto/research/leads_chase_20261112/tools/unit_test.py
(git crypto 44d35033fa, sha256 6389cfa6...), cryptanalysis fb3f88a0 (not used). Closure = `G.closure(Layout(k, 4), gen, p)` with `gen` copied from s1_freq_probe.py.
Notation: (alpha, beta) = (a/t^2, b/t^3); "tan" = 3t^2 + a; dim6 = dim V_4(6) (794 columns; generic 605 on unit instances). Host: shared 4 vCPU, at most 2 closure processes at any time, `ulimit -v 6000000`.
Measured = closure dims and unit tests (exact linear algebra over F_p). Derived = loci obtained by fitting/CRT on measured dims; the lift of the cubic is an inference from 6 primes, verified on 2 more.

## Headline
1. F_19 census, k = 6, D = 4: 3422 admissible instances (all (a,b,t) with nonsingular curve, nine S_3 coefficients nonzero, t != 0, target not order 2, unit at k = 7), 578 depart (16.9%).
   dim6 classes: 605 x2844, 794 x216 (1 in V_4(6)), 603 x168, 602 x150, 600 x16, 599 x14, 607 x14. Census exhaustive: no sampling. New classes beyond the (602, 603, 794) of the S1 probe: 599, 600, 607.
2. The dim6 class is a function of (alpha, beta) alone on admissible instances: 277 (alpha, beta) groups, 0 with more than one class (F_19), and 150/150 random plus 92/92 departing groups re-evaluated at a new random t over F_43 agree.
3. Exact separation (F_19, all 3422 instances, TP 362, FN 0, FP 0; data/check_qh.out):
   - tan = 3t^2 + a = 0: class 603 (168) and 600 (16), i.e. x(2T) = -2t. Every admissible instance with tan = 0 departs.
   - Q(alpha, beta) = 0 and tan != 0: classes 602 (150), 599 (14), 607 (14). Q is a nodal plane cubic (node at (alpha,beta) = (-3, 2), the degenerate curve y^2 = (x-1)^2 (x+2) with T at the node; node tangents beta - 2 = (alpha+3)/2 and beta - 2 = -(alpha+3)):
     Q = 5377 a^3 + 10384 a^2 b + 6548 a b^2 + 1352 b^3 + 27709 a^2 + 36028 a b + 11364 b^2 + 47435 a + 31524 b + 26615 in (alpha, beta); in (A, B) = (alpha + 3, beta - 2):
     10384 A^2 B + 5377 A^3 + 6548 A B^2 + 1352 B^3 - 84 (2B - A)(A + B).
     Weighted-homogeneous form in (a,b,t): Q_h = 5377 a^3 t^3 + 10384 a^2 b t^2 + 6548 a b^2 t + 1352 b^3 + 27709 a^2 t^5 + 36028 a b t^4 + 11364 b^2 t^3 + 47435 a t^7 + 31524 b t^6 + 26615 t^9.
   - tan = 0 and Q = 0 (one point per prime, the intersection of the line with the cubic): class 600.
   - Within Q = 0, the secant slope m = B/A from the node selects the sub-class: m = -2 gives 599 (checked at p = 19, 29, 37, 41), m = -5/4 gives 607 (p = 19, 23, 29, 37, 47; unique rational with |num| <= 120, den < 60 consistent with all five), all other Q points 602.
4. Class 794 (1 in V_4(6)) is NOT separated. No candidate invariant separates it; the F_p points of the class are about 0.85 p per prime (16, 19, 27, 32 at p = 19, 23, 29, 37) and lie on no plane curve of degree <= 6 (nullity of the degree-d fit is 0 for d <= 4 at p = 19, 23, 29; d <= 6 at p = 37; 32 points vs 28 monomials), nor on any of the tan = 0 / Q curves (0 of 216 instances).
   Hypothesis "794 = zero of g appears first at a level 2^8..2^10": falsified (all 48 sampled class-794 instances and 70 others are unit at k = 8, 9, 10; data/hi_unit19.jsonl).
5. Out-of-sample over F_43 (prediction frozen before any F_43 closure: FROZEN_PREDICTION_F43.md sha256 6553911c..., FROZEN_PREDICTION_F43_v2.md sha256 f06ae995..., pred43.json sha256 e7257618...): full t = 1 plane scan, 1849 cells, 1645 valid.
   - P3' (exact): the predicted set of 76 valid points (603 x38, 602 x35, 607 x1, 599 x1, 600 x1) equals the observed set of valid points with dim in {599, 600, 602, 603, 607} exactly, with identical dims at all 76 points: 0 mismatches, 0 extra, 0 missing. PASS.
   - P1 (tan0 gives 603/600, 1 or 2 with 600): observed 603 x38, 600 x1. PASS. P2 (unit dims within {599,600,602,603,605,607,794}): PASS (unit dims 605 x1324, 602 x31, 603 x34, 794 x24, 599, 607, 600 x1 each).
   - P5 (class C not on a plane curve of degree <= 5): nullity 0 for degrees 1..6 on the 31 class-C points. PASS (degree 6 not covered by the frozen claim; 31 points vs 28 monomials).
   - P6 (t-independence): 150/150 random valid-unit instances and 92/92 departing groups agree. PASS.
   - P4 (class-C count in 28..46 and unit departing fraction in 0.12..0.30): count 31 in range, but the unit departing fraction at t = 1 is 0.065 (92/1416), outside the frozen interval. The fraction half of P4 FAILS (the frozen interval was taken from instance-level F_19 numbers, 16.9%; the t = 1 plane over valid unit cells is a different denominator; not a refined claim).
6. Verification of the lift without refit: Q (integer) separates the cubic classes exactly at p = 19, 23, 29, 37, 41, 47 (the fit primes) and at the unused p = 53 (9 points) and p = 59 (5 points) samples, with zero false positives in 468 + 382 sampled cells.

## Commands and data (all under this directory)
- tools/census.py (F_19 census; two worker processes, finished, markers data/census19_w{0,1}.jsonl.done{0,1}) -> data/census19.jsonl (3422 rows, sha256 7354ae9c...). PROTOCOL.md was written before this run.
- tools/plane.py p 1 w 2 out (full (alpha, beta) plane at t = 1; fields nonsingular/nine/f0/unit7/dim6/one6): p = 19 (data/plane19_t1.jsonl, sha256 3c000a2b...), 23, 29, 31, 37, 43. Finished, `.done` markers.
- tools/sample_cubic.py (random cells until 14 cubic-class points): p = 41 (681 closures), 47 (794). Stopped by me for p = 53 (468 closures) and 59 (382) and never started for 61, 67: partial samples used only for verification.
- tools/nullsp.py, fitcubic.py, node.py (node-centred fits), crt.py (CRT lift), planeclass.py, analyze19.py (data/analyze19.out), check_qh.py, feats.py + scan_pred.py (invariant scan), hi_unit.py, k7.py, predict43.py, eval43.py (data/eval43.json), p6_43.py, p6b_43.py.
- Lift procedure: per-prime fit of the degree-3 curve through the cubic-class points (nullity 1 each), then node-centred; q2 = kappa (2B - A)(A + B) verified at all 6 fit primes with p-independent shape; kappa and the cubic coefficients, multiplied by 10384, are small integers by CRT over 19, 23, 29, 37, 41, 47 (kappa x 10384 = -84, A^3: 5377, AB^2: 6548, B^3: 1352, A^2B: 10384; modulus 9.0e8, the other candidate denominators 2596 and 1298 appeared first). A rational lift with ten coefficients from 6 primes is an inference; its test is the F_43, 53, 59 agreement above, not the CRT.
- Other measured facts: p = 31 is anomalous (2 has order 5 mod 31; generic dim6 is not 605 and no valid cell lands on 605), so p = 31 was not used for fits. k = 7 closure on 36 sampled F_19 instances: the classes persist as deficits (602 -> 826, 603 -> 828, 599 -> 824, 600 -> 825, 607 -> 835, 794 -> 857, 605 -> 829; data/k7_19.json); whether additional instances depart only at k = 7 was not tested.
- Invariant scan on F_19 (j-invariant, f, disc and their characters, c11/c10/c00 = -2(1+alpha), -2(alpha+2beta), alpha^2-4beta, tan, point order on E/twist, 2-adic and odd parts, orders of roots of x^3+ax+b and of S_3(x,x,1) in the ring quotients, factor patterns, x(2T)): single-value predicates for class 794 reach at best TP 5 / FP 11 (w == 3); nothing separates it. For tan = 0 the scan returns alpha == 16 (= -3), tan == 0, x(2T) == 17 (= -2), all TP 15 / FP 0. x(nT) = c x(mT) for small rational c (n, m <= 16) holds for at most 12% of Q points: Q is not an x(nT) relation.

## Reading
- The non-generic closure at (6, 4) has three exceptional families over small F_p: the line tan = 0 (x(2T) = -2t), a nodal cubic through the degenerate curve with T at the node (about p - 5 points per prime), and a third, unidentified, family (class 794) of about 0.85 p points per prime. Each is codimension one in the (alpha, beta) plane, consistent with a probability of order 1/p per family; at p = 43 the two identified families give 76 of 1645 valid cells.
- The class depends on (a, b, t) through (a/t^2, b/t^3), i.e. through the curve with the target normalised to x = 1: it is a property of the pair (curve up to x-scaling, T), not of the mu_N coset.
- Whether Q (or the integer 10384 = 2^4 x 11 x 59 and the other coefficients) are tied to k = 6 (for instance to 2^6) was not tested at other k; the k = 7 persistence above is at instance level only.
- Not shown: any consequence for an attack. Per the earlier report these loci cannot be steered to with a shared factor base.

## Dropped / blocked
- No time-rule truncation: census exhaustive; plane scans complete for the primes stated. Class 794 locus not identified (no exact condition); next command idea: scan p = 41 or 43 planes (data exists for 43) for a higher-degree curve with nullity test at degree 7..8 (needs >= 45 class-C points, i.e. p >= 53 full planes: `python3 tools/plane.py 53 1 w 2 out`, about 45 min), or compute the minimal certificate weight of 1 in V.
