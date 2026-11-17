# Pilot 03: cut-variable branching with bounded nogood memory (m=4) - PROTOCOL

Written 2026-11-14 before any measured run. Pilot only, toy sizes, one core, not claim data.
Every number this pilot produces is an unverified assertion [pcc:unverified].

## Claim under test
For the m=4 point decomposition problem in the chained encoding
`S3(x1,x2,u)=0, S3(x3,x4,v)=0, S3(u,v,x(R))=0` (x_i in V, dim V = l; u = x(P1+P2), v = x(P3+P4)
full n-bit field elements), forcing a CDCL solver to branch first on the n bits of the cut
variable u lowers the growth of conflicts with l, compared with the same solver's default
branching on the same CNF.

## Prediction (from the idea card)
Cut-first conflict slope on non-decomposable targets drops toward 2 bits per l
(MITM-like), while default branching sits near or above 3 bits per l.

## Kill criterion (fixed now)
Fit log2(median conflicts) against l by least squares over the non-decomposable (UNSAT) cells,
per arm, using only cells where at least half the runs finished inside the cap.
KILL if slope(cut-first, arm C) > slope(default, arm D) - 0.3 bits/l, or if arm C cannot be
fitted on >= 3 sizes because it hits the cap where arm D does not (reported as
"kill: treatment censored where the baseline finishes").
ALIVE only if slope(C) <= slope(D) - 0.3 with >= 3 uncensored sizes in both arms.
Otherwise INCONCLUSIVE.

## Instances
- Field F_{2^19} = F2[z]/(z^19+z^5+z^2+z+1), n = 19 fixed for all l (so only l varies).
- Curve y^2+xy = x^3+x^2+1 (Koblitz form, a=1, b=1). Binary S3(a,b,c) = (ab+ac+bc)^2 + abc + b_curve.
- V = span{1,z,...,z^(l-1)}. l in {2,3,4,5}. (l=6 only if l=5 finishes with large margin; otherwise not run.)
- Per cell 8 targets: 4 decomposable (planted: 4 rational base points with x in V, R = their sum,
  no intermediate point at infinity) and 4 non-decomposable (random x(R) of a rational point,
  rejected until the algebraic MITM oracle says no solution). l=5 may be cut to 2+2 targets
  if the calibration run shows the cell would exceed the budget; any such cut is reported.
- Instance seed = 20261114000 + 100*l + idx, Python `random.Random(seed)`. Recorded per row.
- The encoding is the usual algebraic relaxation: x_i range over all of V (not only x of rational
  points) and u, v over all of F_{2^19}; the oracle decides exactly this relaxed system.

## Encoding (identical CNF for every arm)
Bit variables: l per x_i, 19 each for u, v. Products are bitwise AND variables; p=x1*x2,
q=x3*x4, w=u*v, and the products p*u, s^2*u^2 etc. Equations per S3:
`p^2 + s^2 u^2 + p u + 1 = 0` (s=x1+x2), top: `w^2 + (u^2+v^2) xR^2 + w xR + 1 = 0`.
XOR rows are chopped into 4-variable XORs with auxiliaries (8 clauses each). No symmetry breaking.

## Arms (all CaDiCaL 1.9.5 through python-sat `Cadical195`, solver seed default 0)
- D   : default branching, no external propagator. Untreated SAT arm (baseline).
- Dp  : default branching with an external propagator attached that observes the u bits and
        never decides (control for the side effects of attaching IPASIR-UP).
- C   : cut-first: IPASIR-UP `decide()` returns the lowest-index unassigned u bit, phase false;
        when all u bits are assigned the solver's own heuristic decides. Otherwise as Dp.
- C-lean : as C, with a small learnt-clause database: `reduceint=10, reducetarget=100,
        reducetier1glue=1, reducetier2glue=1`.
- C-keep : as C, with `reduce=0` (no learnt clause is ever deleted).
C vs Dp isolates the branching order; C-lean / C / C-keep is the memory dial at fixed branching.
Deviation from the idea card, stated in advance: python-sat exposes no hard cap on the learnt
database, so the dial is CaDiCaL's reduction schedule, not an exact clause count. Peak database
size is not observable through this interface and is not reported.

## Reference counts (not solvers; computed per l)
enumeration 2^(3l)/4!, MITM 2^(2l) time and memory, linear-leaf scan count 2^(2l)
(the baseline to be measured against per the task: 2^((m-2)l) * poly), plus the oracle's actual
number of quadratic solves and |U| = number of distinct reachable u.

## Metric
Primary: conflicts (also decisions, propagations, restarts, cut decisions made by `decide()`),
from `accum_stats()`. Secondary: wall seconds. Decomposable and non-decomposable reported separately.
Cap: 40 s wall per run (interrupt); a capped run is recorded as censored with its conflicts so far.

## Order
For each target, all arms on the same CNF, arm order rotated by target index (interleaved).

## Validation before measurement
1. n=7, l=2: for all 128 values of x(R), SAT (arm D and arm C) vs an exhaustive scan that does not
   use the quadratic solver vs the oracle; all three must agree.
2. Planted instances must be SAT in every arm; every SAT model is re-checked in field arithmetic
   (three S3 values zero, x_i in V) and lifted to rational points with signs where all x_i are
   rational (lift result recorded; non-rational models are counted, not hidden).
3. Every finished run's SAT/UNSAT must equal the oracle.

## Calibration
One instance per l with seed 999 (not part of the cells) is run on arm D and arm C only to decide
whether l=5 is run with 8 or 4 targets and whether l=6 is attempted. Its rows are kept in results.json
under "calibration".

## Amendment 1 (2026-11-14, after validation smoke and calibration, before any cell run)
Facts that forced the change (all kept in results.json under "calibration"):
- `interrupt()` does not stop `Cadical195` through python-sat; the wall cap is replaced by a
  conflict cap (`conf_budget`). A capped run is censored at the cap.
- n=19 is out of reach: the first validation instance (n=19, l=3, planted) did not finish arm D in >100 s.
- Calibration, seed 999, non-decomposable, cap 150000 conflicts: n=13 l=3 and l=4: arms D and C both
  capped; n=17 l=3: D UNSAT at 73502 conflicts, C capped. n=7 l=2 (xR=5): D 1226, C 1364 conflicts.
  So cost on this encoding is driven by n (the free n-bit cut u and v), and n=19 with l up to 5 does
  not fit the budget.
Changed design (everything else as written above):
- PRIMARY series: n=11 fixed (F2[z]/(z^11+z^2+1)), l in {1,2,3}; 4 non-decomposable targets per cell,
  4 planted decomposable targets for l>=2 (l=1 has no rational base point with x in V={0,1}\{0}: x=1 is
  not rational on this curve, so the l=1 cell has no planted targets). l=4 is not usable at n=11
  (about 10 expected relaxed solutions per target, no UNSAT targets). Cap 200000 conflicts per run.
  All five arms.
- SECONDARY series (n effect, arms D and C only): n=13 (z^13+z^4+z^3+z+1), l in {2,3,4},
  2 non-decomposable targets per cell, cap 600000 conflicts.
- Seeds: 20261114000 + 10000*n + 100*l + idx.
- Kill criterion unchanged, applied to the PRIMARY series. With only l in {1,2,3} at n=11 the fit has
  3 points and one of them (l=1) is degenerate-small; the verdict is to be read with that limit.

## Amendment 2 (2026-11-14, during the primary series, before any decomposable row existed)
Planted rational targets are not usable at n=11: V has no rational x at l=2 and exactly one (x=6) at
l=3, so "4 planted targets" would be one target (x(4P)) and the generator stalled in rejection.
The l=3 process was killed with no rows written and restarted for the non-decomposable targets only
(same seeds). Decomposable targets are now random rational x(R) accepted when the oracle says the
relaxed system has a solution (seeds base+50+i, mode `relaxed`), l in {2,3}, 4 per cell, all five arms.
Their models are mostly non-rational (x_i or the intermediate points live on the twist); the lift
status of every model is recorded. The n=11 l=1 and l=2 non-decomposable rows already written are kept.
