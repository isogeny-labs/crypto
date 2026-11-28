# Mapped plane-quartic index calculus recovered the source scalar

Recovered **k=183049** for the exact source pair D1 and B=2*(3,21), on the
registered genus3/F127curve. Scalar multiplication verified the result on the
source hyperelliptic Jacobian and the destination Jac(Q1). Changing k to k+1
failed on both. The native hyperelliptic IC control, run on this exact pair,
returned the same183049 with407trials and72relations (seed20260916).

Frozen source8b074fa80042/checkpoint20261115T164952Z preceded one release run.
Exit0, empty stderr, no scientific rerun, altered target, cap or fallback.
The generator/target scalar relation was not provided to transport or solver:
the target was selected geometrically, not constructed by a supplied k.

| Stage | Recorded outcome |
|---|---|
| Public D1 map |853trials,841regular,5zeros,12scalar-chart/4nonsplit exclusions|
| Public B map |38trials,37regular,1zero,1scalar-chart exclusion|
| Line sweep |16257projective lines,664fully split canonical divisors|
| Linear algebra |129points,128anchored columns,rank127,nullity1|
| Controls |all664row residuals,one corrupted-row rejection,8principal-divisor checks|
| Generator descent |0translates,roots89/91/94,scaled log47376|
| Target descent |13translates,roots5/18/47,scaled log619220|
| Verification |source=true,destination=true,wrong scalar rejected=true|
| Same-pair HEC IC |k183049,trials407,relations72|

Descent base divisor indices0/1/4, first nonzero-log point index1 with
step_log359037. Target descent exclusions at translates0..12 are all retained.
All129factor-base kernel coefficients, source/target divisors, mapped encodings
and selected pairs are in RUN.txt. Both mappings re-executed actual section
scans/jets; earlier target encodings were not loaded. The two algorithm paths
share arithmetic and are not independent implementations.

This joins the previously constructed kernel/quotient with public transport,
line relations, nullspace solving, rational descent and actual source scalar
verification. Input kernel certificate, normalized sections, Q1 and calibration
are loaded from prior executed construction. This is therefore a complete
correctness path after construction, not a cold complete-cost measurement.
All setup must be charged in the speed evaluation.

The current collector explicitly sweeps q²+q+1lines, enumerates rational points
with the pinned routine and uses dense full-base elimination. It is a correctness
reference, not an implementation of the optimized asymptotic Diem algorithm.
Next replace these quadratic/cubic pieces with the intended point enumeration,
large-prime relation engine and sparse solve, then compare the exact same source
targets with construction/calibration/maps/LA/descent/verification included.
No timing ratio,10x, novelty or scaling claim. Genus2/4 remain separate required
work. Closed records, defaults, existing canonical graphs and leaderboard unchanged.
New graph STAGES.svg records exact ranks/counts and the executed path.
