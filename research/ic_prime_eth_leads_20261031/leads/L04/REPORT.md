# L04 report: strong-backdoor census, chained S3, field-linear + root-finding propagation

Verdict: CLOSED (constant offset). B* = (m-1)s - O(1); offset does not grow with s.

## Propagator (l04 sources: a.h b.h c.h d.h l04.cpp)
Curve y^2=x^3+Ax+B over F_p, p = 3 mod 4, about ms+2 bits. Windows x_i in [0,2^s), m windows, all identical.
System: S3(x0,x1,t0), S3(t0,x2,t1) ..., S3(t_{m-3},x_{m-1},x_R); S3 = Semaev (x1-x2)^2 z^2 - 2[(x1+x2)(x1x2+A)+2B] z + (x1x2-A)^2-4B(x1+x2).
Each variable: finite domain of size <= cap (determined) or undetermined. Window symbol domain = values consistent with fixed bits
(optionally only those with f(x) square = filter). Rules to fixpoint: constraint with 0 undetermined -> keep supported tuples
(conflict if a domain empties); exactly 1 undetermined -> union of quadratic roots over the product of the other two domains
(a window var intersects its domain with it; a t var becomes determined only if the union has <= cap values; leading coeff 0 handled).
Decided = conflict or all x_i,t_k determined. Backdoor = set of window bits such that all 2^|B| assignments decide.
Search: exhaustive over all bit subsets, upward by size (backdoors are upward closed), first size with a backdoor = B*;
swap(x0,x1) symmetry pruning (exact, S3 symmetric). Upper bound (m-1)s checked explicitly (first m-1 windows fixed).
Monotonicity tested: mono.cpp, 2.88M samples, 0 violations.

## Instances
Per cell and cap: 3 curves (own random prime and A,B), per curve 16 planted-SAT + 16 UNSAT targets (96 instances).
UNSAT verified by independent EC-arithmetic brute force (pair-sum sets). Exceptions (time): m=4,s=6,cap 2 and 8: 2+2 per curve (12 instances).
Soundness (all runs): S3 vs EC self-test 0 bad; planted solution never conflicts (full + 300 random partial masks);
UNSAT instances conflict on every full assignment (exhaustive for ms<=16, 3000 samples otherwise); for ms<=16 the number of
non-conflicting full assignments is >0 iff SAT. sound_bad = 0 everywhere.

## Commands
cd to this dir; clang++ -O2 -std=c++17 -pthread -o l04 l04.cpp; ./l04 m s cap filt nper ncurves seed threads budget_s
e.g. ./l04 3 7 4 1 16 3 107 8 600 ; raw outputs in raw/*.txt ; python3 an.py > analysis.txt (tables, slopes).
Seeds: m=3: 100+s, m=4: 200+s, filter-off ablation 300+s / 400+s.

## Results (B*, offset = (m-1)s - B*; full table in analysis.txt)
Offset ranges (min..max over instances), filter on:
- m=3 cap2: s=4..7 offsets 1-3,1-3,1,1.   cap4: 3-5,3-5,3-4,3.   cap8: 5-7,5-7,5-7,5-6.
- m=4 cap4: s=4,5,6 offsets 4-5, 2-4, 2-3.   cap8: 7, 4-7, 4-6.   cap2: the m-1 windows do NOT decide (ub fails),
  B* = 9, 15, 18 = (m-1)s exactly for s=5,6 (offset 0), offset 0-3 at s=4.
- Filter off (cap4): m=3 offsets 3-4,3,3,3 ; m=4 offsets 3-4, 2.
Offset is about 2*log2(cap)-1 for m=3 (cap 2/4/8 -> 1/3/5) plus small pseudo-random fluctuation from the QR pattern of the curve; it is
the same for SAT and UNSAT targets (B* depends on the curve and cap, not on the target). It shrinks or stays flat as s grows.
Slope of per-curve mean B* vs s (OLS, 95% t CI): m=3: 2.40 [2.01,2.79], 2.50 [2.04,2.96], 2.27 [1.72,2.81], filter-off 2.20 [2.01,2.39] (m-1 = 2).
m=4: 4.0-4.3 with CIs [3.0,5.0..5.3] containing 3 (inflated by the s=4 small-window cells where the offset is largest; s=5,6 only: offsets 2-4).
No slope is below m-1.
Minimal-backdoor profiles (window class): m=3: (s-3 bits of window 0 ... ) i.e. (s-3,0,s-1)-type: one window fully free, one window with
~2 bits free (cap), one with 1 bit free. m=4: (s,s-1,0,s-1) and permutations: one window entirely free, others missing 1-2 bits.
Always window bits only; the free window is determined by root-finding intersected with the sparse window (density 2^s/p).

## Why t and slope variables cannot beat it
t_k (and slope lambda_k) are b = ms+2 bit quantities and the propagator has no bit-level link from their bits to anything:
a branch on j bits of a t leaves a domain of size 2^(b-j) > cap unless j >= b - log2(cap). That costs >= b - 3 bits
versus (m-1)s - O(1) = b - s - O(1): worse by about s - O(1) >= 1 bit for every s>=4 cell, and grows with s.
(Window bits are the only low-entropy variables; t,lambda are functions of them.)

## Decision rule and verdict
Rule: offset not growing with s -> closed; slope below m-1 with a named class -> reopen. Observed: offsets constant or decreasing in s,
slopes >= m-1 (CIs include m-1). VERDICT: closed. Also note the cap is cost: enumerating pair domains costs cap^2 root-findings per node,
so B* + 2 log2(cap) >= (m-1)s - O(1) in work bits; the "saving" is the cap-sized enumeration moved inside the propagator, not structure.
Caveats: s<=7 only; filter makes B* curve-dependent (QR luck); m=4 s=6 cap2/8 only 12 instances; prime-field only (binary analogue not run);
t/slope argument is a derivation, not measured (they are not branchable in this propagator).

## Skeptic re-run
./l04 3 7 4 1 16 3 107 8 600 | grep -c "off=3 "   (expect ~96, B*=11, offset 3) ; ./mono (expect 0 violations)

## Next step
File as known negative for L2/L5 exponent hopes. If a reopen is wanted, change the propagator class (e.g. linear algebra over t with
slope variables as unknowns, or Groebner-style), not the cap.
