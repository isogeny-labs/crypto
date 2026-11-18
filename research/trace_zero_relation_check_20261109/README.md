# Does the trace-zero Weil restriction shrink relation systems?

The bounded check finds a small, explicit coordinate reduction; it does not
supply a smaller full trace-zero relation-finding system at matched parameters.
All new numerical and implementation statements here are locally checked
producer assertions, without signed independent standing [pcc:unverified].

## Three different objects

For ECC2K-130, use E_0/F2: y²+xy=x³+1, n=131, tau(x,y)=(x²,y²), and
r=680564733841876926932320129493409985129, the pinned subgroup order.
The challenge polynomial is z^131+z^13+z²+z+1.

1. The group trace S_n(P)=sum_{i=0}^{n-1} tau^i(P) takes values in E(F2).
   Its kernel T_n is the rational point group of the dimension-(n-1)
   trace-zero subvariety of Res(E). Since #E(F2)=4 and gcd(n,4)=1,
   the trace is onto: on E(F2) it is [n]. Thus
   |T_131|=#E(F_(2^131))/4=r. The challenge subgroup is already T_131;
   this removes the cofactor, not a large part of that subgroup
   [pcc:unverified].
2. Field trace Tr(x) is a linear bit. On E_0, Tr(x(P))=0 characterizes
   2E, not T_131=4E. A trace-zero x-subspace includes the order-two point.
   Full group-trace membership must not be replaced by this one bit.
3. Frobenius on linear field-coordinate spaces is governed by
   F2[t]/(t^n-1). At n=131, ord_131(2)=130, so invariant dimensions
   are 0,1,130,131. This concerns coordinate subspaces, not all nonlinear
   bases, all Z[tau] mechanisms or isogeny simplicity [pcc:unverified].

## What Z[tau] supplies here

For E_0, tau²+tau+2=0, bar(tau)=-1-tau, and

~~~
tau-1 = bar(tau)²
(tau-1) S_n(tau) = tau^n-1
Norm(tau-1) = 4
Norm(S_n(tau)) = #E(F_(2^n))/4.
~~~

The primary literature's exact sequence is
0 -> E(F2) -> E(F_(2^n)) --(tau-1)--> T_n -> 0.
On rational points Frobenius is bijective, and
[4]=tau² bar(tau)²=tau²(tau-1), so image(tau-1)=4E.
This is a concrete cheap map into T_n, but its image here is cofactor
clearing. These identities do not constitute a new relation oracle
[pcc:unverified].

The native audit checks ring norms against an independent point-count
recurrence at n=31,53,83,131 and the pinned n=83/131 subgroup integers.
No new universal theorem or Lean proof is claimed; the structural facts
are cited results. The Lean executable is unavailable here.

## A real, limited coordinate reduction

Keep the target, arity m, initial subspace V and allowed point base
B={P in T_n: x(P) in V} fixed. Let ell=dim(V), and let
epsilon=rank(Tr restricted to V), either zero or one. Replacing V
coordinates by coordinates of V intersect ker(Tr) preserves B exactly.

For an all-affine S3 chain, layout counts are

~~~
naive:                      m ell + (m-2)n
field-trace summands:        m(ell-epsilon) + (m-2)n
also affine intermediates:  m(ell-epsilon) + (m-2)(n-1).
~~~

Affine intermediate sums of points in T_n also have field trace zero.
This removes m epsilon+(m-2) variables. It does not encode infinity
intermediates: the known infinity-pattern obligation remains.
Nominal component rows remain (m-1)n, and Boolean degree bounds remain
2 for m=2 and 3 for a chain. Rows may cancel after substitution; these
are layout/upper-bound counts, not instantiated Groebner matrices or
measured solving degrees [pcc:unverified].

For the challenge polynomial basis, Tr(z^j)=1 exactly at j=0,129.
With V=span(1,z,...,z^16), ell=17 and epsilon=1:

| n=131, same B and target | naive variables | summand trace | also affine intermediates | nominal S3 rows |
| --- | ---: | ---: | ---: | ---: |
| m=2 | 34 | 32 | 32 | 131 |
| m=3 | 182 | 179 | 178 | 262 |
| m=5 | 478 | 473 | 470 | 524 |

These counts are derived and checked, not timings [pcc:unverified].
If V already lies inside ker(Tr), epsilon=0: no summand coordinate
disappears. For m=3 only one affine intermediate bit is removable.
Choosing a different ell-dimensional trace-zero base is not a matched-base
comparison unless its point set and membership cost are controlled.

## The full trace-zero construction at matched parameters

Gorla–Massierer section 3 chooses a one-dimensional factor-base curve by
setting n-2 x-coordinates to zero in the trace-zero variety. It leaves two
coordinates per point, imposes trace membership, uses m=n-1 summands,
and has 2n-1 equations in 2(n-1) variables.

At q=2,n=131 this literal base has at most four abscissae/eight affine
points. With m=130, its sums occupy at most C(137,7)=153802846692 points.
A uniform target in the pinned r-point group therefore has success
probability at most C(137,7)/r, below 2^-90. This is a bound for this
fixed finite base and fixed arity, not a lower bound for adaptive searches,
larger or nonlinear bases, or orbit-expanded bases [pcc:unverified].

Keep the same two-coordinate summands and m=130 when comparing encodings:

| Representation | coordinate + intermediate variables | equations |
| --- | ---: | --- |
| Direct S_131, naive matched coordinate base | 260 | 131 descended components plus required base membership |
| GM direct trace-zero formulation | 260 | 261, including 130 trace-membership equations |
| All-affine chained S3, same coordinates | 17028 | 16899 nominal S3 components plus required base membership |

The 17028-to-260 change removes chain auxiliaries and trades degree; it is
not a trace-zero dimension gain. Naive direct S_131 already has 260 variables.
Its formal per-argument degree is 2^129 and total degree 130*2^129.
These are unreduced Semaev degrees, not actual Boolean degrees at q=2,
which are bounded by the Boolean variable count. Compare actual degrees,
sparsity, lifting and solver cost before interpreting this trade-off
[pcc:unverified].

The paper explicitly fixes n in its asymptotic-in-q advantage and reports
exponential growth in n. Its practical demonstrations concern n=3 and 5.
That does not justify specializing a hidden constant to q=2,n=131 and
announcing an efficient relation finder.

## Native checks and evidence scope

~~~
cargo test --locked --manifest-path \
  research/trace_zero_relation_check_20261109/native/Cargo.toml \
  -- --nocapture --test-threads=1
sha256sum -c research/trace_zero_relation_check_20261109/SOURCES.sha256
~~~

Twenty-two local tests pass: four mechanism checks plus 18 reused production
field-arithmetic tests [pcc:unverified]. The small crate imports production
field and tally modules by path; it does not build the root cryptanalysis
library. SOURCES.sha256 pins those inputs; checks.txt retains raw output.
No timer or performance claim is used. Normal test-harness duration is not
a benchmark.

| field degree | #E | group-trace kernel | field-x-trace kernel |
| --- | ---: | ---: | ---: |
| 3 | 4 | 1 | 2 |
| 5 | 44 | 11 | 22 |
| 7 | 116 | 29 | 58 |
| 9 | 508 | 127 | 254 |

The audit checks image(tau-1)=ker(group trace), trace values in E(F2),
frozen-field irreducibility by Rabin's criterion, production trace
coordinates, same-base retention, norms and the multiset bound.
Finite checks are not independent signed verification [pcc:unverified].

## Result and next discriminating check

There is a small implementable elimination beyond orbit folding:
per-point field-trace coordinate reduction. This audit does not establish
a smaller matched full group-trace-zero relation system, lower solving
degree, higher yield or runtime gain at n=83/131 [pcc:unverified].

A stronger result needs an explicit T_n coordinate/membership construction
on a matched-size point base, followed by baseline/candidate polynomial
construction and lifting checks. Retain variables, nonzero ANF equations,
monomials, actual degrees, infinity patterns, symmetry, encoding/setup,
failures and solver work. Freeze source, seeds and resources before timing.
The existing checkpoint/EKS access gate blocks that separate benchmark.
No paid experiment ran; no leaderboard or benchmark record changes.
Other nonlinear Z[tau] mechanisms remain open.

## Sources checked

- Gorla–Massierer, [Index Calculus in the Trace Zero Variety,
  arXiv:1405.1059v2](https://arxiv.org/pdf/1405.1059), introduction,
  definitions 2.2–2.7, sections 3.2–3.3 and system (7).
- Gorla–Massierer, [Point compression for the trace zero subgroup over a
  small degree extension field, arXiv:1403.0126](https://arxiv.org/pdf/1403.0126),
  section 3: summation-polynomial membership, exceptions and compression.
- Galbraith–Granger–Merz–Petit,
  [On index calculus algorithms for subfield curves, SAC 2020](https://sacworkshop.org/SAC20/files/preproceedings/18-IndexCalculus.pdf),
  introduction and section 5. They distinguish the trace-zero proposal from
  invariant/orbit bases and compare equal-size bases.
- Kosters–Yeo, [Notes on summation polynomials](https://arxiv.org/pdf/1503.08001),
  Proposition 4.2, for the different F2-valued homomorphism Tr(x+a).
- Repository: RESEARCH_KOBLITZ_INDEX_CALCULUS.md; corrected hyperelliptic
  note; RESEARCH_ECC2K130_DECOMPOSITION.md; production f2m.rs and pinned
  n=83/131 fixtures. Historical timings were not rerun or used as new evidence.
- Sources checked (KB): MCP and wrapper executable unavailable; session
  log records the fallback sources. No novelty claim.
