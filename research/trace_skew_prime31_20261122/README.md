# Trace–skew radical pair image for Prime31 Semaev systems
Date: 2026-11-22. Status: **new candidate presentation, exact mathematical lemma and independent arithmetic replay; original Rust/CI and solver-speed measurements pending**.

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
- Kousidis–Wiemers (2019), [On the first fall degree of summation polynomials](https://doi.org/10.1515/jmc-2017-0022); first-fall degree is not a measured solving-degree or end-to-end complexity guarantee.
- `research/radical_pair_image_20260921/RESULTS.md`: existing radical symmetric pair results and norm-equation warning. `src/cryptanalysis/semaev_prime_poly.rs`: original Rust arithmetic. `research/fass_prime31_20261122/PROTOCOL.md` (PR #4921): related symmetric-S3 strategy.
- Sibling live PRs #4920, #4922, #4891, #4901.
- Shared research-KB MCP and ChatGPT Spaces both unavailable via this non-interactive runtime; GitHub default-branch research search and live PR inspection supplied the fallback. Novelty audit is **incomplete**.
