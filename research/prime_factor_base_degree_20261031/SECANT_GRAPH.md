# A positive degree reduction for split graph factor bases

2026-10-31. **Derived result:** for two summands, a completely split degree-d
graph intersection gives an exact secant presentation with homogeneous
regularity d+1, compared with 2d+1 for its dense S3 presentation. All four
sign branches are retained. This is a presentation-specific mathematical
reduction. External novelty, iterative F4 degree and end-to-end benefit are
not established. The secant/group identity itself is classical.

| Requested obligation | Status |
| --- | --- |
| A prime-field factor base yielding a lower precisely defined degree | Written theorem below, including an explicit d=2 instance: 5 to 3 |
| Complete membership, summation, signs and exceptional cases | Exact equivalence below for every finite target; identity target handled separately |
| Matched-size and same-encoding control | Explicit four-abscissa nongraph control: secant degree 4, dense degree 5 |
| Checkable algebra | Exact integer expansion, native certificate replay and a Lean monomial theorem; full geometric/polynomial bridge is written, not formalized |
| Newness relative to the literature | Unresolved; known chord/tangent and Riemann–Roch antecedents are explicitly credited |
| Experiment, F4 trace, natural yield, cost and scaling | Not run; no measurements inferred from the theorem |
| P-256 / 17-term application | Not constructed or proved by this two-summand theorem |

## 1. Invariant and hypotheses

Let k=Fp, p>3, and E: y²=f(x)=x³+Ax+B be nonsingular. Let h in k[x]
have degree d>=2 and nonzero leading coefficient a. Require

    h(T)²-f(T) = a² product(v in V)(T-v),      |V|=2d,                  (1)

with distinct V contained in k. This is an exact, squarefree complete-splitting
condition, not an assumption that a random polynomial splits. No field roots
or curve-lift filters are omitted. Define the full signed base

    F = {(v,h(v)), (v,-h(v)) : v in V}.

If h(v)=0, count that point once. In the explicit odd-prime-order fixture
all h(v) are nonzero, so there are 4d signed points and 2d columns.

For a specified generator list G, define delta(G) as the least D such that
the degree-D component of k[variables]/<top(g):g in G> is zero. This is
the homogeneous degree convention in THEORY.md. It is not bounded-Macaulay
refutation degree, actual iterative F4 solving degree, or a representation
invariant. In particular a change to the presentation must be disclosed.

## 2. Complete three-variable system

Fix an affine target R=(r,s) on E. For EACH (epsilon,eta) in {+1,-1}²,
use variables x,z,m and the four equations

    g1 = m²-x-z-r = 0,
    g2 = xz-r²-rm²-A-2ms = 0,
    qx = epsilon*h(x)-m(x-r)+s = 0,
    qz = eta*h(z)-m(z-r)+s = 0.                                    (2)

The slope m and both factor-base slots remain variable. There is no target-
dependent base construction and no discarded branch. With L(T)=m(T-r)-s,
write c=s²-r³-Ar-B. The integer polynomial identity is

    f(T)-L(T)²
      = (T-r)*((T-x)*(T-z)-T*g1-g2)-c.                             (3)

`native/secant_graph.rs` reuses the repository identity-certificate issuer
and checker. It also expands both sides exactly over Z; this additional
check proves the displayed polynomial equality by coefficient equality,
without a finite-field sampling hypothesis. The `identity.certificate/v1`
record separately retains the standard v1 evaluated tier (`proof: null`).
There is no claim that this identity is new or independently reviewed.
Its emitted identifier is **IDC1h621ed1a51b5f3ca8**, in
[`secant/identity.json`](secant/identity.json).

**Soundness.** On a root of (2), (3) factors the intersection cubic as
`(T-r)(T-x)(T-z)`. Equations qx and qz put P=(x,epsilon*h(x)) and
Q=(z,eta*h(z)) on that line. Evaluating (3) at x and z proves they are
curve points. Equation (1) then forces x,z into the exact rational set V;
no field equations or extra lift branch is needed. The line also contains
-R=(r,-s). Intersection multiplicities give P+Q+(-R)=O, hence P+Q=R.
If x=z the cubic has the required double root, so tangency is enforced;
mere collinearity without multiplicity would have been insufficient.

**Completeness.** Every P,Q in F with P+Q=R finite has a finite secant or
tangent slope m. A vertical pair sums to O and cannot represent this R.
The intersection cubic has roots x,z,r with their multiplicities; coefficient
comparison gives g1=g2=0. At least one of the four sign branches supplies
their y coordinates and therefore qx=qz=0. This includes repeated points
and occurrences with x=r. When R=O use exact inverse pairs separately;
no finite-target degree claim includes that special target.

## 3. Exact homogeneous degree

For d>=3, the top forms generate exactly

    J = (m², xz, x^d, z^d).                                      (4)

Indeed top(g1)=m² and top(g2)=xz-rm²; their span contains xz.
The remaining leading forms are epsilon*a*x^d, eta*a*z^d.
An uncovered monomial x^i z^j m^k has i,j<d, at least one of i,j zero,
and k<=1. Its total degree is at most d. The degree-d monomial
`x^(d-1)*m` survives. Therefore **delta=d+1**, with Hilbert series

    (1+t)*(1+2t+2t²+...+2t^(d-1)).

Lean checks the universal exponent-coverage and strict-reduction arguments
in `proof/SecantGraphTop.lean`. The bridge from polynomial ideals to these
exponent predicates, and the equivalence to the group relation, are the
written proofs here, not a claim of a full Lean formalization.

For d=2, the top ideal instead is

    (m², xz, x²-alpha*m*x, z²-beta*m*z),
    alpha=epsilon/a, beta=eta/a.

All degree-three monomials vanish: those containing m² or xz do so directly;
`m*x²=alpha*m²*x=0`, `m*z²=0`, `x³=alpha*m*x²=0`, and similarly z³.
The four degree-two generators are linearly independent in a six-dimensional
space. Thus its degree-two quotient has dimension two and **delta=3=d+1**.
The Hilbert series is 1+3t+2t². This quadratic case is written, not Lean checked.
The fixed F17 quadratic case additionally has a checked
[`upper/lower certificate`](secant/homogeneous-degree.json): for each of
the four branches it expresses every degree-three monomial as a linear
combination of generator multiples and supplies a degree-two dual functional
annihilating the top generators while evaluating to one on m*x. The native
checker replays these identities rather than trusting the claimed degree.

The same V in the exact dense system `F_V(x),F_V(z),S3(x,z,r)` has
delta=|V|+1=2d+1, by THEORY.md. Consequently the reduction is strict for
every d>=2 satisfying (1). One extra variable and four complete branches
are charged; the degree comparison does not prove a work improvement.

## 4. Explicit ordinary-prime fixture and control

The fixture record emits the exact ICV1 model/slug and the existing
`ecbench.factor_base_dump/v1` / FB1 identity preimage. Registry and EC1 joins
are explicitly pending, not invented. Its curve has p=17, A=B=2, full prime
order 19, and generator G=(5,1); the existing pilot tests check that curve's
order and group arithmetic. Take

    h(T)=4T²+9T+9,
    V={5,6,10,16},
    h(T)²-(T³+2T+2)=16(T-5)(T-6)(T-10)(T-16) over F17.           (5)

Its oriented points are G,2G,3G,13G, with coordinates
`(5,1),(6,3),(10,6),(16,4)`. Both signs remain in the dumped base.
The general proof supplies **dense degree 5 versus secant degree 3**.

The same-cardinality control has W={0,5,6,10} and oriented polynomial
`g(T)=8T³+6T²+7T+6`. It interpolates `(0,6),(5,1),(6,3),(10,6)`.
For this control retain `F_W(x)=F_W(z)=0` in addition to the four secant
equations; omitting them would admit unwanted roots of g²-f.
Its top ideal is `(m²,xz,x³,z³)` (the degree-four membership tops are
redundant), giving **secant degree 4**, versus dense degree 5.

This control cannot be made into a four-point quadratic graph by rechoosing
its signs: its points up to sign are G,2G,3G,7G. A quadratic graph with four
distinct abscissae has divisor of zeros summing to O. Every signed sum of
1,2,3,7 is an odd integer between -13 and 13; none is 0 modulo 19. Thus
this condition fails for every orientation. This uses the usual divisor/group
correspondence, not a new construction theorem.

The comparison now separates the dense-to-secant encoding change (5 to 3
on the candidate) from the same-encoding base comparison (4 to 3). It does
not assert optimality over all representations of the control.

## 5. Costs, novelty and next discriminating check

Four branches have to be processed; relations repeated across them must be
deduplicated and group replayed. The candidate uses three variables versus
two in the dense model. The number of all monomials through the claimed
degree is `binomial(d+4,3)` versus `binomial(2d+3,2)` in the dense model.
At d=2 these are 20 and 21, respectively, per branch/presentation; no runtime
ratio follows. For large d the first grows cubically and the second quadratically.
Complete splitting and constructing a useful large base have not been priced.

Primary sources read: [PKM 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
especially its composed-map factor-base definition; and the
[MIT 18.783 group-law derivation](https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf),
including tangency. Repository sources reviewed: the factor-base encyclopedia,
the 24-design register (including learned L(4O) coordinates), and the existing
PKM/symmetric implementations. Riemann–Roch/Nagao antecedents were identified;
full-text fetches of ePrint 2015/1022 and Faugere–Gaudry–Huot–Renault were
denied (403); the Vitse thesis fetch timed out. These are novelty-audit gaps.
Keyword searches found no exact prior statement of (4) with this comparison;
that is not evidence of literature-wide novelty. The exact delta theorem is
the component derived here; independent prior-art and mathematical review remain.

The next experiment must compare complete target populations on candidate,
same-V dense, and W secant/dense inputs; retain all four branches, doubles,
SAT/UNSAT cells, independent exact group replay, F4 degree, width and total
operations. First certify the fixed fixture, then preregister a growing d/field
ladder and holdouts. The native exporter emits the complete inputs, not results.
The existing S3 checkpoint gate failed again (no configured upload route), so
no experimental panel or timing was launched. Proof checking and certificate
issuance are deductive development work, separate from that pending experiment.
