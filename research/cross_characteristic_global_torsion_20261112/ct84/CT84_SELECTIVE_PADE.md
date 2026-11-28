# CT84 — Selective 37-adic rational-function reconstruction (one public order-seven target)

**Status:** [pcc:unverified]. Bounded fixed-instance constructive recovery with exact integer arithmetic, **not** a large-order or cost-improving ECDLP method.

## Research question and result

CT82 proved that at the *fixed* degree-one prime v|37 of the global seven-torsion point field K=Q(T), reduction gives E(K)[7] ≅ E(F37)[7]. CT83 established that a naive 24-dimensional, bounded-coefficient global-element lift can need many source digits before *universal* uniqueness holds.

Can one recover just one rational expression for an encoded source target without scanning the seven scalar candidates? **For the public source generator P=(9,4) and encoded target Q=(27,9), yes—conditional on an explicitly specified, small numerator/denominator degree budget.** The program is not supplied with the unknown scalar and never enumerates a source multiple table. It reconstructs a short rational function from p-adic root data, then validates it globally by a separate, exact group-law identity.

Fix the CT80 degree-24 x-coordinate field L=Q(alpha), with alpha a root of psi_7(X), the fixed place v where alpha≡9 mod37, and the same characteristic-two place as CT80. Let a,b be the roots of psi_7 over Z/37^18Z reducing to 9 and 27 respectively; since psi_7 is squarefree mod37, both are uniquely Hensel-lifted.

Search for a rational expression R(X)=A(X)/B(X), with deg A≤4 and deg B≤3, from **only** the congruence

    A(a) - b B(a) ≡ 0 (mod 37^18),

where B(a) is a unit mod37. Its nine integral coefficients form a full-rank congruence lattice of dimension nine. An independently written exact-Fraction LLL with delta=3/4 returns the primitive vector

    A(X) = X^4 + 20 X^2 + 158 X + 21,
    B(X) = 4 X^3 - 4 X^2 - 40 X - 79.

The output is **certified globally**, rather than accepted merely because its digits fit: using the classical generalized-Weierstrass duplication formula for y²+y=x³−x²−10x−20,

    D(X) = (2y+1)² = 4X³−4X²−40X−79,
    M(X) = 3X²−2X−10,

we have the exact polynomial identity

    A(X) = M(X)² + (1−2X) D(X),     B(X)=D(X).

Therefore R(alpha) is the *actual global x-coordinate of [2]T*, not a spurious p-adic coincident element. At the fixed source place R(alpha) reduces to 27, and the global doubling ordinate reduces to 9. CT82's prime-to-37 injectivity then identifies the unique preimage of the encoded Q. The use of [2] in this paragraph is **after-the-fact verification of the discovered formula**, not an input label to the LLL recognizer.

## Characteristic-two specialization and known-answer test

At CT80's fixed binary prime, D(alpha)≡1 and A(alpha)≡1+alpha^4. Thus, in CT80's F_(2^12) polynomial basis, the target x-coordinate is **17**. Doubling the same oriented global point T=(alpha,Y) gives the target y-coordinate **Y+77**, recorded as the pair (77,1) in the established tower representation F_(2^24). The independent JavaScript implementation checks the binary group law, yielding precisely

    source P=(9,4)  -> target U=((2,0),(0,1))
    source Q=(27,9) -> target V=((17,0),(77,1))

The input scalar relation is then verified by target arithmetic. The negative source-root control rejects x=28 as an image of this recovered rational function. The opposite encoded ordinate (27,27) would require the global negative point, not a second independently chosen prime; this is not claimed as an independently reconstructed case here.

## Exact reproducibility

Files:
- `ct84_selective_pade.py`: reconstructs the division polynomial from CT80 recurrences, lifts two *input-conditioned* roots, builds the nine-dimensional lattice, performs rational-exact LLL, primitive-normalizes its shortest vector, and checks the exact algebraic duplication identity and binary residue.
- `ct84_independent.js`: separately reconstructs psi_7 with BigInt, performs root lifts, checks the recovered rational-function congruence and exact group-law identity, and implements the F_(2^12)/F_(2^24) target operations. **It does not independently run LLL**, so agreement certifies the result of the reduction, not the implementation of the reduction algorithm.

From repo root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct84
python3 -B "$D/ct84_selective_pade.py" > /tmp/ct84-py.json
python3 -O -B "$D/ct84_selective_pade.py" > /tmp/ct84-pyopt.json
node "$D/ct84_independent.js" > /tmp/ct84-js.json
node --no-opt "$D/ct84_independent.js" > /tmp/ct84-jsopt.json
cmp /tmp/ct84-py.json /tmp/ct84-pyopt.json
cmp /tmp/ct84-py.json /tmp/ct84-js.json
cmp /tmp/ct84-py.json /tmp/ct84-jsopt.json
printf '%s  %s\n' ca0abbaf20ab4c70bfe99eb3a4867c0b3e3285f58daf79142f46df445fd15195 /tmp/ct84-py.json | sha256sum -c
```

Local normal/optimized Python and normal/no-opt Node executions agreed byte-for-byte with the displayed digest, before commit. Independent exact-head CI is a separate gate.

## Boundaries, hypotheses and cost

- **Hypothesis selected before recovery:** numerator degree ≤4, denominator degree ≤3; the program does not discover this degree budget. Low degree here is consistent with doubling, and may not cover arbitrary encoded targets. Only the (27,9) target is actually reconstructed through LLL.
- **Important distinction from CT83:** 18 source digits suffice for one narrow, lucky rational-function relation followed by an exact certificate. This does **not** contradict the 78-digit information-capacity requirement for the *entire unrestricted 24-coefficient box* in CT83; no uniform uniqueness at 18 digits is claimed.
- **Algorithm limitations:** ordinary LLL can return a short spurious relation at low precision or fail to recover an existing relation; the exact validation prevents false acceptance but does not guarantee success. No certified success probability, coefficient-height search bound, general arbitrary-point method, all-seven selective-lift test, scaling across ell, or complexity separation is established.
- **Cost:** one 18-digit 37-adic root pair, a nine-dimensional exact LLL with rational Gram–Schmidt, division-polynomial construction, exact identity checking and binary arithmetic. This is a fixed small fixture, not measured against matched rho/IC. A portable full cold cost must include discovering degree budgets, computing psi_ell, constructing the number field, precision escalation, validating each candidate, and target DLP.
- **Safety/gates:** no paid EKS, GPU or cloud work, protected merge, PCC signature or benchmark claim. Native genus-2/3 and other-repo PRs remain untouched.

## Sources checked / prior art

- CT80, CT82 and CT83 reports in this repository, plus root PR #3877, native cryptanalysis PR #1523 and stacked PR #4074.
- Classical exact lattice basis reduction: Lenstra–Lenstra–Lovász (1982); [FLINT LLL documentation](https://flintlib.org/doc/algorithms/linear-algebra/lll.html), which explicitly describes algebraic-number integer-relation applications and the exact-vs-fast arithmetic tradeoff.
- [Espitau–Joux, Certified lattice reduction (2019)](https://arxiv.org/abs/1905.11743), which motivates independent certificates rather than treating numeric fits as exact global claims.
- Sage number fields and division polynomials already cited in CT80/83.
- **KB unavailable:** no shared `research-kb` tool loaded through this connector runtime, and no checked-out wrapper; the above repository reports and primary public literature were the fallback. No new mathematical novelty claim for LLL, Padé or division-polynomial constructions.

## Next gate

Seek a degree-budget-free input-uniform root recognizer for *all* seven oriented public targets at the **same** source and target primes; independently verify each global coordinate's torsion equations, determine exact height and precision budgets, then test increasing ell and compare full cold costs. Do not silently substitute group-law scalar labels during recognition.
