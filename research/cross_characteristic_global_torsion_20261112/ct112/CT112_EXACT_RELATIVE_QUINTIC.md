# CT112 — Exact relative quintic over the characteristic-zero eleven-torsion line field

**2026-11-14. Scientific standing: [pcc:unverified].** This is a characteristic-zero algebraic certificate and reproducible implementation, *not* online source-to-binary transfer, a new DLP algorithm, or a cold-cost speed result.

## Precise result

Use CT102–110's fixed elliptic curve E: y²+y=x³−x²−10x−20, nonzero global T∈E[11], x-coordinate α of degree 60, degree-12 line invariant

    S = sum_{m=1}^5 x([m]T),

and CT109's proved monic q(S) of degree 12. This experiment **constructs all four missing coefficients of the irreducible relative quintic** over Q(S), not just the source/binary reductions:

    H_T(Z) = prod_{m=1}^5 (Z - x([m]T))
           = Z^5 - S Z^4 + C_3(S)/L Z^3 + C_2(S)/L Z^2
                    + C_1(S)/L Z + C_0(S)/L.

Here C_j∈Z[S] have degree ≤11, their 48 integral coefficients, low-to-high, are frozen in `reconstructed.json`, and the positive common denominator is

    L = 4533818870072688401806183463043171997544122883665891108971666020.

This explicit polynomial defines Q(x(T))/Q(S), an **irreducible degree-five** extension by the CT106 [Q(x(T)):Q(S)]=5 theorem. Orientation of T still requires the separate quadratic step over the x-coordinate field.

The coefficient denominator has **v₂(L)=2**, so naïvely reducing the power-basis coefficients modulo 2 is not valid; CT110–111's 2-integral order and fixed binary-prime labeling remain necessary. Other factors of L must also be checked before any unramified specialization. This valuation is a property of this representation, not a lower bound on solving cost.

## How it was found, and what certifies it

Exploratory arithmetic formed ψ₁₁ and the five classical rational x-multiplication functions N_m(X)/D_m(X), then computed elementary-symmetric coefficients in F_p[X]/(ψ₁₁). Linear algebra against (1,S,...,S¹¹) at 179 usable primes produced candidate rational coefficient vectors by CRT and rational reconstruction. Prime 827 had a singular power-basis interpolation matrix and was excluded. Twenty-seven held-out primes 1151–1409 (when denominators were invertible) matched all 48 recovered coefficients. **These modular tests are not the proof.**

Define integral polynomial coefficients h_j(X) of

    prod_{m=1}^5 (D_m(X) Z - N_m(X)) = sum_{j=0}^5 h_j(X) Z^j.

Then B(X)=h_5(X) has degree 50 and A(X)=−h_4(X) has degree 51, with S=A(α)/B(α). Each C_j(X)=Σ_{i=0}^{11}c_{ji}X^i is the frozen integer polynomial. For every j=0,1,2,3 the implementations verify the **exact integer polynomial divisibility**

    ψ₁₁(X) | [ Σ_{i=0}^{11} c_{ji} A(X)^i B(X)^(11−i)
              − L h_j(X) B(X)^10 ]   in Z[X].

All four cross-differences have degree **561**, and all four exact **integer** quotients have degree **501**, with zero remainders. Since deg ψ₁₁=60 and it is irreducible, B(α)≠0, so these identities prove every coefficient of H_T over Q(S) unconditionally within CT102/106's exact field-degree results. No characteristic-zero coefficients are inferred solely from CRT.

Two structurally different verifiers:
- Python reuses CT109's independently verified integer division-polynomial source, recursively builds symmetric coefficients, then performs a separate exact integer long division; uses raising checks under `python -O`.
- BigInt JavaScript separately constructs the recurrences and enumerates all **32 root subsets**, then performs its own exact long division. It does not use the Python verifier or CT109's runtime.

The fixed source known-answer control reduces at p=59, S=32, giving coefficients `(27,19,45,31,27,1)`, exactly CT106. A one-unit mutation of a frozen constant coefficient changes a cross-difference by B^11; because ψ₁₁(19)=0 and B(19)≠0 mod59, that mutant **cannot** be divisible by ψ₁₁. Each verifier checks the source root and nonzero B(19) negative-control premises.

Developmental normal Python, optimized Python, normal Node and `--no-opt` Node output match byte-for-byte, SHA256

    28239819cd788d819e0fe0d8819dd850b520a2ea90acd9926f75d5fbff12c7ce

The committed exact-head Actions check is separate from local replay. Independent signed scientific PCC certification, Rust/Conductor full-suite audit and additional mixed/ramified divisors are not included in CT112.

## Reproduce

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct112
python3 -B "$D/ct112_exact_quintic.py" >/tmp/ct112-py.json
python3 -O -B "$D/ct112_exact_quintic.py" >/tmp/ct112-pyopt.json
node "$D/ct112_exact_quintic_independent.js" >/tmp/ct112-js.json
node --no-opt "$D/ct112_exact_quintic_independent.js" >/tmp/ct112-jsopt.json
cmp /tmp/ct112-py.json /tmp/ct112-pyopt.json
cmp /tmp/ct112-py.json /tmp/ct112-js.json
cmp /tmp/ct112-py.json /tmp/ct112-jsopt.json
printf '%s  %s\n' 28239819cd788d819e0fe0d8819dd850b520a2ea90acd9926f75d5fbff12c7ce /tmp/ct112-py.json | sha256sum -c
```

## Research consequence and next gates

The full **degree-12 / degree-5 characteristic-zero tower** is now explicit; a computationally meaningful online transfer still needs a p-adic source-coordinate-to-global-root recognizer, the correct **degree-10 orientation torsor**, and evaluation at CT111's fixed 2-integral binary prime. The quintic contains all five x-roots but deliberately loses the scalar orientation; simply solving H_T in Q(S) does not produce an online ECDLP-preserving cross-characteristic map.

Next: express H_T over CT110's 2-maximal basis rather than the nonmaximal S power basis, specialize it at CT111's fixed binary prime, and test whether a public Q source lift selects an oriented global conjugate **without precomputed scalar labels**. Only an actual online algorithm justifies full cold-cost measurement with number-field construction, p-adic precision, F_(2^40) arithmetic and source rho/BSGS/IC.

## Prior work and ownership

Checked root PR #3877, stacked #4102, CT102/105/106/109/110, CT108's separate basis lead, and recent cryptanalysis CT45/46 and xedni PR #1518. Consulted published PARI relative-number-field documentation, Magma number-field/relative-extension handbook, and Shoup's 1997 generic-group bounds. Research-KB MCP is not loaded and no checkout wrapper is available: **KB unavailable**, not an absence-of-prior-work claim. CT111 binary-prime orientation is independently in progress on parent #4102; CT112 changes only its own new directory/workflow and avoids its files. No EKS Spot or paid compute; no new speedup, protected merge or certification claim. [pcc:unverified].
