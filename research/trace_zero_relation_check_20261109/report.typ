#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Trace-zero relation systems beyond Frobenius orbits
2026-11-09 · Isogeny-labs/crypto · bounded structural check · unsigned local evidence

*Result:* A small per-coordinate field-trace elimination is available.
The full trace-zero Weil-restriction construction does not by itself
establish a smaller matched relation-finding system for ECC2K-130.
Twenty-two local unit tests pass; no solver benchmark or signed
verification is asserted.

#image("layouts.svg", width: 100%)

== Keep the trace maps distinct

For E₀: y²+xy=x³+1 and tau(x,y)=(x²,y²), group trace Sₙ sums n
Frobenius conjugate points and takes values in E(F₂).
The trace-zero carrier in Res(E) has dimension n−1.
At n=131, E(F₂) has four points and the trace is onto, so its kernel
has order r=680564733841876926932320129493409985129. This is already
the challenge subgroup.

The different bit Tr(x) characterizes 2E, not 4E.
Exact toy checks at n=3,5,7,9 give group-trace kernel sizes
1,11,29,127 and field-x-trace kernel sizes 2,22,58,254.
These membership conditions are not interchangeable.

In Z[tau], tau²+tau+2=0, and tau−1=bar(tau)² has norm four.
Gorla–Massierer's exact sequence maps E(F₂ⁿ) onto Tₙ using tau−1.
Since [4]=tau²(tau−1), this image is precisely cofactor clearing.
The audit checks the ring identity, independent point-count recurrence,
pinned degree-83/131 orders and toy image/kernel equality.

== Matched coordinates

Keep B={P in Tₙ: x(P) in V}, the target and arity fixed.
Let ell=dim(V), epsilon=rank(Tr restricted to V).
An all-affine S3 chain has m ell+(m−2)n variables.
Field-trace coordinates for summands and affine intermediates give
m(ell−epsilon)+(m−2)(n−1). This preserves legitimate affine-chain
relations. Infinity intermediates still need their own patterns.

In the challenge basis z¹³¹+z¹³+z²+z+1, field trace is supported at
positions 0 and 129. For V spanned by 1,z,…,z¹⁶, epsilon=1.
The figure gives derived layout counts. Nominal component rows remain
(m−1)n and degree bounds remain quadratic for m=2, cubic for a chain.
These are not instantiated Groebner systems or measured solving degrees.

If V already lies in the field-trace kernel, epsilon=0: no summand bit
disappears. Choosing a larger or different base is not a matched comparison.

== The full trace-zero formulation

Gorla–Massierer section 3 uses two coordinates per point, n−1 summands,
and 2n−1 equations in 2(n−1) variables. At q=2,n=131 the literal
factor base contains at most four abscissae/eight affine points.
Its 130-term sums occupy at most C(137,7)=153802846692 points.
A uniform target in the pinned r-point group has coverage below 2⁻⁹⁰.
This only concerns that fixed base and arity, not adaptive, larger,
nonlinear or orbit-expanded constructions.

At matched m=130 with two coordinates per summand, naive direct S₁₃₁
already has 260 coordinate variables. GM also has 260, with 261 equations
including 130 trace-membership equations.
A chained S3 representation has 17028 coordinate/intermediate variables
and 16899 nominal components. That change removes chain auxiliaries and
trades degree; it is not a trace-zero dimension gain.

Formal unreduced S₁₃₁ degree is 2¹²⁹ per argument. After reduction over
F₂, Boolean degree is bounded by the variable count. Do not turn a formal
degree bound into measured complexity. The paper fixes n in its
q-asymptotic advantage and reports exponential growth in n.
Its practical cases concern degrees 3 and 5.

== Scope and reproduction

Native command: cargo test --locked --manifest-path
research/trace_zero_relation_check_20261109/native/Cargo.toml
-- --nocapture --test-threads=1.
The small crate reuses production field arithmetic without rebuilding the
root library. Four mechanism checks plus 18 field tests pass locally.
checks.txt and SOURCES.sha256 retain output and dependencies.

No timing, relation yield, actual solving degree, full-DLP result or signed
independent verdict is established. A stronger claim needs an explicit
low-degree trace-zero chart on a matched base, exact polynomial construction,
lifting/infinity checks and a controlled solver comparison.
Existing checkpoint/EKS access blocks that separate benchmark.
Other nonlinear Z[tau] methods remain open.

Primary sources:
#link("https://arxiv.org/pdf/1405.1059")[Gorla–Massierer, Index Calculus
in the Trace Zero Variety, sections 2–3];
#link("https://arxiv.org/pdf/1403.0126")[their compression paper, section 3];
#link("https://sacworkshop.org/SAC20/files/preproceedings/18-IndexCalculus.pdf")[Galbraith–Granger–Merz–Petit, SAC 2020, introduction and section 5];
#link("https://arxiv.org/pdf/1503.08001")[Kosters–Yeo, Proposition 4.2].
README.md gives repository pointers and all comparison assumptions.

Knowledge-base tools and wrapper executable were unavailable; source fallback
is recorded in the session log. No novelty or universal impossibility claim.
