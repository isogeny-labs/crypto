# Implemented Miller recurrence with a Frobenius anchor

Status: implemented in recurrence.rs and integrated in lib.rs; native exact
checks and independent packet replay pass. No timing comparison was run.
The previous dense builder is retained unchanged as the reference.

Let r be an odd prime, h=(r-1)/2, and g=product(X+a) for a square-free feature
with at most three shifts. If g(0)=0, factor out X once: g=X u. Build u^h
and shift by h. Here degree(u)<=2, so degree(u^h)<=r-1. The empty feature is 1.
Otherwise g_0 is nonzero. Put F=g^h=sum f_n X^n, d=degree(g)<=3, f_0=g_0^h.
From g F'=h g' F, coefficient comparison gives

    n g_0 f_n = sum_{i=1}^{min(d,n)} ((h+1)i-n) g_i f_{n-i}.

For n not divisible by r, this determines f_n using at most d previous
coefficients. This is the standard J.C.P. Miller power recurrence. In this
characteristic, naive division by n is invalid at n=r. Since d*h<2r,
there is at most one such positive singular index. For degree<=2 there is none.

The missing coefficient is anchored by an identity valid over F_r:

    g F^2 = g^(2h+1) = g^r = g(X^r).

The coefficient of X^r on the right is g_1. On the left, the only terms
involving unknown f_r are 2 g_0 f_0 f_r. Everything else uses earlier
coefficients. Let

    R = sum_{i=0}^d g_i sum_{j=0}^{r-i} f_j f_{r-i-j},

omitting (i,j)=(0,0) and (0,r), so no f_r occurs. Then

    f_r = (g_1-R)/(2 g_0 f_0).

The denominator is nonzero because r is odd and g_0,f_0 are nonzero. The
recurrence resumes at r+1 and determines every remaining coefficient through
d*h. Induction below r, the exact anchor, and induction above r establish
equality with g^h. A single O(r) convolution slice suffices for the anchor.
Using precomputed inverses for residues 1..r-1, each column therefore has
O(r+log r) field work for bounded d; the log term covers initial powers or
inverses. A shared inverse table can be built in O(r) field/integer operations
for prime r. Retained coefficient storage remains O(r) per column, and the
dense elimination cost is unchanged. These are derived operation bounds,
not measured timings.

Do not differentiate only the reduced representative: formal differentiation
does not descend to F_r[X]/(X^r-X), since (X^r-X)'=-1. If F=P+(X^r-X)Q,
then the reduced differential equation has the extra term

    g P' - h g' P = g Q modulo (X^r-X).

Working with unreduced F until construction is complete avoids losing Q.

Literature: the recurrence matches FLINT's documented J.C.P. Miller formula:
https://fredrikj.net/fdoc/html/fmpz_poly/_fmpz_poly_pow_multinomial.html
(read 2026-11-05; canonical flintlib.org fetch and linked GitHub source fetch
failed). The page identifies integer exact division, not division in F_r,
and carries a notice about its ongoing documentation revision. The derivation
above supplies the finite-field argument explicitly; the characteristic-r
anchor is not asserted to follow from the integer implementation. No novelty
priority claim is made. The implementation and native comparisons below exercise this anchor,
including zero shifts and n=r boundary cases.


## Implemented domain and integration

Context accepts a caller-certified odd prime 3<=r<=u64::MAX/2 and an explicit
max_rows bound before allocating its shared inverse table. Labels contain
at most three distinct sorted canonical shifts. Invalid domains, duplicate or
unsorted shifts, and exceeded bounds are rejected. The constructor retains
its separate max_columns bound. These are resource limits, not primality
certification. Inverse checks reject arithmetic degeneracy but do not prove
that the caller supplied a prime.

The table uses inv[1]=1 and inv[k]=-(floor(r/k))*inv[r mod k] modulo r.
Each column constructs its degree-at-most-three g, factors out a zero shift,
computes f_0, runs the recurrence with one anchor when needed, then folds the
completed ordinary polynomial modulo X^r-X. Formal differentiation is never
applied as if it descended to the quotient. The zero-shift branch shifts the
result by h after powering the remaining factor.

Native field products use u128 intermediates; additions of canonical residues
fit u64 because r<=u64::MAX/2. Recurrence weights are formed in i128 before
canonical reduction. Each field column needs O(r+log r) field operations,
hence O(r) for fixed degree, after one O(r) inverse-table setup with integer
divisions. The dense reference uses quadratic polynomial products. For m
features the builder changes from O(m r^2) to O(m r+m log r+r) field work.
These are source-derived operation bounds, not measured wall-time gains or
bit-complexity claims. Coefficient matrices still occupy O(r m) slots and
elimination still costs O(r^2 m) with the unchanged reference algorithm.

Production elimination, back substitution and residual recomposition are
byte-identical to the previous implementation from `let mut matrix` through
the final return. Only the column-building block and module route change.
The old polynomial helpers remain available to native reference tests.

## Exact checks and artifact custody

One optimized Rust build with overflow-checks=yes and one bounded invocation
pass all 49 native tests, none filtered. Every square-free feature of degree
at most three is compared coefficient-by-coefficient for r in
{3,5,7,11,17,31,479}; q=min(8,r-1) except q=16 for r479. This is 1037 columns
and 571 singular anchors. All 697 reference columns over F479 match, including
455 nonzero-constant cubics crossing index r. Zero shifts, empty features,
small primes, invalid labels and allocation limits are included.

The complete constructor retains its q15 OutsideSpan control with observed
rank477. For q16 it gives observed rank479, 478 terms and the same 56196286
inner elimination updates. Its saved packet is byte-identical to the dense
packet. All479 point identities and eight public-source integrations pass.
The prior independent checker binary was hash-verified against its immutable
receipt and invoked on this newly saved packet. It confirms the polynomial
identity and all479 inputs without producer imports. Its existing corrupted-
packet rejection remains historical evidence and was not rerun here.

Twenty producer/reference source/data hashes verify before and after build.
The replay receipt binds checker source, checker binary and output packet.
Four compiler warnings remain in the raw log. The original4563-case cohort
is unchanged; no cloud job, comparative panel or new timing claim was made.

## Next task

The feature-column work now has a short recurrence, while Gaussian elimination
still treats the columns as unrelated dense vectors. Next investigate whether
these recurrences yield a usable structured linear system or reusable factor
without losing the q15 inconsistency control. Include construction, source,
decoder and independent verification when evaluating complete recovery cost.
