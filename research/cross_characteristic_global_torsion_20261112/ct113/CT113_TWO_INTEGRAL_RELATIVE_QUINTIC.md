# CT113 — Exact 2-integral relative quintic and three binary prime specializations

**2026-11-14. Scientific standing [pcc:unverified].** Fixed 11a1 eleven-torsion number field. This gives an explicit *integral reduction* of CT112's characteristic-zero relative quintic, not an arbitrary-input ECDLP transfer or speed result.

## Precise theorem

Use CT109's degree-12 line field M=Q(S), CT110's 2-maximal order R=Z[S,U] with U=(S^4+S+1)/2, and CT112's relative quintic

    H_T(Z) = Z^5 - S Z^4 + sum_{j=0}^3 C_j(S)/L · Z^j,

where all 48 coefficients were independently integer-certified and

    L = 4533818870072688401806183463043171997544122883665891108971666020
      = 4 * L_odd,  L_odd odd.

Set h(S)=S^4+S+1. Exact integer monic long division yields the unique h-adic expansion of each frozen polynomial C_j of degree at most 11:

    C_j(S) = a_{j,0}(S) + a_{j,1}(S) h(S) + a_{j,2}(S) h(S)^2,
    deg a_{j,k}<4.

**Certificate:** Every coefficient of every a_{j,0} is divisible by four, and every coefficient of every a_{j,1} is divisible by two. This holds over Z, not just as a pointwise finite-field congruence. Consequently

    C_j(S)/L = [a_{j,0}(S)/4 + (a_{j,1}(S)/2) U + a_{j,2}(S) U^2] / L_odd.

The right side belongs to R localized at 2, so all four formerly apparently nonintegral coefficients possess exact 2-integral representatives. **CT112's even denominator is not an obstruction to specialization once CT110's integral basis is used.**

## Binary residues in the actual separated algebra

CT110 proves R/2R = F16[s,u]/(s^4+s+1, u^3+1). Write omega=s^2+s, omega^2=omega+1. CT111 associates the fixed CT103 prime with u=1. The other primes are u=omega and u=omega^2.

Because L_odd is odd, its inverse is one modulo 2. In the F16 polynomial basis with s=0x2, f(s)=s^4+s+1 and omega=0x6, the **complete relative quintics**, in ascending coefficient order, are:

| Binary prime | [Z^0,Z^1,Z^2,Z^3,Z^4,Z^5] |
| --- | --- |
| u=1 (fixed CT103) | [9,6,3,1,2,1] |
| u=omega | [4,10,15,7,2,1] |
| u=omega² | [15,15,13,6,2,1] |

All three are **irreducible over F16**. Independent field code verifies:

    gcd(H, Z^16-Z)=1,
    gcd(H, Z^256-Z)=1,
    Z^(16^5) ≡ Z (mod H).

Therefore the binary x-coordinate residue degree remains 4×5=20, while the oriented ordinate requires the separate quadratic step to degree 40, as in CT103. None of this implies an inexpensive computational mapping from characteristic 59.

**Fixed-prime compatibility:** Python and independently coded BigInt JavaScript construct classical x([m]T) rational functions (m=1,…,5) from division-polynomial recurrences, then evaluate at CT103's degree-20 binary root t modulo g(t)=0x130027. The resulting five-element root product matches u=1's integral quintic coefficientwise in F_(2^20) under the actual S residue. The other binary prime specializations remain distinct.

## Independent tests and negative controls

- Python: integer h-adic monic divisions of the 48 frozen coefficients, exact divisibility, finite-field F16 irreducibility, and direct F_(2^20) root-product comparison.
- BigInt JavaScript: independent integer division, finite-field modular exponent/GCD, and independent F_(2^20) product.
- Mutation C_0(0) -> C_0(0)+1 fails required /4 integrality. Incorrect binary prime labels change the polynomial.
- Both optimized and ordinary execution paths use raising checks.

Reproduce from the root of the repository:

    D=research/cross_characteristic_global_torsion_20261112/ct113
    python3 -B "$D/ct113_integral_binary_quintic.py" > /tmp/ct113-py.json
    python3 -O -B "$D/ct113_integral_binary_quintic.py" > /tmp/ct113-pyopt.json
    node "$D/ct113_integral_binary_quintic_independent.js" > /tmp/ct113-js.json
    node --no-opt "$D/ct113_integral_binary_quintic_independent.js" > /tmp/ct113-jsopt.json
    cmp /tmp/ct113-py.json /tmp/ct113-pyopt.json
    cmp /tmp/ct113-py.json /tmp/ct113-js.json
    cmp /tmp/ct113-py.json /tmp/ct113-jsopt.json

## Consequence and next decisive gate

This establishes **the 2-integral degree-12/degree-5 binary specializations** at all three CT110 residue primes, explicitly including CT103's fixed prime. It does not select one of the ten oriented global lifts above arbitrary source Q; no DLP transfer is complete.

Next: construct a scalar-independent 59-adic source-prime-to-global-conjugate recognizer inside fixed degree-120 K=Q(T), with global y-certificate. Measure complete cold setup, 59-adic precision, integral polynomial arithmetic, target F_(2^40) operations and destination log recovery against matched optimized rho/BSGS and IC; do not benchmark only reduction.

## Scope, prior art and gates

Sources checked: CT102/103 fixed residue embeddings; CT105 automorphism torsor; CT109 exact minimal polynomial; CT110 2-maximal order; CT111 prime labels; CT112 exact relative quintic. PARI nfbasis local-order documentation (https://sources.debian.org/src/pari/2.15.2-1/src/functions/number_fields/nfbasis) provides existing general theory; no novelty claim for integral-basis algorithms. AGENTS.md and 2026-11-14 session log read. Research-KB MCP was not loaded, and its repo wrapper was unreachable without a checkout. Concurrent CT112, CT110 and cryptanalysis/autoresearcher branches were left untouched.

No paid EKS/Spot run, protected merge, leaderboard promotion, independent signed PCC, full Rust/Conductor run or ECDLP speedup claim. [pcc:unverified].
