# Degree-eight rational-pole and nonbranch linear-square screen

Frozen 16 November 2026 UTC before counted runs. Fix carrier masks 32280 and
35349, source masks 68 and 84, and `K=F_3[a]/(a^5+2a+1)`. This arm covers
`B=L(x-c)^m` for `m=2,4,6` and `B=LQ_i(x-c)^m` for `m=2,4`, where `L=x`
and `c∈K×`. The excluded `c=0` cases are branch-supported denominators
already screened in `DEGREE8_SPARSE_PROTOCOL.md`. An irreducible quadratic
`Q_i` has no K-rational root, so all retained linear factors are coprime
to the branch polynomial.

With no quadratic branch pole, `M=f/L` has degree ten. CRT gives one
numerator residue `A=(Bh mod M)` per branch assignment. The coefficient
`[x^9]A` must vanish to permit `deg A=8`; solve its polynomial equation
in `c`, with a full-field fallback if the polynomial is identically zero.
There are 243 branch assignments per shape.

With one quadratic branch pole, `M=f/(LQ_i)` has degree eight. Every
numerator is `A=(Bh mod M)+λM`, with `λ∈K×`. The polynomial-square leading
coefficient forces `λ` to be a nonzero square, giving 121 values. There
are 405 branch assignments and `405·242·121=11,859,210` logical candidates
per shape and cell. Necessary square evaluations at ten nonzero K-points
filter candidates. For fixed `i` and evaluation point `z≠c`, the square
class of `f(z)B(z)` is independent of `c` because `(z-c)^m` is a square;
the source precomputes this class once. At `z=c`, `B(z)=0`, so the
necessary condition is automatic and that point is skipped.

Every survivor receives the exact full identity
`f C²=B(A³+A²B+bB³)` plus degree and coprimality checks. All-zero
coefficient cases are enumerated, not discarded. Source-bound receipts
retain all counters. A positive hit requires independent map, subgroup,
GHS solve, matched rho and prior-work checks. A null result closes only
these five denominator shapes for the finite census.
