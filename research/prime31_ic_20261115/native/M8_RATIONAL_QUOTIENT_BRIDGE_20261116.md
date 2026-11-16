# Prime31 m8 rational-base quotient bridge (Rust-only, 2026-11-16)

**Status:** checked-in Rust preprocessing/semantic experiment, **[pcc:unverified]**. No non-generic eight-summand root extraction or complete cheaper ECDLP claim. Source is `native/examples/prime31_m8_quotient_bridge.rs`.

## Why this is the next meaningful test

The completed published APS-style Rust control in [PR #4374](https://github.com/isogeny-labs/crypto/pull/4374) recovered 12/12 prime31 targets, but it solves S3 quadratics across explicitly enumerated labelled pairs. It is generic meet-in-the-middle equivalent. The [PR #4373](https://github.com/isogeny-labs/crypto/pull/4373) m8 chained-S3 effort ended with all target-conditioned F4 arms censored at the dense matrix cap. [#4228](https://github.com/isogeny-labs/crypto/pull/4228) and [#4349](https://github.com/isogeny-labs/crypto/pull/4349) provide native quotient/certification and selected-root infrastructure but not a completed 31-bit eight-summand root oracle. The selected-root constructor itself is **not** a target-dependent m8 solution and its optimization still lacks runtime validation.

Prior-art primary papers: Petit–Kosters–Messeng, *Algebraic approaches to ECDLP over prime fields* (PKC 2016, DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, *On DLP for prime-field elliptic curves* (2018, DOI 10.1016/j.ffa.2018.01.009). No new cryptanalytic complexity theorem or priority claim. Research-KB/Conductor/EKS inaccessible in this tool environment: bounded GitHub source and published primary papers are the fallback.

## Explicit algebraic representation

Work exclusively over the existing ordinary cofactor-one prime31 curve

    p=2147483647; E: y²=x³+847514846*x+2087444724;
    #E=n=2147446831 prime.

The depth-six Dickson fibre has 64 rational abscissae, of which the existing deterministic `Base::build` admits 31 with nonzero Fp ordinates. Let those admitted abscissae be `x_1,...,x_31`. The **squarefree** finite rational factor-base polynomial is

    H(X) = product_i (X-x_i), degree 31.

A point x lies in the prescribed original rational base iff `H(x)=0`, after the inherited sign/ordinate-lift convention. Hence the **rational coordinate** factor-base membership of *one leaf* can be represented with one degree-31 equation instead of six nested tower variables. For eight leaves the original tower form has 48 quadratic variables plus three auxiliary summation abscissae; the explicit-H presentation has eight leaf x variables plus the same three auxiliaries. This is an exact Fp-rational-root correspondence on the *admitted* base; it is **not** equality of the two polynomial ideals over algebraic closures, and it does not itself address y/sign lifting, identity intermediates or multiplicity.

Define an abscissa-only pair-support polynomial

    K(U) = product_(i<=j) S3(x_i,x_j,U).

Every off-diagonal factor is quadratic, and each diagonal factor is linear (the affine x of `2P_i`; inverse-pair identity has no affine x). Therefore `deg K = 2*C(31,2)+31=961`. Its finite rational roots are precisely the nonidentity x-coordinates of signed pair sums from the admitted factor-base points, with possible multiplicity. Building K uses only factor-base x-coordinates and Fp polynomial multiplication. Group addition in the new Rust example is **only** an independent post-construction semantic check for all 961 signed affine pair roots, never a discovery shortcut.

Group eight leaves as four signed pairs, giving four auxiliary pair abscissae `U_1,...,U_4`. The remaining target-dependent test is schematically

    K(U_1)=K(U_2)=K(U_3)=K(U_4)=0,
    S5(U_1,U_2,U_3,U_4,x_R)=0,

with signed point-lifting and full target `y_R` verification afterward, and exceptional intermediate identities handled separately. This reduces *number of variables*, not total finite support: `31^8=961^4=852891037441` ordered coordinate assignments. A naive dense quotient tensor of that dimension is infeasible. **The non-generic hypothesis is that selected-root, target-conditioned quotient/subresultant elimination can avoid materializing it**, with original-root equivalence certificates. Replacing it with group-sum lookups or an explicitly enumerated K-root join would merely recreate generic MITM.

## Rust-only gate

The new program reuses `prime31_algebraic_ic::{Arithmetic, Base, PRIME31, check_curve}`. It constructs H and K from public abscissae; asserts H is squarefree on the admitted x and rejects all 33 nonadmitted Dickson x; checks degree H=31, degree K=961, exactly 465 quadratic and 31 linear S3 factors, and independently verifies K vanishes at each of 961 signed pair-sum x values computed only after K construction. It checks the exact combinatorial dimension equality.

    cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_quotient_bridge
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_quotient_bridge

A focused Rust-only test/run step is added to `.github/workflows/prime31-s4-split-20261116.yml`. **No source-specific execution has yet been observed in this tool session.** The expected counts are assertions, not a claimed CI result.

## Next experiment: actual m8 target-conditioned selected-root quotient

1. **Representation and exceptional branches:** implement selected rational roots of `K(U)` as a certified original-input quotient, with explicit repeated-root and infinity branches. Test B=4,8 then B=31 without using EC additions in root discovery. Pair-support degrees are 16,64,961; do not assume the high-degree worker accepts them.
2. **Target-dependent four-way intersection:** create the compact S5 resultant/circuit, evaluate resultant/GCD in the small selected-root subalgebra, and require exact rational-root reconstruction and an original-polynomial certificate rather than materializing all `961^4` tuples.
3. **Proper IC-frontier benchmark:** same natural target encodings and arithmetic/accounting for PKM-style algebraic oracle, APS-style one-relation Rust control, native dense/tower F4, certified quotient, and external mature CAS if available. Incomplete/censored runs are not negative relation evidence or completed-call timing.
4. **Complete scalar solve:** only if naturally extracted relations/one-relation identities verify with full points, attempt twelve held-out unknown logs. Charge factor-base and quotient construction, each failed query, extraction, proof checking, and linear algebra. Compare optimized rho separately after IC-frontier comparison; independent PCC required for promotion.

## Decision

This tests an **exact representation bridge**, not a speedup. We now have a concrete path from 48 tower coordinates to an eight-leaf rational quotient, and a four-pair expression that fits the four-variable *interface count* of compact root selection. The gigantic quotient dimension is the explicit obstruction; only a certified target-conditioned selected-root method that avoids exhaustive support expansion can advance this lead. No m8 solve or faster-than-frontier outcome is inferred.


## Radical-quotient admission gate (new Rust source; execution pending)

Extended `prime31_m8_quotient_bridge.rs` to compute `gcd(K,K')` **over Fp using a Rust polynomial Euclidean algorithm**, directly from the field-constructed K961, before ever using group arithmetic for witness checks. A zero-degree gcd is the necessary and sufficient squarefreeness test for K. Earlier #4188 producer-local Python arithmetic reported `gcd(K,K')=1` for B31; this Rust gate provides a future source-bound check instead of silently reusing that Python result. At the moment of authoring, **the new Rust GCD assertion has not been observed to compile or execute on its new commit**; the prior passing H/K semantic check in [Actions 43788581330](https://github.com/isogeny-labs/crypto/actions/runs/43788581330) predates this addition and does not prove it.

If the new assertion passes, the quotient `Fp[U]/(K(U))` is reduced; since every S3 pair factor over the admitted rational curve points has all roots in Fp, it is a product of 961 copies of Fp (with distinct coordinate roots). That is **not an algorithmic speed win**: a fully split and enumerated pair-root quotient is mathematically just a generic MITM support catalogue. The scientifically interesting unresolved possibility is exploiting the **compact S3 product circuit and target-conditioned S5 identity** without decoding its 961 roots or expanding all `961^4` pair tuples. This distinction remains a hard admissibility criterion for any non-generic future solver.
