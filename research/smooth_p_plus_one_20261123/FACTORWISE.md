# Factorwise fine-fiber constraints

2026-11-25. Written argument and frozen finite-audit protocol.
Standing: producer-local, unsigned; formal proof and extraction remain open.

## Construction and prior work

Use the same qualified catalog and public factor bases as [FULL_CUTOFF.md](FULL_CUTOFF.md).
Let phi=T_h, h=d-3, with h=1 or2 in this component, and H_b(X)=phi(X)-b.
For an occupied catalog multiplicity m_b introduce a monic Q_b of degree m_b.
Replace the repeated-image coefficient pushforward by

\[
 H_b\bmod Q_b=0\quad\hbox{for every occupied }b,\qquad
 Q_\theta=\prod_b Q_b,
\]

where Q_theta is the unchanged monic RR5 norm quartic in theta=(alpha,beta,t).
There are four factor coefficients and three RR5 parameters. Monic division
gives four remainder equations; the quartic equality gives four more.
No tuple or sign list is supplied to an extractor by this construction.
The constructor is first exposed only to a finite witness audit.

This reuses polynomial divisibility. The lab's
[quotient-product addendum](../prime31_four_summand_20261122/QUOTIENT_PRODUCT_ADDENDUM.md)
already uses a monic cofactor to encode Q|h for the entire liftable base.
Here the divisibility is applied separately to low-degree map fibers, with
factor coefficients rather than a cofactor for the entire base. At h<=2 the
expanded remainders have degree at most four; this is a presentation change,
not a new factor-base construction or an established complexity result.

Petit–Kosters–Messeng [PKC2016, Algorithm1 and sections3.2/3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf)
already give composed-map and auxiliary-isogeny factor bases and separate
setup, relation-solving and linear-algebra costs. The
[first-collision study](../prime31_ic_20261115/native/DICKSON_FIRST_COLLISION_S3_20261115.md)
shows why all branches must be charged. Exact prior sources checked include
the catalog argument, current cutoff/parser and production cofactor adapter.
KB status and two factorwise/divisibility searches fail with unavailable
executable; this targeted review supports no exhaustive novelty assertion.

## Rational-point coverage on the existing open chart

Assume the terminal map fiber is separable, and restrict to four distinct
liftable abscissae, disjoint from the target abscissa, with nonzero y and
nonzero RR5 leaf denominators X+t. The existing catalog fixes m_b.

For a valid signed relation define Q_b as the product of X-x over its leaves
with phi(x)=b. Then Q_b divides H_b and their product is Q_theta. Conversely,
if the displayed equations hold, each Q_b divides a separable H_b. Thus its
roots are simple. Distinct H_b are coprime because H_b-H_c=c-b is a nonzero
constant, so the product is square-free and has the prescribed image multiset.
Extraction must still check rational roots, signed base membership,
target-disjointness and the group sum. For a surviving quartic its factor Q_b
is uniquely determined by the subset of roots over b. Hence the construction
neither loses nor duplicates the admissible signed relations of the old
chart. This statement concerns the guarded solution set, not equality of the
two unsaturated ideals. [pcc:unverified]

The full H_b may include roots outside the liftable base B. The capacity
catalog is built from B but does not remove those extra algebraic roots from
the new equations. Signed membership remains mandatory, as before.

## Tangent calculation at an admissible relation

Write H_b=Q_b W_b. Since H_b is square-free, gcd(Q_b,W_b)=1. A first-order
variation over k[epsilon]/(epsilon^2), with each factor monic, satisfies

\[
 0=\delta Q_b W_b+Q_b\delta W_b.
\]

Reducing modulo Q_b makes W_b a unit, so delta Q_b=0, since its degree is
less than m_b. Monic division defines W_b uniquely as a polynomial function
of Q_b; differentiating its exact quotient/remainder identity shows that
the same kernel calculation applies to the remainder equations. Their
Jacobian on the four factor coefficients therefore has rank four. This is
the usual separable divisor calculation, independent of the number of
distinct occupied image values. [pcc:unverified]

It remains to check the RR5 map itself. Let

\[
 F_\theta=(X-u)(\alpha+\beta X)+(Y-v)(X+t),\qquad (u,v)=-R.
\]

At a leaf P_i=(x_i,y_i), F(P_i)=0 and
F(-P_i)=-2y_i(x_i+t) is nonzero. The norm is a fixed nonzero multiple of
(X-u)Q_theta, with leading normalization independent of theta. If delta Q
vanishes, differentiating the norm at x_i gives delta F(P_i)=0. Also
delta F(-R)=0 identically. The variation delta F has pole order at most four
at O: it is a combination of (X-u), X(X-u) and (Y-v). Five distinct affine
zeros force it to vanish, so all three parameter variations are zero by
linear independence of these functions. Thus DQ_theta has rank three on the
stated open chart. [pcc:unverified]

In the combined system, the four remainder equations first force delta q=0;
the four product equations then force delta theta=0. Its eight-by-seven
Jacobian has column rank seven. The corresponding geometric point is isolated
and reduced: its local maximal ideal has zero cotangent space, and Nakayama's
lemma forces that ideal to vanish. This local conclusion does not cover
excluded points or assert that the entire unsaturated ideal is radical.
It also does not imply that the fixed separating forms or a bounded solver
will finish. [pcc:unverified]

For comparison, the old pushforward system has effective rank at most
min(3,r), with r distinct fine images. The new factor coefficients separate
that repeated-image differential. The prior F127 solver refusal alone does
not establish this rank defect as its full algebraic cause.

## Proof map and formalization status

| Statement | Written dependency | Lean status |
|:--|:--|:--|
| Guarded coverage and unique factors | Coprime H_b; monic divisibility | Open; no checked declaration |
| Remainder Jacobian rank four | Separable quotient and dual-number differentiation | Open; divisor/remainder differential bridge needed |
| RR5 differential rank three | Norm differentiation and five-zero pole bound | Open; normalized RR5 function/divisor bridge needed |
| Combined local reducedness | Tangent kernel and Nakayama | Open; coordinate-local-ring bridge needed |

The formalization dependency check found no installed lean/lake or local
crypto-lean checkout. The connected library metadata specifies Lean4.35.0
and Mathlib revision cfc55fc58714a59dc06e6e79edc6d1d95140d3c2.
Those metadata reads are not a pinned theorem environment or a build.
No Lean file, axioms receipt, signed proof or global radical certificate is
represented. The finite audit below checks the actual constructed equations;
it cannot replace the general argument or its outstanding formalization.

## Frozen finite audit

After source tests/build and a separately published executable identity, run
one Rust `factorwise-audit` for each of the existing F127 depth4 and F8191
depth4/depth5 fixtures. Each receives one worker,2GiB address space and
120 seconds plus five seconds termination grace. Attempt all three even if
an earlier one fails. Preserve stdout, stderr and exit status; no retries,
cap changes, solver calls, new target search or EKS/paid run are authorized
by this protocol. The finite targets are generated from signed leaf sums,
so this is a witness audit rather than relation collection.

For every unsigned tuple and sixteen sign choices, retain identity and
target-abscissa exclusions. On every remaining binding check the unique RR5
parameters, norm quartic, nonzero leaf denominators, factor equations and
actual formal Jacobian. Require fiber rank4, RR5 rank3 and combined rank7.
Evaluate the old cutoff Jacobian at the same binding and retain its rank
distribution. Reconcile every tuple/sign count with the preceding catalog
audit. Record per-chart polynomial term ranges and maximum input degrees;
these are construction sizes, not solver-work estimates.

Tests cover malformed chart refusal, all integer partition shapes supported
at h1/h2 with scalar value checks at roots, and a two-image relation whose
factorwise Jacobian has full column rank. Mutating each of its seven binding
coordinates must fail its equations. Tests do not supply positive outputs
to a relation oracle. No extraction or recovery method changes here.

Retain the report, figure sources, inspected PDF, source/binary identities,
checksums, local claim standings and distinct development-flow schema.
Future extraction requires a separately frozen full-output comparison with
the original guards and complete branch costs.
