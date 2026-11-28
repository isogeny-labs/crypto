# Full rational-point cutoff partition

2026-11-25. Frozen construction and finite qualification protocol.
The arguments and local checks have unsigned standing. [pcc:unverified]

## Scope and source comparison

Continue the user's smooth p+1 index-calculus objective by extending the
existing coefficient cutoff beyond the regular four-coarse-image chart.
The component is a target-independent catalog and a finite witness audit.
It supplies neither a new solver invocation nor a cost comparison.
The existing recovery modes, solver limits and historical records stay frozen.

Sources checked: Petit–Kosters–Messeng, [PKC 2016, sections 3.2/3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
for composed-map factor bases, auxiliary isogenies and the unresolved cost of
elimination; [REFINEMENT.md](REFINEMENT.md) and [REPORT.md](REPORT.md) for the
existing RR5/Graeffe construction; and the lab's
[first-collision study](../prime31_ic_20261115/native/DICKSON_FIRST_COLLISION_S3_20261115.md)
for the observed cost of covering every branch. This extends this lane's
implementation; the map construction and polynomial identity are prior art.
KB status and both cutoff/multiplicity searches failed because its executable
is unavailable. No exhaustive novelty review or new general mechanism is claimed.

## Rational-point partition argument

Let the existing factor base have a set B of distinct liftable abscissae in a
separable split fiber of T_d(X)=c, where T_j is the j-fold composition of
f(X)=X²−2. Put h=d−3 and phi=T_h. For each occupied fine image b, let
n_b count the distinct x in B with phi(x)=b. The catalog consists of

\[
 m=(m_b),\qquad 0\leq m_b\leq n_b,\qquad \sum_b m_b=4,
 \quad P_m(Z)=\prod_b (Z-b)^{m_b}.
\]

Every four-element subset of B determines one such multiplicity vector.
Conversely, choosing m_b distinct elements from each fiber realizes every
catalog entry. The fibers are disjoint, so these choices give distinct
abscissae. Unique factorization of monic polynomials distinguishes the P_m.
Thus the catalog partitions all unordered distinct-abscissa tuples in B.
The catalog is generated from capacities, without enumerating those tuples.
Tuple enumeration belongs solely to the finite checker. Signs do not affect
the assigned chart. [pcc:unverified]

For a monic quartic Q with roots x_i counted with multiplicity, pushforward
through phi gives the polynomial with roots phi(x_i), with multiplicity.
Consequently equality phi_*Q=P_m selects exactly that multiset. All four
original roots that survive extraction and signed-base membership then belong
to this chart. Retaining multiplicities is essential: replacing P_m by its
square-free part changes its degree and its constraint. No equality of
unsaturated ideals or preservation of local reducedness is asserted.
[pcc:unverified]

For a fixed nonidentity target R, restrict to four signed affine base points
whose abscissae, together with x(R), are distinct. Their sum is R exactly when
the divisor P1+P2+P3+P4+(-R)-5O is principal. The normalized RR5 function is

\[
 F(X,Y)=(X-u)(\alpha+\beta X)+(Y-v)(X+t),\qquad (u,v)=-R.
\]

The coefficient of XY can be normalized to one: otherwise a nonzero function
of pole order at most four would have five distinct zeros. A function with
this divisor is unique up to scalar. The norm quotient yields the quartic
with the four input abscissae. On this chart, a zero X+t at a leaf would also
make its opposite a zero of F, producing a sixth zero; the odd-order fixtures
have no nonidentity two-torsion. Therefore each such signed relation has a
valid parameter assignment in exactly one catalog chart. The converse still
uses extraction, four distinct roots, target-disjointness, signed membership
and group-sum replay. Identity targets and target-abscissa overlap remain
outside this four-leaf chart, as in Direct.reference. [pcc:unverified]

## Multiplicity and differential caution

The coefficient pushforward identity, writing Q(Z)=U(Z²)+ZV(Z²), is

\[
 f_*Q(X)=U(X+2)^2-(X+2)V(X+2)^2.
\]

The separate native certificate executable issues and replays this classical
degree-eight identity in root variables using the unchanged repository
`identity.certificate/v1` implementation. Its finite-field evaluation record
does not certify the abstract partition, RR5 argument or a cost result.

At a tuple whose fine images have r distinct values, columns of the
root-to-coefficient differential coincide within each equal-image group.
Its rank is at most r. Composing with the three RR5 parameters bounds the
effective Jacobian rank by min(3,r). Thus charts with at most two distinct
fine images cannot inherit the regular chart's rank-three argument. When all
four images differ, the earlier separability/RR5 argument gives rank three.
Three-image charts require inspection; no rank is prescribed for them beyond
the upper bound. The audit evaluates the actual system Jacobian and subtracts
the rank of the auxiliary graph equations. [pcc:unverified]

The number of charts is the coefficient of z^4 in
\(\prod_b(1+z+\cdots+z^{\min(4,n_b)})\), bounded above by
\(\binom{k+3}{4}\) for k occupied images. It can substantially exceed the
old at-most-sixteen regular charts. Any later solver comparison must charge
catalog construction, every branch, failed work, extraction, relation
collection and linear algebra. Smaller systems alone give no cost conclusion.
[pcc:unverified]

## Frozen finite qualification

Inputs are exactly the existing F127 depth-four and F8191 depth-four/five
fixtures. Their curves are y²=x³+x+7 with order109 and y²=x³+x+5 with order8053.
Base construction is unchanged. These are public development fixtures.

For each input, the Rust `partition-audit` command will:

1. Construct the capacity catalog, preserving repeated images; enumerate
   every unordered distinct-abscissa tuple and require exactly one chart.
   Reject duplicate, malformed, absent and empty charts.
2. Enumerate all sixteen sign assignments per tuple. Record identity and
   target-abscissa exclusions separately. Every remaining target is the
   actual sum of the four public signed points; no target logarithm is used.
3. Solve the four linear RR5 interpolation equations, require a unique
   consistent solution and nonzero leaf denominators, and compare the exact
   norm quartic to the direct product of four linear factors.
4. Build the existing cutoff system and its graph assignment from transformed
   roots. Require every equation to vanish. Evaluate its formal Jacobian,
   eliminate over the fixture field, and record effective ranks per chart.
5. Reconcile all signed bindings, require the differential upper bound and
   rank three for four distinct fine images, and include every old regular
   chart. Emit the complete catalog with tuple and rank counts.

The source tests separately exercise capacity saturation, omitted/duplicate/
altered charts, and refusal of unregistered fixture depths. Existing tests
remain required. The identity command uses32 statement-derived points,
16 additional points with fixed checker seed20261125, and a false-identity
mutation. All are producer-local; the existing checker is shared source.

Freeze source and exact binaries before the following command script runs
once. Each audit gets one worker, 2GiB virtual memory and120 seconds plus a
five-second termination grace. Every planned audit runs even if a prior audit
fails. The identity issue/replay commands get30 seconds each. No automatic
retry, solver call, cap increase, admitted timing, EKS submission or canonical
experiment-export validation is part of this component. Every exit and stderr
is retained, and output paths refuse replacement. Total planned wall allowance
is420 seconds plus termination grace. Any failure remains evidence and requires
a separately versioned correction before further qualification.

The full-cost comparison and original objective remain in
[the existing problem](../../problems/refined-rr5-cold-comparison.md).
