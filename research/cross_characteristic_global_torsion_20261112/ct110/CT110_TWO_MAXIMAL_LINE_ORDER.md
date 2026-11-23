# CT110 — Explicit 2-maximal line-field order and all three binary prime projectors

**2026-11-14; scientific standing [pcc:unverified].** This certifies a local integral-order construction in CT109's specific degree-12 field. It does **not** establish an efficiently computable arbitrary-input ECDLP transfer or select a global torsion orientation.

## Theorem

Let `M=Q(S)`, where `S=Σ_{m=1}^5 x([m]T)` for the fixed 11a1 nonzero eleven-torsion point, and let `q(S)=0` be CT109's exact monic irreducible degree-12 polynomial. Put

    h(Z)=Z^4+Z+1,                U=h(S)/2.

**The order R=Z[S,U] is 2-maximal.** More precisely:

1. U is an algebraic integer.
2. The twelve elements `S^i U^j` for 0≤i<4 and 0≤j<3 form an integral Z-basis of R, and `[R:Z[S]]=2^12`.
3. The residue algebra is

       R/2R = F2[s,u]/(s^4+s+1, u^3+1) ≅ F16 × F16 × F16.

   Thus there are **exactly three distinct degree-four primes over 2**, separated by the residue of U.
4. Since CT109 independently established `v2(disc(q))=24`, the lattice discriminant satisfies `v2(disc(R))=24-2·12=0`. This proves 2-maximality; no unstated 2-adic index remains.

This is an explicit alternative to the separately developed CT108 `(S,e2)` saturation program. It uses only the already-certified **single invariant S** and one half-integral correction.

## Exact integer identity — not a guessed local lift

Compute the unique h-adic polynomial expansion of CT109's q:

    q(Z) = h(Z)^3 + 2 b2(Z) h(Z)^2 + 4 b1(Z) h(Z) + 8 b0(Z).

The three integer polynomials, coefficients in **ascending** order, are:

    b0 = [-663823472279, -1120353714872, -519756762560, -62185700152]
    b1 = [ -44901452068,     2189696424,   -1015888018,    -77134740]
    b2 = [     -1082794,        -271564,         -2750,          -10]

The two implementations reconstruct `q` exactly over the integers and check all divisibility constraints. Substituting `h(S)=2U` into `q(S)=0` gives the monic equation

    U^3 + b2(S) U^2 + b1(S) U + b0(S) = 0.                  (∗)

This proves U is integral **globally**. The residue parities are `b2≡b1≡0` and `b0≡1` modulo 2, hence (∗) reduces to `u^3+1=0`.

## Exact lattice, multiplication closure and index

Because h is monic of degree four, the ordered polynomials `Z^i h(Z)^j`, 0≤i<4 and 0≤j<3, have consecutive distinct leading degrees 0,…,11 and leading coefficient one. Their transition matrix to `1,S,…,S^11` is unimodular. Replacing `h(S)^j` by `U^j=h(S)^j/2^j` multiplies the lattice determinant by

    2^(-4·0-4·1-4·2) = 2^(-12).

The Z-span of the twelve proposed elements is a **ring**: multiplication by S uses `S^4=2U−S−1`, and multiplication by U uses the monic cubic (∗); its coefficient polynomials have degrees ≤3. Both exact scripts also explicitly reduce and check each of the 24 generator-by-basis products modulo q. Therefore the span equals `R=Z[S,U]`, and its index over `Z[S]` is exactly `2^12`.

The discriminant-index identity together with CT109's previously certified valuation 24 makes the R-discriminant odd and proves that `R⊗Z₂` is the full local maximal order (finite étale `Z₂` algebra).

## Explicit binary prime idempotents

In `F16=F2[s]/(s^4+s+1)`, define `ω=s²+s=s^5`, so `ω²=s²+s+1` and `ω²+ω+1=0`. The roots of `u³+1` are the distinct elements `1,ω,ω²`.

The three *residue-algebra* idempotents are

    e_1   = u² + u + 1
    e_ω   = ω u² + ω² u + 1
    e_ω²  = ω² u² + ω u + 1.

They satisfy `e_i(1),e_i(ω),e_i(ω²)` = the respective coordinate unit vectors, hence `e_i²=e_i`, `e_ie_j=0` for i≠j and `Σe_i=1` in **R/2R**. They are not field idempotents in M itself. Both implementations check all nine evaluations over their own characteristic-two arithmetic.

The corresponding primes are described by `(2,U+1)`, `(2,U+S²+S)` and `(2,U+S²+S+1)` in R, each with residue degree four.

## Independent reproduction

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct110
python3 -B "$D/ct110_two_integral_order.py" > /tmp/ct110-py.json
python3 -O -B "$D/ct110_two_integral_order.py" > /tmp/ct110-pyopt.json
node "$D/ct110_two_integral_order_independent.js" > /tmp/ct110-js.json
node --no-opt "$D/ct110_two_integral_order_independent.js" > /tmp/ct110-jsopt.json
cmp /tmp/ct110-py.json /tmp/ct110-pyopt.json
cmp /tmp/ct110-py.json /tmp/ct110-js.json
cmp /tmp/ct110-py.json /tmp/ct110-jsopt.json
printf '%s  %s\n' 3b0fcd1fee2f85aaa9d45bb30a446c44171b8e3bcdda14b5e55b55cbd9d11f4a /tmp/ct110-py.json | sha256sum -c
```

Python uses integer monic polynomial division and a direct generator-closure certificate. Independent JavaScript uses BigInt monic quotient arithmetic and independently assembled binary-field projectors; neither imports the CT109 verifier. All four producer-local normal/optimized modes emitted the identical JSON digest **3b0fcd1fee2f85aaa9d45bb30a446c44171b8e3bcdda14b5e55b55cbd9d11f4a**. Their agreement does not replace independently signed PCC, exact-head CI or native/full-suite audit. Output `disc_v2_from_CT109` is a dependency on CT109, not a fresh CT110 discriminant recomputation.

## Scope and next fixed-prime gate

This **resolves the index/three-prime separation problem in the degree-12 field M** and strengthens the previous mod-2 failure of the monogenic S order with a concrete integral basis. It does not yet identify which of `u=1,ω,ω²` corresponds to the **fixed CT103 binary prime** in the common degree-120 oriented field: determining that label requires an actual 2-adic specialization of U (or an equivalent compatible prime-identification certificate). Nor does this provide the relative quintic over Q[S], the oriented degree-10 extension, or a scalar-blind map for public Q. These remain the decisive barriers.

Next: compute U's residue in the three already constructed CT101 binary primes from an actual 2-adic invariant lift, bind the correct projector to the fixed target and verify against CT108's independently constructed basis. Then extend to the relative quintic/oriented lift, and measure complete cold cost, without pretending prime separation itself reduces DLP complexity.

## Source audit

- Repo CT101/103/105/106/107/109 (fixed tower, prime images, characteristic-zero q, exact index), root #3877, stacked #4102, AGENTS.md and latest 2026-11-14 session log.
- Published arithmetic references: Magma [maximal-order creation](https://docs.magma-maths.org/GlobalFields/NumberFieldsAndOrders/creation.html) and [Montes algorithm](https://magma.maths.usyd.edu.au/magma/handbook/text/421). Both describe general existing order computation; this explicit example is **not** advertised as a novel general algorithm.
- **KB unavailable:** research-KB MCP not loaded, repository wrapper unreachable from this connector runtime. No general novelty claim.
- Kept concurrent CT108 and cryptanalysis/crypto-autoresearcher work untouched. No paid EKS/Spot launch, protected merge, ledger promotion, complete cold-speed advantage or signed PCC certification. [pcc:unverified]
