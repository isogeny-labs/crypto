# Joint sieve and sparse-label cost case

Derived2026-11-15 after the linear cold screen. This is a conditional algorithmic
cost case, not measured speed, new asymptotics or a novelty claim.

Primary source read: *Improved sieving on algebraic curves*, Section2.2/2.3,
https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf.
The point sieve and its double-large-prime use are known Sarkar--Singh/
Joux--Vitse methods. GTTD graph/cycle distinctions are in PROTOCOL.md.

## Sparse labels need no auxiliary unknown per sieve batch

Let D=[u,v]=aD1+bD2 be degree g. Set t=v-lambda*u and
Q_lambda=(t^2-f)/u; normalize its leading coefficient to obtain the residual
degree-g+1 Mumford polynomial. The principal divisor of y-t gives
D + sum_j epsilon_j P_j =0, when the residual splits and orientations and
multiplicities are correct. Thus after projecting to the ell subgroup,
sum_j epsilon_j L_j + b*k = -a. This row contains at most g+2unknown
entries (g+1places and k), irrespective of H and the number of batches.
The known scalar a is an RHS, not another unknown. Whole-Jacobian group
checks must precede projection; labels modulo ell alone do not certify them.

Therefore retaining an auxiliary unknown for every D is unnecessary when D
has two public challenge labels a,b. A walk in D1,D2 maintains those labels
with constant scalar work per step. Charge all jump construction and additions;
do not use the harness's known k to choose them. A factor-base-step walk would
instead carry growing anchor labels: it defeats this sparse-label advantage.
Marginal uniformity of D2+iD1 for a uniform challenge does not prove useful
independence across consecutive batches or rank. Those require tests/falsifiers.

At a small-base point (x,y), u(x)!=0, the bucket key consistent with t is
lambda=(v(x)-y)/u(x). Both signs share the same u inverse; y=0 contributes
one point only. Skip u(x)=0 explicitly. A key with at least g-1DISTINCT hit
abscissae gives known small roots, leaving degree<=2 to split. Repeated hits
are not distinct roots; verify division, multiplicities and orientations.
Construct the actual Q before admission. Low-degree D needs its own branch,
not silent use of the degree-g formula. Touch only populated buckets; an O(q)
scan or clear for every batch destroys the short-sieve cost case.

## Conditional supply calculation

Write B for the oriented small-point count (approximately2H in the absence of
ramification), q for field size. Under a uniform rare-key occupancy heuristic,
the number of buckets with g-1distinct hits is approximately

    q * (B/q)^(g-1) / (g-1)!.

After a conditional quadratic split probability near1/2, this predicts
Y approximately B^(g-1)/(2*(g-1)!*q^(g-2)) raw two-LP candidates per batch.
This is NOT an exact distribution: the two signed keys from one point,
shared divisor and algebraic correlations, duplicate relations, >g-1hits and
coincidences all matter. Count useful independent rows rather than these hits.

An O(H) small-base scan then gives O(q^(g-2)/H^(g-2)) work per raw partial.
If O(q)partials suffice for graph/rank completion up to logarithmic factors,
harvesting is O(q^(g-1)/H^(g-2)). At H=q^(1-1/g) this is O(q^(2-2/g)),
the established double-LP exponent. For g2 it is O(q), independent of H at
this level. This calculation supplies no new exponent or automatic 10xconstant.
Sparse LA with O(H)rows of bounded/polylog weight has nominal O(H^2)Krylov
work, but graph-expanded rows need not have bounded weight. A dense H^3core
instead defeats the classical balance for g3/g4. Charge actual row weights,
cycle-expansion work, Krylov iterations/restarts and rank failures explicitly.

## Combined cost and admission threshold

For K batches, admitted partials M, total expression construction/expansion W,
row nonzeros Z and algebra A, complete cold cost is

    C_catalogue + C_jumps + C_descents + K*C_scan(H)
      + C_candidates + C_dedup + C_forest(M,W) + C_LA(Z,A) + C_verify.

The lazy gate removes mandatory H-wide beta materialization on unions; it
does not bound W, Z or A. Its ordered-map overhead must remain in clocks.
The previous compact free-harvest bound3.62uses fixed downstream costs and
therefore cannot be applied unchanged to this different relation distribution.
Conversely, it cannot be evaded by omitting those costs from this expression.

Before another timing experiment: gate raw principal identities, challenge
labels, lazy/dense prefix emissions, projected rows and final k on identical
real curve streams; record candidate occupancy, unique relations, unions,
pins, pure rows, rank, W,Z and all allocations/caps. Freeze complete cold arms
against the fastest existing controls with A/A and both forest variants.
Reject a 10xcost case if observed retained yields/rank and charged nonharvest
work cannot fit within one tenth of that same-target control total. Preserve
all genus2/3/4losses; no target/curve selection using observed timings.

The next implementation is this known sieve as a strong baseline plus sparse
labels, not another equivalent rational arithmetic rewrite. Mathematical
novelty and a justified10xmechanism remain separate outstanding requirements.
