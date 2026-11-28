# Certified adaptive image recurrence

2026-11-07 · T-1820 · Derived algebra and exact native producer checks. [pcc:unverified]

## Construction and domain

Let r be caller-certified prime, 7 <= r <= u64::MAX/2. Let G be a monic squarefree polynomial of degree 0 <= n < r, splitting in F_r in the recovery application. Let m >= 2 divide r-1. Work in the quotient algebra A=F_r[X]/G, with B=X^m mod G. This is polynomial arithmetic over the original scalar field; no extension-field DLP is introduced.

Write the distinct images of G's roots under x -> x^m as v_1,...,v_t, with multiplicities w_i. Then 1 <= w_i <= n < r, so every w_i is nonzero in F_r. Define mu_j=Tr_A(B^j), starting at j=0 with mu_0=n, and D(Y)=product_i(Y-v_i). The output envelope is D(X^m)/X^((m-1)[D(0)=0]). The empty input G=1 gives D=1 and no moments.

Generate moment prefixes of lengths 2,4,8,..., capped at 2n. At each prefix compute its shortest monic linear recurrence C using Berlekamp-Massey. Evaluate C(B) modulo G by Horner arithmetic. Accept only when this remainder is zero. Inflate C with the existing capped zero-corrected pullback. No candidate roots are enumerated by this runtime.

## Why the certificate proves the exact image polynomial

For all j >= 0, mu_j=sum_i w_i v_i^j, using v_i^0=1 even at zero. D therefore annihilates the whole sequence. Conversely, any polynomial C annihilating the whole sequence satisfies sum_i w_i C(v_i)v_i^j=0 for j=0,...,t-1. The Vandermonde matrix on distinct v_i is invertible; w_i are nonzero; hence C(v_i)=0 for every i. The infinite sequence's minimal monic recurrence is exactly D, including its zero factor.

For any finite prefix, D is a valid recurrence of degree t (vacuously if the prefix is shorter than t), so a shortest prefix recurrence has degree at most t. If the quotient certificate C(B)=0 passes, evaluation at each root of G gives C(v_i)=0. Thus D divides C. Combined with deg C <= t and monicity this yields C=D. This is an exact stopping certificate, not a heuristic based on an unchanged recurrence degree.

At prefix length 2t, any recurrence C of degree l <= t supplies at least t consecutive recurrence equations. Written in forward-polynomial order, the first t equations are again the invertible Vandermonde system on w_i C(v_i); therefore D divides C. Thus the shortest recurrence is D and certification must pass by 2t moments. Under doubling, the stopping length L is at most 2n and strictly less than 4t, or equals 2 when t=1. A finite cap below the next doubling checkpoint can reject even when an intermediate prefix would suffice; no claim of optimal stopping under arbitrary caps is made.

Dropping mu_0 is incorrect when zero is an image: its contribution occurs only at index0. A named negative control over F7 uses roots {0,1,6}, m=2. The shifted sequence is constant2 and returns Y-1, omitting zero; its quotient certificate fails. The correct polynomial is Y(Y-1).

## Work and storage accounting

Let M(n) be polynomial multiplication cost; the existing Newton division backend reduces a degree-2n product modulo monic G in O(M(n)). Computing B costs O(M(n) log m). The trace functional is obtained from -T Q'/Q, Q=T^n G(1/T), using the first n root power sums, in O(M(n)). Sequential moment extension uses O(t) quotient products and O(nt) scalar pairing operations.

Berlekamp-Massey is restarted at geometrically growing prefix lengths. The total scalar recurrence work is O(t^2); this native implementation uses Fermat inversion on each nonzero discrepancy, adding O(t log r) scalar multiplications over all prefixes. Each candidate certificate costs its degree in quotient products. Prefix lengths sum to O(t), so all certificates together cost O(t M(n)). The resulting conservative core bound is O(M(n)(log m+t)+nt+t^2+t log r), with O(n+t) stored coefficient words plus polynomial-backend temporaries. These are derived bounds, not measured total-cost superiority.

Squarefree input validation, capped output construction of degree N, original-parent validation and divisibility checks remain charged separately. The previous blocked projection may be preferable when t is large; this path targets strongly colliding images. A selector still needs full implementation-level accounting. The moment-count cap is not a total RAM cap.

## Exact checks and evidence

The generated harness preserves the previous sequential/blocked/lcm controls. It checks the adaptive images and envelopes against their whole polynomials on all6,522 accepted subsets over r=7,11 and all eligible m, plus larger cases (r,n,m)=(65537,8,128),(65537,16,64),(65537,32,32). References construct factors by independent schoolbook multiplication. Every nonempty small case also corrupts the constant coefficient of D and requires certificate rejection. Accepted moment lengths are bounded against the independently counted distinct image set. Repeated-root, degree-r, moment cap and empty-input controls are explicit. [pcc:unverified]

The public adapter replaces only the envelope construction within retained-history replay. Its63 fixtures, schedules, source setup and stopping rules are inherited unchanged: two full F7/r13 panels with m=3,6; frozen20 F509/r479/m2 targets;17 prescribed F701/r683/m11 targets. Scalar labels occur only in test references. Every scalar, route and query field is checked against the lcm path. The original4,563-case cohort is unchanged. [pcc:unverified]

validation-01 passed before the named transient/empty/cap controls were added; its adaptive.rs and generated trace.rs snapshots are preserved. validation-02 is the final source check. Receipts contain compiler version, commands, exact stdout and dependency hashes. No timing comparison, cloud trial or independent signed verdict is supplied. [pcc:unverified]

## Literature and remaining work

Primary documentation: [FLINT nmod_poly recurrence section](https://flintlib.org/doc/nmod_poly/index.html) and [FLINT modular minimal-polynomial interface](https://flintlib.org/doc/fmpz_mod_poly/index.html). These document shortest sequence recurrences and their incremental computation. Berlekamp-Massey, trace projection and minimal-polynomial recovery are established methods; no novelty claim is made for them. The proof above binds finite-prefix acceptance to the exact quotient annihilation test in this recovery path. KB tools were unavailable.

Next: account for source cost, quotient certificate overhead and image density when selecting between lcm closure, blocked full-image reconstruction and adaptive recurrence. Blocked partial projection and faster recurrence synthesis can be investigated without changing the exact certificate.
