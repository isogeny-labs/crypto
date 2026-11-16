# Moment-to-root conversion and shrinking quotient updates

T-1793, 2026-11-07. Producer derivation/software checks; unsigned and not independently verified.

## Conversion

Let I(X) be the exact aggregate indicator in F_r[X]/(X^r-X), with candidate set S and m=|S| after at least one observed sign. Then m<=h=(r-1)/2. For 1<=j<=m, the previous coefficient identity gives the candidate power sums

    mu_j = sum_(t in S) t^j = -[X^(r-1-j)]I.

No constant-coefficient boundary is crossed since m<=h<r-1. Define q_0=1 and

    q_j = -(1/j) sum_(i=1)^j mu_i*q_(j-i),  1<=j<=m.

Newton identities imply Q(Z)=sum q_j Z^j=product_(t in S)(1-tZ). Reversing the m+1 coefficients yields the monic G(X)=product_(t in S)(X-t). All denominators1..m are invertible modulo caller-certified prime r. Zero candidates are included: their factor in Q has degree0 but the fixed m+1 reversal correctly supplies X in G. If m=0, G=1 represents the empty set. Conversion before the first constraint is rejected.

The implementation reads m coefficients, performs m(m+1)/2 multiply-accumulate terms and m coefficient scalings, and allocates m+1 output coefficients. It reuses the existing dense state's inverse coefficients. After successful conversion, Hybrid replaces its Dense enum variant with Roots, dropping the dense coefficient vector and inverse array. Peak conversion storage still includes both representations. This is a statement about stored vectors, not measured process RSS.

## Further observations in the smaller quotient

The roots of G are distinct and rational because S is a subset of F_r. For a nonzero observed sign s at offset a,

    G_new = monic_gcd(G, (X+a)^h mod G - s).

For s=0, use monic_gcd(G,X+a). Exponentiation by squaring reduces every product modulo the current G; no degree-h polynomial is expanded. The gcd keeps exactly the roots satisfying the observation, with no repeated factors. Its degree is the exact candidate count, and its negative next-to-leading coefficient is the first moment. A linear result supplies the scalar; G=1 remains empty under all later observations. The public API cannot construct an arbitrary RootSet with unvalidated coefficients.

Hybrid switches only when the count is at most an explicit root_limit. It retains the aggregate state before that threshold. Later root-polynomial degrees never increase. Invalid offsets/signs are rejected before updates. root_limit=0 is rejected by Hybrid construction (the recovery wrapper reaches that check when recovery starts).

## Cost and direct-gcd comparison

At conversion degree m, the implemented Newton recurrence is O(m^2) scalar-field work, after the earlier dense aggregate work. A root update uses O(M(m) log r) modular-power work plus GCD cost G(m). The implementation uses ordinary Euclidean transitions with the existing Newton-division/Karatsuba polynomial engine; no half-GCD complexity is attributed to it. G(m)<=O(m*M(m)) is a conservative bound for this implementation. State is m+1 root coefficients plus polynomial temporaries. Source oracle work and scalar verification remain separately necessary.

The earlier direct-gcd method can start immediately with (X+a)^h-s for a nonzero first sign, or X+a for zero. It therefore avoids aggregate conversion altogether. The new route is a representation bridge for aggregate-derived moments, not a demonstrated improvement over direct gcd. A direct-first-polynomial control in the native tests matches every frozen transcript count and final singleton. It shares the same later gcd engine and is not an independent implementation. The next useful scaling task is to obtain the needed moments without materializing the dense indicator, or use the bridge where those moments are already available cheaply.

## Integration and checks

The generated projector prefix preserves the previous native code. Recovery substitutes Hybrid for Projector, adds the explicit root threshold and branch diagnostics, and preserves source validation, consecutive shifts, query cap and mandatory [candidate]P=Q verification. generation.json pins the original input and generated outputs. No target scalar label enters runtime. Current source word-size/prime/projection restrictions and original4563-case cohort remain unchanged.

validation-01 passes three selected methods,157 imported filtered.324 transcript-prefix checks over r5,7,11,31 reconstruct exact root polynomial coefficients and compare hybrid updates against both dense moments and test-only products of enumerated linear factors. Empty state, root0, repeats, contradictions, caps and invalid observations are covered. All20 frozen public-point cases recover with final verification. They execute178 root-state updates after conversion with threshold16. The one-query control remains unresolved. This count is a software branch diagnostic, not a timing comparison. Test/normal builds retain16/26 warnings; no failures.28 source/input hashes pinned; legacy normal-build exclusions retained.

## Sources checked and next work

KB wrapper unavailable. Read aggregate DERIVATION.md/lib.rs, saved CONTINUATION.md algebraic-shift-extractor summary, native polynomial divrem and recovery code. Standard Newton identities are documented by FLINT: https://flintlib.org/doc/fmpz_poly/_fmpz_poly_power_sums_to_poly.html . That API is over integers; the recurrence is used here over F_r only with denominators proven nonzero. Finite-field modular powers and gcd filtering: https://flintlib.org/doc/fmpz_mod_poly/getting-started.html . Reused standard algebra, no novelty claim. Shared arithmetic means no independent review.

Next investigate a direct power-sum interface with certified candidate count, reducing initial dense work. Faster Newton-series conversion is possible if m grows, but threshold16 does not justify that change here. Existing PR merge/ledger blockers remain separate from these local software checks; no cloud experiment or timing panel ran.
