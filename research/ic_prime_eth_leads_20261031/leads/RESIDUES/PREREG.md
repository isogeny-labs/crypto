# Pre-registration (written before any run)
## A. Harness accounting
Relation := a target answered SAT (decomposition verified by the sweeper). Cutoff C := per-solve conflict cap; censored runs are charged C conflicts.
Metric Y_b(C) = (# SAT answers with conflicts <= C) / (sum_i min(conflicts_i, C)) pooled over targets of a curve; headline = mean over curves of the pooled
ratio and ratio-of-sums over curves; 95% CI = bootstrap over curves (10 curves, clustered). C in {10k, 30k, 100k}; b in {6, 9, 12}; also report
censored counts and Y per propagation. Slope of log2(1/Y) per 3 bits b is reported descriptively only.
Setup restatement (L12 tables, existing raw, no new solves): for each cell and N in {16,64,N_max}: inc total conflicts, fresh total (lower bound if censored),
setup_est = inc_total(N) - N*tail (tail = mean inc conflicts of the last quarter of targets, floored at 0), tail per relation, break-even N vs fresh,
and the explicit-enumeration reference in group-operation units: setup (2|F|)^(m-1), per target 2|F| lookups (|F| from p,a,b,l in targets.tsv header).
Conflicts and group operations are different units; report the ratio conflicts/op, do not add them.
## B. Cube classes (b=9, m=3, s=3; 10 curves x 16 targets = 160 instances)
Classes on the 9 window bits (x1,x2,x3 low s bits each; bit index 0 = LSB): 
 hi: top bit of each window (3 bits -> 8 cubes); lo: LSB of each window (8 cubes); one: all 3 bits of window x3 (8 cubes); rnd: 3 window bits fixed by a seeded random choice per instance (8 cubes).
Cube-and-conquer cost per instance := sum over cubes of conflicts (and propagations), each cube cap 100000 (conquer = same kissat default on CNF+units). 
Direct := plain kissat on the same CNF, cap 100000. Primary statistic := per-curve median over its 16 instances of log2(CnC_total/direct) for conflicts and propagations;
then mean over curves with bootstrap-over-curves 95% CI. Instances with any cap hit are reported (censored lower bounds). Secondary: max-cube conflicts/direct (parallel wall proxy).
Decision (stated in advance): a class "helps" only if the CI of log2(total ratio) lies below 0 on BOTH conflicts and propagations; otherwise constant/no effect.
Nothing here tests an exponent: one size.

## Amendment 1 (before the sweep runs, after a 60-seed probe with 0 targets, raw/probe.txt)
Random primes give factor-base sizes |F| (abscissae) from 0 to 11 at the same nominal b, which would swamp the b-trend. Curves were therefore
selected ONLY on |F| (never on solver outcome): b=6 |F|=3 (|F|=2 admits no planted m=3 target), b=9 |F|=4, b=12 |F|=8 (= 2^(l-1)); first 10 seeds each
(raw/selected_seeds.txt). Planted targets at b=6 repeat the single triple's sum class. Sandbox refuses setpriority, so `nice` is a no-op here (see analysis).
