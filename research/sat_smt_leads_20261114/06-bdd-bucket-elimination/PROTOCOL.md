# Idea 6 pilot protocol: BDD bucket elimination for the PDP (written before any measured run)

Date: 2026-11-14. Pilot only, toy sizes, one core, pure Python. Every number this
pilot produces is an unverified pilot assertion ([pcc:unverified]).

## Claim under test

For the point decomposition problem on a binary Koblitz-style curve
(y^2 + xy = x^3 + 1 over F_{2^n}, factor base x in V = span{1,z,..,z^{l-1}}),
deciding the Weil-descended Semaev system with reduced ordered BDDs and bucket
(variable) elimination needs a peak live node count that grows slower in l than
a meet-in-the-middle table, i.e. slower than 2^{ceil(m/2) l}.

## Prediction (the idea's)

Fitted slope of log2(peak live nodes) against l is below ceil(m/2) bits per l
(m=3: below 2; m=2 sanity cell: below 1).

## Kill criterion (fixed now)

Taking, per m, the arm (variable order x elimination schedule) with the LOWEST
fitted slope of log2(median peak live nodes) vs l:

- KILL if that slope >= ceil(m/2) - 0.2 bits per l (m=3: >= 1.8), or
- KILL if blow-up (node cap 4e6 unique nodes or 300 s per run) prevents 3 sizes of l.

The verdict is taken on m=3. m=2 is a sanity cell and is reported separately
(threshold 0.8) but does not decide the verdict.

## Encoding (one encoding, fixed)

- m=2: n Boolean constraints, bit r of S3(x1,x2,xR) = p^2 + s^2 xR^2 + p xR + b,
  p = x1 x2, s = x1 + x2, over 2l bit variables (quadratic ANF).
- m=3: split with one field variable t (n bits): S3(x1,x2,t) = 0 (side A, n
  constraints, target independent) and S3(t,x3,xR) = 0 (side B, n constraints),
  over 3l + n bit variables. Each constraint is built as one BDD from its ANF.
  No Tseitin/gate auxiliaries, no direct S4.

## Arms (treatment): variable order x elimination schedule

Variable orders (top to bottom):
- `xint_t`  : x bits interleaved across summands (x1[0],x2[0],x3[0],x1[1],..), then t   (required order)
- `xblk_t`  : x1 block, x2 block, x3 block, then t                                     (alternative)
- `t_xint`  : t bits first, then interleaved x                                          (alternative)

Elimination schedules (bucket elimination: for each variable in the schedule,
conjoin all pool functions mentioning it, smallest first, then quantify it):
- `t_first`  : t bits, then x bits (monolithic conjunction)
- `x_first`  : x3 bits, x1/x2 bits, then t (builds T(t) and U(t) separately: BDD analogue of MITM)
- `x3_t_x12` : x3 bits -> T(t); then t bits (T conjoined with side A); then x1,x2 (BDD analogue of the linear 2+1 scan)

m=2: orders `int` (x1[j],x2[j]), `blk` (x1|x2), `rev` (x1[j],x2[l-1-j]); single
schedule (all constraints mention all variables), constraints conjoined in index order.

## Baselines (same instances)

Counted, solver independent:
- enumeration: 2^{(m-1)l} tuples (m=3: 2^{2l} pairs then lookup; counted as 2^{2l})
- MITM memory: 2^{ceil(m/2) l} entries (m=3: 2^{2l}; m=2: 2^l)
- linear 2+1 scan (m=3): 2^l leaf solves; m=2: 2^l scan
- measured brute-force ground truth (MITM over (x1,x2) vs x3) on the same
  targets gives the SAT/UNSAT answer every BDD run is checked against.

The untreated SAT arm is not re-run here (prior repo figures: about one conflict
per tuple); the baselines above are the counted ones the kill criterion names.

## Metric

Primary: peak live BDD nodes = max over all steps of the number of nodes
reachable from all functions currently in the pool (plus the result just
built). Canonical for a given order, independent of cache/GC policy.
Also recorded: initial pool size (all constraints, before any conjunction), max
single constraint size, max size of any intermediate conjunction/quantification
result (`peak_work`), total apply calls (recursive non-terminal calls to
and/or/xor/exists, caches cleared per run) for elimination, and separately for
constraint construction. Wall time secondary.

Slope: least squares of log2(median over targets) against l, per arm.

## Sizes, targets, seeds

- m=3: l = 3,4,5 with n = 11,13,17 (smallest prime >= 3l); l=6 (n=19) attempted, reported as blow-up if it hits the cap.
- m=2: l = 4,5,6,7,8 with n = smallest prime >= 2l (11,11,13,17,17).
- 8 targets per cell: 4 planted (R = P1+..+Pm, x(Pi) in V, real curve points), 4
  random curve points (decomposable or not, by ground truth; reported separately).
- RNG: random.Random(20261114*1000 + m*100 + l). Arms are run interleaved per target.
- If budget forces fewer targets at the largest size, that is stated in README.

## Validation before measuring

1. S3 checked against real point addition (x(P1+P2), x(P1-P2) are roots).
2. Planted targets must be SAT; a model must lift to curve points with
   +-P1+-P2+-P3 = R.
3. On the smallest cells the full BDD solution set (x1..xm) must equal the
   brute-force solution set; on the smallest m=3 cell brute force enumerates
   all t in F_{2^n} as well.
4. Every measured run's SAT/UNSAT answer is compared with ground truth.

## Not in scope

Gate-level/Tseitin encodings, ZDDs, dynamic reordering, and-exists relational
product, m=4, l > 6 for m=3.
