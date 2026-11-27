# Factorwise fibers remove the audited tangent defect

2026-11-25 · `factorwise_v1` · producer-local, unsigned.

The separate-fiber equations have **full column rank at all 17,606 admissible
RR5 bindings** in the three frozen fixtures. The old pushforward has a
one-dimensional tangent space at 12/16/1,104 of those bindings, respectively;
the new system has zero tangent dimension at every checked binding. All tuple,
exclusion and old-rank counts reconcile with the previous catalog audit.
Claim PCC-20261125-5bc001 remains unverified. [pcc:unverified]

## Finite equation and Jacobian controls

| Fixture | Charts | Admissible bindings | Old rank-two bindings | New rank-seven bindings | New terms |
|:--|--:|--:|--:|--:|:--|
| F127, degree 16 |36|1,038|12|1,038|32–41|
| F8191, degree 16 |18|560|16|560|32–41|
| F8191, degree 32 |119|16,008|1,104|16,008|35–45|

The old systems use three/three/seven variables. The new presentation always
uses seven variables and eight equations, with maximum input degree four.
Rank seven is full column rank for the new presentation; the comparable
quantity across presentations is the tangent dimension. Counts and term
ranges are finite construction observations, not solver-work estimates.
[pcc:unverified]

![The same admissible witnesses are checked in both presentations. Labels give affected bindings over all checked bindings.](factorwise-ranks.pdf){width=100%}

All three planned audits run once and exit zero with empty stderr. They check
all 70/35/1,001 distinct-abscissa tuples and all sixteen sign choices per tuple.
Sixteen identity targets and 74 target-abscissa overlaps remain excluded by
the original chart. Every remaining binding satisfies the new equations,
fiber-Jacobian rank four, RR5-Jacobian rank three and combined rank seven.
The old equations and old ranks are evaluated at the same binding.
No algebraic solver is called. [pcc:unverified]

The targets here are generated from signed leaf sums. This is an exhaustive
witness audit on those finite bases, with shared catalog/interpolation/rank
source; it is not a relation-collection run or an independent implementation.

\newpage

## Mechanism and exact scope

For each occupied fine image \(b\), introduce a monic \(Q_b\) of its prescribed
multiplicity and impose

\[
 (T_h(X)-b)\bmod Q_b(X)=0,\qquad Q_\theta(X)=\prod_b Q_b(X).
\]

Distinct fibers are coprime and each is separable. Therefore each admissible
quartic has a unique factor assignment. This preserves the guarded relation
set and separates equal-image factors before taking coefficient constraints.
The entire map fiber can contain nonliftable abscissae; signed base membership
and group replay remain required. [pcc:unverified]

Writing \(H_b=Q_b W_b\), its infinitesimal factorization equation gives
\(\delta Q_b W_b=0\) modulo \(Q_b\). Coprimality makes \(W_b\) a unit, so \(\delta Q_b=0\).
The four factor coefficients are thereby fixed to first order. On the RR5
open chart, a norm-preserving parameter variation gives a function of pole
order at most four vanishing at five distinct points, hence is zero. The
remaining three parameters are fixed to first order as well. This establishes
local full column rank under the stated hypotheses. The full written argument
and proof map are in [FACTORWISE.md](FACTORWISE.md); theorem claim
PCC-20261125-d46228 is unsigned and has no checked Lean declaration.
[pcc:unverified]

![Separable factors supply the four coefficient constraints before the RR5 tangent condition.](factorwise-flow.pdf){width=100%}

The argument covers admissible relation points. Excluded points may still
affect the unsaturated ideal. The F127 solver refusal is preserved; this
audit does not determine its entire algebraic cause or demonstrate that a
fixed separating form will work. New rank and degree alone give no total-cost
bound. Formalization still needs the divisor/remainder differential, RR5
function/divisor and local-ring bridges listed in the proof map.

## Provenance and next step

Source **3747e09eae**, before-call identity **d0e5cc767b**. All 15 before-call
file hashes remain unchanged. The source passes 42 feature tests and 15 default
tests and builds; claim PCC-20261125-de166a is unsigned. Controls include
scalar remainder checks across supported partition shapes and rejection of
seven single-coordinate mutations of a two-image witness. [pcc:unverified]

[Raw records](evidence/factorwise_v1/), [summary](evidence/factorwise_v1/summary.json),
[reconciliation](evidence/factorwise_v1/catalog_reconciliation.json),
[reproduction](FACTORWISE_REPRODUCE.md). The prior-art comparison reuses the
lab's quotient-product divisibility, PKC 2016 composed maps/isogenies and the
first-collision cost warning; KB was unavailable. No general novelty,
global radical certificate, scalar recovery, EKS execution or timing ratio
is claimed. Earlier reports and performance graphs stay unchanged.

Next integrate this presentation into a separately frozen full-catalog
extraction panel with the same targets, deadline, completeness guard and
whole-answer comparison. Charge every branch before a cold recovery study.
The existing five-file artifact migration remains pending; these new raw
audit files are all below the policy threshold.
