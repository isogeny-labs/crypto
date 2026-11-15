# Item 7 PROTOCOL: exceptional loci of the two-summand mu_{2^k} chain closure (written 2026-11-14, before data)

Engines: ic_bounds_growth_lib (dense exact closure over F_p; Layout(k, D)), unit test research/leads_chase_20261112/tools/unit_test.py.
Closure generator `gen` is copied verbatim from research/leads_chase_20261112/tools/s1_freq_probe.py (S_3 coefficients reduced mod x^N-1).

## Part 1: F_19 census at (k, D) = (6, 4)
Cells: all (a, b, t) in {0..18}^3 with 4a^3+27b^2 != 0 mod 19, t != 0, all nine S_3 coefficients nonzero (len(s3_coeffs) == 9),
t^3+at+b != 0 (target not of order 2), and resultant_unit(19, a, b, t, 7)['unit'] true. For each: dim V_4(6) (reference 605, 794 columns)
and whether the monomial 1 lies in V_4(6). No sampling unless the 2.5 h rule bites (est. 0.66 s per closure, 2 workers).
k = 7 dims are computed for every k=6-departing instance and for a uniform sample (seed fixed in the script) of 150 generic ones.
Pre-registered expectation: departure rate near 1/10 (observed 5/30 at l = 19 earlier) with classes 602, 603, 794 dims at k=6, rest 605.

## Part 2: locus search (all on the F_19 census, no new data)
Candidate invariants, computed for each instance: j-invariant (and a^3/b^2 class), 3t^2+a, t^3+at+b (and its quadratic character),
point order on E or its quadratic twist (x-only target), disc_{x1} and disc_{x2} of S_3 at the target, vanishing of all pairwise
coefficient linear relations (ratios of the nine coefficients equal to small constants or each other, in F_19), multiplicative order of
t, of each S_3 coefficient, of each root of A(x2), B(x2), C(x2) and of disc polynomials; whether roots of S_3(x1,x1,t) (x-coordinate
x with 2P_x... i.e. points P with P+P=target in x-only sense) lie in mu_{2^infinity} extensions; whether the ratios of coefficients are 2^j-th powers.
Selection rule: a condition "separates" a class if it holds for every instance of the class and for no instance of the others (exact),
otherwise report best partial with counts (TP, FP, FN). Tests by exhaustive scan of simple families, not by tuning to a single instance.
Falsifier of "the departure is an exceptional codimension-one locus described by a polynomial/order condition": best candidate has FN+FP > 0 and
no family reaches exact separation, in which case only partial separation is reported.

## Part 3: out-of-sample test over F_43
The condition found in Part 2 is frozen in a dated section "FROZEN PREDICTION" below (plus a sha256 of the prediction file) BEFORE any F_43
closure is computed. Then closures are computed for all predicted-departing instances (or uniform sample of them if > time budget) and a
uniform sample of predicted-generic ones; report TP/FP/FN and per-class agreement. Falsifier: any predicted-class mismatch rate above 10% of the
tested instances, or zero departures found among predicted-departing ones.

## Time/resource rules
<= 2 threads (2 worker processes, OMP_NUM_THREADS=1), ulimit -v 6000000, nohup for long runs.

## Amendment 1 (2026-11-14, after the F_19 census, before any other prime was computed)
Findings from F_19 that change the plan (details in RESULTS.md): the dimension class is a function of (alpha, beta) = (a/t^2, b/t^3) alone
on the admissible set (277 groups, 0 non-constant), so a plane scan at t = 1 replaces triple scans; the 3t^2+a = 0 locus and one plane cubic Q(alpha,beta)=0
separate classes 603/600 and 602/599/607/600 exactly. To lift Q to characteristic 0 (an integer cubic, by CRT / rational reconstruction) and to
look for the class-794 locus, plane scans at t = 1 (all (alpha, beta), same closure code, k = 6, D = 4) are added for the TRAINING primes 23, 29, 31, 37.
F_43 stays held out: no F_43 closure is computed until the FROZEN PREDICTION section below exists with its sha256. Primes 23..37 are not out-of-sample tests.

## FROZEN PREDICTION (2026-11-14, before any F_43 data)
Training evidence used: F_19 census; plane scans at p = 23, 29, 37; partial random samples at p = 41, 47, 53, 59 (cubic fits only; 53, 59 incomplete).
Primes 31 (anomalous: generic dim not 605, 2 has order 5 mod 31), and 43 (held out) excluded. Prediction text: FROZEN_PREDICTION_F43.md,
sha256 6553911c64d8772c8b2b8e0b846c160e83890718e3a4bd9681c7be163f48029d. Test: full t=1 plane at p = 43 (tools/plane.py 43 1 w 2), plus 150 random-t instances for P6.

## Amendment 2 / FROZEN PREDICTION v2 (2026-11-14, before any F_43 closure; an earlier launch of the F_43 scan was killed after 275 cells and its output deleted unread)
Integer lift of the cubic found and verified (see RESULTS.md). Exact F_43 prediction in FROZEN_PREDICTION_F43_v2.md (sha256 f06ae9959bb40fe74a633192e580ec98605f9cdad08aee2c4a4556d5afa57c3b) with data/pred43.json
(sha256 e7257618f8c4f44ad8695abe0fe292a5549adbb1e1dc0002bb061d3af7546393) computed from the cubic alone. v1 items P1, P2, P4, P5, P6 unchanged.
