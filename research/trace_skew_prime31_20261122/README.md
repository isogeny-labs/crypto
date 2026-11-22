# Trace–skew radical pair image for Prime31 Semaev systems
Date: 2026-11-22. Status: **new candidate presentation, exact mathematical lemma and independent arithmetic replay; original Rust/CI and solver-speed measurements pending**.

## Generalized m-fold symmetric trace-moment equations (new 2026-11-22 result)

The two-generator pair system below extends to **every fixed arity m < char(k)**. This targets the m=4 (S5) prime-field index-calculus path directly, without expanding independent ordered factor-base constraints.

Let W be B pairwise distinct k-rational field elements, let h(T)=prod_(w in W)(T-w), and let q(T)=prod_(i=1)^m(T-x_i)=T^m-e1*T^(m-1)+...+(-1)^m*e_m. Define m polynomial equations in the symmetric coefficients e1,...,em:

```text
H_j(e1,...,em) := SUM_(i=1)^m x_i^j * h(x_i),       j=0,...,m-1.
```

These are bona fide polynomials in the e's by the fundamental theorem of symmetric polynomials. **Exact set-theoretic theorem:** over the algebraic closure, if characteristic p=0 or p>m, the common zero set of all H_j is exactly the image of the multisets of m elements of W. It contains C(B+m-1,m) points, including every repeated-root diagonal and NO points with x_i outside the factor base (even after extension-field splitting).

**Proof.** Group the roots x_i by their t distinct values z_1,...,z_t with multiplicities n_l <= m. The first t equations read

```text
V(z_1,...,z_t) * (n_1*h(z_1),...,n_t*h(z_t))^T = 0,
V_(j,l) = z_l^j, j=0,...,t-1.
```

V is invertible over the algebraic closure because the z_l are distinct. Since 0<n_l<=m<p, each n_l is invertible and h(z_l)=0. Thus every z_l is in W, proving sufficiency and completeness. Conversely tuples in W satisfy every H_j.

**Stronger simple-root Jacobian lemma:** at an image point of the multiset (x_1,...,x_m), with distinct-root multiplicities n_w, the Jacobian of the m H equations with respect to e1,...,em is

```text
det( dH_j / d e_k ) =
   (-1)^[m(m-1)/2] * (PRODUCT_w n_w!) * (PRODUCT_(i=1)^m h'(x_i)).
```

The right-hand side is nonzero for squarefree h and p>m, so the ideal (H0,...,H_(m-1)) is radical; it is a zero-dimensional complete intersection. A proof is obtained by differentiating the power sums and taking the confluent Vandermonde limit at coinciding x_i: ordinary Vandermonde factors cancel the e-coordinate Jacobian, while the coalescent w-block contributes n_w! times h'(w)^{n_w}. **Status:** algebraic proof and symbolic/exact modular tests, not Lean-checked. The determinant formula requires the precise equation order H0,...,H_(m-1) and coordinate order e1,...,em.

**Degree-B triangular preconditioning (exact, stronger than raw moments):** The naive H_j are of ordinary total degree B+j in the elementary coefficients, which is an unnecessary and potentially devastating F4 penalty. Let e_0=1 and define instead

```text
G_j(e)=SUM_(a=0)^j (-1)^a e_a H_(j-a)(e), j=0,...,m-1.
     =(-1)^j SUM_(i=1)^m h(x_i) e_j(x_1,...,x_(i-1),x_(i+1),...,x_m).
```

The transformation H→G is lower-unitriangular over k[e], and its polynomial inverse exists; **the generated ideals and complete zero schemes are IDENTICAL** (no saturation or missing repeated roots). At every common root the derivative matrix transforms by the same unitriangular matrix, so the Jacobian determinant and radical proof above remain valid.

**Theorem: each G_j has ordinary total degree in e1,...,em at most B (for B>=1).** Proof: each summand of G_j arising from h's monomial x_i^k has degree at most max(k,1) in *each individual x variable*, so the whole symmetric polynomial has individual degree ≤B. A symmetric polynomial in m variables with degree at most B in each x_i is a polynomial in elementary symmetric e1,...,em of ordinary total degree at most B. To prove the latter rigorously: polynomials of total e-degree≤B map injectively into the separately-degree≤B symmetric subspace, and both spaces have dimension binomial(B+m,m): respectively monomials of total degree≤B in m e's, and orbits of monomials with each x-exponent≤B. Consequently the injection is an isomorphism. This works over arbitrary coefficient rings where the fundamental theorem of symmetric polynomials applies; characteristic >m remains necessary for zero-set/radical exactness.

For the original S3 pair (m=2), G0=H0 and G1=H1−e1 H0; the pair S3 equation becomes **quadratic** in (e1,e2), compared with its **quartic** ordered-coordinate S3. At m=4, this delivers four radical membership equations of degree ≤B in four symmetric variables, with no need to first compute the larger B+1-element DRL radical basis from the earlier experiment. It may reduce input-system size, **not yet the actual Gröbner solving degree or total work**.

**Independent SymPy exact expansion, not Rust execution:** for the fully split test h(T)=prod_(w=1)^B(T-w), m=4:
| B | naive H0,H1,H2,H3 degrees | preconditioned G0,G1,G2,G3 degrees | monomial counts in G0/G1/G2/G3 |
| ---: | --- | --- | --- |
| 3 | 3,4,5,6 | 3,3,3,3 | 7/8/7/5 |
| 4 | 4,5,6,7 | 4,4,4,4 | 12/13/12/8 |
| 6 | 6,7,8,9 | 6,6,6,6 | 27/31/29/19 |
| 8 | 8,9,10,11 | 8,8,8,8 | 53/62/59/39 |

The same degree flattening was checked symbolically for m=2 and m=3 at B=3,4,6,8. The existing source now implements `reduced_trace_moments` including the exact Jacobian adjustment and F17 root-equivalence checks. This is an **input-presentation degree reduction**; it cannot be promoted to a measured F4 or complete ECDLP speedup until native execution and matched controls.

**Division-free implementation:** Let P_n=SUM_i x_i^n. Newton identities give P0=m; for 1<=n<=m, P_n=sum_(i=1)^(n-1)(-1)^(i-1)e_i P_(n-i) + (-1)^(n-1)n*e_n; for n>m, P_n=sum_(i=1)^m(-1)^(i-1)e_i P_(n-i). Then H_j=sum_(k=0)^B h_k P_(j+k). The new [Rust original-crate example](../../examples/prime31_trace_moment.rs) implements this recurrence and its exact forward-mode derivatives, determinant and factorial formula; m=2,3,4 are tested. It uses neither square-root extraction nor inversion in equation/Jacobian **evaluation** (test-only determinant certification uses field inversions). For fixed m, evaluation has O(B) field complexity; **finding solutions** to these equations is not thereby O(B).

**Independent numerical evidence in this run** (Python modular-arithmetic verifier, not Rust): exhaustive all F17 coefficient tuples e in F17^m, W={0,1,3,5}, m=2/3/4: respectively 289 / 4,913 / 83,521 assignments, with exactly 10 / 20 / 35 expected solutions and every solution's Jacobian nonzero. Separate positive multiset/closed-form-Jacobian replay: F101 with B4 and B5 for m2..4 (10,20,35 and 15,35,70 cases), and F2147483647 with W={0,1,2,4,6,7,8,10} for m2..4 (36,120,330 cases), all vanish and match the determinant formula exactly. These last field values are merely algebraic factor-base values, not necessarily all x-coordinates of curve points; elliptic-curve admissibility remains a separate solver gate.

**Exact Semaev use.** For m=4, the polynomial S5(x1,x2,x3,x4,x_R) is symmetric in the first four coordinates and therefore expressible as S5(e1,e2,e3,e4,x_R). Combine the four H_j membership equations with S5 in symmetric coordinates, or test the split-pair version below with two pair-image systems plus three chained S3 equations. Both require actual signed-point and possible intermediate infinity witness checking; neither has been benchmarked. Symmetric S5 materialization and F4 can remain exponentially costly in m or degree B. This result does **not** establish a faster relation oracle or solve the prime-field ECDLP.

**Literature originality boundary:** using symmetric coordinates, Newton sums and reduced unordered images is established algebraic technique; the repository's September radical-pair work is particularly close. The exact presentation above is new research work in this PR, but mathematical priority and any asymptotic speed advantage are **unverified**.

## Research question and overlap
Can we eliminate the once-per-base radical Gröbner preprocessing of the existing unordered-pair image, while retaining its low-dimensional, **reduced** scheme and the same exact Semaev relation witnesses? This is a different question from PR #4920 (signed-root table), #4921 (FASS symmetric-chain protocol), #4922 (shared F4 elimination), #4891 (basis-specific F4), #4901 (cold Prime31 solver), and the existing `research/radical_pair_image_20260921/RESULTS.md`. The latter already benchmarked radical vs direct presentations for small fields and reports a strong **solver-stage** improvement: at base size 11 on a GF(29) holdout, cold batch time ratio 0.0967. It also warns that the previous *two norm equations* carry nonreduced multiplicity. That finding is a primary incumbent and disqualifies a novelty claim for mere elementary symmetrisation.

The present candidate is **not** the norm-product pair. It gives two explicit recurrence-generated *trace–skew* equations whose affine ideal is radical. Whether a two-generator preprocessed-free representation beats the earlier `b+1`-generator DRL radical basis in F4, or the best complete IC pipeline, is **open**. No external mathematical novelty claim is established.

## Exact two-generator radical theorem (written proof, not Lean-checked)

Let `k` be any field of characteristic != 2, `W subset k` a finite set of `B` **distinct** elements and `h(T)=prod_(w in W)(T-w)`. Write `s=x+y`, `t=xy`, `Delta=s^2-4t=(x-y)^2`, where `x,y` are formal roots of `T^2-sT+t` over an algebraic closure of `k`. Define two symmetric polynomials
```text
A(s,t) = h(x)+h(y),
K(s,t) = (x-y)*(h(x)-h(y)).
```
These are genuine polynomials over `k[s,t]`. Then the affine common zero set of `A,K` over the algebraic closure is **exactly**
```text
{(w_i+w_j, w_i*w_j) : w_i,w_j in W, i<=j},
```
with `B(B+1)/2` points, and the ideal `<A,K>` is radical.

**Zero-set proof:** if `x!=y`, `K=0` forces `h(x)=h(y)`; `A=0` and char!=2 imply both vanish. If `x=y`, `A=2*h(x)=0` implies `x in W`. Conversely every unordered pair of roots yields `A=K=0`. Because `h` splits into simple linear factors, roots of the corresponding quadratic are rational in `k` at all solutions, so quadratic splitting introduces no extra extension-field solutions.

**Reduced-scheme proof:** use the Jacobian determinant of `(A,K)` with respect to `(s,t)`. At an off-diagonal pair `(x,y)=(w_i,w_j), i!=j`, the change from `(x,y)` to `(s,t)` has determinant `x-y`, and differentiation yields
```text
det( d(A,K) / d(s,t) ) = -2*h'(w_i)*h'(w_j).
```
At a diagonal point `s=2w, t=w^2`, use the local parameters `(s,Delta)`: `A=2*h(s/2) + O(Delta)`, `K=Delta*h'(s/2)+O(Delta^2)`. Since `det(d(s,Delta)/d(s,t))=-4`,
```text
det( d(A,K) / d(s,t) ) = -4*h'(w)^2.
```
Both are nonzero since h is squarefree and char!=2. Thus every zero is a simple transverse complete-intersection point. The zero set is finite, so the Jacobian criterion gives a zero-dimensional radical ideal. This is a written algebraic proof, **not** a Lean kernel audit.

## Division-free implementation

Set `P_k=x^k+y^k`, `Q_k=(x-y)(x^k-y^k)`. Then
```text
P_0=2, P_1=s;    Q_0=0, Q_1=Delta
U_(k+1) = s*U_k - t*U_(k-1), for U=P or Q.
A = sum_k h_k P_k; K = sum_k h_k Q_k.
```
The two equations, and their **exact** partial derivatives, are evaluable in O(B) field operations and O(1) working storage using forward-mode jets. No inversions, square roots, term-order conversion or resultant expansion are used at this stage. `examples/prime31_trace_skew_pair.rs` imports the repository's `semaev_prime_poly::{Fp,Curve}` and includes the checks below.

The relation system under test is
```text
A(s,t)=K(s,t)=0,       S3_sym(s,t,u)=0,
h(x3)=0,               S3(u,x3,xR)=0,
```
with `S3_sym(s,t,z)=(s^2-4t)*z^2 -2*(s*(t+a)+2b)*z+(t-a)^2-4*b*s`. This uses two invariant base coordinates rather than two ordered x variables. A diagonal factor-base pair is retained. A finite `u` chart **omits** an intermediate infinity sum (P + (-P) = O); the full relation generator must add an explicit O branch and verify signed actual group sums. Merely solving the polynomial system is not a validated ECDLP relation without that verification.

## Independent arithmetic replay in this run

A separately written Python modular-arithmetic verifier evaluated the recurrence and its differentiated Jacobian, independent of the Rust source, with these results:

| Field | Factor-base size | Scope | Observed |
| --- | ---: | --- | --- |
| F101 | B=1,2,4,6 | Exhaustive all 101^2 `(s,t)` per base (40,804 membership evaluations) | Exactly 1,3,10,21 expected image points; 35/35 Jacobians nonzero, matching the proved formula |
| F2147483647 | B=32 | First 32 rational x abscissae on pre-existing E:y²=x³+847514846*x+2087444724 | All 528/528 unordered pairs vanish; 528/528 Jacobians nonzero and formula-correct |
| F2147483647 | B=32 | 5,000 off-base control pairs `(s,t)=(1000+i,10000+3*i)` | 0 false accepted points (their `s` lies outside the possible pair-sum range) |
| F2147483647 | B=32 | Same 528 pairs, `z=625037715` | 528/528 equality between direct S3 and symmetric S3 formula |

These are independently executed **Python arithmetic checks**, not original-source Rust tests, no F4 timing and no complete DLP results. They do not certify the prior fixture's complete group order; use the existing verified curve fixture and subgroup checks for actual IC work.

## Pre-registered next measurement / pass-fail

1. Run `cargo test --release --locked --example prime31_trace_skew_pair` and `cargo run --release --locked --example prime31_trace_skew_pair` in an original-repo Rust toolchain. Verify 101-field exhaustive equality, nonzero Jacobians, Prime31 S3 identity and 5,000 negative controls. Current runtime has **no rustc/cargo**, so this is **not yet passed**.
2. On the **same** ordinary Prime31 fixture and matched base W, compare exactly five algebraic presentations with the same F4 engine: (i) original ordered `h(x1),h(x2)`, (ii) prior norm-product two-equation image, (iii) this trace–skew radical pair, (iv) previously implemented full radical DRL image, (v) non-polynomial signed-root / balanced join control. Include full target panels and exception charts. Retain identical exact relation witnesses, rank, limits, and order/pricing.
3. Sweep B=4,8,16,32 on Prime31 plus smaller-prime holdouts for any feasible F4 windows. Report cold preprocessing, graded matrix dimensions, solving degree, per-target wall/CPU and peak RSS; all timeouts and failures remain in the denominator. Baseline is the best previous radical and F4 method, **not** naive brute force.
4. Only if the trace–skew system is solver-stage competitive, integrate the existing Rust IC relation collector and verify authentic labelled independent rows, full rank/target-specific quotient rank, signed witnesses, complete unknown-target log recovery, and matched optimized negation rho and best project IC costs.
5. A speedup in asymptotic exponent needs a **growing prime-field family**, end-to-end slope bounds with uncertainty and a proven or defensibly analysed sub-generic decomposition algorithm, not a constant-size p31 speedup. Neither theorem nor performance evidence here establishes it.

## Critical asymptotic obstacle and a compact quadratic-circuit alternative

The degree-B theorem is **NOT** an efficient materialization bound. A generic polynomial in m elementary variables of total degree B can have up to C(B+m,m) monomials. For m=4, the upper-bound support sizes are 495 at B=8, 58,905 at B=32, 814,385 at B=64, 12,082,785 at B=128, and **8,882,684,790 at B=677**. Not every G_j reaches that support, but a dense expanded F4 input at index-calculus-sized B can be prohibitively expensive. If the actual expansion costs Θ(B^4), even the optimistic m4 model with B≈p^(1/5) incurs p^(4/5) **cold preprocessing**, worse than generic rho. This is a conditional *method obstruction*, not a lower bound on the best algebraic decomposition algorithm.

The proposed alternative exposes the **Newton recurrence itself** as a sparse quadratic circuit. Introduce fresh variables `U_1,...,U_(B+m-1)`, fix U0=m, and impose

```text
for k=1,...,B+m-1:
  U_k = SUM_(i=1)^(k-1) (-1)^(i-1) e_i U_(k-i) + (-1)^(k-1)*k*e_k  if k<=m;
  U_k = SUM_(i=1)^m     (-1)^(i-1) e_i U_(k-i)                       if k>m.
for j=0,...,m-1:
  SUM_(k=0)^B h_k U_(j+k) = 0.
```

All equations have total degree ≤2, contain O(mB) monomial occurrences for fixed m, and are an **exact graph lift** of H_j=0 because the triangular U recurrence uniquely determines every U_k from e. They introduce O(B) additional variables and constraints, however, so ordinary generic F4 may still be much worse. A structured recurrence-elimination or black-box engine that avoids both the Θ(B^m) expansion and huge F4 matrices would be necessary for an asymptotic advantage.

The original-crate Rust [F4 pilot](../../examples/prime31_trace_f4_pilot.rs) includes this as an additional `--circuit` arm in the m2 planted S3 comparison, with the same original F4 backend, bounded timeouts, full equation setup and exact planted power-sum verification. **Neither compile/test evidence nor a measured speed ratio exists yet**. The next discriminating experiment is expanded degree-B G versus quadratic circuit at B=3/4/6/8, then m4 chained S3 and finally scalable structured elimination, rather than materializing massive B677 polynomials.

## Original-crate Rust F4 pilot: actual S3 relations and m4 chains (not yet executed)

[examples/prime31_trace_f4_pilot.rs](../../examples/prime31_trace_f4_pilot.rs) is checked in against the repository's existing `crypto_lib::cryptanalysis::f4_fp::{f4,F4Options,normalise}`, not a fresh CAS or Python performance harness. It symbolically generates all ordinary-`Fp` Newton moments and unitriangular degree-B reductions, verifies `max(total_degree(G_j)) <= B`, converts to the **same Grevlex F4 backend**, and records per-arm input degree, term count, matrix rows/cols, `solving_degree`, `degree_reached`, field operations, setup time, F4 time, timeout and degree cap. **The code has not been compiled or run in original Rust; no observed ratio is asserted.**

Supported experiments:

| Command option | Source curve and target | Matched arms | Gate / limitation |
| --- | --- | --- | --- |
| default | Prime31, existing cofactor-one ordinary fixture, B=3/4, planted **G+2G=3G** with x(G)=625037715, x(2G)=1923055775, x(3G)=265866980 | Ordered `h(x1),h(x2),S3(x1,x2,z)` versus symmetric raw H0,H1,S3(e1,e2,z) versus symmetric degree-B G0,G1,S3(e1,e2,z) | Exact same signed relation, original direct S3 degree 4 vs pair-symmetric S3 degree 2; all solutions/group signs still require replay |
| `--m4` | Prime31 B=3/4, four-factor **membership-only** | Ordered 4 h equations vs raw H0..H3 vs degree-B G0..G3 | Algebraic instrumentation only, **not** S5 relation solver |
| `--m4-s3` | Prime31, B=4 selected base x(G),x(2G),x(3G),x(4G), genuine planted **G+2G+3G+4G=10G** with u=x(3G),v=x(7G) | Ordered six-variable chain `S3(x1,x2,u),S3(x3,x4,v),S3(u,v,z)` plus four h; versus the exact same 6-variable chain using pair invariants (s12,t12,s34,t34,u,v) and 4 raw H or 4 degree-B G constraints | Original unspecialized auxiliary u,v; each arm has its own bounded F4 budget; F4 may time out, handle infinity charts separately in a complete solver |

The planted m4 S3 identities have been independently checked with Python big-integer arithmetic modulo p=2147483647 (all three equal zero), as well as the published project fixture group multiples. **This is a planted correctness seed, not an unbiased factor-base relation-yield experiment**; it must be followed by exact small-field full zero-set checks, unknown-index uniform target studies, independent target relation rank, original Rust execution and matched best prior-DRL/rho. Stop censored F4 cells rather than treating a timeout as a solved system. Do not use single-target F4 time as a complete ECDLP cost.

The `--m4-s3` chain tests a **pairwise radical quotient** (two pairs), not the global 4-invariant `G0..G3` plus expanded symmetric S5; both remain design choices with separate costs. The nearest repository incumbent is the September radical unordered-pair suite; comparison with its stored DRL basis and independent replay are mandatory before presenting any F4 speed result as competitive.

## Asymptotic decision boundary (explicit **heuristic**, not an attack theorem)

For a growing family of ordinary prime-field curves with subgroup order r≈p, suppose a factor base has B=p^β independently behaving x-values. In the *unsaturated* random-sum model a signed m-summand target has relation probability Θ(B^m/p); collecting Θ(B) **independent** rows needs Θ(p/B^(m-1)) target shifts. If an actually nonenumerative relation oracle costs Θ(B^α) operations per shift, and the algebraic relation matrix costs Θ(B^ω) operations (`ω≈2` optimistic sparse-linear-algebra surrogate), then the combined exponent is

```text
T_rel ≈ p * B^(α-m+1),    T_LA ≈ B^ω
τ_m(β) = max(1 + β*(α-m+1), ω*β),  for m*β<1,
β_opt = 1/(ω+m-1-α),   τ_opt = ω/(ω+m-1-α)
```

This is only valid where the probability/independent-rank model and the oracle assumptions both hold; it omits setup, failed decompositions, target descent, collisions and large-prime graph effects. In particular it **does not** prove a lower bound or optimal complexity for index calculus.

At ω=2 the model gives:

| Number of summands | Per-shift exponent α | Optimistic complete-relation+LA exponent | Assessment |
| --- | ---: | ---: | --- |
| m=3 | 1 (one factor-base scan) | 2/3 | Worse than ideal rho's 1/2 |
| m=3 | 0 (hypothetical B-independent oracle) | 1/2 | Merely matches rho |
| m=4 | 1 | 1/2 | Merely matches rho |
| m=4 | 0 | 2/5 | Potential exponent lever **if** actually realized and fully charged |

**Decision:** m=4 with a verified sublinear-in-B query oracle is the next high-value asymptotic target; m=3 and generic B^2 tables are needed as controls. The trace–skew pair presentation is only a mechanism to test for such an oracle; it has *not* been shown to make α<1. Measure the actual F4 scaling and maintain the stronger existing radical-DRL incumbent before claiming an algorithmic advantage.

## Literature / internal sources
- Semaev (2004), [Summation polynomials and the discrete logarithm problem on elliptic curves](https://eprint.iacr.org/2004/031); original S3 construction.
- Faugère, Huot, Joux, Renault, Vitse (EUROCRYPT 2014), [*Symmetrized Summation Polynomials: Using Small Order Torsion Points to Speed Up Elliptic Curve Index Calculus*](https://doi.org/10.1007/978-3-642-55220-5_3); explicitly uses elementary symmetric coordinates for Semaev equations and the smaller polynomial systems they yield. Their results depend partly on small-order torsion and extension-field factor bases. **Therefore symmetrizing Semaev itself is established prior art, not a novelty claim for this PR.** The investigational difference here is an explicit characteristic-controlled, radical, degree-B factor-base membership presentation G_j and the matched Prime31 F4 comparison.
- Kousidis–Wiemers (2019), [On the first fall degree of summation polynomials](https://doi.org/10.1515/jmc-2017-0022); first-fall degree is not a measured solving-degree or end-to-end complexity guarantee.
- `research/radical_pair_image_20260921/RESULTS.md`: existing radical symmetric pair results and norm-equation warning. `src/cryptanalysis/semaev_prime_poly.rs`: original Rust arithmetic. `research/fass_prime31_20261122/PROTOCOL.md` (PR #4921): related symmetric-S3 strategy.
- Sibling live PRs #4920, #4922, #4891, #4901.
- Shared research-KB MCP and ChatGPT Spaces both unavailable via this non-interactive runtime; GitHub default-branch research search and live PR inspection supplied the fallback. Novelty audit is **incomplete**.
