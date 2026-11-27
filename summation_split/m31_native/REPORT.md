# Native M31 projective constraints: component report

**November 23, 2026 Pacific | PR #5087 | producer-local, unsigned evidence.**

The component supplies exact local quotient arithmetic, matched polynomial
presentations and independent correctness checks for the next M31 solver
comparison. Actual matched F4 runtime, peak memory and the required 50
projected solver outputs per configuration remain unmeasured. [pcc:unverified]

## Scope and outputs

The coordinate field is `p=2147483647`, and the curve remains
`E: y^2=x^3-3x+21`. Original input and projected coordinates are in M31;
Fp2 is a witness representation. Every matched arm uses rank-two affine
forms in `t,u` followed by the same section `t(t-1)=u(u-1)=0`. Its four
points are M31 points. This is a bounded correctness workload, not a
natural factor-base scaling experiment. The factor-base map is unchanged.

- [Protocol and execution conditions](PROTOCOL.md)
- [Local quotient arithmetic](src/quotient.rs)
- [Matched system construction and guarded derivatives](src/systems.rs)
- [Bounded quotient F4 prototype](src/f4.rs)
- [Independent multiplicity gate](src/multiplicity.rs)
- [All-equation witness checks](src/system_certificate_tests.rs)
- [Multiplicity proof and limitations](multiplicity.md)
- [Native report renderer](report.rs), [PDF](REPORT.pdf), [editable SVG](report.svg)

Bounded native quotient F4 completes one m=3 geometric sequential case: 561/561 pairs, 10 matrices, maximum raw product and normalized matrix-column degree 6. A separate ordinary Buchberger checker validates the resulting ideal. [pcc:unverified]

This is a graded reverse-lexicographic reference prototype with simple
bounded matrices, not the optimized production engine. The single completed
case has 30 nonfixed basis polynomials. The independent ordinary checker
examines 561 candidate critical pairs, performs 528 ordinary pairs, checks
11 original generators and 30 reverse containments, and obtains a basis
with 33 polynomials. Two earlier verifier-budget failures remain preserved:
a 32-pair step cap that had been treated as a batch cap, and a 20,000-step
reduction cap. The producer was unchanged; the third bounded verification
completed. This is one actual degree-six case, not a 50-case projected panel
or a runtime/RSS comparison. The complete crate unit-test total is recorded
in the final integration receipt, not inferred here. [pcc:unverified]

## Distinct local correctness observations

| Observation | Count | Exact scope |
|---|---:|---|
| Specialized Semaev fibers | 500 | Full coefficient products, rational root sets and multiplicities |
| Positive tensor certificates | 792 | All generated equations on actual signed-point witnesses |
| Monolithic comparisons | 400 | Exact polynomial values on the complete common section |
| Coordinate partial checks | 750 | Mixed positive Semaev tuples |
| Coefficient mutations rejected | 500 | Specialized fiber corruption controls |
| Pure-sector cases with simple roots | 300 | Reject unconditional derivative constraints |

The specialized panel has 50 cases per sector/arity cell for pure plus,
pure minus and mixed positives at `m=3,4,5`, plus 50 forced infinity and
repeated-coordinate cases. It retains 4,630 finite signed branches and 90
infinity branches, with 3,240 rational roots. The tensor gate uses 50
fixtures in every `m=3,4` / three decomposed arms / two witness-mode cell.
The 400 monolithic checks are `2 arities * 50 planes * 4 section points`.
The all-equation certificates do not establish that a solver recovered all
projected roots, and are not counted as completed Groebner solves.
[pcc:unverified]

## The local quotient

For each geometric internal node,

\[
J=(z^2-z,zw-w,zs-s,w^2-s),\qquad (A,B,C)=(1-z+s,w,z).
\]

At `z=0` the chart is the identity `(A,B,C)=(1,0,0)`. At `z=1`,
`s=w^2`, giving the affine Kummer line. Thus

\[
\mathbb F_p[z,w,s]/J\cong\mathbb F_p\times\mathbb F_p[w].
\]

The standard monomials are `1,z,s^k (k>=1),w*s^k (k>=0)`.
Local normal form never increases total degree. This is a local algebra
fact; coupling the cubic tensors may still produce high global degree.
The exact relation to the prior preclosure appears in
[M31_LOCAL_PRECLOSURE.md](../M31_LOCAL_PRECLOSURE.md). [pcc:unverified]

The bounded F4 prototype must retain overlaps with the chart relations and
distinguish completion from resource exhaustion. Raw product degree,
normalized matrix degree and pair-LCM degree are separate telemetry.
Neither the quadratic generator degree nor a local first fall supplies a
bound on the whole system's solving degree.

## The guarded multiplicity constraint

For a mixed rational Semaev zero with every ordinate nonzero, the two
Frobenius-sector sums coincide in rational two-torsion. Each sector must
contain at least two leaves. Frobenius acts without fixed points on the
relevant sign classes modulo global negation, forcing even multiplicity
in every coordinate fiber. Consequently every coordinate partial vanishes.
The proof retains projective infinity branches. [pcc:unverified]

In the common section algebra, compute all leaf symbols, including the
target,

\[
e_i=f(x_i)^{(p-1)/2},\qquad
G=\prod_i e_i^2-\prod_i\frac{e_i^2+e_i}{2}-
\prod_i\frac{e_i^2-e_i}{2}.
\]

`G=1` on mixed nonzero sectors and `G=0` on pure or zero-ordinate
branches. The multiplicity-balanced arm appends

\[
\operatorname{NF}(G\,\partial_t S),\qquad
\operatorname{NF}(G\,\partial_u S).
\]

Differentiate the unreduced monolithic polynomial first, then reduce in
the common section. The added constraints preserve projected zero sets.
Equality of nonradical ideals or multiplicities is not asserted. The exact
gcd `gcd(X^3-3X+21,X^p-X)=1` excludes rational zero-ordinate leaves on this
frozen curve; this does not justify deleting that branch for other curves.
[pcc:unverified]

Pure-sector simple roots show why the guard is necessary. In particular,
ordinary factor-base points on `E(Fp)` and the rational target are pure
plus. A mixed-sector lemma is not an unconditional replacement for their
Semaev equation. Charge the guard, full monolithic construction, formal
derivatives and all added rows to the multiplicity-aware arm.

## Historical operation counts

The archived `m=5` generation counts are **8,424,950** for the previous
pair norm and **6,420,200** for Horner. With the common root-extraction GCD
included, the recorded ratios at `m=3,4,5` are **1.006x, 1.022x, 1.056x**.
These are historical field multiplication counts; this continuation does
not relabel them as new measurements or F4 speedups. See
[the archived Horner note](../M31_HORNER_FINDINGS_20261124.md).

## The next comparison remains required

| Arm | Included preparation |
|---|---|
| Monolithic Semaev | Complete fixed-degree polynomial construction |
| Preclosed sequential | Leaf lifts, local charts and sequential cubic tensors |
| Preclosed balanced | The identical leaves/charts with balanced tree shape |
| Multiplicity balanced | Balanced tensors plus full guarded-derivative preparation |

Run at least 50 actual projected solver outputs per configuration after
the execution conditions in [PROTOCOL.md](PROTOCOL.md) are met. Compare
native quotient F4 with the applicable original Rust engine and exact
sparse msolve F4 under matched order and resource assumptions. Retain full
projected solutions, raw/normalized matrix degrees, dimensions/nonzeros,
pair counts, complete preparation/solve/extraction costs, per-process peak
RSS and every censored attempt. No unmeasured value is entered as zero.

The algebra toolchain is available via workspace PR #109. Shared
Conductor/research-KB access, authenticated EKS lease/capacity information
and checkpoint custody remain execution dependencies. No paid comparison
job is claimed here. The four-point section alone cannot establish natural
factor-base scaling or complete ECDLP improvement. [pcc:unverified]

## Primary references and status

- [Diem 2011](https://doi.org/10.1112/S0010437X10005075), Proposition 3.2(c) and Lemma 3.5: finite-flat
  projective fibers and fixed homogeneous resultants.
- [Caminata-Gorla](https://arxiv.org/abs/2112.05579): distinctions among solving degree, last fall
  degree and related invariants.
- [Kosters-Yeo](https://arxiv.org/abs/1503.08001): first-fall pitfalls; its characteristic-two results
  are not M31 bounds.
- [Tracked source and evidence lane](https://github.com/isogeny-labs/crypto/pull/5087).

No novelty priority, Lean theorem, signed PCC verdict, measured runtime
advantage or measured memory advantage is established by this report.
