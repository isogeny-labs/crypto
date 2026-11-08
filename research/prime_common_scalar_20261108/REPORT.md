# Common scalar repair: finite evidence and the next denominator

All producer results below carry unsigned standing. ClaimPCC-20261108-4f9871
records the finite census; theorem records and exact hypotheses are listed in
lean/PROOF_MAP.md. Nothing here supplies a signed independent verdict.
[pcc:unverified]

## What changed and what was actually run

The prior candidate `B_d^2/rho_d` preserved source-node values but its rough
entry-dependent unit increased the mode count. Define the polynomial

`R(T)=sum_s B_s(T)/rho_s^2`,  `W_d(T)=R(T) B_d(T)^2`.

Lagrange node identities give `R(t_s)=1/rho_s`, hence `W_d(t_s)=B_d(t_s)`.
The new factor depends only on T, not the matrix-entry difference d. This
preserves the smooth squared-Cauchy expansion. The actual source-grid identity
is CommonScalar.node_preserved, not an assumed unspecified normalizer.
[pcc:unverified]

The frozen census covers L16,32,64; all nonzero t in[-L+1,L-1]; zeta=-1,1,3;
q16,24,32,48; both Newton and Smith representations; all required tiles and
crossing rows. It compares original, entry square and common scalar at equal
precision, plus entry ceil(4q/3) and scalar2q precision proxies for q16,24.
The two physical processes have the same20928 records, byte for byte. A
separate native grouped-generating-function implementation recounts41856
integer tuple totals and checks every prospective cell key. Newton differences
reconstruct every rational row; Smith transformations reconstruct every
residue matrix. Embedded original A/A, doubling, node and modular48 controls
are retained. [pcc:unverified]

| Representation | Kernel | Budget | Fewer | Equal | More | Cells |
|---|---|---|---:|---:|---:|---:|
| Newton | entry | same |1944|0|672|2616|
| Newton | scalar | same |2616|0|0|2616|
| Smith | entry | same |1176|0|1440|2616|
| Smith | scalar | same |2616|0|0|2616|
| Newton | entry | proxy |540|0|768|1308|
| Newton | scalar | proxy |72|36|1200|1308|
| Smith | entry | proxy |300|0|1008|1308|
| Smith | scalar | proxy |0|36|1272|1308|

Fewer/more refers to candidate exponent tuples with weight sum below the
registered precision. These are exact component counts, not operation counts
for the complete construction, elapsed speedups, or samples from a curve
population. No confidence interval or p-value applies to this deterministic
finite census. Full original-control and per-level results are in
reports/summary.json and reports/per-level.csv. [pcc:unverified]

One logical CPU and2GiB address-space cap per invocation,900s timeout, no
retries, no restores. Both receipts record exit0. Lean import replay was also
running on the host; no CPU isolation or valid clock is claimed. Host load
can delay an exact calculation but cannot account for the tuple inequalities.
Requested2 processes, attempted2, completed2, not completed0, excluded0.
[pcc:unverified]

## Why the precision-proxy loss does not end the idea

The polynomial degrees are at most2L-1,4L-2,6L-3 for original, entry square and
polynomial scalar square. The doubled-precision comparison deliberately charges
a conservative proxy, and it loses almost everywhere for Smith. It does not
prove this precision increase necessary. In particular, the existing
matched-recovery-independent.md uses **all** source levels, not merely the
odd-node sufficient bound. Under its stated integrality and precision
assumptions, the original multiplicity already supports degree below4ML for
u>=3. [pcc:unverified]

A simple margin makes the issue transparent. The first three levels supply

`h L [u+(u-1)/2+(u-2)/4] > 4 M L`,

when h>0,u>=3,hu>=3M. The bracket inequality has a Lean proof,
RationalScalar.three_level_margin. This is a numerical multiplicity lemma;
it is not by itself a finite-precision recovery proof. The existing source
recovery analysis still needs to be connected to the proposed evaluator.
[pcc:unverified]

## Selected successor: common rational denominator

Use `D(T)=sum_s B_s(T)` and `U_d(T)=B_d(T)^2/D(T)`.
Differentiating C(T)=product_s(T-t_s) gives `D=C'/C'(0)`.
At a source node D(t_s)=rho_s, a nonzero two-adic unit, so U preserves every
node value. RationalScalar.denominator_node, denominator_node_unit and
node_preserved prove these actual node statements. [pcc:unverified]

For a homogeneous contraction with exactly M kernel factors per term,
`D^M sum_j w_j product_k U_(j,k) = sum_j w_j product_k B_(j,k)^2`.
RationalScalar.clear_homogeneous_sum proves the general finite-sum identity
under the explicit nonzero-denominator and equal-cardinality hypotheses.
The cleared numerator degree is at most M(4L-2); a generic polynomial degree
bound is proved, but the actual puncture-polynomial/evaluator bridge remains
outside that formal theorem. Clearing does not introduce the extra
polynomial-normalizer degree. [pcc:unverified]

Also `U_d*(R D)=W_d`. If R and D are units at a fixed chart value, all Newton
coefficients differ by the same unit and the Smith module is changed by an
invertible scalar. This gives equal valuation profiles at that value. The
completed experiment checked R, not D as a new registered candidate; do not
present its scalar counts as measurements of the rational successor.
[pcc:unverified]

### Written chart-unit argument, not yet formalized

Fix node t and alpha=2L. For u!=t, `v2(t-u)<=a`, so
`alpha/(t-u)` is an even two-adic integer. Write

`B_t(t+alpha z)=rho_t product_(u!=t)(1+alpha z/(t-u))`.

For s!=t,

`B_s(t+alpha z)=rho_t [alpha z/(t-s)] product_(u!=t,s)(1+alpha z/(t-u))`.

Thus B_t has unit constant coefficient and even nonconstant coefficients;
all coefficients of every other B_s are even. D has constant rho_t and even
nonconstant coefficients. Its inverse formal series is integral because the
constant is a unit. The same argument applies to R, whose constant is
1/rho_t and whose weights rho_s^-2 are units. This is a coefficientwise
argument over the local integral ring, stronger than a pointwise test.
It is presently a written proof obligation, not a completed Lean series theorem.
[pcc:unverified]

A direct construction of D at a point uses a product and its derivative:
start P=1,Q=0, and update Q<-Q(T-t_s)+P before P<-P(T-t_s).
That takes O(L) ring operations, not a free oracle. Truncated-series
construction and reciprocal, denominator clearing, interpolation and every
recursive entry call must all be charged before a total cost improvement.
[pcc:unverified]

## Origin and jet completion

Zero is a source node. A repair preserving all source-node values necessarily
retains the identity at zero. CommonScalar.matrix_origin proves this for the
actual polynomial candidate; jet_origin_obstruction proves W(0)+C(0)H(0)=W(0)
when C(0)=0. Rank-directed H may alter higher jets, but it cannot replace that
identity contribution while preserving the zero-node value. The rational
successor also retains this origin. [pcc:unverified]

## Audit trail and next proof

Source555cd9430 and freeze12e310bda were published before both process starts.
Export has two physical rows and no valid clocks; canonical JSON schemas and
explicit study-scoped binding checks pass after metadata corrections.
The first export omitted required byte lengths; its exact failure survives.
A premature analyzer call before replay completion failed byte comparison;
only analysis was rerun after actual completion. No third panel occurred.
[pcc:unverified]

The selected next obligation is to formalize the chart-series unit argument,
then bind homogeneous denominator clearing and the full-node recovery lemma
to the actual recursive contraction program, with all construction costs.
PLAN.md fixes the milestones and rejection tests. No leaderboard or runtime
comparison is promoted by these component results.

Sources: prior study RESULTS_local_20261107.md and exact native code;
prime_common_kernel_20261030/sources.md and its pinned D.4 primary sources;
prime_tensor_variants_20261030/reviews/matched-recovery-independent.md;
Mathlib polynomial big-operator and degree lemmas at the pinned revision;
canonical experiment-protocol skill/schemas. KB searches unavailable127,
repository/atlas fallback. Novelty and literature-exhaustion not certified.
