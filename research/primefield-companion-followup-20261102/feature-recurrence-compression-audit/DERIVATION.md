# Replacing coefficient scans with residue-section powering

## Exact recurrence and its degree cost

For prime r, h=(r-1)/2, m>=3 dividing h, L=h/m, let c_j be the coefficient of X^(1+jm) in (X-1)^h. Then c_0=(-1)^(h-1)h and

    delta(j)c_(j+1)=alpha(j)c_j,
    alpha(J)=(-1)^m product_(t=0)^(m-1)(h-1-mJ-t),
    delta(J)=product_(t=0)^(m-1)(2+mJ+t).

For v in any commutative F_r-algebra, t_j=c_j v^j, S_j=sum_(i=0)^j t_i, use

    M(J) = [[alpha(J)v, 0], [alpha(J)v, delta(J)]].

The ordered product M(N-1)...M(0), N=L-1, applied to (c_0,c_0) equals the common denominator D times (t_N,S_N), where D=product delta(j). Each denominator factor is between2 and1+mN<r, so D is a nonzero scalar and remains invertible even if the quotient algebra has zero divisors. No inverse of v is needed. Triangular products compose as (a2*a1, b2*a1+d2*b1, d2*d1).

The b-step symbolic block M(J+b-1)...M(J) has denominator entry degree exactly mb in J, with leading coefficient m^(mb), nonzero modulo r. Here mb<=mN<h<r. Its other entries have degree at most mb. Thus a dense block representation has an mb degree cost, while applying b-step blocks needs ceil(N/b) block values. The parameter expression mb+N/b is at least2*sqrt(mN). This is a representation/work-count audit of this explicit strategy, not a lower bound on every recurrence algorithm or a measured runtime. In a prospective fast polynomial block/multipoint implementation, both degree and quotient-ring multiplication cost must be charged. The implemented audit deliberately uses simple convolution/Horner operations, not a fast BGS engine.

Bostan/Gaudry/Schost's primary section6 theorem is stated for degree-one matrix entries and counts base-ring operations. That theorem cannot be applied with degree m and quotient arithmetic treated as free. Source: https://cs.uwaterloo.ca/~eschost/publications/pollard.pdf, pp17-18, Theorem14; read2026-11-08. The first author-host URL timed out; this alternate author copy was available.

## A direct way around the scan

Let R=F_r[Y]/g(Y), with monic nonconstant g. Let v be any element of R, including a nonunit. Work in the finite algebra

    A=R[X]/(X^m-v).

The modulus is monic, so A is free of rank m over R and every element has a unique representative of X-degree<m. Expanding the binomial and applying X^(jm+e)=v^j X^e gives

    (X-1)^h mod (X^m-v) = sum_(e=0)^(m-1) F_e(v) X^e.

Therefore binary powering directly returns all mode evaluations. It never constructs the F_e coefficient vectors or walks their recurrence. This identity does not require m|h, m|(r-1), v invertible, or g squarefree. The implementation restricts r>=7, m>=2, canonical inputs and a caller-supplied component-slot cap; primality is caller-certified. The scalar source/orbit adapter has its separate eligibility conditions.

`power.rs` multiplies two m-component representatives and folds each exponent i+j>=m once by multiplying its coefficient by v. Since i,j<m, no second wrap is needed. Standard binary-power induction then proves the returned representative equals (X-1)^h. For h>0 it uses bit_length(h)-1+popcount(h) algebra products.

The dense multiplication used here makes exactly m^2 quotient products for coefficient pairs and m(m-1)/2 additional quotient products for wraps; zero terms are not specially skipped. Each quotient product includes reduction modulo g. Thus the explicit bound is O(m^2 log(r)) quotient products, plus additions, allocations/copies and modular division. Logical output storage is m*deg(g) base-field coefficients; the cap is not peak process memory. Full setup and peak temporary storage still count. Fast convolution or a flattened quotient implementation may change this cost, but is not implemented or timed here.

This is computation in finite quotient algebras over the original prime field, not transport of the ECDLP to an extension-field instance. It reuses the modular-character-power principle already present in ../feature-native-signed-image/orbit.rs (`character_residue`) and reference.rs (`pullback`). The new API returns residue-class sections for a supplied quotient value. No novelty claim.

## Exact controls and interpretation

Dense-reference controls independently build (X-1)^h by repeated multiplication, extract all residue classes, and evaluate them by Horner in R. Cases include nonunit v, repeated g factors and m not dividing h. A separate r=998244353,m=7 control verifies all seven sections at eight scalar quotient values by the weighted roots-of-unity identity

    sum_j zeta^((e-h)j) chi(x-zeta^j) = m*x^e*F_e(x^m).

The exact order of zeta=3^((r-1)/7) is checked. The mode1 polynomial would have71303168 coefficients; each section-power call uses34 products in a seven-component scalar quotient algebra. This is a source-derived operation count checked by instrumentation, not a timing comparison or a count of base-field multiplications. No large coefficient vector is built for this control. The fixture supplies scalar x only to construct and check the reference identity; the section evaluator accepts v and never receives x.

The next integration target is mode evaluation after candidate pruning, where deg(g) is small. Initial candidate construction, source sign extraction and candidate retention are not compressed by this API. Benchmark selection must charge m and deg(g), not compare log(r) algebra products directly with scalar operations. The separate signed/mode2 performance panel remains unrun.

## Formal proof scope and sources

Algebra.lean checks triangular composition, scalar normalization and one-wrap substitution over generic commutative rings using Lean4.34.1, bundled Std only. Axioms are propext, Classical.choice, Quot.sound. These small lemmas are not a full formalization of free quotient modules, residue-section identity, binomial recurrence or runtime refinement. Those remain written proofs and finite checks here. No separate lean4checker replay.

Sources: the primary recurrence paper above; predecessor signed-image/{orbit.rs,reference.rs,metered-polynomial.rs}; exact local signed recurrence derivation. KB unavailable (MCP absent, wrapper no executable); scoped source search used. The explicit block audit informs the change in implementation direction; it does not establish an impossibility result for compressed recurrence representations.
