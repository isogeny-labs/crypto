# Sequential constraints in the retained quotient ring

T-1852, 2026-11-07. Producer derivation and deterministic software checks; unsigned evidence [pcc:unverified].

## Construction

For caller-certified prime r, m>=2 dividing r-1, exact-order zeta and nonzero radius b, let h=(r-1)/2, A=m b^h, t=(b^m)^(-1). The existing compiled mode polynomials F_e and public source observations f_e give

    C0(Z) = A F0(tZ) - f0
    C2(Z) = f1^2 F2(tZ) - A f2 F1(tZ)^2  (m >= 3).

Previously both polynomials were built before either intersection. The new traced adapter applies C0 first, obtains the retained monic polynomial G, then constructs C2 only in F_r[Z]/(G). Subsequent C0 is likewise constructed modulo G. Existing full-polynomial constraints stay intact for reference adapters. The coordinate-field sign source and all protocol/collector artifacts are unchanged.

## Equality proof

The canonical projection to F_r[Z]/(G) preserves addition, scaling and multiplication. Hence computing residues of F1(tZ), F2(tZ), squaring the first residue and combining them yields C2 mod G. Since gcd(G,C)=gcd(G,C mod G), the final monic state equals the original sequential gcd state. The same argument handles later C0. The unit state stays unit and can return without building further polynomials.

When the initial domain remains implicit, construct only C0. For m<=h its leading coefficient is nonzero: F0 has degree floor(h/m), and the binomial coefficient at that exponent is nonzero because h<r; A and t are nonzero. Thus an identically zero C0 requires m>h. A divisor of r-1 larger than h is r-1 itself, making every selected mode constant. The explicit constant C2 fallback therefore covers every valid compiled mode in this case; inconsistent internal coefficient shapes return Arithmetic.

## Block Horner without expanded transformed modes

Write n=deg G>=1 and F(X)=sum_j X^(jn) F_j(X), deg F_j<n. Precompute powers t^i for 0<=i<n and B=(tZ)^n mod G. Since G is monic, B_i=-t^n G_i, so this step needs no length n+1 monomial. Process the borrowed coefficient chunks from highest to lowest via

    R <- (R B mod G) + F_j(tZ).

Induction over chunks proves R=F(tZ) mod G. Every live residue has at most n coefficients; each explicit multiplication output has at most 2n-1 coefficients. The input F is borrowed, not transformed into a full temporary vector. The reducer uses ceil(L/n)-1 multiplication/reduction pairs for L input coefficients, at most L coefficient-scaling multiplications and L additions, and 2n field multiplications to construct its powers and step. C2 reuses that context for both modes and adds one residue square/reduction. This bounds application-level temporaries; polynomial-kernel internal workspace is separate. No timing dominance is asserted: extra reductions may trade arithmetic for storage. For n=1 this becomes scalar Horner evaluation.

The first C0 and compiled coefficient vectors remain dense. Their streaming/recurrence representation, and the unchanged public-source grid cost, remain next construction targets.

## Deterministic validation

`validation01/receipt.json` pins the compiler, transitive sources and stdout. Native checks run with overflow checks enabled. `validation01/check.stdout` records:

- 1,440 scaled residue comparisons against full substitution then polynomial remainder: primes 7,17,97,257; n=1,2,3,8,17; six lengths; four seeds; t=1,2,r-1. Zero polynomials and G=Z^n included. Explicit product-length assertions remain enabled.
- 4,413 sequential mode-state comparisons against old full constraints for every m dividing r-1, r=7,11,13,17,31,97, every scalar and three radii applied sequentially.
- Six constant-mode controls with identically zero C0 and nonzero C2.
- Inherited 8,600 implicit-domain intersections and complete native arithmetic controls.
- 617 additional public recoveries with 5,929 source calls and zero unresolved; inherited 63 public comparisons with 511 source calls.

Known scalars and root enumeration occur only in checker fixtures. Runtime recovery receives public points and checks its recovered scalar against the target. The original 4,563-case cohort is unchanged. These are software correctness checks, not an admitted workload measurement or independent signed verification. No cloud execution. Historical full Cargo fixture/deadline failure remains; no unrelated full-suite rerun. No native compile/assertion failures in this round.

Replay from the repository root into a fresh directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-residue-constraints/validate-recovery.py /tmp/residue-replay-unique
```

## Sources checked and publication

KB unavailable in this runtime; exact predecessor `feature-native-implicit-domain/{modes.rs,implicit.rs,trace-adapter.rs}` and scalar `feature-native-hgcd-roots/polynomial.rs` inspected. Primary FLINT nmod_poly documentation https://flintlib.org/doc/nmod_poly/index.html supplies standard modular-polynomial arithmetic context. Block Horner and quotient-ring projection are established algebra, with no novelty claim. `generate.py` pins the copied predecessor files and changes only the traced mode-constraint call and checker entrypoint. Older reports and canonical graphs remain historical evidence; `flow.svg` depicts only this new construction.
